use crate::ansi::{ORANGE, VALID};
use ansi::abbrev::{B, BLU, CYA, D};
use std::path::{Path, PathBuf};
use ux::print;

pub struct Symlink {
    pub path: PathBuf,
    pub exist: Exist,
    pub target: PathBuf,
}

pub enum Exist {
    Yes(FileType),
    No,
}

pub enum FileType {
    File,
    Dir,
    Symlink(Result<(), PathBuf>),
}

impl std::fmt::Display for Symlink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{B}{CYA}{path}{D} -> {B}{target}{D}",
            path = self.path.display(),
            target = self.target.display()
        )
    }
}

impl Symlink {
    pub fn create(&self) -> std::io::Result<()> {
        std::os::unix::fs::symlink(&self.target, &self.path)?;
        self.print_action("created", Some(VALID));
        Ok(())
    }

    pub fn remove(&self) -> std::io::Result<()> {
        std::fs::remove_file(&self.path)?;
        self.print_action("removed", Some(ORANGE));
        Ok(())
    }

    pub fn print_action(&self, action: &str, color: Option<&str>) {
        print::start_end(
            format!("{self}").as_str(),
            format!("{c}{action}{D}", c = color.unwrap_or("")).as_str(),
        );
    }
}

pub fn remove_file(path: &Path) -> std::io::Result<()> {
    std::fs::remove_file(path)?;
    print::start_end(
        format!("{B}{path}{D}", path = path.display()).as_str(),
        format!("{ORANGE}removed{D}").as_str(),
    );
    Ok(())
}

pub fn remove_dir(path: &Path) -> std::io::Result<()> {
    std::fs::remove_dir(path)?;
    print::start_end(
        format!("{B}{BLU}{path}{D}", path = path.display()).as_str(),
        format!("{ORANGE}removed{D}").as_str(),
    );
    Ok(())
}
