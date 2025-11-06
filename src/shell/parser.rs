use crate::shell::lexer::{Token, lex};

pub enum AstNode {
    Command(CommandNode),
    Pipeline(Vec<CommandNode>),
}

pub struct CommandNode {
    pub name: String,
    pub args: Vec<String>,
}

pub fn parse_line(line: &str) -> Result<AstNode, String> {
    let tokens = lex(line);
    let mut commands: Vec<CommandNode> = Vec::new();
    let mut current_words: Vec<String> = Vec::new();

    for tok in tokens {
        match tok {
            Token::Word(w) => {
                if !w.trim().is_empty() {
                    current_words.push(w);
                }
            }
            Token::Pipe => {
                if let Some(cmd) = words_to_command(&current_words) {
                    commands.push(cmd);
                }
                current_words.clear();
            }
            _ => {
                // ignore redirs for now
            }
        }
    }

    if let Some(cmd) = words_to_command(&current_words) {
        commands.push(cmd);
    }

    if commands.is_empty() {
        // no real command
        return Err("empty command".into());
    }

    if commands.len() == 1 {
        Ok(AstNode::Command(commands.remove(0)))
    } else {
        Ok(AstNode::Pipeline(commands))
    }
}

fn words_to_command(words: &[String]) -> Option<CommandNode> {
    if words.is_empty() {
        return None;
    }
    let name = words[0].trim();
    if name.is_empty() {
        return None;
    }
    let args = words[1..].to_vec();
    Some(CommandNode {
        name: name.to_string(),
        args,
    })
}
