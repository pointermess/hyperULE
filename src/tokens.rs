use std::cmp::PartialEq;
use crate::ast::{HuleExpressionResultExt, Operator};
use crate::parser::{AstParser, AstParserError};

#[derive(PartialEq, Copy, Clone, Debug,)]
pub enum TokenType {
    Undefined = 0,
    Unknown = 1,
    Identifier = 2,
    ConstIntegerExpression = 3,
    ConstStringExpression = 4,
    BracketOpen = 5,
    BracketClose = 6,
    CurlyBracketOpen = 7,
    CurlyBracketClose = 8,
    SquareBracketOpen = 9,
    SquareBracketClose = 10,
    Assign = 11,
    Equal = 12,
    NotEqual = 13,
    GreaterThan = 14,
    LowerThan = 15,
    GreaterEqualThan = 16,
    LowerEqualThan = 17,
    And = 18,
    Or = 19,
    Comma = 20,
    Semicolon = 21,
    Plus = 22,
    Minus = 23,
    Divide = 24,
    Multiply = 25,
}

impl TokenType {
    pub fn to_string(&self) -> String {
        match self {
            TokenType::Undefined => "Undefined".to_string(),
            TokenType::Unknown => "Unknown".to_string(),
            TokenType::Identifier => "Identifier".to_string(),
            TokenType::ConstIntegerExpression => "ConstIntegerExpression".to_string(),
            TokenType::ConstStringExpression => "ConstStringExpression".to_string(),
            TokenType::BracketOpen => "BracketOpen".to_string(),
            TokenType::BracketClose => "BracketClose".to_string(),
            TokenType::CurlyBracketOpen => "CurlyBracketOpen".to_string(),
            TokenType::CurlyBracketClose => "CurlyBracketClose".to_string(),
            TokenType::SquareBracketOpen => "SquareBracketOpen".to_string(),
            TokenType::SquareBracketClose => "SquareBracketClose".to_string(),
            TokenType::Assign => "Assign".to_string(),
            TokenType::Equal => "Equal".to_string(),
            TokenType::NotEqual => "NotEqual".to_string(),
            TokenType::GreaterThan => "GreaterThan".to_string(),
            TokenType::LowerThan => "LowerThan".to_string(),
            TokenType::GreaterEqualThan => "GreaterEqualThan".to_string(),
            TokenType::LowerEqualThan => "LowerEqualThan".to_string(),
            TokenType::And => "And".to_string(),
            TokenType::Or => "Or".to_string(),
            TokenType::Comma => "Comma".to_string(),
            TokenType::Semicolon => "Semicolon".to_string(),
            TokenType::Plus => "Plus".to_string(),
            TokenType::Minus => "Minus".to_string(),
            TokenType::Divide => "Divide".to_string(),
            TokenType::Multiply => "Multiply".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    pub value: String,
    token_start : u32,
    token_end : u32
}

impl HuleExpressionResultExt for Result<Operator, AstParserError> {
    fn or_reset(self, program : &mut AstParser, index : usize) -> Self
    {
        match &self {
            Ok(_) => {
                self
            }
            Err(_) => {
                program.tokens.set_current_token_index(index);
                self
            }
        }
    }
}

impl Token {
    pub fn new(value : String, token_start : u32, token_end : u32) -> Token {
        Token {
            token_type: TokenType::Unknown,
            value,
            token_start,
            token_end,
        }
    }

    pub fn get_calculated_token_type(&self) -> TokenType {
        let token: &str = &self.value;
        match token {
            "(" => TokenType::BracketOpen,
            ")" => TokenType::BracketClose,
            "{" => TokenType::CurlyBracketOpen,
            "}" => TokenType::CurlyBracketClose,
            "[" => TokenType::SquareBracketOpen,
            "]" => TokenType::SquareBracketClose,
            "=" => TokenType::Assign,
            "==" => TokenType::Equal,
            "!=" => TokenType::NotEqual,
            ">" => TokenType::GreaterThan,
            "<" => TokenType::LowerThan,
            ">=" => TokenType::GreaterEqualThan,
            "<=" => TokenType::LowerEqualThan,
            "&&" => TokenType::And,
            "||" => TokenType::Or,
            "," => TokenType::Comma,
            ";" => TokenType::Semicolon,
            "+" => TokenType::Plus,
            "-" => TokenType::Minus,
            "/" => TokenType::Divide,
            "*" => TokenType::Multiply,
            _ => {
                if token.starts_with('"') && token.ends_with('"') {
                    TokenType::ConstStringExpression
                } else if let Ok(int) = &self.value.parse::<i32>() {
                    TokenType::ConstIntegerExpression
                } else if let Ok(int) = &self.value.parse::<i32>() {
                    TokenType::ConstIntegerExpression
                } else {
                    TokenType::Identifier
                }
            }
        }
    }

    pub fn get_token_type(&mut self) -> TokenType {
        if self.token_type != TokenType::Unknown {
            return self.token_type;
        }

        self.token_type = self.get_calculated_token_type();
        self.token_type
    }
}