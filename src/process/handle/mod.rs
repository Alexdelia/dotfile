mod dir;
use dir::handle_dir;
mod file;
use file::handle_file;
mod symlink;
use symlink::handle_symlink;

use crate::unix::{Exist, FileType, Symlink};

pub fn handle(symlink: &Symlink, interactive: bool) -> Result<(), std::io::Error> {
    match &symlink.exist {
        Exist::Yes(p) => match p {
            FileType::Symlink(target) => handle_symlink(symlink, target, interactive),
            FileType::File => handle_file(symlink, interactive),
            FileType::Dir => handle_dir(symlink, interactive),
        },
        Exist::No => symlink.create(),
    }
}
