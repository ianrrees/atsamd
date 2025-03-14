//! Handling of BSP examples

use crate::error::{Error, Result};
use clap::Subcommand;
use git2::{Repository, Status};
use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::{File, copy, read_dir, read_to_string, remove_file};
use std::io::Write;
use std::path::PathBuf;
use toml::{Table, Value};

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Distribute examples amongst BSPs
    Distribute {
        /// Path to the examples, defaults to examples_dir in config file
        examples: Option<PathBuf>,
        /// Path to the BSPs, defaults to bsps_dir in config file
        bsps: Option<PathBuf>,
    },
}

/// Entry point for example management
pub fn run(config: Table, commands: &Commands) -> Result<()> {
    let example_config = config
        .get("example")
        .ok_or(Error::Other("No example section in TOML".to_string()))?;

    match commands {
        Commands::Distribute { examples, bsps } => {
            let examples_path = if let Some(path) = examples {
                path
            } else {
                let config_value = example_config.get("examples_dir").ok_or(Error::Other(
                    "No examples path specified in arguments or configuration".to_string(),
                ))?;
                let config_str = config_value
                    .as_str()
                    .ok_or(Error::Other("examples_dir is not a string".to_string()))?;
                &PathBuf::from(config_str)
            };

            let bsps_path = if let Some(path) = bsps {
                path
            } else {
                let config_value = example_config.get("bsps_dir").ok_or(Error::Other(
                    "No BSP path specified in arguments or configuration".to_string(),
                ))?;
                let config_str = config_value
                    .as_str()
                    .ok_or(Error::Other("bsps_dir is not a string".to_string()))?;
                &PathBuf::from(config_str)
            };

            clear_bsp_example_dirs(&bsps_path)?;

            distribute(&examples_path, &bsps_path, example_config)
        }
    }
}

/// Removes .rs files from BSP example dirs iff committed in Git
fn clear_bsp_example_dirs(bsps_path: &PathBuf) -> Result<()> {
    let repo = Repository::discover(bsps_path)?;

    for bsp_example_dir in read_dir(bsps_path)?.filter_map(|entry| {
        if let Ok(entry) = entry {
            let path = entry.path().join("examples");
            if path.is_dir() { Some(path) } else { None }
        } else {
            None
        }
    }) {
        let mut to_delete = Vec::new();
        let mut untracked_changes = false;

        for entry in read_dir(bsp_example_dir)? {
            let path = entry?.path();
            if path.is_file() && path.extension() == Some(&OsString::from("rs")) {
                let status = repo.status_file(path.as_path())?;
                if status.is_empty() || status.contains(Status::WT_DELETED) {
                    to_delete.push(path);
                } else {
                    untracked_changes = true;
                    if status.contains(Status::WT_NEW) {
                        eprintln!("{} is not committed", path.to_string_lossy());
                    } else if status.intersects(Status::WT_MODIFIED) {
                        eprintln!("{} has uncommitted changes", path.to_string_lossy());
                    } else {
                        eprintln!(
                            "{} has unhandled Git status {:?}",
                            path.to_string_lossy(),
                            status
                        );
                    }
                }
            }
        }

        if untracked_changes {
            eprintln!("Aborting");
            return Err(Error::Logged);
        } else {
            for path in to_delete {
                remove_file(path)?;
            }
        }
    }
    Ok(())
}

/// Distributes each source file the examples directory to the BSPs
///
/// A combination of the source file names and examples.toml determine which
/// BSPs the example applies to.  Filenames are of the form
/// `destination-example_name.rs` ; if `destination` matches a BSP directory
/// name then the file is simply copied as
/// `destination/examples/example_name.rs`` , otherwise then the file is
/// processed as a template and the config file is expected to contain an array
/// entry of the boards that it applies to:
///
/// ```toml
/// # n.b. the leading `example` refers to the `manage example` subcommand
/// [example.destination.example_name]
/// boards = ["some", "list", "of", "boards"]
/// ```
fn distribute(examples: &PathBuf, bsps_path: &PathBuf, examples_toml: &Value) -> Result<()> {
    // TODO error out if bsps isn't a directory

    // Generate a list of the BSPs based on directory names: ["metro_m0", ...
    let bsp_dirs: Vec<_> = read_dir(bsps_path)?
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    // file_name() returns None only if path terminates in `..`,
                    // and read_dir() won't yield those
                    Some(path.file_name().unwrap().to_owned())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Filter the example directory contents to get files with "rs" extension
    for rust_source_path in read_dir(examples)?.filter_map(|entry| {
        if let Ok(entry) = entry {
            let path = entry.path();
            if !path.is_file() {
                return None;
            }
            if let Some(extension) = path.extension() {
                if extension == "rs" {
                    return Some(path);
                }
            }
        }
        return None;
    }) {
        // Above filter means we know this is a file and therefore has a name:
        let source_name = rust_source_path.file_name().unwrap();

        // ...but perhaps it's not a UTF-8 name (required as it may select from
        // TOML)
        let example_target_name =
            rust_source_path
                .file_stem()
                .unwrap()
                .to_str()
                .ok_or(Error::Other(format!(
                    "Non-UTF8 characters detected in {:?}",
                    rust_source_path
                )))?;

        // We split the example names on the first hyphen to determine whether
        // they're generic -and need to refer to config for destinations- or
        // specific to just one BSP.

        let parts: Vec<&str> = example_target_name.splitn(2, "-").collect();

        if parts.len() != 2 {
            return Err(Error::Other(format!(
                "Example file {} doesn't conform to naming conventions",
                source_name.to_string_lossy()
            )));
        }

        let target = parts[0];
        let example_name = parts[1];

        let is_generic = !bsp_dirs.contains(&OsString::from(target));

        let boards = if is_generic {
            let example_config = examples_toml
                .get(target)
                .and_then(|list| list.get(example_name));

            let toml_array = example_config
                .and_then(|c| c.get("boards").and_then(|a| a.as_array()))
                .ok_or(Error::Other(format!(
                    "config entry for generic example `{}` doesn't exist or doesn't have a `boards` array",
                    rust_source_path.file_name().unwrap().to_string_lossy()
                )))?;

            let mut boards = Vec::new();
            for entry in toml_array {
                if let Some(s) = entry.as_str() {
                    boards.push(s)
                } else {
                    return Err(Error::Other(format!(
                        "Non-string entry in `boards` array for {example_name}: {:?}",
                        entry
                    )));
                }
            }

            boards
        } else {
            vec![target]
        };

        // Handlebars is designed around storing multiple templates at a time,
        // but here there doesn't really seem to be a need for that.
        let mut handlebars = Handlebars::new();

        if is_generic {
            let source = read_to_string(&rust_source_path)?;
            handlebars
                .register_template_string(example_name, source)
                .map_err(|err| {
                    eprintln!("Error while rendering {example_name} for {:?}:", boards);
                    eprintln!("{}", err);
                    Error::Logged
                })?;
        }

        for board in boards {
            if board.is_empty() {
                return Err(Error::Other(format!("Empty board name for {example_name}")));
            }

            // TODO make the examples directory?

            let rendered_path = bsps_path
                .join(PathBuf::from(board))
                .join(PathBuf::from("examples"))
                .join(PathBuf::from(example_name).with_extension("rs"));

            if is_generic {
                let mut data = BTreeMap::new();
                data.insert("bsp".to_string(), board);

                let rendered = handlebars.render(example_name, &data).map_err(|err| {
                    eprintln!("Error while rendering {example_name} for {board}:");
                    eprintln!("{}", err.reason());
                    Error::HBRender(err)
                })?;

                // TODO if there's an existing file, compare it for equality and
                // error out if we'd change the file

                let mut filebuf = File::create(rendered_path)?;
                filebuf.write_all(rendered.as_bytes())?;
            } else {
                copy(&rust_source_path, rendered_path)?;
            }
        }
    }

    Ok(())
}
