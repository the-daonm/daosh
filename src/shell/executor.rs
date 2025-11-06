use crate::shell::plan::{ExecCommand, ExecPipeline};
use crate::shell::{builtins, env::ShellEnv};

pub fn execute_pipeline(pipe: &ExecPipeline, env: &mut ShellEnv) {
    if pipe.commands.is_empty() {
        return;
    }

    if pipe.commands.len() == 1 {
        execute_command(&pipe.commands[0], env);
        return;
    }

    // TODO: real pipes; for now run sequentially
    for cmd in &pipe.commands {
        execute_command(cmd, env);
    }
}

fn execute_command(cmd: &ExecCommand, env: &mut ShellEnv) {
    let prog = cmd.program.trim();
    if prog.is_empty() {
        // don't yell, just ignore
        env.last_status = 0;
        return;
    }

    // builtins first
    if builtins::is_builtin(prog) {
        let status = builtins::run_builtin(prog, &cmd.args, env);
        env.last_status = status;
        return;
    }

    // external
    let mut child = match std::process::Command::new(prog).args(&cmd.args).spawn() {
        Ok(c) => c,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("daosh: command not found: {}", prog);
                }
                _ => {
                    eprintln!("daosh: failed to execute '{}': {}", prog, e);
                }
            }
            env.last_status = 127;
            return;
        }
    };

    match child.wait() {
        Ok(status) => {
            env.last_status = status.code().unwrap_or(1);
        }
        Err(e) => {
            use std::io::ErrorKind;

            match e.kind() {
                ErrorKind::BrokenPipe => {
                    eprintln!("daosh: broken pipe while waiting for '{}'", cmd.program);
                }
                ErrorKind::TimedOut => {
                    eprintln!("daosh: process '{}' timed out", cmd.program);
                }
                _ => {
                    eprintln!("daosh: failed to wait for '{}': {}", cmd.program, e);
                }
            }

            env.last_status = 1;
        }
    }
}
