use assert_cmd::Command;
use filetime;
use std::fs;
use std::os::unix::fs::MetadataExt;
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
    let temp_dir = tempdir().unwrap();
    let src_file = temp_dir.path().join("src.txt");
    let des_file = temp_dir.path().join("des.txt");
    fs::write(&src_file, "Content").unwrap();

    let mut cmd = Command::cargo_bin("pbcp").unwrap();
    cmd.arg("--preserve=mode,ownership,timestamps")
        .arg(&src_file)
        .arg("--")
        .arg(&des_file);
    cmd.assert().success();

    let src_metadata = fs::metadata(&src_file).unwrap();
    let des_metadata = fs::metadata(&des_file).unwrap();

    // Check permissions
    assert_eq!(src_metadata.permissions(), des_metadata.permissions());

    // Check ownership
    assert_eq!(src_metadata.uid(), des_metadata.uid());
    assert_eq!(src_metadata.gid(), des_metadata.gid());

    // Check timestamps
    assert_eq!(
        src_metadata.modified().unwrap(),
        des_metadata.modified().unwrap()
    );
    assert_eq!(
        src_metadata.accessed().unwrap(),
        des_metadata.accessed().unwrap()
    );
}

#[test]
fn test_preserve_links() {
    let temp_dir = tempdir().unwrap();
    let src_file = temp_dir.path().join("src.txt");
    let link_file = temp_dir.path().join("link.txt");
    let des_file = temp_dir.path().join("des.txt");
    
    fs::write(&src_file, "Content").unwrap();
    std::os::unix::fs::symlink(&src_file, &link_file).unwrap();

    let mut cmd = Command::cargo_bin("pbcp").unwrap();
    cmd.arg("--preserve=links")
        .arg(&link_file)
        .arg("--")
        .arg(&des_file);
    cmd.assert().success();

    assert!(des_file.is_symlink());
    assert_eq!(fs::read_link(&des_file).unwrap(), fs::read_link(&link_file).unwrap());
}

#[test]
fn test_preserve_all() {
    let temp_dir = tempdir().unwrap();
    let src_file = temp_dir.path().join("src.txt");
    let link_file = temp_dir.path().join("link.txt");
    let des_file = temp_dir.path().join("des.txt");
    
    fs::write(&src_file, "Content").unwrap();
    std::os::unix::fs::symlink(&src_file, &link_file).unwrap();

    let mut cmd = Command::cargo_bin("pbcp").unwrap();
    cmd.arg("--preserve=all")
        .arg(&link_file)
        .arg("--")
        .arg(&des_file);
    cmd.assert().success();

    assert!(des_file.is_symlink());
    assert_eq!(fs::read_link(&des_file).unwrap(), fs::read_link(&link_file).unwrap());

    // Test regular file preservation with all attributes
    let des_file2 = temp_dir.path().join("des2.txt");
    let mut cmd = Command::cargo_bin("pbcp").unwrap();
    cmd.arg("--preserve=all")
        .arg(&src_file)
        .arg("--")
        .arg(&des_file2);
    cmd.assert().success();

    let src_metadata = fs::metadata(&src_file).unwrap();
    let des_metadata = fs::metadata(&des_file2).unwrap();

    assert_eq!(src_metadata.permissions(), des_metadata.permissions());
    assert_eq!(src_metadata.uid(), des_metadata.uid());
    assert_eq!(src_metadata.gid(), des_metadata.gid());
    assert_eq!(src_metadata.modified().unwrap(), des_metadata.modified().unwrap());
    assert_eq!(src_metadata.accessed().unwrap(), des_metadata.accessed().unwrap());
}
