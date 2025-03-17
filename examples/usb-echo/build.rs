use std::fs::{copy, create_dir};
use std::process::exit;

#[allow(dead_code)]
enum Target {
    Samd21,
    Samd51,
}

fn main() {
    #![allow(unused_mut)]
    #![allow(unused_assignments)]

    let mut target: Option<Target> = None;

    // TODO this should cache the last target selection somewhere, and if that
    // has changed error out and instruct the user to re-build.  The target
    // selection is made from .cargo/config.toml before build.rs is run, so the
    // compiled bin will wind up targeting whatever was selected on the
    // /previous/ build (perhaps just defaulting to the host architecture, which
    // errors).

    // TODO possible to grab memory.x and .cargo/config.toml from the _boards dependency?

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
