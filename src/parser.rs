use super::database::Database;
use super::types::TypedValue;
use std::iter::once;

#[derive(Debug, PartialEq)]
pub enum Command {
    // TODO convert this types to &str
    Set { key: String, value: TypedValue },
    Get { key: String },
}

pub fn parse(text: &str) -> Result<Command, &str> {
    let mut token_iter = text.split_whitespace();

    let Some(first_token) = token_iter.next().and_then(|token| Some(token.to_lowercase()) ) else { return Err("Empty expression");};
    let tokenized: Vec<&str> = once(first_token.as_str()).chain(token_iter).collect();

    match tokenized[..] {
        ["set", key, value] => Ok(Command::Set {
            key: key.to_string(),
            value: TypedValue::from_str(value),
        }),
        ["get", key] => Ok(Command::Get {
            key: key.to_string(),
        }),
        _ => Err("Unkwon command or invalid number of arguments"),
    }
}

pub fn do_command(db: &mut Database, command: Command) -> Option<&TypedValue> {
    match command {
        Command::Set { key, value } => {
            db.set(key, value);
            None
        }
        Command::Get { key } => db.get(key),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_set_foo_bar() {
        assert_eq!(
            Ok(Command::Set {
                key: String::from("foo"),
                value: TypedValue::Text(String::from("bar")),
            }),
            parse("SET foo bar")
        );
    }
    #[test]
    fn simple_set_spam_eggs() {
        assert_eq!(
            Ok(Command::Set {
                key: String::from("spam"),
                value: TypedValue::Text(String::from("eggs"))
            }),
            parse("SET spam eggs")
        );
    }
    #[test]
    fn set_value_is_correctl_parsed_as_numeric() {
        assert_eq!(
            Ok(Command::Set {
                key: String::from("foo"),
                value: TypedValue::Numeric(1.0)
            }),
            parse("SET foo 1")
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
    // fn set_value_wrapped_in_quotes_is_read_as_string() {
    //     assert_eq!(
    //         Ok(Statement::Set {
    //             key: String::from("foo"),
    //             value: String::from("1")
    //         }),
    //         parse("SET foo \"1\"")
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
            Ok(Command::Set {
                key: String::from("foo"),
                value: TypedValue::Text(String::from("bar")),
            }),
            parse("Set foo bar")
        );
    }
    #[test]
    fn set_starts_with_whitespace() {
        assert_eq!(
            Ok(Command::Set {
                key: String::from("foo"),
                value: TypedValue::Text(String::from("bar")),
            }),
            parse(" set foo bar")
        );
    }
    #[test]
    fn set_ends_with_whitespace() {
        assert_eq!(
            Ok(Command::Set {
                key: String::from("foo"),
                value: TypedValue::Text(String::from("bar")),
            }),
            parse("set foo bar ")
        );
    }
    #[test]
    fn set_no_args_returns_invalid_err() {
        parse("set").expect_err("Should error");
    }
    #[test]
    fn set_only_one_arg_returns_invalid_err() {
        parse("set foo").expect_err("Should error");
    }
    #[test]
    fn set_more_than_2_args_returns_invalid_err() {
        parse("set foo").expect_err("Should error");
    }
    #[test]
    fn simple_get_foo_bar() {
        assert_eq!(
            Ok(Command::Get {
                key: String::from("foo")
            }),
            parse("GET foo")
        );
    }
    #[test]
    fn simple_get_spam() {
        assert_eq!(
            Ok(Command::Get {
                key: String::from("spam")
            }),
            parse("GET spam")
        );
    }
    #[test]
    fn empty_string_returns_empty_statement() {
        assert_eq!(Result::Err("Empty expression"), parse(&("")));
    }

    #[test]
    fn insert_and_delete() {
        let mut db = Database::new();
        do_command(
            &mut db,
            Command::Set {
                key: String::from("foo"),
                value: TypedValue::Text(String::from("bar")),
            },
        );
        assert_eq!(
            do_command(
                &mut db,
                Command::Get {
                    key: String::from("foo")
                }
            ),
            Some(&TypedValue::Text(String::from("bar")))
        )
    }
}
