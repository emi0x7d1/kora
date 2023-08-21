use unicode_ident::{is_xid_continue, is_xid_start};

use crate::{
    error::SyntaxError,
    token::{Token, TokenKind},
};

pub struct Lexer<'source> {
    /// Original, unmodified source code.
    /// This is used to calculate the current position in the source code.
    original_source_code: &'source str,

    /// Source code that has yet to be lexed.
    /// After some code has been lexed, this will be sliced to not include that code.
    source_code: &'source str,

    /// Syntax errors.
    errors: Vec<SyntaxError>,
}

impl<'source> Lexer<'source> {
    pub fn new(source_code: &'source str) -> Self {
        Self {
            original_source_code: source_code,
            source_code,
            errors: Vec::new(),
        }
    }

    pub fn consume_token(&mut self) -> Option<Token<'source>> {
        // We finished tokenizing the source code
        if self.source_code.len() == 0 {
            return None;
        }

        // Return the trivia token, if any.
        let trivia_token = self.consume_trivia();
        if trivia_token.is_some() {
            return trivia_token;
        }

        let mut chars = self.source_code.chars();

        let current_char = chars.next();
        let next_char = chars.next();

        match current_char {
            Some(current_char) => {
                // Check for two-char tokens
                if let Some(next_char) = next_char {
                    let two_char_token = self.consume_two_chars_token(current_char, next_char);
                    if two_char_token.is_some() {
                        return two_char_token;
                    }
                }

                // Check for one-char tokens
                let one_char_token = self.consume_one_char_token(current_char);
                if one_char_token.is_some() {
                    return one_char_token;
                }

                // Check for keywords
                let keyword_or_identifier_token = self.consume_keyword_or_identifier_token();
                if keyword_or_identifier_token.is_some() {
                    return keyword_or_identifier_token;
                }

                // If none of the past consumers got a token, then the reason is that the next
                // character is illegal
                let token_text = &self.source_code[..1];
                self.source_code = &self.source_code[1..];
                return Some(Token {
                    kind: TokenKind::Illegal,
                    text: token_text,
                });
            }
            None => return None,
        }
    }

    /// Consumes the next two-char token (if any) such as `&&` or `+=`.
    pub fn consume_two_chars_token(
        &mut self,
        current_char: char,
        next_char: char,
    ) -> Option<Token<'source>> {
        let token_kind = match (current_char, next_char) {
            ('+', '=') => TokenKind::PlusEqual,
            ('-', '=') => TokenKind::MinusEqual,
            ('*', '=') => TokenKind::MultiplyEqual,
            ('/', '=') => TokenKind::DivideEqual,
            ('%', '=') => TokenKind::ModuloEqual,
            ('&', '&') => TokenKind::AndAnd,
            ('|', '|') => TokenKind::OrOr,
            ('!', '=') => TokenKind::NotEqual,
            ('<', '=') => TokenKind::LessThanEqual,
            ('>', '=') => TokenKind::GreaterThanEqual,
            ('<', '<') => TokenKind::LessThanLessThan,
            ('>', '>') => TokenKind::GreaterThanGreaterThan,
            (_, _) => return None,
        };

        let text = &self.source_code[..2];
        self.source_code = &self.source_code[2..];

        Some(Token {
            kind: token_kind,
            text,
        })
    }

    /// Consumes the next one-char token (if any) such as `+` or `|`.
    pub fn consume_one_char_token(&mut self, current_char: char) -> Option<Token<'source>> {
        let token_kind = match current_char {
            '=' => TokenKind::Equal,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Multiply,
            '/' => TokenKind::Divide,
            '%' => TokenKind::Modulo,
            '&' => TokenKind::And,
            '|' => TokenKind::Or,
            '^' => TokenKind::Caret,
            '!' => TokenKind::Not,
            '<' => TokenKind::LessThan,
            '>' => TokenKind::GreaterThan,
            '(' => TokenKind::LeftParenthesis,
            ')' => TokenKind::RightParenthesis,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            ';' => TokenKind::Semicolon,
            ':' => TokenKind::Colon,
            _ => return None,
        };
        let text = &self.source_code[..1];
        self.source_code = &self.source_code[1..];
        Some(Token {
            kind: token_kind,
            text,
        })
    }

    /// Consumes the next keyword token (if any).
    pub fn consume_keyword_or_identifier_token(&mut self) -> Option<Token<'source>> {
        let mut chars = self.source_code.chars();
        let Some(first_char) = chars.next() else {
            return None;
        };
        // Check if the first char has the Unicode XID_Start property.
        if !is_xid_start(first_char) {
            return None;
        }

        let mut keyword_width = first_char.len_utf8();

        // Consume only valid XID_Continue characters.
        for char in chars {
            if !is_xid_continue(char) {
                break;
            }
            keyword_width += char.len_utf8();
        }

        let token_text = &self.source_code[..keyword_width];
        self.source_code = &self.source_code[keyword_width..];

        let token_kind = match token_text {
            "def" => TokenKind::Def,
            "extend" => TokenKind::Extend,
            "with" => TokenKind::With,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "struct" => TokenKind::Struct,
            _ => TokenKind::Identifier,
        };

        Some(Token {
            kind: token_kind,
            text: token_text,
        })
    }

    /// Consumes whitespace and comments.
    /// This function will never join two type of trivia in the same token.
    /// This means that a token is either whitespace or a comment, but not both.
    pub fn consume_trivia(&mut self) -> Option<Token<'source>> {
        let whitespace = self.consume_whitespace();
        if whitespace.is_some() {
            return whitespace;
        }

        let comment = self.consume_comment();
        if comment.is_some() {
            return comment;
        }
        None
    }

    /// Consumes whitespace.
    pub fn consume_whitespace(&mut self) -> Option<Token<'source>> {
        let mut whitespace_width = 0;

        for current_char in self.source_code.chars() {
            // If we found a non-whitespace character, stop iterating
            if !current_char.is_whitespace() {
                break;
            }
            // If it is whitespace, increment the whitespace width by
            // the length of the character in UTF-8
            whitespace_width += current_char.len_utf8();
        }

        // Check if there was any whitespace found
        if whitespace_width > 0 {
            // If there was whitespace, extract the token text from the
            // source code
            let token_text = &self.source_code[..whitespace_width];

            // Update the source code by removing the consumed whitespace
            self.source_code = &self.source_code[whitespace_width..];

            // Return a Token with the kind set to Trivia and the extracted token text
            Some(Token {
                kind: TokenKind::Trivia,
                text: token_text,
            })
        } else {
            // If no whitespace was found, return None
            None
        }
    }

    /// Consumes a comment include the newline.
    pub fn consume_comment(&mut self) -> Option<Token<'source>> {
        // Abort if the source code does not start with `//`
        if !self.source_code.starts_with("//") {
            return None;
        }

        // We initialize it to `2` because we want to include the `//` characters at the start.
        let mut comment_width = 2;

        for current_char in self.source_code.chars() {
            // Increment the comment width by the length of the current character in UTF-8 bytes
            comment_width += current_char.len_utf8();

            // We stop until we find a newline character but after we have included it in the token text
            if current_char == '\n' {
                break;
            }
        }

        // If the comment width is greater than 0, extract the token text and update the source code
        if comment_width > 0 {
            let token_text = &self.source_code[..comment_width];
            self.source_code = &self.source_code[comment_width..];

            Some(Token {
                kind: TokenKind::Trivia,
                text: token_text,
            })
        } else {
            // If the comment width is 0, return None
            None
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Token<'source>;
    fn next(&mut self) -> Option<Self::Item> {
        self.consume_token()
    }
}
