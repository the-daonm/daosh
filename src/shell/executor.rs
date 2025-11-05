use crate::shell::plan::{ExecCommand, ExecPipeline};
use crate::shell::{builtins, env::ShellEnv};

pub fn execute_pipeline(pipe: &ExecPipeline, env: &mut ShellEnv) {
    if pipe.commands.len() == 1 {
        // single command
        execute_command(&pipe.commands[0], env);
        return;
    }

    // multiple commands
    for cmd in &pipe.commands {
        execute_command(cmd, env);
    }
}

fn execute_command(cmd: &ExecCommand, env: &mut ShellEnv) {
    // builtins
    if builtins::is_builtin(&cmd.program) {
        let status = builtins::run_builtin(&cmd.program, &cmd.args, env);
        env.last_status = status;
        return;
    }

    // external process
    let mut child = match std::process::Command::new(&cmd.program)
        .args(&cmd.args)
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}: {e}", cmd.program);
            env.last_status = 127;
            return;
        }
    };

    match child.wait() {
        Ok(status) => {
            env.last_status = status.code().unwrap_or(1);
        }
        Err(e) => {
            eprintln!("failed to wait: {e}");
            env.last_status = 1;
        }
    }
}
