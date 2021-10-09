use crate::CorpusError;
use path_absolutize::Absolutize;
use std::path::{Path, PathBuf};

/// The options for available root locations.
///
/// When the "xdg" feature is enabled, the variants `XDGData`, `XDGConfig`,
/// and `XDGCache` variants become available. When enabled, the raw strings
/// "xdg-data", "xdg-config", and "xdg-cache" automatically are interpreted
/// as those variants rather than as a raw path.
#[derive(Debug, PartialEq)]
pub enum RootLocation {
    #[cfg(feature = "xdg")]
    XDGData,

    #[cfg(feature = "xdg")]
    XDGConfig,

    #[cfg(feature = "xdg")]
    XDGCache,

    Raw(PathBuf),
}

impl RootLocation {
    pub fn path(&self) -> Result<PathBuf, CorpusError> {
        let path = match self {
            #[cfg(feature = "xdg")]
            p @ (Self::XDGData | Self::XDGConfig | Self::XDGCache) => {
                use etcetera::base_strategy::{BaseStrategy, Xdg};

                let strategy = Xdg::new().map_err(|_| CorpusError::NoHomeDir)?;
                let path = match p {
                    Self::XDGData => strategy.data_dir(),
                    Self::XDGConfig => strategy.config_dir(),
                    Self::XDGCache => strategy.cache_dir(),
                    _ => unreachable!(
                        "The outer match already scopes the set of available variants to these 3."
                    ),
                };
                Ok(path)
            }
            Self::Raw(path) => Ok(path.to_path_buf()),
        }?;

        Ok(path
            .absolutize()
            .map_err(|_| CorpusError::InvalidCurrentDir)?
            .to_path_buf())
    }
}

impl From<&str> for RootLocation {
    fn from(s: &str) -> Self {
        match s {
            #[cfg(feature = "xdg")]
            "xdg-data" => Self::XDGData,

            #[cfg(feature = "xdg")]
            "xdg-config" => Self::XDGConfig,

            #[cfg(feature = "xdg")]
            "xdg-cache" => Self::XDGCache,

            raw => Self::Raw(PathBuf::from(raw)),
        }
    }
}

impl<'a> From<&'a Path> for RootLocation {
    fn from(path: &'a Path) -> Self {
        Self::Raw(path.to_path_buf())
    }
}

impl From<PathBuf> for RootLocation {
    fn from(path: PathBuf) -> Self {
        Self::Raw(path)
    }
}

impl From<String> for RootLocation {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::RootLocation;

    #[test]
    fn test_debug() {
        let root = RootLocation::from("path");
        let debug = format!("{:?}", root);
        assert_eq!(debug, r#"Raw("path")"#);
    }

    mod from {
        use std::path::{Path, PathBuf};

        use super::super::RootLocation;

        #[test]
        fn test_raw_str() {
            let root = RootLocation::from("path");
            assert_eq!(root, RootLocation::Raw("path".into()));
        }

        #[test]
        fn test_raw_string() {
            let root = RootLocation::from(String::from("path"));
            assert_eq!(root, RootLocation::Raw("path".into()));
        }

        #[test]
        fn test_raw_path() {
            let root = RootLocation::from(Path::new("path"));
            assert_eq!(root, RootLocation::Raw("path".into()));
        }

        #[test]
        fn test_raw_path_buf() {
            let root = RootLocation::from(PathBuf::from("path"));
            assert_eq!(root, RootLocation::Raw("path".into()));
        }

        #[test]
        #[cfg(feature = "xdg")]
        fn test_xdg_data() {
            let root = RootLocation::from("xdg-data");
            assert_eq!(root, RootLocation::XDGData);
        }

        #[test]
        #[cfg(feature = "xdg")]
        fn test_xdg_config() {
            let root = RootLocation::from("xdg-config");
            assert_eq!(root, RootLocation::XDGConfig);
        }

        #[test]
        #[cfg(feature = "xdg")]
        fn test_xdg_cache() {
            let root = RootLocation::from("xdg-cache");
            assert_eq!(root, RootLocation::XDGCache);
        }
    }

    mod path {
        use super::super::RootLocation;

        #[test]
        fn test_raw() {
            let current_dir = std::env::current_dir().unwrap();
            let path = RootLocation::from("path").path().unwrap();
            assert_eq!(path, current_dir.join("path"));
        }

        #[test]
        #[cfg(feature = "home")]
        fn test_xdg_data() {
            let home = dirs_next::home_dir().unwrap();
            let path = RootLocation::from("xdg-data").path().unwrap();
            assert_eq!(path, home.join(".local/share"));
        }

        #[test]
        #[cfg(feature = "home")]
        fn test_xdg_config() {
            let home = dirs_next::home_dir().unwrap();
            let path = RootLocation::from("xdg-config").path().unwrap();
            assert_eq!(path, home.join(".config"));
        }

        #[test]
        #[cfg(feature = "home")]
        fn test_xdg_cache() {
            let home = dirs_next::home_dir().unwrap();
            let path = RootLocation::from("xdg-cache").path().unwrap();
            assert_eq!(path, home.join(".cache"));
        }
    }
}
