use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::fs;

#[derive(Debug, PartialEq)]
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


#[derive(Debug, PartialEq)]
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

fn lines_from_file(filename: impl AsRef<Path>) -> Option<Vec<String>> {
    let data = fs::read_to_string(filename).expect("Unable to read file");
    println!("{}", data);
    None
    // match File::open(filename) {
    //     Ok(file) => {
    //         let buf = BufReader::new(file);
    //         let vec = buf.lines()
    //             .map(|l| l.expect("Could not parse line"))
    //             .collect();
    //         Some(vec)
    //     },
    //     Err(e) => None,
    // }
}

pub fn lines_to_file() {
    let data = "Some data!";
    let mut f = File::create("/test.txt").expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
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
    match lines {
        Some(i) => search_vec(&i, &search_string),
        None => None,
    }
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    // use crate::storage::*;

    #[test]
    fn test_parse_ticket() {
        assert_eq!(parse_ticket(&String::from("BUG-123456")), Some(Ticket {
            prefix: Prefix::BUG,
            number: 123456,
        }));
        assert_eq!(parse_ticket(&String::from("bug-123456")), Some(Ticket {
            prefix: Prefix::BUG,
            number: 123456,
        }));
        assert_eq!(parse_ticket(&String::from("BUG 123456")), Some(Ticket {
            prefix: Prefix::BUG,
            number: 123456,
        }));
        assert_eq!(parse_ticket(&String::from("123456")), None);
        assert_eq!(parse_ticket(&String::from("BUG")), None);
        assert_eq!(parse_ticket(&String::from("BUG-1234561")), None);
        assert_eq!(parse_ticket(&String::from("BUG-12345")), None);
    }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
