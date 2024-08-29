use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Ticket {
    prefix: Option<Prefix>,
    number: Option<i32>,
}

#[derive(Debug, PartialEq)]
pub enum TicketError {
    NumberNotAnInteger,
    NumberNotSixDigits,
    PrefixInvalid,
    MissingPart,
}

impl fmt::Display for TicketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TicketError::NumberNotAnInteger =>
                write!(f, "Ticket number isn't an integer"),
            TicketError::NumberNotSixDigits =>
                write!(f, "Ticket number isn't six digits"),
            TicketError::PrefixInvalid =>
                write!(f, "Invalid Prefix, must be BUG/FEATURE"),
            TicketError::MissingPart =>
                write!(f, "Missing part of ticket name"),
        }
    }
}

impl Ticket {
    pub fn to_string(&self) -> String {
        let prefix: String;
        let number: String;
        match (self.prefix.as_ref(), self.number.as_ref()) {
            (Some(a), Some(b)) => {
                prefix = match a {
                    Prefix::FEATURE => String::from("FEATURE"),
                    Prefix::BUG => String::from("BUG"),
                    // None => unreachable!(),
                };
                number = b.to_string();

            },
            (None, Some(b)) => {
                prefix = String::from("");
                number = b.to_string();
            },
            _ => {unreachable!();}

        }
        format!("{prefix}-{}", number)
    }

    pub fn completed(&self) -> bool {
        match (self.prefix.as_ref(), self.number.as_ref()) {
            (Some(_a), Some(_b)) => {
                return true;
            },
            _ => {
                return false;
            }
        }
    }
}


#[derive(Debug, PartialEq)]
enum Prefix {
    FEATURE,
    BUG,
}

pub fn parse_ticket(input_string: &String) -> Result<Option<Ticket>, TicketError> {
    let input_string_no_spaces = input_string.replace(" ", "-");
    let parts = input_string_no_spaces.split("-");
    let parts_vec: Vec<&str> = parts.collect();
    // optionally check whether
    // let mut ticket: Option<Ticket> = None;
    let prefix: Option<Prefix>;
    let value: Option<i32>;

    if parts_vec.len() == 2 {
        prefix = match parts_vec[0].to_uppercase().as_ref() {
            "BUG" => Some(Prefix::BUG),
            "FEATURE" => Some(Prefix::FEATURE),
            _ => return Err(TicketError::PrefixInvalid),
        };

        value = match parts_vec[1].parse::<i32>() {
            Ok(i) => {
                if i < 1000000 && i > 99999 {
                    Some(i)
                } else {
                    return Err(TicketError::NumberNotSixDigits);
                }
            },
            Err(_error) => return Err(TicketError::NumberNotAnInteger),
        };
    } else if parts_vec.len() == 1 {
        prefix = None;
        value = match parts_vec[0].parse::<i32>() {
            Ok(i) => {
                if i < 1000000 && i > 99999 {
                    Some(i)
                } else {
                    return Err(TicketError::NumberNotSixDigits);
                }
            },
            Err(_error) => return Err(TicketError::NumberNotAnInteger),
        };
    } else {
        return Err(TicketError::MissingPart);
    };

    return Ok(Some(Ticket {
        prefix: prefix,
        number: value,
    }))


    // match (prefix, value) {
    //     (Some(a), Some(b)) => {
    //         return Ok(Some(Ticket {
    //             prefix: a,
    //             number: b,
    //         }))
    //     },
    //     _ => return Ok(None),
    // };
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    // use crate::storage::*;

    #[test]
    fn test_parse_ticket() {
//         assert_eq!(parse_ticket(&String::from("BUG-123456")), Ok(Some(Ticket {
//             prefix: Prefix::BUG,
//             number: 123456,
//         })));
//         assert_eq!(parse_ticket(&String::from("bug-123456")), Ok(Some(Ticket {
//             prefix: Prefix::BUG,
//             number: 123456,
//         })));
//         assert_eq!(parse_ticket(&String::from("BUG 123456")), Ok(Some(Ticket {
//             prefix: Prefix::BUG,
//             number: 123456,
//         })));
//         assert_eq!(parse_ticket(&String::from("123456")), Err(TicketError::MissingPart));
//         assert_eq!(parse_ticket(&String::from("BUG")), Err(TicketError::MissingPart));
//         assert_eq!(parse_ticket(&String::from("BUG-1234561")), Err(TicketError::NumberNotSixDigits));
//         assert_eq!(parse_ticket(&String::from("BUG-12345")), Err(TicketError::NumberNotSixDigits));
//         assert_eq!(parse_ticket(&String::from("BUG-12345a")), Err(TicketError::NumberNotAnInteger));
//         assert_eq!(parse_ticket(&String::from("BUD-123456")), Err(TicketError::PrefixInvalid));
    }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
