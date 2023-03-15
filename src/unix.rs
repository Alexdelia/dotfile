use crate::ansi::VALID;
use ansi::abbrev::{B, BLU, CYA, D};
use std::path::PathBuf;

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
    Symlink(Result<(), PathBuf>),
}

impl std::fmt::Display for Symlink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{B}{CYA}{}{D} -> {B}{}{D}",
            self.path.display(),
            self.target.display()
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
        self.print_action("removed", Some(BLU));
        Ok(())
    }

    pub fn print_action(&self, action: &str, color: Option<&str>) {
        print_action(&format!("{self}"), action, color);
    }
}

fn print_action(start: &str, end: &str, color: Option<&str>) {
    let termsize::Size { cols, .. } =
        termsize::get().unwrap_or(termsize::Size { cols: 0, rows: 0 });

    let color = color.unwrap_or("");

    if start.len() + end.len() >= cols as usize {
        println!("{start}\n{color}{end}{D}");
    } else {
        println!(
            "{start} {color}{end:>w$}{D}",
            w = cols as usize - 1 - start.len()
        );
    };
}
