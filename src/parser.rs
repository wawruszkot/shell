pub struct Command {
    pub cmd: Option<String>,
    pub args: Vec<String>,
    pub redirections: Vec<Redirection>
}

pub enum AST {
    ExecNode (Command),
}

#[derive(Debug)]
pub enum ParseError {
    RedirectionInvalidFilePath
}

pub enum Redirection {
    Output(String),
    OutputAppend(String),
    Error(String),
    ErrorAppend(String),
}

pub fn parse_input(tokens: Vec<String>) -> Result<Command, ParseError> {
    let mut tokens = tokens.into_iter();
    let cmd = tokens.next();
    let mut args: Vec<String> = Vec::new();
    let mut redirections = Vec::new();

    while let Some(token) = tokens.next() {
        match token.as_str() {
            ">" | "1>" => {
                let path = tokens.next().ok_or(ParseError::RedirectionInvalidFilePath)?;
                redirections.push(Redirection::Output(path));
            },
            ">>" | "1>>" => {
                let path = tokens.next().ok_or(ParseError::RedirectionInvalidFilePath)?;
                redirections.push(Redirection::OutputAppend(path));
            }
            "2>" => {
                let path = tokens.next().ok_or(ParseError::RedirectionInvalidFilePath)?;
                redirections.push(Redirection::Error(path));
            }
            "2>>" => {
                let path = tokens.next().ok_or(ParseError::RedirectionInvalidFilePath)?;
                redirections.push(Redirection::ErrorAppend(path));
            }
            _ => {
                args.push(token)
            }
        }
    }
    Ok (Command {
        cmd,
        args,
        redirections
    })
}