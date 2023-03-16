use crate::ansi::{M, VALID};
use crate::unix::{remove_file, Symlink};
use ansi::abbrev::D;
use ux::ask_yn;

pub fn handle_file(symlink: &Symlink, interactive: bool) -> Result<(), std::io::Error> {
    if interactive
        && !ask_yn(
            &format!(
                "found already existing file:
\t{M}{path}{D}
should it be replaced with the symlink:
\t{M}{path}{D} -> {VALID}{target}{D}
?",
                path = symlink.path.display(),
                target = symlink.target.display(),
            ),
            true,
        )
        .unwrap()
    {
        return Ok(());
    }

    remove_file(&symlink.path)?;
    symlink.create()
}
