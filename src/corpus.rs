use path_absolutize::Absolutize;

use std::path::{Path, PathBuf};

pub enum MaybePath<'a> {
    Path(&'a Path),
    CurrentDir,
}

impl<'a> From<Option<&'a Path>> for MaybePath<'a> {
    fn from(maybe_path: Option<&'a Path>) -> Self {
        if let Some(path) = maybe_path {
            Self::Path(path)
        } else {
            Self::CurrentDir
        }
    }
}

impl<'a> From<&'a Path> for MaybePath<'a> {
    fn from(path: &'a Path) -> Self {
        Self::Path(path)
    }
}

impl<'a> From<&'a str> for MaybePath<'a> {
    fn from(path: &'a str) -> Self {
        Self::Path(Path::new(path))
    }
}

impl<'a> MaybePath<'a> {
    fn to_path_buf(&self) -> PathBuf {
        match self {
            Self::Path(path) => path.to_path_buf(),
            Self::CurrentDir => {
                let current_dir = std::env::current_dir();
                current_dir.unwrap_or_else(|_| Path::new(".").to_path_buf())
            }
        }
    }
}

#[derive(Debug)]
pub struct Corpus {
    root_location: PathBuf,
    relative_path: PathBuf,
    extension: Option<String>,
}

impl Corpus {
    /// Constructs a new [`crate::Corpus`] instance.
    ///
    /// The recommended construction method is the [`crate::builder()`] function
    /// and the [`crate::CorpusBuilder`] methods to produce a canonical
    /// [`crate::Corpus`] instance.
    pub fn new<I: Into<PathBuf>>(
        root_location: I,
        relative_path: I,
        extension: Option<String>,
    ) -> Self {
        let root_location = root_location.into();
        let relative_path = relative_path.into();
        Self {
            root_location,
            relative_path,
            extension,
        }
    }

    /// Computes the "corpus" path which corresponds to an `input` source [`Path`].
    ///
    /// The input path is the source path (typically the current directory),
    /// and returns the corresponding "corpus" path.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::{Path, PathBuf};
    /// use corpus::{builder, RootLocation};
    ///
    /// let corpus = builder()
    ///     .with_root("/home/.config")
    ///     .relative_to("/home")
    ///     .with_name("foo")
    ///     .with_extension("toml")
    ///     .build()
    ///     .unwrap();
    ///
    /// let path = corpus
    ///     .path("/home/bar/baz.toml");
    ///
    /// assert_eq!(path, PathBuf::from("/home/.config/foo/bar/baz.toml"));
    /// ```
    pub fn path<'a, I: Into<MaybePath<'a>>>(&self, input: I) -> PathBuf {
        let input = input.into().to_path_buf();
        let path = input
            .absolutize()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|_| input.clone());

        let relative_path = path.strip_prefix(&self.relative_path).unwrap_or(&path);

        let mut abs_path = self.root_location.join(relative_path);
        if let Some(ext) = &self.extension {
            abs_path = abs_path.with_extension(ext);
        };
        abs_path
    }

    fn ancestors<'a, I: Into<MaybePath<'a>>>(&self, input: I) -> impl Iterator<Item = PathBuf> {
        self.path(input)
            .ancestors()
            .filter(|p| {
                let qualified_root = if let Some(ext) = &self.extension {
                    self.root_location.with_extension(ext)
                } else {
                    self.root_location.clone()
                };

                if qualified_root == *p {
                    true
                } else {
                    p.strip_prefix(&self.root_location).is_ok()
                }
            })
            .map(|p| {
                if let Some(ext) = &self.extension {
                    p.with_extension(ext)
                } else {
                    p.to_path_buf()
                }
            })
            .collect::<Vec<PathBuf>>()
            .into_iter()
    }

    /// Finds the nearest ancestor [`Corpus::path`] which actually exists.
    ///
    /// - If `Corpus::path` would resolve to `/some/path/foo/bar`, and that path
    ///   exists, return that path
    /// - If that path does not exist but `/some/path/foo` does, return that path
    ///   instead.
    /// - Continue traversing upwards until hitting the `relative_path`
    pub fn find_nearest<'a, I: Into<MaybePath<'a>>>(&self, input: I) -> Option<PathBuf> {
        self.ancestors(input)
            .filter(|p| p.strip_prefix(&self.root_location).is_ok())
            .find(|p| p.exists())
    }

    /// Gets the concrete path which corresponds to an input corpus `path`.
    ///
    /// The input path is the "corpus" path (essentially the output of the
    /// [`Corpus::path`] method), and returns the corresponding path
    /// (essentially the input of the [`Corpus::path`] method).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::{Path, PathBuf};
    /// use corpus::{builder, RootLocation};
    ///
    /// let corpus = builder()
    ///     .with_root("/home/.config")
    ///     .relative_to("/home")
    ///     .with_name("foo")
    ///     .with_extension("toml")
    ///     .build()
    ///     .unwrap();
    ///
    /// let path = corpus
    ///     .get_source_path("/home/.config/foo/bar/baz.toml")
    ///     .unwrap();
    ///
    /// assert_eq!(path, PathBuf::from("/home/bar/baz"));
    /// ```
    pub fn get_source_path<P: Into<PathBuf>>(&self, path: P) -> Option<PathBuf> {
        let path = path.into();
        self.relative_path
            .join(path.strip_prefix(&self.root_location).unwrap_or(&path))
            .with_extension("")
            .absolutize()
            .map(|p| p.to_path_buf())
            .ok()
    }
}
