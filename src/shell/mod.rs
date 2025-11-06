pub mod builtins;
pub mod env;
pub mod executor;
pub mod expand;
pub mod input;
pub mod lexer;
pub mod parser;
pub mod plan;
pub mod prompt;

use env::ShellEnv;

pub fn run() {
    let mut env = ShellEnv::new();

    loop {
        // read line
        let line = match input::read_line_with_prompt(&env) {
            Some(l) => l,
            None => break, // EOF
        };

        if line.trim().is_empty() {
            continue;
        }

        // parse into AST
        let ast = match parser::parse_line(&line) {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("parse error: {e}");
                continue;
            }
        };

        // expand + lower to exec plan
        let exec_pipeline = expand::expand_and_plan(ast, &env);

        // execute
        executor::execute_pipeline(&exec_pipeline, &mut env);

        if env.should_exit {
            break;
        }
    }
}
