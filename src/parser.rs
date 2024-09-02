use std::iter::once;

#[derive(Debug, PartialEq)]
pub enum TypedValue {
    Numeric(f64),
    Text(String),
}

impl TypedValue {
    fn from_str(as_str: &str) -> Self {
        match as_str.parse::<f64>() {
            Ok(num) => TypedValue::Numeric(num),
            Err(_) => TypedValue::Text(String::from(as_str)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    // TODO convert this types to &str
    Set { key: String, value: TypedValue },
    Get { key: String },
}

pub fn parse(text: &str) -> Result<Statement, &str> {
    let mut token_iter = text.split_whitespace();

    let Some(first_token) = token_iter.next().and_then(|token| Some(token.to_lowercase()) ) else { return Err("Empty expression");};
    let tokenized: Vec<&str> = once(first_token.as_str()).chain(token_iter).collect();

    match tokenized[..] {
        ["set", key, value] => Ok(Statement::Set {
            key: key.to_string(),
            value: TypedValue::from_str(value),
        }),
        ["get", key] => Ok(Statement::Get {
            key: key.to_string(),
        }),
        _ => Err("Unkwon command or invalid number of arguments"),
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
                value: TypedValue::Text(String::from("bar")),
            }),
            parse("SET foo bar")
        );
    }
    #[test]
    fn simple_set_spam_eggs() {
        assert_eq!(
            Ok(Statement::Set {
                key: String::from("spam"),
                value: TypedValue::Text(String::from("eggs"))
            }),
            parse("SET spam eggs")
        );
    }
    #[test]
    fn set_value_is_correctl_parsed_as_numeric() {
        assert_eq!(
            Ok(Statement::Set {
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
            Ok(Statement::Set {
                key: String::from("foo"),
                value: TypedValue::Text(String::from("bar")),
            }),
            parse("Set foo bar")
        );
    }
    #[test]
    fn set_starts_with_whitespace() {
        assert_eq!(
            Ok(Statement::Set {
                key: String::from("foo"),
                value: TypedValue::Text(String::from("bar")),
            }),
            parse(" set foo bar")
        );
    }
    #[test]
    fn set_ends_with_whitespace() {
        assert_eq!(
            Ok(Statement::Set {
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
        assert_eq!(Result::Err("Empty expression"), parse(&("")));
    }
}
