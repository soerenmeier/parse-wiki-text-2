// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

extern crate parse_wiki_text;

mod test;
mod test_cases;

fn main() {
    let mut args = std::env::args();
    match args.nth(1) {
        None => return test::run_test(&Default::default()),
        Some(command) => match &command as _ {
            "file" => {
                if let Some(path) = args.next() {
                    if args.next().is_none() {
                        match std::fs::read_to_string(path) {
                            Err(error) => {
                                eprintln!("Failed to read file: {}", error);
                                std::process::exit(1);
                            }
                            Ok(file_contents) => {
                                println!(
                                    "{:#?}",
                                    parse_wiki_text::Configuration::default().parse(&file_contents)
                                );
                                return;
                            }
                        }
                    }
                }
            }
            "text" => {
                if let Some(wiki_text) = args.next() {
                    if args.next().is_none() {
                        println!(
                            "{:#?}",
                            parse_wiki_text::Configuration::default()
                                .parse(&wiki_text.replace("\\t", "\t").replace("\\n", "\n"))
                        );
                        return;
                    }
                }
            }
            _ => {}
        },
    }
    eprintln!("invalid use");
    std::process::exit(1);
}
