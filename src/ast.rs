use crate::parser::{AstParser, AstParserError};
use crate::tokens::Token;

#[derive(Clone, Debug)]
pub struct TokenInfo {
    first_token: Token,
    last_token: Token
}

#[derive(Clone, Debug)]
pub enum Operator {
    Equals,
    NotEquals,

    GreaterThan,
    GreaterOrEqualsThan,
    LowerThan,
    LowerOrEqualsThan,

    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone, Debug)]
pub enum HuleExpression {
    Boolean(bool),
    String(String),
    Integer(i32),
    Identifier(String),
    Call(String, Vec<HuleExpression>),
    Binary {
        left: Box<HuleExpression>,
        right: Box<HuleExpression>,
        operator: Operator,
    },
}

pub trait HuleExpressionResultExt {
    fn or_reset(self, program : &mut AstParser, index : usize) -> Self;
}

impl HuleExpressionResultExt for Result<HuleStatement, AstParserError> {
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

impl HuleExpressionResultExt for Result<HuleExpression, AstParserError> {
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

impl HuleExpressionResultExt for Result<Token, AstParserError> {
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

#[derive(Debug, Clone)]

pub struct HuleParameter {
    pub(crate) data_type: String,
    pub(crate) name: String,
}

#[derive(Debug, Clone)]
pub enum HuleStatement {
    Undefined,

    VariableDecl {
        data_type: String,
        name: String,
        value: Option<HuleExpression>,
    },

    VariableDef {
        name: String,
        value: HuleExpression,
    },


    StructDecl {
        name: String,
        params: Vec<HuleParameter>,
    },

    StructDef {
        name: String,
        members: Vec<String>,
    },

    IfStatement {
        condition: HuleExpression,
        body: Vec<HuleStatement>,
    },

    FunctionDef {
        name: String,
        parameters: Vec<HuleParameter>,
        return_type : String,
        body: Vec<HuleStatement>,
    },

    Return(HuleExpression),
}

#[derive(Debug)]
pub struct HuleProgramAst {
    pub body: Vec<HuleStatement>,
}