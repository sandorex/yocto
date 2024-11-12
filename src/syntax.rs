use regex::Regex;
use crate::token::Token;
use std::sync::LazyLock;

pub static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?x)
        (?P<id>[[[:alpha:]]_][[[:alpha:]]\d_]*)                        # identifier / keyword
        |(?P<num>\d+(?:\.\d+)?)                                        # integer / float
        |(?P<str>"[^"\\]*(?:\\.[^"\\]*)*"|'[^'\\]*(?:\\.[^'\\]*)*')    # string (single or double quote)
        |(?P<sym>
            [[:punct:]]                                                # any other symbol
        )
        |(?P<ws>\s+)                                                   # any whitespace
    "#).expect("Regex failed to compile")
});

/// Default syntax used when no other is defined
pub static DEFAULT_SYNTAX: LazyLock<Syntax> = LazyLock::new(|| {
    let kw = vec!["if", "else"];
    let sym = vec!["=>"];

    Syntax::new(SyntaxParams {
        keywords: &kw,
        sym_sequences: &sym,
    }).unwrap()
});

/// Construct syntax from params
///
/// ALL PARAMS ARE PASSED TO REGEX VERBATIM SO BE CAREFUL
pub struct SyntaxParams<'a> {
    /// Identifiers that count as keywords
    pub keywords: &'a [&'a str],

    /// Parse these multi-symbol sequences as single sym token
    pub sym_sequences: &'a [&'a str],
}

pub struct Syntax {
    /// The regex used for syntax matching
    re: Regex,
}

impl Syntax {
    /// Create new syntax
    ///
    /// symbol_sequences are to parse multi-symbol tokens properly (ex. '==' is one token not two)
    pub fn new(params: SyntaxParams) -> Result<Self, regex::Error> {
        let re_kw = if !params.keywords.is_empty() {
            // add '|' so i can just prepend it to the regex string
            format!("(?P<kw>{})|", params.keywords.join("|"))
        } else {
            // empty but safe to prepend
            "".to_string()
        };

        // regex for identifiers
        let re_id = r#"(?P<id>[[[:alpha:]]_][[[:alpha:]]\d_]*)"#;

        // regex for numbers / floats
        let re_num = r#"(?P<num>\d+(?:\.\d+)?)"#;

        // regex for double and single quoted strings
        let re_str = r#"(?P<str>"[^"\\]*(?:\\.[^"\\]*)*"|'[^'\\]*(?:\\.[^'\\]*)*')"#;

        // regex for symbols
        let re_sym = {
            const PREFIX: &str = r#"(?P<sym>"#;
            const SUFFIX: &str = r#"[[:punct:]])"#;

            if params.sym_sequences.is_empty() {
                format!("{}{}", PREFIX, SUFFIX)
            } else {
                // basically OR them
                format!("{}{}|{}", PREFIX, params.sym_sequences.join("|"), SUFFIX)
            }
        };

        // regex for whitespace
        let re_ws = r#"(?P<ws>\s+)"#;

        // NOTE: re_kw does not have | intentionally!
        let raw = format!("{re_kw}{re_id}|{re_num}|{re_str}|{re_sym}|{re_ws}");
        dbg!(&raw);

        Ok(Self {
            re: Regex::new(&raw)?,
        })
    }

    pub fn parse<'a>(&self, input: &'a str) -> Vec<Token<'a>> {
        self.re.captures_iter(&input)
            .map(|c| Token::new(&c))
            .collect()
    }
}
