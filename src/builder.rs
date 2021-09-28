use crate::{root::RootLocation, Corpus, CorpusError};
use std::env;
use std::path::{Path, PathBuf};

pub struct CorpusBuilder {
    root_location: Option<RootLocation>,
    relative_path: Option<PathBuf>,
    path: Option<PathBuf>,
    name: Option<String>,
    extension: Option<String>,
}

impl Default for CorpusBuilder {
    fn default() -> Self {
        Self {
            root_location: None,
            relative_path: None,
            path: None,
            name: None,
            extension: None,
        }
    }
}

impl CorpusBuilder {
    pub fn relative_to<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.relative_path = Some(path.into());
        self
    }

    #[cfg(feature = "home")]
    pub fn relative_to_home(self) -> Result<Self, etcetera::HomeDirError> {
        let home_dir = etcetera::home_dir()?;
        Ok(self.relative_to(home_dir))
    }

    pub fn at_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn at_current_path(self) -> std::io::Result<Self> {
        Ok(self.at_path(env::current_dir()?))
    }

    pub fn with_root<R: Into<RootLocation>>(mut self, root: R) -> Self {
        self.root_location = Some(root.into());
        self
    }

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_extension<S: Into<String>>(mut self, extension: S) -> Self {
        self.extension = Some(extension.into());
        self
    }

    pub fn build(self) -> Result<Corpus, CorpusError> {
        let root_location = {
            let mut root = self
                .root_location
                .unwrap_or_else(|| RootLocation::Raw(Path::new("/").to_path_buf()))
                .path()?;
            if let Some(name) = self.name {
                root = root.join(name)
            }
            root
        };

        let path = self.path.unwrap_or_else(|| Path::new(".").to_path_buf());

        let relative_path = self
            .relative_path
            .unwrap_or_else(|| Path::new("/").to_path_buf());

        Ok(Corpus::new(
            root_location,
            relative_path,
            path,
            self.extension,
        ))
    }
}
