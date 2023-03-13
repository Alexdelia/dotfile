use super::error::ParseTomlError;
use crate::ansi::{BE, BW};
use crate::symlink::{Env, EnvType, Exist, Symlink, Target};
use ansi::abbrev::D;
use miette::{Diagnostic, Result};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub fn end_build(file: String, env: &Env) -> Result<()> {
    for e in env {
        match e {
            EnvType::Grouped(mut grouped) => {
                for symlink in &mut grouped.symlink {
                    if let Err(e) = process(symlink) {
                        return Err(ParseTomlError::new(
                            file,
                            Some(&grouped.title),
                            symlink.path.to_string_lossy().to_string().as_str(),
                            e.to_string(),
                            String::new(),
                            file!().to_string(),
                            Some(vec![e]),
                        )
                        .into());
                    }
                }
            }
            EnvType::Alone(symlink) => {
                if let Err(e) = process(&mut symlink) {
                    return Err(ParseTomlError::new(
                        file,
                        None,
                        symlink.path.to_string_lossy().to_string().as_str(),
                        e.to_string(),
                        String::new(),
                        file!().to_string(),
                        Some(vec![e]),
                    )
                    .into());
                }
            }
        }
    }

    Ok(())
}

fn process(symlink: &mut Symlink) -> Result<()> {
    check_path_pattern(symlink)?;
    symlink.path = to_absolute(&symlink.path)?;
    symlink.target.path = to_absolute(&symlink.target.path)?;

    exist(&symlink.path)?;
    symlink.target.exist = find_exist_type(&symlink.target.path)?;

    Ok(())
}

fn check_path_pattern(symlink: &Symlink) -> Result<()> {
    todo!();

    // check if can append current dir data to target path
}

#[derive(Error, Diagnostic, Debug)]
#[error("could not {BW}read {BE}{path}{D}")]
#[diagnostic(code(parse::symlink))]
struct ParseSymlinkError {
    #[source]
    source: std::io::Error,
    path: String,
}

fn exist(path: &Path) -> Result<()> {
    let e = match path.try_exists() {
        Ok(e) => {
            if !e {
                ParseSymlinkError {
                    source: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
                    path: path.to_string_lossy().to_string(),
                }
            } else {
                return Ok(());
            }
        }
        Err(e) => ParseSymlinkError {
            source: e,
            path: path.to_string_lossy().to_string(),
        },
    };

    // Err(ParseTomlError::new(

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

fn find_exist_type(path: &Path) -> Result<Exist> {
    todo!();

    // dbg!(std::env::current_dir());

    // let l = vec!["noexist", "exist", "sml_exist", "sml_noexist"];
    // for p in l {
    //     dbg!(p);
    //     let p = to_absolute(format!("~/goinfre/{}", p).as_str());
    //     dbg!(&p);
    //     if p.is_ok() {
    //         dbg!(
    //             fs::symlink_metadata(p.as_ref().unwrap()),
    //             p.unwrap().exists()
    //         );
    //     }
    // }
}
