mod handle;
use handle::handle;

use crate::ansi::M;
use crate::env::{Env, EnvType, Update};
use ansi::abbrev::{D, I};
use ux::ask_yn;

pub fn process(env: Env, interactive: bool) -> std::io::Result<()> {
    for e in env {
        match e {
            EnvType::Grouped(grouped) => {
                println!("[{M}{}{D}]", grouped.title);

                if match grouped.update {
                    Update::Always => true,
                    Update::Never => false,
                    Update::Optional => {
                        interactive
                            && ask_yn(
                                &format!(
                                    "should {M}{}{D} be updated?\t{I}(update = 'optional'){D}",
                                    grouped.title
                                ),
                                true,
                            )
                            .unwrap()
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
        println!()
    }

    Ok(())
}
