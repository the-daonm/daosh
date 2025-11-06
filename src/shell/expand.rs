use crate::shell::env::ShellEnv;
use crate::shell::parser::{AstNode, CommandNode};
use crate::shell::plan::{ExecCommand, ExecPipeline};

pub fn expand_and_plan(ast: AstNode, env: &ShellEnv) -> ExecPipeline {
    match ast {
        AstNode::Command(cmd) => {
            let ec = expand_command(&cmd, env);
            if ec.program.is_empty() {
                ExecPipeline { commands: vec![] }
            } else {
                ExecPipeline { commands: vec![ec] }
            }
        }
        AstNode::Pipeline(cmds) => {
            let mut out = Vec::new();
            for c in cmds.iter() {
                let ec = expand_command(c, env);
                if !ec.program.is_empty() {
                    out.push(ec);
                }
            }
            ExecPipeline { commands: out }
        }
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

// expand $VAR anywhere in the word
fn expand_word(word: &str, env: &ShellEnv) -> String {
    let mut out = String::new();
    let mut chars = word.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            let mut name = String::new();
            while let Some(&ch) = chars.peek() {
                if ch == '_' || ch.is_ascii_alphanumeric() {
                    name.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            let val = env.get_var(&name).unwrap_or_default();
            out.push_str(&val);
        } else {
            out.push(c);
        }
    }

    out
}
