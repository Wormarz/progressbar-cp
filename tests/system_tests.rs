use assert_cmd::Command;
use filetime;
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
    let temp_dir = tempdir().unwrap();
    let src_file = temp_dir.path().join("src.txt");
    let des_file = temp_dir.path().join("des.txt");
    fs::write(&src_file, "New content").unwrap();
    fs::write(&des_file, "Old content").unwrap();

    // Set the source file's modification time to be older
    let mtime = filetime::FileTime::from_system_time(
        std::time::SystemTime::now() - std::time::Duration::from_secs(10),
    );
    filetime::set_file_mtime(&src_file, mtime).unwrap();

    let mut cmd = Command::cargo_bin("pbcp").unwrap();
    cmd.arg("--update").arg(&src_file).arg("--").arg(&des_file);
    cmd.assert().success();

    // Content should remain unchanged
    let des_content = fs::read_to_string(&des_file).unwrap();
    assert_eq!(des_content, "Old content");

    // Set the source file's modification time to be newer
    let mtime = filetime::FileTime::from_system_time(
        std::time::SystemTime::now() + std::time::Duration::from_secs(10),
    );
    filetime::set_file_mtime(&src_file, mtime).unwrap();

    let mut cmd = Command::cargo_bin("pbcp").unwrap();
    cmd.arg("--update").arg(&src_file).arg("--").arg(&des_file);
    cmd.assert().success();

    // Content should be updated
    let des_content = fs::read_to_string(&des_file).unwrap();
    assert_eq!(des_content, "New content");
}

#[test]
fn test_preserve_attributes() {
    todo!()
}
