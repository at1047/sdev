// use crate::shell;
// use std::process::Command;
use git2::{ Repository, BranchType, Error };

fn show_branch(repo: &Repository) -> Result<(), Error> {
    let head = match repo.head() {
        Ok(head) => Some(head),
        // Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
        //     None
        // }
        Err(e) => return Err(e),
    };
    println!("{}", repo.workdir().unwrap().display());

    let local_branches = repo.branches(Some(BranchType::Local)).unwrap();
    local_branches.for_each(|x| println!("{}", x.unwrap().0.name().unwrap().unwrap()));

    let remote_branches = repo.branches(Some(BranchType::Remote)).unwrap();
    remote_branches.for_each(|x| println!("{}", x.unwrap().0.name().unwrap().unwrap()));

    // println!(
    //     "# On branch {}",
    //     head.unwrap_or("Not currently on any branch")
    // );
    Ok(())
}

pub fn run(test: &String) -> anyhow::Result<()> {
    println!("test checkout");
    println!("{}", test);

    let repo = Repository::open(".")?;
    show_branch(&repo)?;
    // shell::new!("git", "branch", "-a").run(true)?;
    // let output = Command::new("git").arg("branch").arg("-a").output()?;
    // String::from_utf8(output.stdout)?
    //     .lines()
    //     .for_each(|x| println!("{:?}", x));

    Ok(())
}
