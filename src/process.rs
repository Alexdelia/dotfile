use crate::ansi::{M, VALID};
use crate::env::{Env, EnvType, Exist, FileType, Symlink, Update};
use ansi::abbrev::D;
use std::io::Read;

pub fn process(env: Env, interactive: bool) -> Result<(), std::io::Error> {
    for e in env {
        match e {
            EnvType::Grouped(grouped) => {
                if match grouped.update {
                    Update::Always => true,
                    Update::Never => false,
                    Update::Optional => {
                        interactive
                            && ask(&format!(
                                "should {M}{}{D} be updated?\t(update = 'optional')",
                                grouped.title
                            ))
                    }
                    Update::Specific(_) => todo!(),
                } {
                    for symlink in grouped.symlink {
                        handle(&symlink, interactive)?;
                    }
                }
            }
            EnvType::Alone(symlink) => {
                handle(&symlink, interactive)?;
            }
        }
    }

    Ok(())
}

fn ask(action: &str) -> bool {
    print!("{} [y/n] ", action);

    let mut buf = [0u8; 1];
    loop {
        std::io::stdin().read_exact(&mut buf).unwrap();

        if buf[0] == b'y' || buf[0] == b'Y' {
            println!();
            return true;
        } else if buf[0] == b'n' || buf[0] == b'N' {
            println!();
            return false;
        }

        print!("\nwaiting for 'y' (yes) or 'n' (no), not '{}' ", buf[0]);
    }
}

fn handle(symlink: &Symlink, interactive: bool) -> Result<(), std::io::Error> {
    match &symlink.exist {
        Exist::Yes(p) => match p {
            FileType::Symlink(b) => handle_symlink(symlink, *b, interactive),
            FileType::File => handle_file(symlink),
            FileType::Dir => handle_dir(symlink),
        },
        Exist::No => symlink.create(),
    }
}

fn handle_symlink(
    symlink: &Symlink,
    same_target: bool,
    interactive: bool,
) -> Result<(), std::io::Error> {
    todo!();
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
