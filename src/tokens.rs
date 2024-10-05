use std::cmp::PartialEq;

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
    ConstStringValue = 26,
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    pub value: String,
    token_start : u32,
    token_end : u32
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
                    TokenType::ConstStringValue
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