use std::io::stdin;

use clap::Parser;
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::{choose, news, popular, releases, walls, MODE};

// Used for core functionality used by the respective paths - not idea but okay for now.
pub trait BaseFlow {
    fn start(&self) -> bool {
        return false;
    }

    fn inquire(&self, question: &str) -> String {
        let mut s: String = "".to_string();
        println!("{}", question);
        stdin().read_line(&mut s).expect("Did not enter a response");

        return s.trim().to_string();
    }

    fn inquire_num(&self, question: &str) -> usize {
        return self
            .inquire(&question)
            .to_string()
            .parse::<usize>()
            .expect("Can\'t parse input");
    }

    fn get_selected_mode(&self, mode_num: usize) -> MODE {
        match mode_num {
            0 => MODE::Choice,
            1 => MODE::Releases,
            2 => MODE::Popular,
            3 => MODE::News,
            4 => MODE::Walls,
            _ => panic!("Invalid selection. Quitting..."),
        }
    }

    fn exec_selected_mode(&self, mode: MODE) -> bool {
        let choice_path: Box<dyn BaseFlow> = match mode {
            MODE::Choice => Box::new(choose::Choice {}),
            MODE::Releases => Box::new(releases::Releases {}),
            MODE::Popular => Box::new(popular::Popular {}),
            MODE::News => Box::new(news::News {}),
            MODE::Walls => Box::new(walls::Walls {}),
            _ => panic!("Should not be here..."),
        };

        let result: bool = choice_path.start();

        if (!result) {
            println!("Ended abruptly...");
        } else {
            println!("Good bye");
        }

        result
    }
}
