use crate::env::{Env, EnvType, Exist, Symlink};

pub fn process(env: Env) {
    for e in env {
        match e {
            EnvType::Grouped(grouped) => {
                for symlink in grouped.symlink {
                    handle(&symlink);
                }
            }
            EnvType::Alone(symlink) => {
                handle(&symlink);
            }
        }
    }
}

fn handle(symlink: &Symlink) {
    match symlink.exist {
        Exist::Yes(_) => {}
        Exist::No => 
    }
}
