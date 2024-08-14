use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Ticket {
    prefix: Prefix,
    number: i32,
}

#[derive(Debug)]
pub enum TicketError {
    NumberNotAnInteger,
    NumberNotSixDigits,
    PrefixInvalid,
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
        }
    }
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

pub fn parse_ticket(input_string: &String) -> Result<Option<Ticket>, TicketError> {
    let input_string_no_spaces = input_string.replace(" ", "-");
    let parts = input_string_no_spaces.split("-");
    let parts_vec: Vec<&str> = parts.collect();
    // optionally check whether
    let mut ticket: Option<Ticket> = None;
    if parts_vec.len() == 2 {
        let prefix = match parts_vec[0].to_uppercase().as_ref() {
            "BUG" => Some(Prefix::BUG),
            "FEATURE" => Some(Prefix::FEATURE),
            _ => return Err(TicketError::PrefixInvalid),
        };

        let value = match parts_vec[1].parse::<i32>() {
            Ok(i) => {
                if i > 99999 && i < 1000000 {
                    Some(i)
                } else {
                    return Err(TicketError::NumberNotSixDigits);
                }
            },
            Err(_error) => return Err(TicketError::NumberNotAnInteger),
        };

        match (prefix, value) {
            (Some(a), Some(b)) => {
                ticket = Some(Ticket {
                    prefix: a,
                    number: b,
                })
            },
            _ => unreachable!(),
        };

    }
    Ok(ticket)
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
