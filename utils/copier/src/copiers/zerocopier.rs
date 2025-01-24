use super::super::FileCopy;
use std::os::fd::{AsRawFd, RawFd};
pub struct Copier {
    buf_sz: usize,
}

impl Copier {
    pub fn new(buf_sz: usize) -> Self {
        Self { buf_sz }
    }

    fn zero_copy(sfd: RawFd, dfd: RawFd, count: usize) -> std::io::Result<u64> {
        let ret = unsafe { libc::sendfile(dfd, sfd, std::ptr::null_mut(), count) };
        if ret < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(ret as u64)
        }
    }
}

impl FileCopy for Copier {
    fn simple_copy_once(
        &mut self,
        src: &mut std::fs::File,
        des: &mut std::fs::File,
    ) -> std::io::Result<u64> {
        let sfd = src.as_raw_fd();
        let dfd = des.as_raw_fd();

        Self::zero_copy(sfd, dfd, self.buf_sz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Read, Write};
    use tempfile;

    #[test]
    fn copy_file_works() {
        let test_str = String::from("copy_file_works test content!");
        let mut copier = Copier::new(4096 * 1024);

        let temp_dir = tempfile::tempdir_in(".").unwrap();
        let temp_dir_path = temp_dir.path();

        let src_file_path = temp_dir_path.join("my-temporary-note.txt");

        let bind = temp_dir_path.to_str().unwrap().to_string() + "/dest.txt";
        let des_file_path = std::path::Path::new(&bind);

        {
            let mut src_file = File::create(&src_file_path).unwrap();
            write!(src_file, "{}", &test_str).unwrap();
        }

        {
            let src_file_reopen = File::open(&src_file_path).unwrap();
            let des_file = File::create(&des_file_path).unwrap();

            print!("from {} to {}", src_file_path.display(), des_file_path.display());

            let ret = copier.copy(src_file_reopen, des_file, None, None).unwrap();

            println!(", {} bytes copied.", ret);
        }

        let mut des_file = File::open(&des_file_path).unwrap();
        let mut des_content = String::new();

        des_file.read_to_string(&mut des_content).unwrap();

        assert_eq!(test_str, des_content);
    }
}

