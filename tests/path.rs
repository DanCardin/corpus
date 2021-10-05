use std::path::PathBuf;

use corpus::{builder, CorpusError, MaybePath};

#[test]
fn test_default() {
    let corpus = builder().build().unwrap();
    let result = corpus.path(MaybePath::CurrentDir);

    assert_eq!(result, std::env::current_dir().unwrap());
}

#[test]
fn test_with_path() {
    let corpus = builder().build().unwrap();
    let result = corpus.path("/foo");

    assert_eq!(result, PathBuf::from("/foo"));
}

#[test]
#[cfg(feature = "home")]
fn test_at_current_path() -> Result<(), CorpusError> {
    let home = dirs_next::home_dir().ok_or(CorpusError::NoHomeDir)?;
    let current_dir = std::env::current_dir().map_err(|_| CorpusError::InvalidCurrentDir)?;
    let relative_current = current_dir
        .strip_prefix(home.clone())
        .unwrap_or(&current_dir);

    let corpus = builder().with_root("/wat").relative_to(home).build()?;
    let result = corpus.path(MaybePath::CurrentDir);

    assert_eq!(result, PathBuf::from("/wat").join(relative_current));
    Ok(())
}

#[test]
fn test_with_root_location() {
    let corpus = builder().with_root("/home/.config").build().unwrap();
    let result = corpus.path("/foo/bar");

    assert_eq!(result, PathBuf::from("/home/.config/foo/bar"));
}

#[test]
fn test_relative_to() {
    let corpus = builder()
        .with_root("/home/.config")
        .relative_to("/home")
        .build()
        .unwrap();
    let result = corpus.path("/home/foo/bar");

    assert_eq!(result, PathBuf::from("/home/.config/foo/bar"));
}

#[test]
fn test_with_name() {
    let corpus = builder()
        .with_root("/home/.config")
        .relative_to("/home")
        .with_name("example")
        .build()
        .unwrap();
    let result = corpus.path("/home/foo/bar");

    assert_eq!(result, PathBuf::from("/home/.config/example/foo/bar"));
}

#[test]
fn test_with_extension() {
    let corpus = builder()
        .with_root("/home/.config")
        .relative_to("/home")
        .with_extension("txt")
        .build()
        .unwrap();
    let result = corpus.path("/home/foo/bar");

    assert_eq!(result, PathBuf::from("/home/.config/foo/bar.txt"));
}
