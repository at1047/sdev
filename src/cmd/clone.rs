use std::path::{Path, PathBuf};

use crate::dep::{Clone, Dep};
use crate::repo::GitRepoSource;
use crate::Config;

pub fn run(source: &GitRepoSource, config: &Config) -> Result<(), String> {
    let url = url_for(source, config);
    let path = path_for(source, config);

    Clone::new(url, path).process();

    Ok(())
}

fn url_for(source: &GitRepoSource, config: &Config) -> String {
    match source {
        GitRepoSource::Name(s) => {
            format!("git@{}:{}/{s}.git", config.host, config.user)
        }
        GitRepoSource::Path(s) => format!("git@{}:{s}.git", config.host),
        GitRepoSource::Url { url, .. } => url.to_string(),
    }
}

fn path_for(source: &GitRepoSource, config: &Config) -> PathBuf {
    config.root.join(match source {
        GitRepoSource::Name(s) => Path::new(&config.host).join(&config.user).join(s),
        GitRepoSource::Path(s) => Path::new(&config.host).join(s),
        GitRepoSource::Url { host, path, .. } => Path::new(host).join(path),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_and_path_use_host_and_user_when_given_name() {
        assert_source_to_url_and_path(
            "sdev",
            "git@github.com:skipkayhil/sdev.git",
            "/home/skipkayhil/src/github.com/skipkayhil/sdev",
        );
    }

    #[test]
    fn url_and_path_use_host_when_given_path() {
        assert_source_to_url_and_path(
            "ruby/ruby",
            "git@github.com:ruby/ruby.git",
            "/home/skipkayhil/src/github.com/ruby/ruby",
        );
    }

    #[test]
    fn url_and_path_are_constructed_when_given_url() {
        assert_source_to_url_and_path(
            "https://aur.archlinux.org/google-chrome.git",
            "https://aur.archlinux.org/google-chrome.git",
            "/home/skipkayhil/src/aur.archlinux.org/google-chrome",
        );
    }

    #[test]
    fn url_and_path_are_constructed_when_given_scp_url() {
        assert_source_to_url_and_path(
            "git@github.com:Byron/gitoxide.git",
            "git@github.com:Byron/gitoxide.git",
            "/home/skipkayhil/src/github.com/Byron/gitoxide",
        );
    }

    fn assert_source_to_url_and_path(raw: &str, expected_url: &str, expected_path: &str) -> () {
        let source: GitRepoSource = raw.parse().unwrap();
        let config = Config {
            host: "github.com".to_string(),
            root: PathBuf::from("/home/skipkayhil/src"),
            user: "skipkayhil".to_string(),
        };

        assert_eq!(expected_url, url_for(&source, &config));
        assert_eq!(PathBuf::from(expected_path), path_for(&source, &config));
    }
}
