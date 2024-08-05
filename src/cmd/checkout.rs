// use crate::shell;
// use std::process::Command;
use git2::{ Repository, BranchType, Error };
use std::io::{stdin,stdout,Write};

fn show_branch(repo: &Repository, branch_type: &BranchType, branch_name: &String) -> Result<Vec<String>, Error> {
    // let head = match repo.head() {
    //     Ok(head) => Some(head),
    //     Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
    //         None
    //     }
    //     Err(e) => return Err(e),
    // };


    let local_branches = repo.branches(Some(*branch_type)).unwrap();
    let local_branch_names: Vec<String> = local_branches
        .flatten()
        .map(|x| x.0.name().unwrap().unwrap().to_string())
        .filter(|name| name.contains(branch_name))
        .collect();
    // println!("{:?}", local_branch_names);
    // local_branch_names.into_iter().for_each(|(i, x)| println!("[{:?}] {:?}", i, x));

    // println!(
    //     "# On branch {}",
    //     head.unwrap_or("Not currently on any branch")
    // );
    Ok(local_branch_names)
}

// fn call_command_with_verify(branch_type: BranchType, args)

pub fn run(branch_type: BranchType, branch_name: &String) -> anyhow::Result<()> {
    let branch_type_str = match branch_type {
        BranchType::Local => "local",
        BranchType::Remote => "remote",
    };
    println!("Looking for branch with string \"{}\", in {} branches", branch_name, branch_type_str);
    let repo = Repository::open(".")?;
    let local_branch_names = show_branch(&repo, &branch_type, branch_name)?;

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
    println!("{:?}", target_branch);

    // shell::new!("git", "branch", "-a").run(true)?;
    //
    // let output = Command::new("git").arg("branch").arg("-a").output()?;
    // String::from_utf8(output.stdout)?
    //     .lines()
    //     .for_each(|x| println!("{:?}", x));
    //


    Ok(())
}
