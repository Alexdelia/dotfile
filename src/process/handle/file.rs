use crate::ansi::{BW, M, VALID};
use crate::unix::{remove_file, Symlink};
use ansi::abbrev::D;
use ansi::{B_BLUE, B_RED};
use ux::{ask, ask_yn, AskKey};

pub fn handle_file(symlink: &Symlink, interactive: bool) -> std::io::Result<()> {
    println!(
        "found already existing file:
\t{M}{path}{D}",
        path = symlink.path.display()
    );

    if !diff(symlink, interactive)? {
        return Ok(());
    }

    if interactive
        && !ask_yn(
            &format!(
                "should {M}{path}{D} be replaced with the symlink:
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

fn diff(symlink: &Symlink, interactive: bool) -> std::io::Result<bool> {
    let original = std::fs::read_to_string(&symlink.target)?;
    let modified = std::fs::read_to_string(&symlink.path)?;
    let patch = diffy::create_patch(&original, &modified);

    let c = if patch.hunks().is_empty() {
        println!("both files are identical");
        'k'
    } else if !interactive {
        'I'
    } else {
        println!(
            "diff:\n--- {B_RED}{system}{D}\n+++ {B_BLUE}{data}{D}\n{patch}\n",
            system = symlink.target.display(),
            data = symlink.path.display()
        );

        ask(
            &format!(
                "what should be done on {M}{path}{D}:
\t{B_BLUE}K{D}eep file in data
\t{B_RED}R{D}eplace file in data with the already existing file in the system",
                path = symlink.path.display(),
            ),
            &[
                AskKey::new('k', Some("keep"), true, Some(B_BLUE)),
                AskKey::new('r', Some("replace"), true, Some(B_RED)),
            ],
            None,
        )
        .unwrap() // panic on purpose
    };

    match c {
        'k' => symlink.print_action("keeped", Some(B_BLUE)),
        'I' => {
            symlink.print_action("skipped", Some(BW));
            return Ok(false);
        }
        'r' => {
            std::fs::copy(&symlink.target, &symlink.path)?;
            symlink.print_action("replaced", Some(B_RED));
        }
        _ => unreachable!("diff() should find char: k, r or I"),
    }

    Ok(true)
}
