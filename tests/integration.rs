use assert_cmd::Command;
use std::path::PathBuf;

#[cfg(feature = "home")]
fn home() -> PathBuf {
    dirs_next::home_dir().unwrap()
}

fn output(path: PathBuf) -> String {
    format!("{}\n", path.to_string_lossy())
}

#[test]
#[cfg(feature = "home")]
fn sauce_example() {
    let mut cmd = Command::cargo_bin("corpus").unwrap();
    let assert = cmd
        .args(&[
            "-n",
            "sauce",
            "-e",
            "toml",
            "-p",
            &home().join("foo/bar").to_string_lossy(),
        ])
        .assert();
    assert
        .success()
        .stdout(output(home().join(".local/share/sauce/foo/bar.toml")));
}
