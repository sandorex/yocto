#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Keyword,
    Identifier,
    Number,
    String,
    Symbol,
    Whitespace,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
    pub raw: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(input: &regex::Captures<'a>) -> Self {
        let kw = input.name("kw");
        let id = input.name("id");
        let num = input.name("num");
        let string = input.name("str");
        let sym = input.name("sym");
        let ws = input.name("ws");

        let (token_type, m) = match (kw, id, num, string, sym, ws) {
            (Some(x), _, _, _, _, _) => (TokenType::Keyword, x),
            (_, Some(x), _, _, _, _) => (TokenType::Identifier, x),
            (_, _, Some(x), _, _, _) => (TokenType::Number, x),
            (_, _, _, Some(x), _, _) => (TokenType::String, x),
            (_, _, _, _, Some(x), _) => (TokenType::Symbol, x),
            (_, _, _, _, _, Some(x)) => (TokenType::Whitespace, x),
            x => panic!("Invalid data from regex {:#?}", x),
        };

        Self {
            token_type,
            start: m.start(),
            end: m.end(),
            raw: m.as_str(),
        }
    }
}
