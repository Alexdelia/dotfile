use crate::ansi::{BW, M, VALID};
use crate::env::{Env, EnvType, Update};
use crate::unix::{Exist, FileType, Symlink};
use ansi::abbrev::{B, D, G, I, N_C, R};
use std::io::Write;
use std::path::PathBuf;

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
                                "should {M}{}{D} be updated?\t{I}(update = 'optional'){D}",
                                grouped.title
                            ))
                    }
                    Update::Specific(name) => name.contains(
                        &hostname::get()
                            .expect("get hostname failed")
                            .into_string()
                            .expect("hostname to str failed"),
                    ),
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
    print!("{} {B}[{G}y{N_C}/{R}n{N_C}]{D} ", action);
    std::io::stdout().flush().expect("flush failed");
    let g = getch::Getch::new();

    loop {
        let c = g.getch().expect("getch failed") as char;

        if c == 'y' || c == 'Y' || c == '\n' {
            println!();
            return true;
        } else if c == 'n' || c == 'N' {
            println!();
            return false;
        }

        print!("\nwaiting for '{G}y{D}' ({G}yes{D}) or '{R}n{D}' ({R}no{D}), not '{M}{c}{D}' ",);
        std::io::stdout().flush().expect("flush failed");
    }
}

fn handle(symlink: &Symlink, interactive: bool) -> Result<(), std::io::Error> {
    match &symlink.exist {
        Exist::Yes(p) => match p {
            FileType::Symlink(target) => handle_symlink(symlink, target, interactive),
            FileType::File => handle_file(symlink),
            FileType::Dir => handle_dir(symlink),
        },
        Exist::No => symlink.create(),
    }
}

fn handle_symlink(
    symlink: &Symlink,
    target: &Result<(), PathBuf>,
    interactive: bool,
) -> Result<(), std::io::Error> {
    match target {
        Ok(_) => {
            symlink.print_action("nothing to do", Some(VALID));
            Ok(())
        }
        Err(target) => {
            if interactive
                && !ask(&format!(
                    "found already existing symlink:
\t{M}{0:?}{D} -> {BW}{target:?}{D}
should it be replaced with:
\t{M}{0:?}{D} -> {VALID}{1:?}{D}
?",
                    symlink.path, symlink.target,
                ))
            {
                return Ok(());
            }

            Symlink {
                path: symlink.path.clone(),
                exist: Exist::No,
                target: target.clone(),
            }
            .remove()?;

            symlink.create()
        }
    }
}

fn handle_file(symlink: &Symlink) -> Result<(), std::io::Error> {
    todo!();
}

fn handle_dir(symlink: &Symlink) -> Result<(), std::io::Error> {
    todo!();
}
