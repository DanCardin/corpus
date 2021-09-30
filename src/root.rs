use crate::CorpusError;
use std::path::PathBuf;

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
        match self {
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
        }
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

impl From<String> for RootLocation {
    fn from(s: String) -> Self {
        Self::from(s.as_ref())
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

    mod from_str {
        use super::super::RootLocation;

        #[test]
        fn test_raw() {
            let root = RootLocation::from("path");
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
}
