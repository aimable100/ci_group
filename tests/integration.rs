use std::process::Command;

#[test]
fn prints_group_markers() {
    let output = Command::new(env!("CARGO_BIN_EXE_basic"))
        .env("GITHUB_ACTIONS", "true")
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("::group::Test Group"));
    assert!(stdout.contains("inside group"));
    assert!(stdout.contains("::endgroup::"));
}

#[test]
fn closes_group_on_panic() {
    let output = Command::new(env!("CARGO_BIN_EXE_panic"))
        .env("GITHUB_ACTIONS", "true")
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("::group::Panic Group"));
    assert!(stdout.contains("::endgroup::"));
}

#[test]
fn silent_outside_github() {
    let output = Command::new(env!("CARGO_BIN_EXE_basic"))
        .env_remove("GITHUB_ACTIONS")
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("::group::"));
    assert!(!stdout.contains("::endgroup::"));
    assert!(stdout.contains("inside group"));
}

#[test]
fn handles_missing_newline() {
    let output = Command::new(env!("CARGO_BIN_EXE_no_newline"))
        .env("GITHUB_ACTIONS", "true")
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\n::group::After Print"));
    assert!(stdout.contains("\n::endgroup::"));
}