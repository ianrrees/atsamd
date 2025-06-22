//! Handling of BSP examples

use crate::error::{Error, Result};
use clap::Subcommand;
use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::fs::{File, copy, read_dir, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use toml::Table;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Distribute examples amongst BSPs
    Distribute {
        /// Path to the examples
        examples: String,
        /// Path to the BSPs
        bsps: String,
    },
}

/// Entry point for example management
pub fn run(commands: &Commands) -> Result<()> {
    match commands {
        Commands::Distribute { examples, bsps } => distribute(examples, bsps),
    }
}

fn distribute(examples: &String, bsps: &String) -> Result<()> {
    let toml = read_to_string(PathBuf::from(examples).join("examples.toml"))?;

    let examples_toml = toml.parse::<Table>()?;

    // TODO error out if this isn't a directory
    let bsps_path = PathBuf::from(bsps);

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

        // ...but perhaps it's not a UTF-8 name (required as it selects from TOML)
        let example_target_name =
            rust_source_path
                .file_stem()
                .unwrap()
                .to_str()
                .ok_or(Error::Other(format!(
                    "Non-UTF8 characters detected in {:?}",
                    rust_source_path
                )))?;

        // We split the example names on the first hyphen, to determine whether
        // they're generic (and need to refer to example.toml for destinations)
        // or specific to just one BSP.

        let parts: Vec<&str> = example_target_name.splitn(2, "-").collect();

        if parts.len() != 2 {
            return Err(Error::Other(format!(
                "Example file {} doesn't conform to naming conventions",
                source_name.to_string_lossy()
            )));
        }

        let target = parts[0];
        let example_name = parts[1];

        let is_generic = target == "generic";

        let example_config = examples_toml
            .get("examples")
            .and_then(|list| list.get(example_name));

        let boards = if is_generic {
            let toml_array = example_config
                .and_then(|c| c.get("boards").and_then(|a| a.as_array()))
                .ok_or(Error::Other(format!(
                    "examples.toml entry for generic example `{example_name}` doesn't have a `boards` array"
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

            // TODO make the examples directory

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

                // TODO if there's an existing file, compare it for equality and error out if
                // we'd change the file

                let mut filebuf = File::create(rendered_path)?;
                filebuf.write_all(rendered.as_bytes())?;
            } else {
                copy(&rust_source_path, rendered_path)?;
            }
        }
    }

    Ok(())
}
