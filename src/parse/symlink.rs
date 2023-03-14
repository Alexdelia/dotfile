use super::error::ParseTomlError;
use crate::ansi::{BE, W};
use crate::env::{Env, EnvType, Exist, FileType, Symlink};
use ansi::abbrev::{B, D};
use miette::{Diagnostic, IntoDiagnostic, Result};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub fn end_build(file: String, env: &mut Env) -> Result<()> {
    for e in env {
        match e {
            EnvType::Grouped(grouped) => {
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
                if let Err(e) = process(symlink) {
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
    path_pattern(symlink)?;
    symlink.path = to_absolute(&symlink.path)?;
    symlink.target = to_absolute(&symlink.target)?;

    exist(&symlink.target)?;
    symlink.exist = find_exist_type(&symlink.path, &symlink.target)?;

    Ok(())
}

fn path_pattern(symlink: &mut Symlink) -> Result<()> {
    if symlink.path.ends_with("*") || symlink.target.ends_with("*") {
        eprintln!(
            "{W}wildcard ({D}{B}*{D}{W}) is not supported yet{D}\t({} = {})",
            symlink.path.to_string_lossy(),
            symlink.target.to_string_lossy()
        );
    }

    let s = symlink.target.to_string_lossy();
    if !s.starts_with('/') && !s.starts_with('~') {
        let root = PathBuf::from(format!(
            "{}/data/",
            std::env::current_dir().into_diagnostic()?.to_string_lossy()
        ));

        assert!(
            root.is_absolute() && root.exists(),
            "{} should exist and have accessible permissions, it is supposed to contain the dotfile",
            root.to_string_lossy()
        );

        symlink.target = root.join(&symlink.target);
    }

    Ok(())
}

#[derive(Error, Diagnostic, Debug)]
#[error("error with {BE}{path}{D}")]
#[diagnostic(code(parse::symlink))]
struct ParseSymlinkError {
    #[source]
    source: std::io::Error,

    path: String,

    #[help]
    help: Option<String>,
}

fn exist(path: &Path) -> Result<()> {
    match path.try_exists() {
        Ok(e) => {
            if !e {
                Err(ParseSymlinkError {
                    source: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
                    path: path.to_string_lossy().to_string(),
                    help: None,
                }
                .into())
            } else {
                Ok(())
            }
        }
        Err(e) => Err(ParseSymlinkError {
            source: e,
            path: path.to_string_lossy().to_string(),
            help: None,
        }
        .into()),
    }
}

fn to_absolute(path: &Path) -> Result<PathBuf> {
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
            help: None,
        }
        .into())
    }
}

fn find_exist_type(path: &Path, expected_target: &Path) -> Result<Exist> {
    let Ok(md) = fs::symlink_metadata(path) else {
		return Ok(Exist::No);
	};

    if md.is_symlink() {
        if std::fs::read_link(path).into_diagnostic()? == expected_target {
            Ok(Exist::Yes(FileType::Symlink(true)))
        } else {
            Ok(Exist::Yes(FileType::Symlink(false)))
        }
    } else if md.is_file() {
        Ok(Exist::Yes(FileType::File))
    } else if md.is_dir() {
        Ok(Exist::Yes(FileType::Dir))
    } else {
        Err(ParseSymlinkError {
            source: std::io::Error::new(
                std::io::ErrorKind::Other,
                "did not manage to determine the type of the file",
            ),
            path: path.to_string_lossy().to_string(),
            help: Some(format!("metadata:\n{:?}", md)),
        }
        .into())
    }
}
