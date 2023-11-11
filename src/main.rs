#![allow(unused)]

mod interactive;
mod releases;
mod base;

// Mode:
// (default) 1 - gives interactive mode.
// 2 for just today's anime
// 3 for popular anime from the current week
// 4 for popular anime news
// 5 for wallpapers

use log::{info, warn, error};
use env_logger;
use env_logger::{Builder, Target};
use std::collections::HashMap;
use std::str::FromStr;

enum MODES {
    Interactive,
    Releases,
    Popular,
    News,
    Wallpapers
}

fn get_type_of<T>(_: &T) -> &str {
    return std::any::type_name::<T>();
}

// type Test = MODES | ANOTHER;

fn main() {
    Builder::new()
    .target(Target::Stdout)
    .init();

    // log::error!("This error has been printed to Stdout");

    // env_logger::init();

    let mode_num = std::env::args()
        .nth(1)
        .expect("no pattern given")
        .parse::<u8>()
        .unwrap();

    println!("mode: {:?}", stringify!(mode_num));

    let selected_mode = match mode_num {
        1 => MODES::Interactive,
        2 => MODES::Releases,
        3 => MODES::Popular,
        4 => MODES::News,
        5 => MODES::Wallpapers,
        _ => MODES::Interactive,
    };

    handle_mode_path(selected_mode);    
}

fn start_interactive(){
    println!("whassp");
    // interactive::

    // let mode_num = std::env::args()
    //     .nth(1)
    //     .expect("no pattern given")
    //     .parse::<u8>()
    //     .unwrap();
}

fn handle_mode_path(selected_mode: MODES) {
    match selected_mode {
        MODES::Interactive => start_interactive(),
        // MODES::Releases => startInteractive(),
        // MODES::Popular => startInteractive(),
        // MODES::News => startInteractive(),
        // MODES::Wallpapers => startWallpaper(),
        _ => panic!("Should not be here")
    }
}

fn test_enum_stuff() {
    #[derive(Debug)]
    struct Coord {
        asd: String
    }

    enum SimpleEnum {
        FirstVariant,
        SecondVariant,
        ThirdVariant,
    }

    #[derive(Debug)]
    enum Location {
        Unknown,
        Anonymous,
        Known(Coord),
    }
    
    enum ComplexEnum {
        Nothing,
        Something(u32),
        LotsOfThings {
            usual_struct_stuff: bool,
            blah: String,
        }
    }
    
    enum EmptyEnum { }
    
    let coord = Coord {
        asd: String::from_str("asd").unwrap()
    };

    let wtf: Location = Location::Known(coord);
    let mut debug_str = String::from_str("WTF ").unwrap();

    info!("{:#?}", wtf);

    debug_str.push_str( get_type_of(&wtf));

    println!("{}", debug_str)
}