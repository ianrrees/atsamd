use std::process::exit;

fn main() {
    #![allow(unused_mut)]

    let mut boards_selected = 0;

    #[cfg(feature = "metro_m0")]
    {
        boards_selected += 1;
    }

    #[cfg(feature = "feather_m0")]
    {
        boards_selected += 1;
    }

    if boards_selected != 1 {
        eprintln!("samd21_boards requires that exactly one board be selected via a feature eg --feature=\"metro_m0\"");
        exit(1);
    }
}
