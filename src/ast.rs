use crate::parser::{AstParser, AstParserError};
use crate::tokens::Token;

#[derive(Clone, Debug)]
pub struct TokenInfo {
    first_token: Token,
    last_token: Token
}

#[derive(Clone, PartialEq, Debug)]
pub enum Operator {
    Equal,
    NotEqual,

    GreaterThan,
    GreaterEqualThan,
    LowerThan,
    LowerEqualThan,

    And,
    Or,

    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Clone, PartialEq, Debug)]
pub enum HuleExpression {
    Undefined,
    Bracketed(Box<HuleExpression>),
    Boolean(bool),
    String(String),
    Integer(i32),
    Identifier(String),
    Call(String, Vec<HuleExpression>),
    Binary {
        left: Box<HuleExpression>,
        operator: Operator,
        right: Box<HuleExpression>,
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

impl HuleExpressionResultExt for Result<Vec<HuleExpression>, AstParserError> {
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
    data_type: String,
    name: String,
}

impl HuleParameter {
    pub fn new(data_type: &str, name: &str) -> HuleParameter{
        HuleParameter {
            data_type: data_type.to_string(),
            name: name.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HuleFuncCall {
    pub name: String,
    pub parameters: Vec<HuleExpression>,
}

#[derive(Clone, Debug)]
pub struct HuleFuncDef {
    pub name: String,
    pub parameters: Vec<HuleParameter>,
    pub return_type : String,
    pub body: Box<HuleStatement>,
}

#[derive(Clone, Debug)]
pub struct  HuleIfStatement {
    pub condition: HuleExpression,
    pub body: Box<HuleStatement>,
}
#[derive(Clone, Debug)]
pub struct  HuleVariableDecl {
    pub data_type: String,
    pub name: String,
    pub value: Option<HuleExpression>,
}

#[derive(Clone, Debug)]
pub struct  HuleVariableDef {
    pub name: String,
    pub value: HuleExpression,
}

#[derive(Clone, Debug)]
pub struct  HuleStructDecl {
    pub name: String,
    pub params: Vec<HuleParameter>,
}

#[derive(Clone, Debug)]
pub struct HuleStructDef {
    pub name: String,
    pub members: Vec<String>,
}
#[derive(Clone, Debug)]
pub struct HuleBody {
    pub items: Vec<HuleStatement>
}

impl HuleBody {
    pub fn new(items : Vec<HuleStatement>) -> HuleBody {
        HuleBody {
            items
        }
    }
}

/// Statements
///
#[derive(Debug, Clone)]
pub enum HuleStatement {
    Undefined,
    VariableDecl(HuleVariableDecl),
    VariableDef(HuleVariableDef),
    StructDecl(HuleStructDecl),
    StructDef(HuleStructDef),
    IfStatement(HuleIfStatement),
    FunctionDef(HuleFuncDef),
    FunctionCall(HuleFuncCall),
    Body(HuleBody),
    Return(HuleExpression),
}


pub trait FunctionCallIterator {
    fn iter_function_calls(&self) -> Box<dyn Iterator<Item = HuleFuncCall> + '_>;
}

impl FunctionCallIterator for Vec<HuleStatement> {
    fn iter_function_calls(&self) -> Box<dyn Iterator<Item = HuleFuncCall> + '_> {
        Box::new(self.iter().filter_map(|statement| {
            if let HuleStatement::FunctionCall(fn_call) = statement {
                Some(fn_call.clone())
            } else {
                None
            }
        }))
    }
}

#[derive(Debug)]
pub struct HuleProgramAst {
    pub body: HuleBody,
}