use std::fs::{copy, create_dir};
use std::process::exit;

#[allow(dead_code)]
enum Target {
    Samd21,
    Samd51,
}

fn main() {
    #![allow(unused_mut)]

    let mut boards_selected = 0;
    #[allow(unused_assignments)]
    let mut target: Option<Target> = None;

    // n.b. if the feature name were instead like board_metro_m0 then this could be generic

    #[cfg(feature = "metro_m0")]
    {
        boards_selected += 1;
        target = Some(Target::Samd21);
    }

    #[cfg(feature = "feather_m0")]
    {
        boards_selected += 1;
        target = Some(Target::Samd21);
    }

    #[cfg(feature = "feather_m4")]
    {
        boards_selected += 1;
        target = Some(Target::Samd51);
    }

    if boards_selected != 1 {
        eprintln!("Exactly one board must be selected via a feature eg --feature=\"metro_m0\"");
        exit(1);
    }

    // TODO this should cache the last target selection somewhere, and if that
    // has changed error out and instruct the user to re-build.  The target
    // selection is made from .cargo/config.toml before build.rs is run, so the
    // compiled bin will wind up targeting whatever was selected on the
    // /previous/ build (perhaps just defaulting to the host architecture, which
    // errors).

    if let Some(target) = target {
        match create_dir(".cargo") {
            Ok(()) => {}
            Err(ref err) if err.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(err) => {
                eprintln!("Error creating .cargo: {:?}", err);
                exit(1);
            }
        }

        match target {
            Target::Samd21 => {
                copy("memory.x.samd21", "memory.x").expect("Failed to copy file");
                copy("config.toml.samd21", ".cargo/config.toml").expect("Failed to copy file");
            }
            Target::Samd51 => {
                copy("memory.x.samd51", "memory.x").expect("Failed to copy file");
                copy("config.toml.samd51", ".cargo/config.toml").expect("Failed to copy file");
            }
        }
    }
}
