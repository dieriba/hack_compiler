use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref KEYWORDS_TABLE: HashMap<&'static str, &'static str> = HashMap::from([
        ("class", "class"),
        ("constructor", "constructor"),
        ("function", "function"),
        ("method", "method"),
        ("field", "field"),
        ("static", "static"),
        ("var", "var"),
        ("int", "int"),
        ("char", "char"),
        ("boolean", "boolean"),
        ("void", "void"),
        ("true", "true"),
        ("false", "false"),
        ("null", "null"),
        ("this", "this"),
        ("let", "let"),
        ("do", "do"),
        ("if", "if"),
        ("else", "else"),
        ("while", "while"),
        ("return", "return")
    ]);
}
