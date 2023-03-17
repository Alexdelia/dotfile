use crate::ansi::{M, VALID};
use crate::unix::{remove_dir, Symlink};
use ansi::abbrev::D;
use ux::ask_yn;

pub fn handle_dir(symlink: &Symlink, interactive: bool) -> std::io::Result<()> {
    if interactive
        && !ask_yn(
            &format!(
                "found already existing directory:
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

    remove_dir(&symlink.path)?;
    symlink.create()
}
