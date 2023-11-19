use crate::{base::BaseFlow, utils::RequestGetter};
use error_chain::error_chain;

use scraper::{Html, Selector};

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub struct Release {
    title: String,
    date: u64,
}

pub struct Releases {}

impl RequestGetter for Releases {}

impl Releases {
    // fn get_recent(&self) -> Result<Vec<Release>> {
    //     let mut res = reqwest::blocking::get("https://subsplease.org/rss/?t&r=1080")?;
    //     let mut body = String::new();

    //     res.read_to_string(&mut body)?;

    //     println!("Status: {}", res.status());
    //     println!("Headers:\n{:#?}", res.headers());
    //     println!("Body:\n{}", body);

    //     Ok((self.get_releases_from_torrent_response(body)))
    // }

    // fn get_releases_from_torrent_response(&self, pageBody: String) -> Vec<Release> {
    //     let mut releasesVec: Vec<Release> = Vec::new();

    //     let asd: std::io::Result<()> = self.parse_torrent_xml();

    //     return releasesVec;
    // }

    // fn parse_torrent_xml(&self) -> std::io::Result<()> {
    //     let file = File::open("file.xml")?;
    //     let file = BufReader::new(file); // Buffering is important for performance

    //     let parser = EventReader::new(file);
    //     let mut depth = 0;

    //     for e in parser {
    //         match e {
    //             Ok(XmlEvent::StartElement { name, .. }) => {
    //                 println!("{:spaces$}+{name}", "", spaces = depth * 2);
    //                 depth += 1;
    //             }
    //             Ok(XmlEvent::EndElement { name }) => {
    //                 depth -= 1;
    //                 println!("{:spaces$}-{name}", "", spaces = depth * 2);
    //             }
    //             Err(e) => {
    //                 eprintln!("Error: {e}");
    //                 break;
    //             }
    //             // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
    //             _ => {}
    //         }
    //     }

    //     Ok(())
    // }

    fn get_for_today(&self) -> bool {
        // let client: Client = reqwest::blocking::Client::new();

        // let response = reqwest::blocking::get("https://subsplease.org/")
        //     .unwrap()
        //     .text()
        //     .expect("Could not retrive Response");

        let res = self.get_request("https://subsplease.org/");

        // parse the HTML document
        let doc_body = Html::parse_document(&res);

        // select the table
        let tables = Selector::parse("#schedule-table").unwrap();

        for table in doc_body.select(&tables) {
            let titles = table.text().collect::<Vec<_>>();
            println!("{}", titles[0]);
        }

        return true;
    }
}

impl BaseFlow for Releases {
    fn start(&self) -> bool {
        // self.get_recent();
        self.get_for_today();

        return true;
    }
}
