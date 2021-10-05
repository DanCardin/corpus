use std::path::PathBuf;

use corpus::builder;

#[test]
fn test_with_extension_doesnt_exist() {
    let corpus = builder()
        .with_root("/home/.config")
        .relative_to("/home")
        .with_extension("frombly")
        .build()
        .unwrap();
    let result = corpus.get_source_path("/home/.config/foo/bar");

    assert_eq!(result, Some(PathBuf::from("/home/foo/bar")));
}
