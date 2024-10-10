use crate::ast::HuleExpression;
use crate::generator::{HyperBackend, HyperNode};
use crate::program::HuleProgram;

struct UleBackend {

}

struct UleScope {
    parent_scope: Option<Box<UleScope>>,
}

struct UleVarDef {
    scope: Box<UleScope>,
    name: String,
    value: String
}

struct UleIfStatement {
    condition: HuleExpression,
    body: Vec<Box<UleNode>>
}

enum UleNode {
    VariableDefinition(UleVarDef),
    IfStatement(UleIfStatement),
    Scope(UleScope)
}

impl HyperBackend<HuleProgram, String> for UleBackend {
    fn generate(source: HuleProgram) -> String {
        "{\n}".to_string()
    }
}