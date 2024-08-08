#[derive(Debug)]
pub enum BranchKind {
    Develop,
    Releases,
}

impl BranchKind {
    pub fn to_string(&self) -> String {
        match *self {
            BranchKind::Develop => String::from("develop"),
            BranchKind::Releases => String::from("releases"),
        }
    }

    pub fn to_full_string(&self, input_str: &String) -> String {
        match *self {
            BranchKind::Develop => format!{"{input_str}"},
            BranchKind::Releases => {
                format!("{}{input_str}", "releases/R6.0_1/BC12022/6.0_1-")
            },
        }

    }

    pub fn to_full_string_origin(&self, input_str: &String) -> String {
        match *self {
            BranchKind::Develop => format!{"{}{input_str}", "origin/"},
            BranchKind::Releases => {
                format!("{}{input_str}", "origin/releases/R6.0_1/BC12022/6.0_1-")
            },
        }

    }

    pub fn to_full_string_with_ticket(&self, ticket_number: &String, input_str: &String) -> String {
        match *self {
            BranchKind::Develop => format!("{}{}{}{input_str}", "users/atai/", ticket_number, "/"),
            BranchKind::Releases => {
                format!("{}{}{}{input_str}", "users/atai/", ticket_number,"/releases/6.0_1-")
            },
        }

    }
}
