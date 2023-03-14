// need to know if file exist
// yes:
//   is it a symlink?
//   yes:
//     does it point to the same file?
//     yes:
//       do nothing
//     no:
//       update symlink
//   no:
//     gring up difference between file and file in data
//     create symlink depending on args
// no:
//   create symlink

use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub const DEFAULT_SYMLINK_FILE: &str = "symlink.toml";

pub type Env = Vec<EnvType>;

#[derive(Debug)] // TODO: remove
pub enum EnvType {
    Grouped(Grouped),
    Alone(Symlink),
}

#[derive(Debug)] // TODO: remove
pub struct Grouped {
    pub title: String,
    pub update: Update,
    pub symlink: Vec<Symlink>,
}

#[derive(Debug)] // TODO: remove
pub enum Update {
    Always,
    Never,
    Optional,
    Specific(Vec<String>),
}

impl FromStr for Update {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "always" => Ok(Update::Always),
            "never" => Ok(Update::Never),
            "optional" => Ok(Update::Optional),
            _ => Err(format!("{s} is not a valid update value")),
        }
    }
}

#[derive(Debug)] // TODO: remove
pub struct Symlink {
    pub path: PathBuf,
    pub exist: Exist,
    pub target: PathBuf,
}

#[derive(Debug)] // TODO: remove
pub enum Exist {
    Yes(FileType),
    No,
}

#[derive(Debug)] // TODO: remove
pub enum FileType {
    File,
    Dir,
    Symlink(bool),
}

pub fn symlink(symlink: Symlink) -> Result<()> {
    match fs::symlink_metadata(&symlink.path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                println!("{:?} is a symlink", symlink.path);
            } else {
                println!("{:?} is not a symlink", symlink.path);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
    // let metadata = fs::symlink_metadata(&symlink.path)
}

fn exist(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok()
}

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
// if path.starts_with("~/") {
// 	let home = std::env::var("HOME").unwrap();
// 	let path = path.replace("~/", &home);
// 	Path::new(&path)
// } else {
// 	Path::new(path)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fmt::format, path::Path};

    #[test]
    fn test_symlink() {
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
}
