use rustyline::Editor;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use crate::completion::ShellHelper;
use crate::parser::parse_input;
use crate::shell::execute;
use crate::tokeniser::tokenise_input;

pub struct CLI {
    rl: Editor<ShellHelper, DefaultHistory>,
}

impl CLI {
    pub fn new() -> rustyline::Result<Self> {
        let mut rl = Editor::new()?;
        rl.set_helper(Some(ShellHelper::new()));
        Ok (Self { rl })
    }

    pub fn repl(&mut self) {
        loop {

            let readline = (&mut self.rl).readline("$ ");
            match readline {
                Ok(line) => {
                    if let Ok(input_tokenised) = tokenise_input(line.as_str()) {
                        for (i, token) in input_tokenised.iter().enumerate() {
                            if i > 0 {
                                print!(" ");
                            }
                            print!("[{}]", token);
                        }
                        println!();
                    } else {
                        println!("ERR unterminated quote");
                    }
                    /*let command = match parse_input(input_tokenised) {
                        Ok(command) => command,
                        Err(err) => {
                            eprintln!("parse error: {:?}", err);
                            continue;
                        }
                    }; */

                    //match execute(command) {
                        //true => continue,
                        //false => break,
                    //}
                },
                Err(ReadlineError::Eof) => break,
                Err(ReadlineError::Interrupted) => {},
                _ => {}
            }

        }
    }

}