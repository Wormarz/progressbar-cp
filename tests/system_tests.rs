use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_recursive_copy() {
    let temp_dir = tempdir().unwrap();
    let src_dir = temp_dir.path().join("src");
    let des_dir = temp_dir.path().join("des");
    fs::create_dir_all(&src_dir).unwrap();
    fs::create_dir_all(&des_dir).unwrap();
    fs::write(src_dir.join("file1.txt"), "Hello, world!").unwrap();

    let mut cmd = Command::cargo_bin("pbcp").unwrap();
    cmd.arg("-r").arg(&src_dir).arg("--").arg(&des_dir);
    cmd.assert().success();

    assert!(des_dir.join("src/file1.txt").exists());
    assert_eq!(
        fs::read_to_string(des_dir.join("src/file1.txt")).unwrap(),
        "Hello, world!"
    );
}

#[test]
fn test_update_copy() {
    todo!()
}

#[test]
fn test_preserve_attributes() {
    todo!()
}
