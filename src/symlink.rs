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

pub struct Symlink<'a> {
    pub path: &'a Path,
    pub target: &'a Path,
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
    match fs::symlink_metadata(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn to_absolute(path: &str) -> Result<PathBuf> {
    let mut path = path.to_string();

    if path.starts_with("~/") {
        let home = std::env::var("HOME").unwrap();
        path = path.replace("~/", &home);
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
    use std::path::Path;

    #[test]
    fn test_symlink() {
        dbg!(std::env::current_dir());

        let l = vec!["noexist", "exist", "sml_exist", "sml_noexist"];
        for p in l {
            let p = to_absolute(p);
            dbg!(p, fs::symlink_metadata(p), p.exists());
        }
    }
}
