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
    assert_eq!(
        fs::read_link(&des_file).unwrap(),
        fs::read_link(&link_file).unwrap()
    );
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
    assert_eq!(
        fs::read_link(&des_file).unwrap(),
        fs::read_link(&link_file).unwrap()
    );

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
fn test_archive_option() {
    let temp_dir = tempfile::tempdir_in(".").unwrap();

    // Create a directory structure
    let src_dir = temp_dir.path().join("src_dir");
    let sub_dir = src_dir.join("sub_dir");
    fs::create_dir_all(&sub_dir).unwrap();

    println!("Source directory: {:?}", src_dir);
    println!("Sub directory: {:?}", sub_dir);

    // Create files in both directories
    let src_file = src_dir.join("src.txt");
    let sub_file = sub_dir.join("sub.txt");
    let link_file = src_dir.join("link.txt");

    println!("Source file: {:?}", src_file);
    println!("Sub file: {:?}", sub_file);
    println!("Link file: {:?}", link_file);

    fs::write(&src_file, "Main content").unwrap();
    fs::write(&sub_file, "Sub content").unwrap();
    std::os::unix::fs::symlink(&src_file, &link_file).unwrap();

    // Set some custom permissions and timestamps
    let mut perms = fs::metadata(&src_file).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(&src_file, perms).unwrap();

    // Create destination directory
    let des_dir = temp_dir.path().join("des_dir");
    fs::create_dir_all(&des_dir).unwrap();
    println!("Destination directory: {:?}", des_dir);

    // Run the command with --archive option
    let mut cmd = Command::cargo_bin("pbcp").unwrap();
    cmd.arg("-a").arg(&src_dir).arg("--").arg(&des_dir);
    cmd.assert().success();

    // Verify directory structure is preserved
    assert!(des_dir.is_dir());

    // The source directory is copied as a subdirectory of the destination
    let des_src_dir = des_dir.join(src_dir.file_name().unwrap());
    println!("Destination source directory: {:?}", des_src_dir);
    assert!(des_src_dir.is_dir());
    assert!(des_src_dir.join("sub_dir").is_dir());

    // Verify files are copied
    let des_src_file = des_src_dir.join("src.txt");
    let des_sub_file = des_src_dir.join("sub_dir").join("sub.txt");
    println!("Destination source file: {:?}", des_src_file);
    println!("Destination sub file: {:?}", des_sub_file);
    assert!(des_src_file.exists());
    assert!(des_sub_file.exists());

    // Verify file contents
    assert_eq!(fs::read_to_string(&des_src_file).unwrap(), "Main content");
    assert_eq!(fs::read_to_string(&des_sub_file).unwrap(), "Sub content");

    // Verify symlink is preserved
    let des_link = des_src_dir.join("link.txt");
    println!("Destination link file: {:?}", des_link);
    assert!(des_link.is_symlink());
    assert_eq!(
        fs::read_link(&des_link).unwrap(),
        fs::read_link(&link_file).unwrap()
    );

    // Verify permissions are preserved
    let src_perms = fs::metadata(&src_file).unwrap().permissions();
    let des_perms = fs::metadata(&des_src_file).unwrap().permissions();
    assert_eq!(src_perms.readonly(), des_perms.readonly());

    // Verify ownership is preserved (if running as root)
    let src_metadata = fs::metadata(&src_file).unwrap();
    let des_metadata = fs::metadata(&des_src_file).unwrap();
    assert_eq!(src_metadata.uid(), des_metadata.uid());
    assert_eq!(src_metadata.gid(), des_metadata.gid());

    // Verify timestamps are preserved (with some tolerance for filesystem differences)
    let src_mtime = src_metadata.modified().unwrap();
    let des_mtime = des_metadata.modified().unwrap();
    let difference = if src_mtime > des_mtime {
        src_mtime.duration_since(des_mtime).unwrap()
    } else {
        des_mtime.duration_since(src_mtime).unwrap()
    };
    assert!(difference.as_secs() < 2); // Allow 2 seconds difference
}

#[test]
fn test_archive_option_with_symlink() {
    let temp_dir = tempfile::tempdir_in(".").unwrap();

    // Create a directory structure
    let src_dir = temp_dir.path().join("src_dir");
    fs::create_dir_all(&src_dir).unwrap();

    // Create files
    let src_file = src_dir.join("src.txt");
    let link_file = src_dir.join("link.txt");

    fs::write(&src_file, "Main content").unwrap();
    // Create a symlink to the file, using the relative path
    std::os::unix::fs::symlink(&src_file.file_name().unwrap(), &link_file).unwrap();

    // Create destination directory
    let des_dir = temp_dir.path().join("des_dir");
    fs::create_dir_all(&des_dir).unwrap();

    // Run the command with --archive option
    let mut cmd = Command::cargo_bin("pbcp").unwrap();
    cmd.arg("-a")
        .arg(&link_file)
        .arg(&src_file)
        .arg("--")
        .arg(&des_dir);
    cmd.assert().success();

    // Verify files are copied
    let des_src_file = des_dir.join("src.txt");
    let des_link = des_dir.join("link.txt");

    assert!(des_src_file.exists());
    assert!(des_link.is_symlink());
    assert_eq!(
        fs::read_link(&des_link).unwrap(),
        fs::read_link(&link_file).unwrap()
    );
}
