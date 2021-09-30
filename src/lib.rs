mod builder;
mod corpus;
mod root;

pub use crate::builder::CorpusBuilder;
pub use crate::corpus::Corpus;
pub use crate::root::RootLocation;

#[derive(thiserror::Error, Debug)]
pub enum CorpusError {
    #[error("There is no home directory")]
    NoHomeDir,

    #[error("Current directory does not exist or insufficient permissions")]
    InvalidCurrentDir,
}

pub fn builder() -> CorpusBuilder {
    CorpusBuilder::default()
}

#[cfg(test)]
mod tests {
    mod corpuserror {
        use super::super::CorpusError;

        #[test]
        fn test_no_home_dir() {
            let error = CorpusError::NoHomeDir;
            let debug = format!("{:?}", error);
            assert_eq!(debug, r#"NoHomeDir"#);
        }
    }

    mod path {
        use std::path::PathBuf;

        use crate::builder;

        #[test]
        fn test_default() {
            let corpus = builder().build().unwrap();
            let result = corpus.path();

            assert_eq!(result, std::env::current_dir().unwrap());
        }

        #[test]
        fn test_with_path() {
            let corpus = builder().at_path("/foo").build().unwrap();
            let result = corpus.path();

            assert_eq!(result, PathBuf::from("/foo"));
        }

        #[test]
        #[cfg(feature = "home")]
        fn test_at_current_path() -> Result<(), crate::CorpusError> {
            let home = dirs_next::home_dir().ok_or(crate::CorpusError::NoHomeDir)?;
            let current_dir =
                std::env::current_dir().map_err(|_| crate::CorpusError::InvalidCurrentDir)?;
            let relative_current = current_dir
                .strip_prefix(home.clone())
                .unwrap_or(&current_dir);

            let corpus = builder()
                .with_root("/wat")
                .relative_to(home)
                .at_current_path()?
                .build()?;
            let result = corpus.path();

            assert_eq!(result, PathBuf::from("/wat").join(relative_current));
            Ok(())
        }

        #[test]
        fn test_with_root_location() {
            let corpus = builder()
                .with_root("~/.config")
                .at_path("/foo/bar")
                .build()
                .unwrap();
            let result = corpus.path();

            assert_eq!(result, PathBuf::from("~/.config/foo/bar"));
        }

        #[test]
        fn test_relative_to() {
            let corpus = builder()
                .with_root("/home/.config")
                .relative_to("/home")
                .at_path("/home/foo/bar")
                .build()
                .unwrap();
            let result = corpus.path();

            assert_eq!(result, PathBuf::from("/home/.config/foo/bar"));
        }

        #[test]
        fn test_with_name() {
            let corpus = builder()
                .with_root("/home/.config")
                .relative_to("/home")
                .at_path("/home/foo/bar")
                .with_name("example")
                .build()
                .unwrap();
            let result = corpus.path();

            assert_eq!(result, PathBuf::from("/home/.config/example/foo/bar"));
        }

        #[test]
        fn test_with_extension() {
            let corpus = builder()
                .with_root("/home/.config")
                .relative_to("/home")
                .at_path("/home/foo/bar")
                .with_extension("txt")
                .build()
                .unwrap();
            let result = corpus.path();

            assert_eq!(result, PathBuf::from("/home/.config/foo/bar.txt"));
        }
    }

    mod find_nearest {
        use std::path::PathBuf;

        use crate::builder;

        #[test]
        fn test_default() {
            let corpus = builder().build().unwrap();
            let result = corpus.find_nearest();

            assert_eq!(result, Some(std::env::current_dir().unwrap()));
        }

        #[test]
        fn test_with_path() {
            let corpus = builder().at_path("/foo").build().unwrap();
            let result = corpus.find_nearest();

            assert_eq!(result, Some(PathBuf::from("/")));
        }

        #[test]
        fn test_with_root_location() {
            let corpus = builder()
                .with_root("/usr")
                .at_path("/usr/local/bin")
                .build()
                .unwrap();
            let result = corpus.find_nearest();

            assert_eq!(result, Some(PathBuf::from("/usr")));
        }

        #[test]
        fn test_relative_to() {
            let corpus = builder()
                .with_root("/usr")
                .relative_to("/usr")
                .at_path("/usr/local/bin/foo")
                .build()
                .unwrap();
            let result = corpus.find_nearest();

            assert_eq!(result, Some(PathBuf::from("/usr/local/bin")));
        }

        #[test]
        fn test_with_name() {
            let corpus = builder()
                .with_root("/usr")
                .relative_to("/usr")
                .at_path("/usr/local/bin/foo")
                .with_name("local")
                .build()
                .unwrap();
            let result = corpus.find_nearest();

            assert_eq!(result, Some(PathBuf::from("/usr/local")));
        }

        #[test]
        fn test_with_extension_doesnt_exist() {
            let corpus = builder()
                .with_root("/home/.config")
                .relative_to("/home")
                .at_path("/home/foo/bar")
                .with_extension("frombly")
                .build()
                .unwrap();
            let result = corpus.find_nearest();

            assert_eq!(result, None);
        }
    }

    mod get_source_path {
        use std::path::PathBuf;

        use crate::builder;

        #[test]
        fn test_with_extension_doesnt_exist() {
            let corpus = builder()
                .with_root("/home/.config")
                .relative_to("/home")
                .with_extension("frombly")
                .build()
                .unwrap();
            let result = corpus.get_source_path("/home/.config/foo/bar");

            assert_eq!(result, Some(PathBuf::from("/home/foo/bar")));
        }
    }
}
