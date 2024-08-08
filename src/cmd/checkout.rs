use crate::shell;
// use std::process::Command;
use git2::{ Repository, BranchType, Error };
use std::io::{stdin,stdout,Write};
use crate::branch::BranchKind;

fn get_branches(repo: &Repository, branch_type: &Option<BranchType>, branch_name: &String) -> Result<Vec<String>, Error> {

    let mut branch_names: Vec<String> = vec![];
    let lower_branch_name = branch_name.to_lowercase();

    if *branch_type == None || *branch_type == Some(BranchType::Local) {
        let local_branches = repo.branches(Some(BranchType::Local)).unwrap();
        let mut local_branch_names: Vec<String> = local_branches
            .flatten()
            .map(|x| x.0.name().unwrap().unwrap().to_string())
            .filter(|name| name.to_lowercase().contains(&lower_branch_name))
            .collect();
        branch_names.append(&mut local_branch_names);
    }
    if *branch_type == None || *branch_type == Some(BranchType::Remote) {
        let remote_branches = repo.branches(Some(BranchType::Remote)).unwrap();
        let mut remote_branch_names: Vec<String> = remote_branches
            .flatten()
            .map(|x| x.0.name().unwrap().unwrap().to_string())
            .filter(|name| name.to_lowercase().contains(&lower_branch_name))
            .collect();
        branch_names.append(&mut remote_branch_names);
    }

    Ok(branch_names)
}

pub fn run_with_ticket(branch_kind: &BranchKind, ticket_number: &String, branch_name: &String, origin_type: Option<BranchType>) -> anyhow::Result<()> {
    let branch_type_str = match origin_type {
        Some(BranchType::Local) => "local",
        Some(BranchType::Remote) => "remote",
        None => "all",
    };
    let branch_full_name = branch_kind.to_full_string_with_ticket(ticket_number, branch_name);
    println!("Looking for a {} branch for ticket {} with string \"{}\", in {} branches", branch_kind.to_string(), ticket_number, branch_full_name, branch_type_str);

    // dbg!(&branch_kind);
    let repo = Repository::open(".")?;
    let local_branch_names = get_branches(&repo, &origin_type, &branch_full_name)?;

    let num_of_branches_found = local_branch_names.len();
    if num_of_branches_found > 0 {
        for (i, x) in local_branch_names.iter().enumerate() {
            println!("[{:?}] {:?}", i, x)
        }
        let mut s=String::new();
        print!("Which branch to checkout: ");
        let _= stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        let branch_index: i32 = s.trim_end()
            .parse::<i32>()
            .expect("Not a valid integer");
        let target_branch = local_branch_names.get(branch_index as usize);
        // println!("{:?}", target_branch);

        match target_branch {
            Some(branch) => {
                shell::new!("git", "checkout", &branch).run_yorn()?;
            },
            None => {
            }
        }
    } else {
        println!("{}", String::from("Existing branches not found. Create new one?"));
        shell::new!("git", "checkout", "-b", &branch_full_name, "--no-track", branch_kind.to_full_string_origin(branch_name)).run_yorn()?;
    }


    Ok(())
}

#[allow(dead_code)]
pub fn run(branch_kind: &BranchKind, branch_name: &String, origin_type: Option<BranchType>) -> anyhow::Result<()> {
    let branch_type_str = match origin_type {
        Some(BranchType::Local) => "local",
        Some(BranchType::Remote) => "remote",
        None => "all",
    };
    let branch_full_name = branch_kind.to_full_string(branch_name);
    println!("Looking for a {} branch with string \"{}\", in {} branches", branch_kind.to_string(), branch_full_name, branch_type_str);

    // dbg!(&branch_kind);
    let repo = Repository::open(".")?;
    let local_branch_names = get_branches(&repo, &origin_type, &branch_full_name)?;

    for (i, x) in local_branch_names.iter().enumerate() {
        println!("[{:?}] {:?}", i, x)
    }
    let mut s=String::new();
    print!("Which branch to checkout: ");
    let _= stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    let branch_index: i32 = s.trim_end()
        .parse::<i32>()
        .expect("Not a valid integer");
    let target_branch = local_branch_names.get(branch_index as usize);
    // println!("{:?}", target_branch);

    match target_branch {
        Some(branch) => {
            shell::new!("git", "checkout", &branch).run_yorn()?;
        },
        None => {

        }
    }

    Ok(())
}
