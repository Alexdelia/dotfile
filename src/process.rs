use crate::ansi::VALID;
use crate::env::{Env, EnvType, Exist, FileType, Symlink};

pub fn process(env: Env) -> Result<(), std::io::Error> {
    for e in env {
        match e {
            EnvType::Grouped(grouped) => {
                for symlink in grouped.symlink {
                    handle(&symlink)?;
                }
            }
            EnvType::Alone(symlink) => {
                handle(&symlink)?;
            }
        }
    }

    Ok(())
}

fn handle(symlink: &Symlink) -> Result<(), std::io::Error> {
    match &symlink.exist {
        Exist::Yes(p) => match p {
            FileType::Symlink(b) => handle_symlink(symlink, *b),
            FileType::File => handle_file(symlink),
            FileType::Dir => handle_dir(symlink),
        },
        Exist::No => symlink.create(),
    }
}

fn handle_symlink(symlink: &Symlink, same_target: bool) -> Result<(), std::io::Error> {
    if same_target {
        symlink.print_action("nothing to do", Some(VALID));
        return Ok(());
    }

    // symlink target is not the same
    todo!();
}

fn handle_file(symlink: &Symlink) -> Result<(), std::io::Error> {
    todo!();
}

fn handle_dir(symlink: &Symlink) -> Result<(), std::io::Error> {
    todo!();
}
