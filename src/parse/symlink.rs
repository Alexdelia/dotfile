// use super::error::ParseError;
use crate::symlink::{Env, EnvType, SymlinkExist, Target};
use miette::{Diagnostic, Result};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
// #[error("could not {BW}read {M}{file}{D}")]
#[error("could not read {file}")]
#[diagnostic(
    code(parse::read),
    // url("{}{}", URL, file!()),
    // help("the file {M}{file}{D} is the file that define symlink")
)]
struct ParseSymlinkError {
    #[source]
    source: std::io::Error,
    path: String,
}

pub fn validate(env: &Env) -> Result<()> {
    for e in env {
        match e {
            EnvType::Grouped(grouped) => {
                for symlink in &grouped.symlink {
                    exist(&symlink.path)?;
                    symlink.target.exist = find_exist_type(&symlink.target.path)?;
                }
            }
            EnvType::Alone(symlink) => {
                exist(&symlink.path)?;
                symlink.target.exist = find_exist_type(&symlink.target.path)?;
            }
        }
    }

    Ok(())
}

fn exist(path: &Path) -> Result<()> {
    if !path.try_exists().map_err(|e| ParseSymlinkError {
        source: e,
        path: path.to_string_lossy().to_string(),
    })? {
        return Err(ParseSymlinkError {
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
            path: path.to_string_lossy().to_string(),
        }
        .into());
    }

    Ok(())
}

// to update arg and return value
fn to_absolute(path: &str) -> Result<PathBuf> {
    let mut path = path.to_string();

    if path.starts_with('~') {
        let home = std::env::var("HOME").unwrap();
        path = path.replacen('~', &home, 1);
    }

    let path = Path::new(&path);
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "did not manage to convert to absolute path",
        ))
    }
}

fn find_exist_type(path: &Path) -> Result<SymlinkExist> {
    dbg!(std::env::current_dir());

    let l = vec!["noexist", "exist", "sml_exist", "sml_noexist"];
    for p in l {
        dbg!(p);
        let p = to_absolute(format!("~/goinfre/{}", p).as_str());
        dbg!(&p);
        if p.is_ok() {
            dbg!(
                fs::symlink_metadata(p.as_ref().unwrap()),
                p.unwrap().exists()
            );
        }
    }
}
