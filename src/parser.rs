use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Statement {
    // TODO convert this types to &str
    Set { key: String, value: String },
    Get { key: String },
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyStatement,
    InvalidStatement(String),
    UnknownCommand,
}

fn fmt_invalid_number_of_args_error(expected_n_args: u8) -> ParseError {
    ParseError::InvalidStatement(format!("Expecting {} arguments", expected_n_args).to_string())
}

pub fn parse(text: &str) -> Result<Statement, ParseError> {
    let token_iter = &mut text.split_whitespace();
    let Some(first_token) = token_iter.next() else { return Err(ParseError::EmptyStatement);};

    match first_token.to_lowercase().as_str() {
        "set" => {
            let [key, value] = token_iter.collect::<Vec<&str>>()[..] else { return Err(fmt_invalid_number_of_args_error(2)); };
            Ok(Statement::Set {
                key: key.to_string(),
                value: value.to_string(),
            })
        }
        "get" => {
            let [key] = token_iter.collect::<Vec<&str>>()[..] else { return Err(fmt_invalid_number_of_args_error(1)); };
            Ok(Statement::Get {
                key: key.to_string(),
            })
        }
        _ => Err(ParseError::UnknownCommand),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_set_foo_bar() {
        assert_eq!(
            Ok(Statement::Set {
                key: String::from("foo"),
                value: String::from("bar"),
            }),
            parse("SET foo bar")
        );
    }
    #[test]
    fn simple_set_spam_eggs() {
        assert_eq!(
            Ok(Statement::Set {
                key: String::from("spam"),
                value: String::from("eggs")
            }),
            parse("SET spam eggs")
        );
    }
    // TODO make this test work
    // #[test]
    // fn set_with_spaces_and_wrapped_in_quotes() {
    //     assert_eq!(
    //         Ok(Statement::Set {
    //             key: String::from("hello world"),
    //             value: String::from("goodbye universe")
    //         }),
    //         parse("SET \"hello world\" \"goodbye universe\"")
    //     );
    // }
    // TODO make this test work
    // #[test]
    // fn get_with_spaces_and_wrapped_in_quotes() {
    //     assert_eq!(
    //         Ok(Statement::Get {
    //             key: String::from("hello world")
    //         }),
    //         parse("GET \"hello world\"")
    //     );
    // }
    #[test]
    fn set_case_unsensitive() {
        assert_eq!(
            Ok(Statement::Set {
                key: String::from("foo"),
                value: String::from("bar"),
            }),
            parse("Set foo bar")
        );
    }
    #[test]
    fn set_starts_with_whitespace() {
        assert_eq!(
            Ok(Statement::Set {
                key: String::from("foo"),
                value: String::from("bar"),
            }),
            parse(" set foo bar")
        );
    }
    #[test]
    fn set_ends_with_whitespace() {
        assert_eq!(
            Ok(Statement::Set {
                key: String::from("foo"),
                value: String::from("bar"),
            }),
            parse("set foo bar ")
        );
    }
    #[test]
    fn set_no_args_returns_invalid_err() {
        match parse("set") {
            Err(ParseError::InvalidStatement(_)) => 1,
            _ => panic!("Did not raise error"),
        };
    }
    #[test]
    fn set_only_one_arg_returns_invalid_err() {
        match parse("set foo") {
            Err(ParseError::InvalidStatement(_)) => 1,
            _ => panic!("Did not raise error"),
        };
    }
    #[test]
    fn set_more_than_2_args_returns_invalid_err() {
        match parse("set foo") {
            Err(ParseError::InvalidStatement(_)) => 1,
            _ => panic!("Did not raise error"),
        };
    }
    #[test]
    fn simple_get_foo_bar() {
        assert_eq!(
            Ok(Statement::Get {
                key: String::from("foo")
            }),
            parse("GET foo")
        );
    }
    #[test]
    fn simple_get_spam() {
        assert_eq!(
            Ok(Statement::Get {
                key: String::from("spam")
            }),
            parse("GET spam")
        );
    }
    #[test]
    fn empty_string_returns_empty_statement() {
        assert_eq!(Result::Err(ParseError::EmptyStatement), parse(&("")));
    }
}
