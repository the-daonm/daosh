use crate::shell::env::ShellEnv;
use crate::shell::parser::{AstNode, CommandNode};
use crate::shell::plan::{ExecCommand, ExecPipeline};

pub fn expand_and_plan(ast: AstNode, env: &ShellEnv) -> ExecPipeline {
    match ast {
        AstNode::Command(cmd) => ExecPipeline {
            // single command → single ExecCommand
            commands: vec![expand_command(&cmd, env)],
        },
        AstNode::Pipeline(cmds) => ExecPipeline {
            // many commands → map each CommandNode → ExecCommand
            commands: cmds.iter().map(|c| expand_command(c, env)).collect(),
        },
    }
}

fn expand_command(cmd: &CommandNode, env: &ShellEnv) -> ExecCommand {
    let program = expand_word(&cmd.name, env);
    let args = cmd
        .args
        .iter()
        .map(|a| expand_word(a, env))
        .collect::<Vec<_>>();

    ExecCommand {
        program,
        args,
        redirs: vec![],
    }
}

// if the whole word is "$FOO" → expand, else keep
fn expand_word(word: &str, env: &ShellEnv) -> String {
    if let Some(name) = word.strip_prefix('$') {
        env.get_var(name).unwrap_or_default()
    } else {
        word.to_string()
    }
}
