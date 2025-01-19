use log::trace;
use std::fs;
use walkdir::WalkDir;

pub fn scan_dir(srcs: &[String]) -> Result<Vec<String>, std::io::Error> {
    let mut new_srcs: Vec<String> = Vec::with_capacity(srcs.len() * 2);
    for src in srcs {
        if fs::metadata(src)?.is_dir() {
            for entry in WalkDir::new(src)
                .into_iter()
                .filter_map(Result::ok)
            {
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

    #[test]
    fn scan_dir_works() {
        let ret = scan_dir(&vec!["../../utils".to_string(), "../../README.md".to_string()]).unwrap();
        // println!("{:?}", ret);
    }
}
