use path_absolutize::Absolutize;

use std::path::PathBuf;

pub struct Corpus {
    root_location: PathBuf,
    relative_path: PathBuf,
    path: PathBuf,
    extension: Option<String>,
}

impl Corpus {
    pub fn new(
        root_location: PathBuf,
        relative_path: PathBuf,
        path: PathBuf,
        extension: Option<String>,
    ) -> Self {
        Self {
            root_location,
            relative_path,
            path,
            extension,
        }
    }

    pub fn path(&self) -> PathBuf {
        let path = self
            .path
            .absolutize()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|_| self.path.clone());

        let relative_path = path.strip_prefix(&self.relative_path).unwrap_or(&path);

        let mut abs_path = self.root_location.join(relative_path);
        if let Some(ext) = &self.extension {
            abs_path = abs_path.with_extension(ext);
        };
        abs_path
    }

    pub fn ancestors(&self) -> impl Iterator<Item = PathBuf> {
        self.path()
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

    pub fn find_nearest(&self) -> Option<PathBuf> {
        self.ancestors()
            .filter(|p| p.strip_prefix(&self.root_location).is_ok())
            .find(|p| p.exists())
    }

    pub fn get_source_path<P: Into<PathBuf>>(&self, path: P) -> Option<PathBuf> {
        let path = path.into();
        Some(
            self.relative_path
                .join(path.strip_prefix(&self.root_location).unwrap_or(&path))
                .with_extension(""),
        )
    }
}
