use anyhow::Result;

enum LiteralType {
    I16,
    Str,
}

trait IsType {
    fn get_type(&self) -> LiteralType;
}

impl IsType for str {
    fn get_type(&self) -> LiteralType {
        LiteralType::Str
    }
}

impl IsType for i16 {
    fn get_type(&self) -> LiteralType {
        LiteralType::I16
    }
}

#[macro_export]
macro_rules! Token {
    [,] => { Token::new(TokenKind::Punctuation(PunctuationKind::Comma)) };
    [.] => { Token::new(TokenKind::Punctuation(PunctuationKind::Dot)) };
    [;] => { Token::new(TokenKind::Punctuation(PunctuationKind::SemiColon)) };
    [!] => { Token::new(TokenKind::UnaryOperator(UnaryOperatorKind::Not)) };
    [~] => { Token::new(TokenKind::UnaryOperator(UnaryOperatorKind::Invert)) };
    [=] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::Equal)) };
    [= =] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::EqualEqual)) };
    [+] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::Plus)) };
    [++] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::PlusPlus)) };
    [+ =] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::PlusEqual)) };
    [-] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::Minus)) };
    [--] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::MinusMinus)) };
    [- =] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::MinusEqual)) };
    [*] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::Star)) };
    [* =] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::StarEqual)) };
    [/] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::Divide)) };
    [/ =] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::DivideEqual)) };
    [>] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::Greater)) };
    [> =] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::GreaterEqual)) };
    [<] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::Less)) };
    [< =] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::LessEqual)) };
    [&] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::And)) };
    [& =] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::AndEqual)) };
    [|] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::Or)) };
    [| =] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::OrEqual)) };
    [&&] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::AndAnd)) };
    [||] => { Token::new(TokenKind::BinaryOperator(BinaryOperatorKind::OrOr)) };
    [class] => { Token::new(TokenKind::Keyword(KeywordKind::Class)) };
    [constructor] => { Token::new(TokenKind::Keyword(KeywordKind::Constructor)) };
    [function] => { Token::new(TokenKind::Keyword(KeywordKind::Function)) };
    [method] => { Token::new(TokenKind::Keyword(KeywordKind::Method)) };
    [field] => { Token::new(TokenKind::Keyword(KeywordKind::Field)) };
    [static] => { Token::new(TokenKind::Keyword(KeywordKind::Static)) };
    [var] => { Token::new(TokenKind::Keyword(KeywordKind::Var)) };
    [int] => { Token::new(TokenKind::Keyword(KeywordKind::Int)) };
    [char] => { Token::new(TokenKind::Keyword(KeywordKind::Char)) };
    [boolean] => { Token::new(TokenKind::Keyword(KeywordKind::Boolean)) };
    [void] => { Token::new(TokenKind::Keyword(KeywordKind::Void)) };
    [true] => { Token::new(TokenKind::Keyword(KeywordKind::True)) };
    [false] => { Token::new(TokenKind::Keyword(KeywordKind::False)) };
    [null] => { Token::new(TokenKind::Keyword(KeywordKind::Null)) };
    [this] => { Token::new(TokenKind::Keyword(KeywordKind::This)) };
    [let] => { Token::new(TokenKind::Keyword(KeywordKind::Let)) };
    [do] => { Token::new(TokenKind::Keyword(KeywordKind::Do)) };
    [if] => { Token::new(TokenKind::Keyword(KeywordKind::If)) };
    [else] => { Token::new(TokenKind::Keyword(KeywordKind::Else)) };
    [while] => { Token::new(TokenKind::Keyword(KeywordKind::While)) };
    [return] => { Token::new(TokenKind::Keyword(KeywordKind::Return)) };
    [$ident:ident] => { Token::new(TokenKind::Identifiers($ident)) };
    [$literal:literal, &str] => { Token::new(TokenKind::Literal(LiteralKind::String($literal))) };
    [$literal:literal, i16] => { Token::new(TokenKind::Literal(LiteralKind::Number($literal))) };
    [EOF] => { Token::new(TokenKind::EOF) }
}

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
    EqualEqual,
    Plus,
    PlusPlus,
    PlusEqual,
    Minus,
    MinusMinus,
    MinusEqual,
    Star,
    StarEqual,
    Divide,
    DivideEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    AndAnd,
    AndEqual,
    Or,
    OrOr,
    OrEqual,
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

enum LiteralKind<'input> {
    Number(i16),
    String(&'input str),
}
enum TokenKind<'input> {
    Identifiers(&'input str),
    Literal(LiteralKind<'input>),
    Keyword(KeywordKind),
    UnaryOperator(UnaryOperatorKind),
    BinaryOperator(BinaryOperatorKind),
    Brackets(BracketKinds),
    Punctuation(PunctuationKind),
    Slash,
    EOF,
}

struct Span {}

impl Span {
    fn new() -> Self {
        Span {}
    }
}

pub struct Token<'input> {
    token_kind: TokenKind<'input>,
    span: Span,
}

impl<'input> Token<'input> {
    fn new(token_kind: TokenKind<'input>) -> Self {
        Token {
            token_kind,
            span: Span {},
        }
    }
}

pub struct Lexer<'input> {
    input: &'input str,
    lines: usize,
    col: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            input,
            lines: 0,
            col: 0,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! duplicate_token {
            () => {};
        }

        macro_rules! bind_equal_token_to_token_or_not {
            ($token: tt, $c:ident, $view:ident) => {{
                let next_token = $view.peek();
                match next_token {
                    Some('=') => {
                        $view.next();
                        Token![$token =]
                    }
                    Some(n) if *n == $c => Token![$token],
                    _ => Token![$token],
                }
            }};
        }
        let c = 'c';
        self.input = self.input.trim_start();
        let mut view = self.input.chars().peekable();
        let c = view.next()?;
        let token = match c {
            '=' => bind_equal_token_to_token_or_not!(=, c, view),
            '+' => bind_equal_token_to_token_or_not!(+, c, view),
            '-' => bind_equal_token_to_token_or_not!(-, c, view),
            '*' => bind_equal_token_to_token_or_not!(*, c, view),
            '/' => bind_equal_token_to_token_or_not!(/, c, view),
            '>' => bind_equal_token_to_token_or_not!(>, c, view),
            '<' => bind_equal_token_to_token_or_not!(<, c, view),
            '&' => bind_equal_token_to_token_or_not!(&, c, view),
            '|' => bind_equal_token_to_token_or_not!(|, c, view),
            _ => Token::new(TokenKind::EOF),
        };

        None
    }
}
