use crate::CorpusError;
use std::path::PathBuf;

#[derive(Debug)]
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
            #[cfg(feature = "xdg")]
            Self::Raw(path) => Ok(path.to_path_buf()),
        }
    }
}

impl From<&str> for RootLocation {
    fn from(s: &str) -> Self {
        match s {
            "xdg-data" => Self::XDGData,
            "xdg-config" => Self::XDGConfig,
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
