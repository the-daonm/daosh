pub enum AstNode {
    Command(CommandNode),
    Pipeline(Vec<CommandNode>),
}

pub struct CommandNode {
    pub name: String,
    pub args: Vec<String>,
}

impl CommandNode {
    pub fn new(name: String, args: Vec<String>) -> Self {
        Self { name, args }
    }
}

pub fn parse_line(line: &str) -> Result<AstNode, String> {
    // split by | first (pipeline)
    let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
    if parts.is_empty() {
        return Err("empty command".into());
    }

    if parts.len() == 1 {
        // single command
        let cmd = parse_command(parts[0])?;
        Ok(AstNode::Command(cmd))
    } else {
        // multiple commands → pipeline
        let mut cmds = Vec::new();
        for p in parts {
            let c = parse_command(p)?;
            cmds.push(c);
        }
        Ok(AstNode::Pipeline(cmds))
    }
}

fn parse_command(s: &str) -> Result<CommandNode, String> {
    let tokens: Vec<&str> = s.split_whitespace().collect();
    if tokens.is_empty() {
        return Err("empty command segment".into());
    }
    let name = tokens[0].to_string();
    let args = tokens[1..].iter().map(|t| t.to_string()).collect();
    Ok(CommandNode::new(name, args))
}
