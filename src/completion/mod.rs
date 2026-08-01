use rustyline::completion::{Completer, Pair};
use rustyline::{Context, Helper};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use crate::builtins::get_builtins;

pub struct ShellHelper {}

impl ShellHelper {
    pub fn new() -> Self {
        Self {}
    }
}

impl Helper for ShellHelper {}

impl Hinter for ShellHelper { type Hint = String; }

impl Highlighter for ShellHelper {}

impl Validator for ShellHelper {}

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let current_word = &line[0..pos+1];
        let builtins = get_builtins();
        let matches: Vec<&str> = builtins.collect();

        let mut prefix = current_word.to_string();
        for m in matches {
            
        }

        todo!()
    }
}
