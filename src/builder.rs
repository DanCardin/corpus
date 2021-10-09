use crate::{root::RootLocation, Corpus, CorpusError};
use std::path::{Path, PathBuf};

pub struct CorpusBuilder {
    root_location: Option<RootLocation>,
    relative_path: Option<PathBuf>,
    name: Option<String>,
    extension: Option<String>,
}

impl Default for CorpusBuilder {
    fn default() -> Self {
        Self {
            root_location: None,
            relative_path: None,
            name: None,
            extension: None,
        }
    }
}

/// Assists in building a [`crate::Corpus`] instance
///
/// The options on the builder can be seen as producing the configuration
/// for a specific user of the library, from which they should be able to
/// use the resulting [`crate::Corpus`] instance to get concrete paths.
///
/// Once configured as desired, use the [`CorpusBuilder::build`] method.
impl CorpusBuilder {
    /// Sets the "relative" directory.
    ///
    /// Typically this is something like $HOME, but more generally is the
    /// path against which input [`Path`]s are compared to determine the least
    /// common denominator between the two paths.
    ///
    /// If the "home" feature is enabled, `relative_to_home` can be used.
    pub fn relative_to<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.relative_path = Some(path.into());
        self
    }

    /// Sets the "relative" directory to the home directory.
    #[cfg(feature = "home")]
    pub fn relative_to_home(self) -> Result<Self, CorpusError> {
        let home_dir = dirs_next::home_dir().ok_or(CorpusError::NoHomeDir)?;
        Ok(self.relative_to(home_dir))
    }

    /// Sets the "root" directory.
    ///
    /// This is the root location of the "corpus" paths for a given builder
    /// configuration.
    pub fn with_root<R: Into<RootLocation>>(mut self, root: R) -> Self {
        self.root_location = Some(root.into());
        self
    }

    /// Sets a "name" sub-directory.
    ///
    /// Given some "root", the "name" would set a sub-directory of the root
    /// under which all generated paths would live. One might typically use
    /// this for setting per-application configuration/data/etc.
    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets a file extension for the generated file name.
    pub fn with_extension<S: Into<String>>(mut self, extension: S) -> Self {
        self.extension = Some(extension.into());
        self
    }

    /// Builds the [`crate::Corpus`] instance given the builder configuration.
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

        let relative_path = self
            .relative_path
            .unwrap_or_else(|| Path::new("/").to_path_buf());

        Ok(Corpus::new(root_location, relative_path, self.extension))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::CorpusBuilder;

    #[test]
    fn test_build_relative_to() {
        let corpus = CorpusBuilder::default()
            .relative_to("/home")
            .with_root("/config")
            .with_name("foo")
            .with_extension("txt")
            .build()
            .unwrap();

        assert_eq!(corpus.root_location, PathBuf::from("/config/foo"));
        assert_eq!(corpus.relative_path, PathBuf::from("/home"));
        assert_eq!(corpus.extension, Some("txt".to_string()));
    }

    #[test]
    #[cfg(feature = "home")]
    fn test_build_relative_to_home() {
        let corpus = CorpusBuilder::default()
            .relative_to_home()
            .unwrap()
            .with_root("/config")
            .with_name("foo")
            .with_extension("txt")
            .build()
            .unwrap();

        assert_eq!(corpus.root_location, PathBuf::from("/config/foo"));
        assert_eq!(
            corpus.relative_path,
            PathBuf::from(dirs_next::home_dir().unwrap())
        );
        assert_eq!(corpus.extension, Some("txt".to_string()));
    }
}
