use anyhow::Result;

enum PunctuationKind {
    Comma,
    Dot,
    SemiColon,
}

enum BracketKinds {
    LParen,
    RParen,
    LSquare,
    RSquare,
    LBraces,
    RBraces,
}

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
    Identifiers,
    Number(i16),
    StringLiteral(&'input str),
    Keyword(KeywordKind),
    UnaryOperator(UnaryOperatorKind),
    BinaryOperator(BinaryOperatorKind),
    Brackets(BracketKinds),
    Punctuation(PunctuationKind),
    Slash,
    EOF,
}

struct Span {}

pub struct Token<'input> {
    token_kind: TokenKind<'input>,
    span: Span,
}

pub struct Lexer<'input> {
    input: &'input str,
    lines: usize,
    col: usize,
}

impl<'input> Lexer<'input> {}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
