use anyhow::Result;

enum UnaryOperatorKind {
    Not,
    Invert,
}

enum BinaryOperatorKind {
    Equal,
    Plus,
    Minus,
    Star,
    Divide,
    Greater,
    Less,
    And,
    Or,
    Dot,
}

enum OperatorKind {
    Plus,
    Minus,
    Star,
    Equal,
}

enum KeywordKind {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}
enum TokenKind<'input> {
    Number(i16),
    StringLiteral(&'input str),
    Identifiers,
    Keyword(KeywordKind),
    UnaryOperator(UnaryOperatorKind),
    BinaryOperator(BinaryOperatorKind),
    EOF,
}

struct Span {}

pub struct Token<'input> {
    token_kind: TokenKind<'input>,
    span: Span,
}

pub struct Lexer<'input> {
    input: &'input str,
}

impl<'input> Lexer<'input> {}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
