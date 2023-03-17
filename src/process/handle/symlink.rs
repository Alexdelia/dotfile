use crate::ansi::{BW, M, VALID};
use crate::unix::{Exist, Symlink};
use ansi::abbrev::D;
use std::path::PathBuf;
use ux::ask_yn;

pub fn handle_symlink(
    symlink: &Symlink,
    target: &Result<(), PathBuf>,
    interactive: bool,
) -> std::io::Result<()> {
    match target {
        Ok(_) => {
            symlink.print_action("nothing to do", Some(VALID));
            Ok(())
        }
        Err(target) => {
            if interactive
                && !ask_yn(
                    &format!(
                        "found already existing symlink:
\t{M}{path}{D} -> {BW}{wrong_target}{D}
should it be replaced with:
\t{M}{path}{D} -> {VALID}{target}{D}
?",
                        path = symlink.path.display(),
                        target = symlink.target.display(),
                        wrong_target = target.display(),
                    ),
                    true,
                )
                .unwrap()
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
