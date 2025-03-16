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

    if let Some(target) = target {
        match target {
            Target::Samd21 => {
                // TODO copy Samd21 files
            }
            Target::Samd51 => {
                // TODO copy Samd51 files
            }
        }
    }
}