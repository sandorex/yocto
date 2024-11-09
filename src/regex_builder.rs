use regex::Regex;
use std::sync::LazyLock;

pub static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?x)
        (?P<id>[[[:alpha:]]_][[[:alpha:]]\d_]*) # identifier / keyword
        |(?P<num>\d+(?:\.\d+)?)               # integer / float
        |(?P<str>"[^"\\]*(?:\\.[^"\\]*)*"|'[^'\\]*(?:\\.[^'\\]*)*')    # string
        |(?P<sym>
            [=<>\-+*&^%!~]=                   # assign or equal
            |[[:punct:]]                      # any other symbol
        )
        |(?P<ws>\s+)                          # any whitespace
    "#).expect("Regex failed to compile")
});
