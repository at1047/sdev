use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub struct Ticket {
    prefix: Prefix,
    number: i32,
}

impl Ticket {
    pub fn to_string(&self) -> String {
        let prefix = match self.prefix {
            Prefix::FEATURE => String::from("FEATURE"),
            Prefix::BUG => String::from("BUG"),
        };
        let number = self.number.to_string();
        format!("{prefix}-{}", &number)
    }
}


#[derive(Debug)]
enum Prefix {
    FEATURE,
    BUG,
}

pub fn parse_ticket(input_string: &String) -> Option<Ticket> {
    let input_string_no_spaces = input_string.replace(" ", "-");
    let parts = input_string_no_spaces.split("-");
    let parts_vec: Vec<&str> = parts.collect();
    // optionally check whether
    let mut ticket: Option<Ticket> = None;
    if parts_vec.len() == 2 {
        let prefix = match parts_vec[0].to_uppercase().as_ref() {
            "BUG" => Some(Prefix::BUG),
            "FEATURE" => Some(Prefix::FEATURE),
            _ => None,
        };

        let value = match parts_vec[1].parse::<i32>() {
            Ok(i) => {
                if i > 99999 && i < 1000000 {
                    Some(i)
                } else {
                    None
                }
            },
            Err(_error) => None,
        };

        match (prefix, value) {
            (Some(a), Some(b)) => {
                ticket = Some(Ticket {
                    prefix: a,
                    number: b,
                })
            },
            _ => {},
        };

    }
    ticket
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn search_vec(file_vec: &Vec<String>, search_string: &String) -> Option<Ticket> {
    let mut ticket: Option<Ticket> = None;
    for line in file_vec {
        if line.contains(search_string) {
            let parts = line.split("-");
            let parts_vec: Vec<&str> = parts.collect();
            if parts_vec.len() == 2 {
                let prefix = match parts_vec[0] {
                    "BUG" => Prefix::BUG,
                    "FEATURE" => Prefix::FEATURE,
                    _ => unreachable!(),
                };
                let value = parts_vec[1].parse::<i32>().unwrap();
                ticket = Some(Ticket {
                    prefix: prefix,
                    number: value,
                });
                // dbg!(ticket);
            }
        }
    }
    ticket
}

pub fn search_storage(search_string: &String) -> Option<Ticket> {
    let lines = lines_from_file("ticket_names.txt");
    search_vec(&lines, &search_string)
}


#[allow(dead_code)]
pub fn search_storage_from_file(search_string: &String) -> Option<Ticket> {
    let file = File::open("ticket_names.txt");

    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut ticket: Option<Ticket> = None;

            for line in reader.lines() {
                match line {
                    Ok(unwrapped_line) => {
                        if unwrapped_line.contains(search_string) {
                            let parts = unwrapped_line.split("-");
                            let parts_vec: Vec<&str> = parts.collect();
                            // optionally check whether
                            if parts_vec.len() == 2 {
                                let prefix = match parts_vec[0] {
                                    "BUG" => Prefix::BUG,
                                    "FEATURE" => Prefix::FEATURE,
                                    _ => unreachable!(),
                                };
                                let value = parts_vec[1].parse::<i32>().unwrap();
                                ticket = Some(Ticket {
                                    prefix: prefix,
                                    number: value,
                                });
                                // dbg!(ticket);
                            }
                        }
                    },
                    Err(_error) => {},
                }

            }
            ticket

        },
        Err(_error) => None,
    }

}
