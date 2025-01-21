use anyhow::Context;
use log::trace;
use std::fs;
use walkdir::WalkDir;

pub fn scan_dir(srcs: &[String]) -> anyhow::Result<Vec<String>> {
    let mut new_srcs: Vec<String> = Vec::with_capacity(srcs.len() * 2);
    for src in srcs {
        if fs::metadata(src)
            .with_context(|| format!("failed to read metadata of {}", src))?
            .is_dir()
        {
            for entry in WalkDir::new(src).into_iter().filter_map(Result::ok) {
                trace!("{}", entry.path().to_str().unwrap());
                new_srcs.push(entry.path().to_str().unwrap().to_string());
            }
        } else {
            new_srcs.push(src.clone());
        }
    }
    Ok(new_srcs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile;

    #[test]
    fn scan_dir_works() {
        let temp_dir = tempfile::tempdir_in(".").unwrap();
        let file_path = temp_dir.path().join("my-temporary-note.txt");
        let _file = File::create(&file_path).unwrap();
        let ret = scan_dir(&vec![temp_dir.path().to_str().unwrap().to_string()]).unwrap();
        println!("{:?}", ret);
        assert_eq!(
            ret,
            vec![
                temp_dir.path().to_str().unwrap().to_string(),
                file_path.to_str().unwrap().to_string()
            ]
        );
    }

    #[test]
    fn scan_stop_at_link() {
        let temp_dir1 = tempfile::tempdir_in(".").unwrap();
        let temp_dir2 = tempfile::tempdir_in(".").unwrap();

        let file_path = temp_dir1.path().join("my-temporary-note.txt");
        let _file = File::create(&file_path).unwrap();

        let link_path = temp_dir2.path().join("my-temporary-note.link");
        let _link = std::os::unix::fs::symlink(temp_dir1.path(), &link_path).unwrap();

        let ret = scan_dir(&vec![temp_dir2.path().to_str().unwrap().to_string()]).unwrap();
        println!("{:?}", ret);
        assert_eq!(
            ret,
            vec![
                temp_dir2.path().to_str().unwrap().to_string(),
                link_path.to_str().unwrap().to_string()
            ]
        );
    }

    #[test]
    fn scan_nonexistent_dir() {
        let ret = scan_dir(&vec!["nonexistent_dir".to_string()]);
        assert!(ret.is_err());
    }
}
