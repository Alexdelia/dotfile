// use super::error::ParseError;
use crate::symlink::{Env, EnvType, Symlink, SymlinkExist, Target};
use miette::{Diagnostic, Result};
use std::path::{Path, PathBuf};
use thiserror::Error;

pub fn process(env: &Env) -> Result<()> {
    for e in env {
        match e {
            EnvType::Grouped(grouped) => {
                for symlink in &grouped.symlink {
                    sub_process(symlink, Some(grouped.title))?;
                }
            }
            EnvType::Alone(symlink) => {
                sub_process(symlink, None)?;
            }
        }
    }

    Ok(())
}

fn sub_process(symlink: &Symlink, title: Option<String>) -> Result<()> {
    symlink.path = to_absolute(&symlink.path)?;
    symlink.target.path = to_absolute(&symlink.target.path)?;

    exist(&symlink.path, title)?;
    symlink.target.exist = find_exist_type(&symlink.target.path)?;

    Ok(())
}

#[derive(Error, Diagnostic, Debug)]
// #[error("could not {BW}read {M}{file}{D}")]
#[error("could not read {path}")]
#[diagnostic(
    code(parse::read),
    // url("{}{}", URL, file!()),
    // help("the file {M}{file}{D} is the file that define symlink")
)]
struct ParseSymlinkError {
    #[source]
    source: std::io::Error,
    title: Option<String>, // search doing it with a wrap or context
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

fn exist(path: &Path, title: Option<String>) -> Result<()> {
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

fn to_absolute(path: &PathBuf) -> Result<PathBuf> {
    let mut path = path.to_string_lossy().to_string();

    if path.starts_with('~') {
        let home = std::env::var("HOME").unwrap();
        path = path.replacen('~', &home, 1);
    }

    let path = PathBuf::from(path);
    if path.is_absolute() {
        Ok(path)
    } else {
        Err(ParseSymlinkError {
            source: std::io::Error::new(
                std::io::ErrorKind::Other,
                "did not manage to convert to absolute path",
            ),
            path: path.to_string_lossy().to_string(),
        }
        .into())
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
