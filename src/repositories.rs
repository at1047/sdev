pub mod git_repos {
    use std::collections::VecDeque;
    use std::io;
    use std::path::{Path, PathBuf};

    use crate::repo::{GitRepo, TryFromAbsoluteError, TryFromFsError};

    #[derive(thiserror::Error, Debug)]
    pub enum FetchAllError {
        #[error("error reading root src dir")]
        ReadRoot(#[source] io::Error),
    }

    #[derive(thiserror::Error, Debug)]
    pub enum FetchOneError {
        #[error("unknown git repo: {path}")]
        UnknownRepo {
            path: PathBuf,
            #[source]
            source: TryFromAbsoluteError,
        },
    }

    pub trait Repository {
        fn fetch_all(&self) -> Result<Vec<GitRepo>, FetchAllError>;
        fn fetch_one(&self, path: &Path) -> Result<GitRepo, FetchOneError>;
    }

    pub struct FileSystemRepository {
        root: PathBuf,
    }

    impl FileSystemRepository {
        pub fn new(root: PathBuf) -> Self {
            Self { root }
        }
    }

    impl Repository for FileSystemRepository {
        fn fetch_all(&self) -> Result<Vec<GitRepo>, FetchAllError> {
            let host_entries = self
                .root
                .read_dir()
                .map_err(FetchAllError::ReadRoot)?
                .filter_map(Result::ok);

            let mut queue = VecDeque::new();
            let mut repos = Vec::new();

            for host_entry in host_entries {
                let host = host_entry.file_name();

                if let Ok(repo_iter) = host_entry.path().read_dir() {
                    queue.extend(repo_iter.filter_map(Result::ok))
                }

                while let Some(dir_entry) = queue.pop_front() {
                    let name = dir_entry.file_name();
                    let path = dir_entry.path();

                    match GitRepo::try_from_fs(&name, &path, &host) {
                        Ok(repo) => repos.push(repo),
                        Err(TryFromFsError::NotARepo) => {
                            if let Ok(dir_iter) = path.read_dir() {
                                queue.extend(dir_iter.filter_map(Result::ok));
                            }
                        }
                        _ => (),
                    }
                }
            }

            Ok(repos)
        }

        fn fetch_one(&self, path: &Path) -> Result<GitRepo, FetchOneError> {
            GitRepo::try_from_absolute(path, &self.root).map_err(|e| FetchOneError::UnknownRepo {
                source: e,
                path: path.to_owned(),
            })
        }
    }
}
