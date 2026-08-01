#[derive(Debug)]
pub enum TokeniseError {
    UnterminatedQuote,
}
pub enum TokenState {
    NormalText,
    Escaped,
    QuotedEscaped,
    SingleQuoted,
    DoubleQuoted,
}
pub fn tokenise_input(buffer: &str) -> Result<Vec<String>, TokeniseError> {
    let mut current_token = String::new();
    let mut token_started = false;
    let mut state = TokenState::NormalText;

    let mut result: Vec<String> = vec![];

    for char in buffer.chars() {
        match char {
            char if char.is_whitespace() => {
                match state {
                    TokenState::NormalText => {
                        if token_started {
                            result.push(std::mem::take(&mut current_token));
                            token_started = false;
                        }
                    }
                    TokenState::Escaped => {
                        current_token.push(char);
                        state = TokenState::NormalText;
                    }
                    TokenState::QuotedEscaped => {
                        current_token.push(char);
                        state = TokenState::DoubleQuoted;
                    }
                    _ => {
                        current_token.push(char);
                    },
                }
            },

            '\\' => {
                match state {
                    TokenState::Escaped => {
                        current_token.push(char);
                        state = TokenState::NormalText;
                    }
                    TokenState::QuotedEscaped => {
                        current_token.push(char);
                        state = TokenState::DoubleQuoted;
                    }
                    TokenState::DoubleQuoted => state = TokenState::QuotedEscaped,
                    TokenState::SingleQuoted => current_token.push(char),
                    _ => state = TokenState::Escaped
                }
            }

            '\'' => {
                match state {
                    TokenState::NormalText => {
                        state = TokenState::SingleQuoted;
                        token_started = true;
                    }
                    TokenState::SingleQuoted => {
                        state = TokenState::NormalText;
                    }
                    TokenState::Escaped => {
                        current_token.push(char);
                        state = TokenState::NormalText;
                    }
                    TokenState::QuotedEscaped => {
                        state = TokenState::DoubleQuoted;
                        current_token.push(char);
                    }
                    TokenState::DoubleQuoted => {
                        current_token.push(char);
                    }
                }
            }
            '"' => {
                match state {
                    TokenState::Escaped => {
                        current_token.push(char);
                        state = TokenState::NormalText;
                    }
                    TokenState::QuotedEscaped => {
                        current_token.push(char);
                        state = TokenState::DoubleQuoted;
                    }
                    TokenState::NormalText => {
                        state = TokenState::DoubleQuoted;
                        token_started = true;
                    }
                    TokenState::DoubleQuoted => {
                        state = TokenState::NormalText;
                    }
                    _ => {
                        current_token.push(char);
                    }
                }
            }

            regular_char => {
                token_started = true;
                match state {
                    TokenState::Escaped         => state = TokenState::NormalText,
                    TokenState::QuotedEscaped   => state = TokenState::DoubleQuoted,
                    _                           => {}
                }
                current_token.push(regular_char);
            }
        }
    }
    if matches!(state, TokenState::SingleQuoted | TokenState::DoubleQuoted | TokenState::QuotedEscaped) {
        return Err(TokeniseError::UnterminatedQuote)
    }
    if token_started {
        if current_token.ends_with('\n') {
            current_token.pop();
        }
        result.push(current_token);
    }
    Ok(result)
}
