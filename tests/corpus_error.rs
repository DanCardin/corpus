use corpus::CorpusError;

#[test]
fn test_no_home_dir() {
    let error = CorpusError::NoHomeDir;
    let debug = format!("{:?}", error);
    assert_eq!(debug, r#"NoHomeDir"#);
}
