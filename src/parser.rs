use super::database::Database;
use super::types::TypedValue;

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Set { key: &'a str, value: TypedValue },
    Get { key: &'a str },
}

pub fn parse(text: &str) -> Result<Command, &str> {
    let token_iter: Vec<&str> = text.split_whitespace().collect();

    if token_iter.len() == 0 {
        return Err("Empty expression");
    }
    let first_token = token_iter[0].to_lowercase();

    match first_token.as_str() {
        "set" => match token_iter[1..] {
            [key, value] => Ok(Command::Set {
                key: key,
                value: TypedValue::from_str(value),
            }),
            _ => Err("Invalid number of arguments"),
        },
        "get" => match token_iter[1..] {
            [key] => Ok(Command::Get { key: key }),
            _ => Err("Invalid number of arguments"),
        },
        _ => Err("Unknown command"),
    }
}

pub fn execute_command<'a, 'b>(
    db: &'b mut Database,
    command: Command<'a>,
) -> Option<&'b TypedValue> {
    match command {
        Command::Set { key, value } => {
            db.set(&key, value);
            None
        }
        Command::Get { key } => db.get(key),
    }
}

pub fn execute(db: &mut Database, command: &str) -> String {
    match parse(&command) {
        Ok(parsed_command) => {
            let res = execute_command(db, parsed_command);
            format!("{res:?}")
        }
        Err(e) => String::from(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_set_foo_bar() {
        assert_eq!(
            Ok(Command::Set {
                key: "foo",
                value: TypedValue::Text(String::from("bar")),
            }),
            parse("SET foo bar")
        );
    }
    #[test]
    fn simple_set_spam_eggs() {
        assert_eq!(
            Ok(Command::Set {
                key: "spam",
                value: TypedValue::Text(String::from("eggs"))
            }),
            parse("SET spam eggs")
        );
    }
    #[test]
    fn set_value_is_correctl_parsed_as_numeric() {
        assert_eq!(
            Ok(Command::Set {
                key: "foo",
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
    //             key: "foo",
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
                key: "foo",
                value: TypedValue::Text(String::from("bar")),
            }),
            parse("Set foo bar")
        );
    }
    #[test]
    fn set_starts_with_whitespace() {
        assert_eq!(
            Ok(Command::Set {
                key: "foo",
                value: TypedValue::Text(String::from("bar")),
            }),
            parse(" set foo bar")
        );
    }
    #[test]
    fn set_ends_with_whitespace() {
        assert_eq!(
            Ok(Command::Set {
                key: "foo",
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
        assert_eq!(Ok(Command::Get { key: "foo" }), parse("GET foo"));
    }
    #[test]
    fn simple_get_spam() {
        assert_eq!(Ok(Command::Get { key: "spam" }), parse("GET spam"));
    }
    #[test]
    fn empty_string_returns_empty_statement() {
        assert_eq!(Result::Err("Empty expression"), parse(&("")));
    }

    #[test]
    fn insert_and_delete() {
        let mut db = Database::new();
        execute_command(
            &mut db,
            Command::Set {
                key: "foo",
                value: TypedValue::Text(String::from("bar")),
            },
        );
        assert_eq!(
            execute_command(&mut db, Command::Get { key: "foo" }),
            Some(&TypedValue::Text(String::from("bar")))
        )
    }
}
