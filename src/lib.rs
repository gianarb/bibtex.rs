const INITIAL_DELIMITER_TYPE: char = '@';
const INITIAL_DELIMITER_TAG: char = '{';
const ENDING_DELIMITER_TAG: char = '}';

#[derive(Debug)]
pub enum BibtexError {
    LeftOver(String),
}

pub fn tokenize(i: &str) -> Result<Vec<Token>, BibtexError> {
    let mut res = vec![];
    let mut w = String::from("");
    let mut ii = 0;
    let tot_len = i.len();

    for c in i.chars() {
        ii += 1;
        if c == '\n' {
            continue;
        }

        if c == INITIAL_DELIMITER_TYPE {
            res.push(Token {
                name: TokenName::InitialDelimiterType,
                value: c.to_string(),
            });
            continue;
        }

        if c == INITIAL_DELIMITER_TAG && res.last().unwrap().name == TokenName::InitialDelimiterType
        {
            res.push(Token {
                name: TokenName::Type,
                value: w.clone(),
            });

            res.push(Token {
                name: TokenName::InitialDelimiterTag,
                value: INITIAL_DELIMITER_TAG.to_string(),
            });
            w = String::new();
            continue;
        }

        if c == ',' && res.last().unwrap().name == TokenName::InitialDelimiterTag {
            res.push(Token {
                name: TokenName::CitationKey,
                value: w.clone(),
            });
            res.push(Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            });
            w = String::new();
            continue;
        }

        if c == '=' {
            res.push(Token {
                name: TokenName::TagName,
                value: w.trim().to_string(),
            });

            w = String::new();
            res.push(Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            });
            continue;
        }

        if c == ',' {
            res.push(Token {
                name: TokenName::TagValue,
                value: w.trim().to_string(),
            });
            res.push(Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            });
            w = String::new();
            continue;
        }

        if c == ENDING_DELIMITER_TAG && ii == tot_len {
            if !w.is_empty() {
                res.push(Token {
                    name: TokenName::TagValue,
                    value: w.trim().to_string(),
                });
            }
            res.push(Token {
                name: TokenName::EndingDelimiterTag,
                value: ENDING_DELIMITER_TAG.to_string(),
            });
            w = String::new();
            continue;
        }

        w.push(c);
    }
    if !w.is_empty() {
        return Err(BibtexError::LeftOver(w));
    }
    Ok(res)
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub name: TokenName,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenName {
    InitialDelimiterType,
    Type,

    CitationKey,

    InitialDelimiterTag,
    TagName,
    Equal,
    TagValue,
    Comma,
    EndingDelimiterTag,
}

#[cfg(test)]
mod tests {
    use crate::{tokenize, Token, TokenName};

    #[test]
    fn it_works() {
        let tt = tokenize(
            r#"@article{mrx05,
auTHor = "Mr. X",
Title = "Something Great",
publisher = "nob",
YEAR = "2005",
}"#,
        );
        let expect: Vec<Token> = vec![
            Token {
                name: TokenName::InitialDelimiterType,
                value: "@".to_string(),
            },
            Token {
                name: TokenName::Type,
                value: "article".to_string(),
            },
            Token {
                name: TokenName::InitialDelimiterTag,
                value: "{".to_string(),
            },
            Token {
                name: TokenName::CitationKey,
                value: "mrx05".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::TagName,
                value: "auTHor".to_string(),
            },
            Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            },
            Token {
                name: TokenName::TagValue,
                value: "\"Mr. X\"".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::TagName,
                value: "Title".to_string(),
            },
            Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            },
            Token {
                name: TokenName::TagValue,
                value: "\"Something Great\"".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::TagName,
                value: "publisher".to_string(),
            },
            Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            },
            Token {
                name: TokenName::TagValue,
                value: "\"nob\"".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::TagName,
                value: "YEAR".to_string(),
            },
            Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            },
            Token {
                name: TokenName::TagValue,
                value: "\"2005\"".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::EndingDelimiterTag,
                value: "}".to_string(),
            },
        ];
        assert_eq!(expect, tt.unwrap());
    }

    #[test]
    fn it_works_with_brackets() {
        let tt = tokenize(
            r#"@article{mrx05,
auTHor = {Mr. X},
Title = "Something Great",
}"#,
        );
        let expect: Vec<Token> = vec![
            Token {
                name: TokenName::InitialDelimiterType,
                value: "@".to_string(),
            },
            Token {
                name: TokenName::Type,
                value: "article".to_string(),
            },
            Token {
                name: TokenName::InitialDelimiterTag,
                value: "{".to_string(),
            },
            Token {
                name: TokenName::CitationKey,
                value: "mrx05".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::TagName,
                value: "auTHor".to_string(),
            },
            Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            },
            Token {
                name: TokenName::TagValue,
                value: "{Mr. X}".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::TagName,
                value: "Title".to_string(),
            },
            Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            },
            Token {
                name: TokenName::TagValue,
                value: "\"Something Great\"".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::EndingDelimiterTag,
                value: "}".to_string(),
            },
        ];
        assert_eq!(expect, tt.unwrap());
    }

    #[test]
    fn it_works_with_last_tag_without_comma() {
        let tt = tokenize(
            r#"@article{mrx05, Title = "1000"
}"#,
        );
        let expect: Vec<Token> = vec![
            Token {
                name: TokenName::InitialDelimiterType,
                value: "@".to_string(),
            },
            Token {
                name: TokenName::Type,
                value: "article".to_string(),
            },
            Token {
                name: TokenName::InitialDelimiterTag,
                value: "{".to_string(),
            },
            Token {
                name: TokenName::CitationKey,
                value: "mrx05".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::TagName,
                value: "Title".to_string(),
            },
            Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            },
            Token {
                name: TokenName::TagValue,
                value: "\"1000\"".to_string(),
            },
            Token {
                name: TokenName::EndingDelimiterTag,
                value: "}".to_string(),
            },
        ];
        assert_eq!(expect, tt.unwrap());
    }

    #[test]
    fn it_works_with_tag_as_num() {
        let tt = tokenize(
            r#"@article{mrx05, Title = 1000,
}"#,
        );
        let expect: Vec<Token> = vec![
            Token {
                name: TokenName::InitialDelimiterType,
                value: "@".to_string(),
            },
            Token {
                name: TokenName::Type,
                value: "article".to_string(),
            },
            Token {
                name: TokenName::InitialDelimiterTag,
                value: "{".to_string(),
            },
            Token {
                name: TokenName::CitationKey,
                value: "mrx05".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::TagName,
                value: "Title".to_string(),
            },
            Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            },
            Token {
                name: TokenName::TagValue,
                value: "1000".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::EndingDelimiterTag,
                value: "}".to_string(),
            },
        ];
        assert_eq!(expect, tt.unwrap());
    }

    #[test]
    fn it_works_with_tag_value_with_double_brackets() {
        let tt = tokenize(
            r#"@article{mrx05,
Title = {{Bib}\TeX},
}"#,
        );
        let expect: Vec<Token> = vec![
            Token {
                name: TokenName::InitialDelimiterType,
                value: "@".to_string(),
            },
            Token {
                name: TokenName::Type,
                value: "article".to_string(),
            },
            Token {
                name: TokenName::InitialDelimiterTag,
                value: "{".to_string(),
            },
            Token {
                name: TokenName::CitationKey,
                value: "mrx05".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::TagName,
                value: "Title".to_string(),
            },
            Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            },
            Token {
                name: TokenName::TagValue,
                value: "{{Bib}\\TeX}".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::EndingDelimiterTag,
                value: "}".to_string(),
            },
        ];
        assert_eq!(expect, tt.unwrap());
    }
    #[test]
    fn fuzz_first() {
        let _ = tokenize(
            r#",
"#,
        );
    }

    #[test]
    fn it_works_e2e_test_from_bibtex_org() {
        let tt = tokenize(
            r#"@article{mrx05,
auTHor = "Mr. X",
Title = {Something Great},
publisher = "nob" # "ody",
YEAR = 2005,
}"#,
        );
        let expect: Vec<Token> = vec![
            Token {
                name: TokenName::InitialDelimiterType,
                value: "@".to_string(),
            },
            Token {
                name: TokenName::Type,
                value: "article".to_string(),
            },
            Token {
                name: TokenName::InitialDelimiterTag,
                value: "{".to_string(),
            },
            Token {
                name: TokenName::CitationKey,
                value: "mrx05".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::TagName,
                value: "auTHor".to_string(),
            },
            Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            },
            Token {
                name: TokenName::TagValue,
                value: "\"Mr. X\"".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::TagName,
                value: "Title".to_string(),
            },
            Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            },
            Token {
                name: TokenName::TagValue,
                value: "{Something Great}".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::TagName,
                value: "publisher".to_string(),
            },
            Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            },
            Token {
                name: TokenName::TagValue,
                value: "\"nob\" # \"ody\"".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::TagName,
                value: "YEAR".to_string(),
            },
            Token {
                name: TokenName::Equal,
                value: "=".to_string(),
            },
            Token {
                name: TokenName::TagValue,
                value: "2005".to_string(),
            },
            Token {
                name: TokenName::Comma,
                value: ",".to_string(),
            },
            Token {
                name: TokenName::EndingDelimiterTag,
                value: "}".to_string(),
            },
        ];
        assert_eq!(expect, tt.unwrap());
    }
}
