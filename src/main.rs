use anyhow::Context;
use clap::Parser;
use log::debug;
use scanner::DirScan;

/// rs_cp - copy files
#[derive(Parser, Debug)]
#[command(
    version,
    about = "rs_cp - copy files",
    long_about = "rs_cp - copy files"
)]
struct Args {
    /// Copy from SRCS... to DES
    srcs_des: Vec<String>,
    /// recursive copy
    #[arg(short, long)]
    recursive: bool,
}

impl Args {
    fn check(&self) -> anyhow::Result<()> {
        if self.srcs_des.len() < 2 {
            Err(anyhow::anyhow!("Need at least a src and a des"))
        } else {
            Ok(())
        }
    }

    fn apart_srcs_des(&self) -> anyhow::Result<(Vec<String>, Vec<String>)> {
        let len = self.srcs_des.len();
        let src_paths = &self.srcs_des[0..(len - 1)];
        let des = &self.srcs_des[len - 1];

        let is_des_dir = std::fs::metadata(des)
            .with_context(|| format!("Failed to get metadata of {}", des))?
            .is_dir();

        match len {
            2 => {
                let is_src_dir = std::fs::metadata(&src_paths[0])
                                .with_context(|| format!("Failed to get metadata of {}", &src_paths[0]))?
                                .is_dir();

                match (is_src_dir, is_des_dir) {
                    (false, true) => {
                        Ok((
                            vec![src_paths[0].clone()],
                            vec![des.clone() + "/" + src_paths[0].rsplit('/').next().unwrap()],
                        ))
                    }
                    (false, false) => {
                        Ok((vec![src_paths[0].clone()], vec![des.clone()]))
                    }
                    (true, true) => {
                        if self.recursive {
                            let scanner = scanner::scanners::basescanner::BaseScanner::new(des);
                            let (src_paths, des_paths) = scanner.scan(src_paths)?;
    
                            Ok((src_paths, des_paths))
                        } else {
                            Err(anyhow::anyhow!(
                                "{} is a directory, should specify -r", src_paths[0]
                            ))
                        }
                    }
                    (true, false) => {
                        Err(anyhow::anyhow!(
                            "The last argument should be a directory when have more than 2 sources"
                        ))
                    }
                }
            }
            _ => {
                if is_des_dir {
                    if self.recursive {
                        let scanner = scanner::scanners::basescanner::BaseScanner::new(des);
                        let (src_paths, des_paths) = scanner.scan(src_paths)?;

                        Ok((src_paths, des_paths))
                    } else {
                        let mut des_paths = Vec::new();
                        for src in src_paths {
                            let is_src_dir = std::fs::metadata(src)
                                .with_context(|| format!("Failed to get metadata of {}", src))?
                                .is_dir();
                            if is_src_dir {
                                return Err(anyhow::anyhow!(
                                    "The last argument should be a directory when have more than 2 sources"
                                ));
                            } else {
                                des_paths.push(des.clone() + "/" + src.rsplit('/').next().unwrap());
                            }
                        }
                        Ok((src_paths.to_vec(), des_paths))
                    }
                } else {
                    Err(anyhow::anyhow!(
                        "The last argument should be a directory when have more than 2 sources"
                    ))
                }
            }
        }
    }
}

fn do_pbcopy(src_paths: &[String], des_paths: &[String], _args: Args) -> anyhow::Result<()> {
    for (src, des) in src_paths.iter().zip(des_paths.iter()) {
        println!("Copy from {} to {}", src, des);
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    env_logger::init();

    debug!("{:?}", args);
    args.check()?;

    let (src_paths, des_paths) = args.apart_srcs_des()?;

    debug!("src_paths: {:?}\ndes_paths: {:?}", src_paths, des_paths);

    Ok(do_pbcopy(&src_paths, &des_paths, args)?)
}
