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
        let id = input.name("id");
        let num = input.name("num");
        let string = input.name("str");
        let sym = input.name("sym");
        let ws = input.name("ws");

        let (token_type, m) = match (id, num, string, sym, ws) {
            (Some(x), _, _, _, _) =>  match x.as_str() {
                "if" | "else" | "elseif"
                | "return"
                | "var" | "const" | "let"
                | "mut"
                | "mod"
                | "for" | "while" | "foreach" | "match" | "loop"
                | "use" | "import"
                | "true" | "false"
                | "fn" | "fun" | "function" | "def"
                | "static" | "pub" | "public" | "private"
                | "define"
                | "struct" | "class"
                | "override"
                => (TokenType::Keyword, x),
                _ => (TokenType::Identifier, x)
            },
            (_, Some(x), _, _, _) => (TokenType::Number, x),
            (_, _, Some(x), _, _) => (TokenType::String, x),
            (_, _, _, Some(x), _) => (TokenType::Symbol, x),
            (_, _, _, _, Some(x)) => (TokenType::Whitespace, x),
            x => panic!("Invalid data from regex {:?}", x),
        };

        Self {
            token_type,
            start: m.start(),
            end: m.end(),
            raw: m.as_str(),
        }
    }
}
