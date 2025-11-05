use std::io::{self, Write};

use crate::shell::env::ShellEnv;
use crate::shell::prompt;

pub fn read_line_with_prompt(env: &ShellEnv) -> Option<String> {
    let p = prompt::make_prompt(env);
    print!("{}", p);
    io::stdout().flush().ok()?;

    let mut line = String::new();
    let n = io::stdin().read_line(&mut line).ok()?;
    if n == 0 {
        // EOF
        println!();
        return None;
    }
    Some(line.trim_end().to_string())
}
