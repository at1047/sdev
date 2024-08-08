use clap::{Args, Parser, Subcommand};
use git2::{ BranchType };
use home::home_dir;

mod cmd;
mod config;
mod dep;
mod repo;
mod shell;
mod tui;
mod branch;
mod storage;

use crate::config::Config;
use crate::branch::BranchKind;
use crate::repo::GitRepoSource;

#[derive(Parser)]
#[command(version, disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Clone a git repository into a standardized path
    Clone { repo: GitRepoSource },
    /// Open a link for the current repository
    #[command(alias("o"))]
    Open(OpenArgs),
    /// Fuzzy attach to a repository's tmux session (creating it if necessary)
    #[command(alias("t"))]
    Tmux,
    /// Checkout
    Checkout {
        #[command(flatten)]
        branch_type: BranchKinds,
        /// Devops ticket number (i.e. BUG-123456)
        ticket_number: String,
        /// Branch name (i.e. develop, JuneMSP_2024)
        branch_name: String,
        /// Branch type (remote, local)
        origin_type: Option<String>,
    },
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct BranchKinds {
    /// use develop branch logic
    #[arg(short, long)]
    develop: bool,
    /// use releases branch logic
    #[arg(short, long)]
    releases: bool,
}

#[derive(Debug, Args)]
struct OpenArgs {
    #[command(subcommand)]
    command: OpenCommands,
}

#[derive(Debug, Subcommand)]
enum OpenCommands {
    /// Open the New Pull Request form for the current branch
    Pr { target: Option<String> },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = Config {
        host: "github.com".to_string(),
        root: home_dir().expect("unknown HOME directory").join("src"),
        user: "skipkayhil".to_string(),
    };

    match &cli.command {
        Commands::Clone { repo } => cmd::clone::run(repo, &config),
        Commands::Open(open) => match &open.command {
            OpenCommands::Pr { target } => cmd::open::pr::run(target),
        },
        Commands::Tmux => cmd::tmux::run(config),
        Commands::Checkout {
            branch_type,
            ticket_number,
            branch_name,
            origin_type,
        } => {
            let (d, r) = (branch_type.develop, branch_type.releases);
            let branch_kind = match (d, r) {
                (true, _) => BranchKind::Develop,
                (_, true) => BranchKind::Releases,
                _ => unreachable!(),
            };

            if origin_type.is_none() {
                cmd::checkout::run_with_ticket(&branch_kind, &ticket_number, &branch_name, None)?;
            } else if *origin_type == Some("local".to_string()) {
                cmd::checkout::run_with_ticket(&branch_kind, &ticket_number, &branch_name, Some(BranchType::Local))?;
            } else if *origin_type == Some("remote".to_string()) {
                cmd::checkout::run_with_ticket(&branch_kind, &ticket_number, &branch_name, Some(BranchType::Remote))?;
            } else {

            }

            Ok(())
        }

    }

}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
