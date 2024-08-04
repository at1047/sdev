// use crate::shell;
// use std::process::Command;
use git2::{ Repository, BranchType, Error };

fn show_branch(repo: &Repository, branch_type: &BranchType, branch_name: &String) -> Result<(), Error> {
    let head = match repo.head() {
        Ok(head) => Some(head),
        // Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
        //     None
        // }
        Err(e) => return Err(e),
    };


    let local_branches = repo.branches(Some(*branch_type)).unwrap();
    let local_branch_names: Vec<String> = local_branches
        .flatten()
        .map(|x| x.0.name().unwrap().unwrap().to_string())
        .filter(|name| name.contains(branch_name))
        .collect();
    println!("{:?}", local_branch_names);

    // println!(
    //     "# On branch {}",
    //     head.unwrap_or("Not currently on any branch")
    // );
    Ok(())
}

pub fn run(branch_type: BranchType, branch_name: &String) -> anyhow::Result<()> {
    println!("test checkout");
    println!("{}", branch_name);
    let repo = Repository::open(".")?;
    show_branch(&repo, &branch_type, branch_name)?;

    // shell::new!("git", "branch", "-a").run(true)?;
    // let output = Command::new("git").arg("branch").arg("-a").output()?;
    // String::from_utf8(output.stdout)?
    //     .lines()
    //     .for_each(|x| println!("{:?}", x));

    Ok(())
}
