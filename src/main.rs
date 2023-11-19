use base::BaseFlow;

mod base;
mod choose;
mod news;
mod popular;
mod releases;
mod utils;
mod walls;

#[derive(Copy, Clone)]
pub enum MODE {
    Choice,
    Releases,
    Popular,
    News,
    Walls,
}

impl MODE {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

fn main() {
    println!("Welcome to the Anime helper!\n\n");
    let start_path: Box<dyn BaseFlow> = Box::new(choose::Choice {});

    start_path.start();
}
