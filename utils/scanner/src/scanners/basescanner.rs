use super::super::DirScan;
use anyhow::{Context, Result};
use log::trace;
use std::fs;
use walkdir::WalkDir;

pub struct BaseScanner<'a> {
    des_path: &'a str,
}

impl BaseScanner<'_> {
    pub fn new<'a>(des_path: &'a str) -> BaseScanner<'a> {
        BaseScanner { des_path }
    }
}

impl DirScan for BaseScanner<'_> {
    fn in_scan_action(
        &self,
        cur_entry: &str,
        strip_depth: u32,
    ) -> Result<(Vec<String>, Vec<String>)> {
        let mut src_paths: Vec<String> = Vec::new();
        let mut des_paths: Vec<String> = Vec::new();

        let parent_entry = cur_entry
            .trim_end_matches('/')
            .trim_end_matches(cur_entry.trim_end_matches('/').rsplit('/').next().unwrap());
        trace!("parent_entry: {}", parent_entry);

        let metadata = fs::metadata(cur_entry)
            .with_context(|| format!("failed to read metadata of {}", cur_entry))?;

        if metadata.is_dir() {
            for entry in WalkDir::new(cur_entry).into_iter().filter_map(Result::ok) {
                let src_entry = entry.path().to_str().unwrap();
                trace!("{} found!", src_entry);

                src_paths.push(src_entry.to_string());

                let des_entry =
                    generate_destination_path(src_entry, parent_entry, self.des_path, strip_depth);
                des_paths.push(des_entry);
            }
        } else {
            src_paths.push(cur_entry.to_string());

            let des_entry =
                generate_destination_path(cur_entry, parent_entry, self.des_path, strip_depth);
            des_paths.push(des_entry);
        }

        Ok((src_paths, des_paths))
    }
}

// Helper function to generate destination path with strip_depth
fn generate_destination_path(
    src_path: &str,
    parent_path: &str,
    des_base_path: &str,
    strip_depth: u32,
) -> String {
    if strip_depth == 0 {
        if parent_path.is_empty() {
            format!("{}/{}", des_base_path, src_path)
        } else {
            format!("{}{}", des_base_path, src_path.replace(parent_path, "/"))
        }
    } else {
        let components: Vec<&str> = src_path.split('/').filter(|s| !s.is_empty()).collect();

        if strip_depth as usize >= components.len() {
            des_base_path.to_string()
        } else {
            let stripped_path = components[strip_depth as usize..].join("/");
            format!("{}/{}", des_base_path, stripped_path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, vec};
    use tempfile;

    #[test]
    fn scan_dir_works() {
        let src_dir = tempfile::tempdir_in(".").unwrap();
        let src_dir_path = src_dir.path();
        let src_file_path = src_dir_path.join("my-temporary-note.txt");
        let _file = File::create(&src_file_path).unwrap();

        let des_dir = tempfile::tempdir_in(".").unwrap();
        let des_dir_path = des_dir.path();

        let scanner = BaseScanner::new(des_dir_path.to_str().unwrap());

        let relative_src = src_dir_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let (src_paths, des_paths) = scanner
            .scan(
                &vec![
                    src_dir_path.to_str().unwrap().to_string(),
                    relative_src.clone(),
                ],
                false, // No stripping
            )
            .unwrap();

        println!("src_paths: {:?}\ndes_paths: {:?}", src_paths, des_paths);

        let sfolder = src_dir_path.to_str().unwrap().to_string();
        let sfile = src_file_path.to_str().unwrap().to_string();
        let sfolder_relative = relative_src.clone();
        let sfile_relative = relative_src + "/my-temporary-note.txt";
        assert_eq!(
            src_paths,
            vec![sfolder, sfile, sfolder_relative, sfile_relative]
        );

        let dfolder = des_dir_path.to_str().unwrap().to_string()
            + "/"
            + src_dir_path.to_str().unwrap().rsplit('/').next().unwrap();
        let dfile =
            dfolder.clone() + "/" + src_file_path.to_str().unwrap().rsplit('/').next().unwrap();

        assert_eq!(
            des_paths,
            vec![dfolder.clone(), dfile.clone(), dfolder, dfile]
        );
    }

    #[test]
    fn scan_file_works() {
        let src_dir = tempfile::tempdir_in(".").unwrap();
        let src_dir_path = src_dir.path();
        let src_file_path = src_dir_path.join("my-temporary-note.txt");
        let _file = File::create(&src_file_path).unwrap();

        let des_dir = tempfile::tempdir_in(".").unwrap();
        let des_dir_path = des_dir.path();

        let scanner = BaseScanner::new(des_dir_path.to_str().unwrap());

        let (src_paths, des_paths) = scanner
            .scan(
                &vec![src_file_path.to_str().unwrap().to_string()],
                false, // No stripping
            )
            .unwrap();

        println!("src_paths: {:?}\ndes_paths: {:?}", src_paths, des_paths);

        let sfile = src_file_path.to_str().unwrap().to_string();
        assert_eq!(src_paths, vec![sfile]);

        let dfile = des_dir_path.to_str().unwrap().to_string()
            + "/"
            + src_file_path.to_str().unwrap().rsplit('/').next().unwrap();
        assert_eq!(des_paths, vec![dfile]);
    }

    #[test]
    fn scan_stop_at_link() {
        let temp_dir = tempfile::tempdir_in(".").unwrap();
        let temp_dir_path = temp_dir.path();
        let file_path = temp_dir_path.join("my-temporary-note.txt");
        let _file = File::create(&file_path).unwrap();

        let src_dir = tempfile::tempdir_in(".").unwrap();
        let src_dir_path = src_dir.path();
        let src_link_path = src_dir_path.join("my-temporary-note.link");
        let _link = std::os::unix::fs::symlink(temp_dir_path, &src_link_path).unwrap();

        let des_dir = tempfile::tempdir_in(".").unwrap();
        let des_dir_path = des_dir.path();

        let scanner = BaseScanner::new(des_dir_path.to_str().unwrap());
        let (src_paths, des_paths) = scanner
            .scan(
                &vec![src_dir_path.to_str().unwrap().to_string()],
                false, // No stripping
            )
            .unwrap();
        println!("src_paths: {:?}\ndes_paths: {:?}", src_paths, des_paths);

        let sfolder = src_dir_path.to_str().unwrap().to_string();
        let slink = src_link_path.to_str().unwrap().to_string();
        assert_eq!(src_paths, vec![sfolder, slink]);

        let dfolder = des_dir_path.to_str().unwrap().to_string()
            + "/"
            + src_dir_path.to_str().unwrap().rsplit('/').next().unwrap();
        let dlink =
            dfolder.clone() + "/" + src_link_path.to_str().unwrap().rsplit('/').next().unwrap();
        assert_eq!(des_paths, vec![dfolder, dlink]);
    }

    #[test]
    fn scan_nonexistent_dir() {
        let des_dr = tempfile::tempdir_in(".").unwrap();
        let des_dir_path = des_dr.path();

        let scanner = BaseScanner::new(des_dir_path.to_str().unwrap());
        let ret = scanner.scan(
            &vec!["nonexistent".to_string()],
            false, // No stripping
        );
        assert!(ret.is_err());
    }

    #[test]
    fn scan_with_boolean_strip() {
        let temp_dir = tempfile::tempdir_in(".").unwrap();
        let temp_dir_path = temp_dir.path().join("test-dir");
        let file_path = temp_dir_path.join("test-file.txt");
        fs::create_dir_all(&temp_dir_path).unwrap();
        let _file = File::create(&file_path).unwrap();

        let des_dir = tempfile::tempdir_in(".").unwrap();
        let des_dir_path = des_dir.path();

        let scanner = BaseScanner::new(des_dir_path.to_str().unwrap());
        let src_path = temp_dir_path.to_str().unwrap();

        // Test with strip = false (include the parent directory)
        let (src_paths_no_strip, des_paths_no_strip) = scanner
            .scan(
                &vec![src_path.to_string()],
                false, // No stripping
            )
            .unwrap();

        println!("With strip=false (include the parent directory):");
        println!(
            "src_paths: {:?}\ndes_paths: {:?}",
            src_paths_no_strip, des_paths_no_strip
        );

        // Test with strip = true (exclude the parent directory)
        let (src_paths_strip, des_paths_strip) = scanner
            .scan(
                &vec![src_path.to_string()],
                true, // Strip parent directory
            )
            .unwrap();

        println!("With strip=true (exclude the parent directory):");
        println!(
            "src_paths: {:?}\ndes_paths: {:?}",
            src_paths_strip, des_paths_strip
        );

        // Get the components of the path
        let components: Vec<&str> = src_path.split('/').filter(|s| !s.is_empty()).collect();

        if components.len() <= 1 {
            // If there's only one component, both results should be the same
            assert_eq!(des_paths_no_strip, des_paths_strip);
        } else {
            // With strip=true, the first path should be just the destination directory
            assert_eq!(des_paths_strip[0], des_dir_path.to_str().unwrap());

            // With strip=false, the first path should include the directory name
            let dir_name = temp_dir_path.file_name().unwrap().to_str().unwrap();
            assert!(des_paths_no_strip[0].ends_with(dir_name));

            // The two paths should be different
            assert_ne!(des_paths_no_strip[0], des_paths_strip[0]);

            // For the file inside the directory:
            // With strip=true, the file path should still include the filename
            assert!(des_paths_strip
                .iter()
                .any(|path| path.ends_with("test-file.txt")));
        }
    }
}
