use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
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
    pub fn to_path_buf(&self) -> PathBuf {
        match self {
            Self::Path(path) => path.to_path_buf(),
            Self::CurrentDir => {
                let current_dir = std::env::current_dir();
                current_dir.unwrap_or_else(|_| Path::new(".").to_path_buf())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::MaybePath;

    #[test]
    fn test_from_str() {
        let path = MaybePath::from("path");
        assert_eq!(path, MaybePath::Path(Path::new("path")));
    }

    #[test]
    fn test_from_path() {
        let path = MaybePath::from(Path::new("path"));
        assert_eq!(path, MaybePath::Path(Path::new("path")));
    }

    #[test]
    fn test_from_option_path() {
        let path = MaybePath::from(Some(Path::new("path")));
        assert_eq!(path, MaybePath::Path(Path::new("path")));
    }

    #[test]
    fn test_from_option_none() {
        let path = MaybePath::from(None);
        assert_eq!(path, MaybePath::CurrentDir);
    }
}
