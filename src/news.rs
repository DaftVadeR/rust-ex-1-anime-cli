use crate::{base::BaseFlow, MODE};
use clap::Parser;

pub struct News {}
impl BaseFlow for News {
    fn start(&self) -> bool {
        let fake_articles = vec![
            "Article 1",
            "Article 2",
            "Article 3",
            "Article 4",
            "Article 5",
        ];

        for article in fake_articles {
            println!("{}", article);
        }

        return true;
    }
}
