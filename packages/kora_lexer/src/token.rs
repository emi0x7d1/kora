#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'source> {
    pub kind: TokenKind,
    pub text: &'source str,
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // # Literals.
    /// Any word made up of valid identifier characters that is not a keyword.
    Identifier,
    /// An integer literal such as `0`, `0x123`, `0o123` and `0b1010`
    IntegerLiteral,
    /// A float literal such as `3.14`.
    FloatLiteral,
    /// A string literal, including prefixed strings, such as `"Hello, world"` or `f"Hello, {world}"`.
    StringLiteral,

    // # Arithmetic operators.
    /// The `=` character.
    Equal,
    /// The `==` character..
    EqualEqual,
    /// The `!=` characters.
    NotEqual,
    /// The `+` character.
    Plus,
    /// The `-` character.
    Minus,
    /// The `*` character.
    Multiply,
    /// The `/` character.
    Divide,
    /// The `%` character.
    Modulo,
    /// The `+=` characters.
    PlusEqual,
    /// The `-=` character.
    MinusEqual,
    /// The `*=` character.
    MultiplyEqual,
    /// The `/=` character.
    DivideEqual,
    /// The `%=` character.
    ModuloEqual,

    // # Logical operators.
    /// The `!` character.
    Not,
    /// The `||` character.
    OrOr,
    /// The `&&` character.
    AndAnd,
    /// The `<` character.
    LessThan,
    /// The `>` character.
    GreaterThan,
    /// The `<=` character.
    LessThanEqual,
    /// The `>=` character.
    GreaterThanEqual,

    // # Bitwise operators.
    /// The `&` character.
    And,
    /// The `|` character.
    Or,
    /// The `^` character.
    Caret,
    /// The `<<` characters.
    LessThanLessThan,
    /// The `>>` characters.
    GreaterThanGreaterThan,

    // # Punctuation.
    /// The `(` character.
    LeftParenthesis,
    /// The `)` character.
    RightParenthesis,
    /// The `[` character.
    LeftBracket,
    /// The `]` character.
    RightBracket,
    /// The `{` character.
    LeftBrace,
    /// The `}` character.
    RightBrace,
    /// The `,` character.
    Comma,
    /// The `.` character.
    Dot,
    /// The `;` character.
    Semicolon,
    /// The `:` character.
    Colon,

    // # Keywords.
    /// The `def` keyword.
    Def,
    /// The `extend` keyword.
    Extend,
    /// The `with` keyword.
    With,
    /// The `if` keyword.
    If,
    /// The `else` keyword.
    Else,
    /// The `for` keyword.
    For,
    /// The `struct` keyword.
    Struct,
    /// Trivia, such as whitespace or comments.
    Trivia,
    /// Illegal character.
    Illegal
}
