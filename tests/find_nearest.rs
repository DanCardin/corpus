use std::path::PathBuf;

use corpus::{builder, MaybePath};

#[test]
fn test_default() {
    let corpus = builder().build().unwrap();
    let result = corpus.find_nearest(MaybePath::CurrentDir);

    assert_eq!(result, Some(std::env::current_dir().unwrap()));
}

#[test]
fn test_with_path() {
    let corpus = builder().build().unwrap();
    let result = corpus.find_nearest("/foo");

    assert_eq!(result, Some(PathBuf::from("/")));
}

#[test]
fn test_with_root_location() {
    let corpus = builder().with_root("/usr").build().unwrap();
    let result = corpus.find_nearest("/usr/local/bin");

    assert_eq!(result, Some(PathBuf::from("/usr")));
}

#[test]
fn test_relative_to() {
    let corpus = builder()
        .with_root("/usr")
        .relative_to("/usr")
        .build()
        .unwrap();
    let result = corpus.find_nearest("/usr/local/bin/foo");

    assert_eq!(result, Some(PathBuf::from("/usr/local/bin")));
}

#[test]
fn test_with_name() {
    let corpus = builder()
        .with_root("/usr")
        .relative_to("/usr")
        .with_name("local")
        .build()
        .unwrap();
    let result = corpus.find_nearest("/usr/local/bin/foo");

    assert_eq!(result, Some(PathBuf::from("/usr/local")));
}

#[test]
fn test_with_extension_doesnt_exist() {
    let corpus = builder()
        .with_root("/home/.config")
        .relative_to("/home")
        .with_extension("frombly")
        .build()
        .unwrap();
    let result = corpus.find_nearest("/home/foo/bar");

    assert_eq!(result, None);
}
