use crate::shell;
// use crate::shell::Colors;
use git2::{ Repository, BranchType, Error };
use std::io::{stdin,stdout,Write};
use crate::branch::BranchKind;
use crate::ticket::*;
use std::process::exit;

fn get_branches(repo: &Repository, branch_type: &Option<BranchType>, branch_name: &String) -> Result<Vec<String>, Error> {

    let mut branch_names: Vec<String> = vec![];
    let lower_branch_name = branch_name.to_lowercase();

    if *branch_type == None || *branch_type == Some(BranchType::Local) {
        let local_branches = repo.branches(Some(BranchType::Local)).unwrap();
        let mut local_branch_names: Vec<String> = local_branches
            .flatten()
            .map(|x| x.0.name().unwrap().unwrap().to_string())
            .filter(|name| name.to_lowercase().contains(&String::from("users/atai")))
            .filter(|name| name.to_lowercase().contains(&lower_branch_name))
            .collect();
        branch_names.append(&mut local_branch_names);
    }
    if *branch_type == None || *branch_type == Some(BranchType::Remote) {
        let remote_branches = repo.branches(Some(BranchType::Remote)).unwrap();
        let mut remote_branch_names: Vec<String> = remote_branches
            .flatten()
            .map(|x| x.0.name().unwrap().unwrap().to_string())
            .filter(|name| name.to_lowercase().contains(&String::from("users/atai")))
            .filter(|name| name.to_lowercase().contains(&lower_branch_name))
            .collect();
        branch_names.append(&mut remote_branch_names);
    }

    Ok(branch_names)
}

pub fn run_with_ticket(branch_kind: &BranchKind, input_ticket_name: &String, branch_name: &String, origin_type: Option<BranchType>) -> anyhow::Result<()> {

    // shell::new_colorful!("git", ("branch", Colors::FG_RED), ("-a", Colors::FG_BLUE)).run_yorn_colorful();

    let branch_type_str = match origin_type {
        Some(BranchType::Local) => "local",
        Some(BranchType::Remote) => "remote",
        None => "all",
    };

    let input_ticket = match parse_ticket(&input_ticket_name) {
        Ok(ticket) => ticket,
        Err(error) => {
            println!("Problem parsing ticket name: {error}");
            exit(1)
        },
    };

    let input_ticket_full_name = match input_ticket {
        Some(ref a) => Some(a.to_string()),
        None => None,
    };

    // let input_ticket_full_name = &input_ticket.unwrap().to_string();
    // println!("input_ticket_full_name: {:?}", input_ticket_full_name);

    // let branch_full_name = branch_kind.to_full_string_with_ticket(&input_ticket_full_name, branch_name);
    let branch_parsed_name = match input_ticket_full_name {
        Some(a) => Some(branch_kind.to_full_string_with_ticket(&a, branch_name)),
        None => None,
    };

    let repo = Repository::open(".")?;
    let local_branch_names: Vec<String>;
    match branch_parsed_name {
        Some(ref a) => {
            println!("Looking for a {} branch with string \"{}\", in {} branches", branch_kind.to_string(), &a, &branch_type_str);
            local_branch_names = get_branches(&repo, &origin_type, &a).unwrap();
        },
        None => {
            println!("Looking for a {} branch with string \"{}\", in {} branches", branch_kind.to_string(), &branch_name, &branch_type_str);
            local_branch_names = get_branches(&repo, &origin_type, &branch_name).unwrap();
        }
    };

    let num_of_branches_found = local_branch_names.len();
    if num_of_branches_found > 0 {
        for (i, x) in local_branch_names.iter().enumerate() {
            println!("[{:?}] {:?}", i, x)
        }
        let mut s=String::new();
        print!("Which branch to checkout: ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        let branch_index: i32 = match s.trim_end()
            .parse::<i32>() {
            Ok(i) => i,
            Err(_error) => {
                println!("Not a valid branch option, exiting...");
                exit(1)
            }
        };
        let target_branch = local_branch_names.get(branch_index as usize);

        match target_branch {
            Some(branch) => {
                shell::new!("git", "checkout", &branch).run_yorn()?;
            },
            None => {
            }
        }
    } else if !branch_parsed_name.is_none() {
        match input_ticket {
            Some(a) => {
                if a.completed() {
                    println!("{}", String::from("Existing branches not found. Create new one?"));
                    let full_name = branch_kind.to_full_string_local(&a.to_string(), branch_name);
                    shell::new!("git", "checkout", "-b", &full_name, "--no-track", branch_kind.to_full_string_origin(branch_name)).run_yorn()?;

                } else {
                    println!("{}", String::from("Ticket name not valid, can't create new branch"));
                }
            },
            None => {
            }
        }
    } else {

    }


    Ok(())
}
