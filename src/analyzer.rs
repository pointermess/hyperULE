use crate::ast::{FunctionCallIterator, HuleFuncCall, HuleProgramAst, HuleStatement};
use crate::program::HuleProgram;

pub enum AnalyzerError {
    EntryPointMissing,
    FunctionRedefined(String),
}

pub struct Analyzer {
    program: HuleProgramAst
}

impl Analyzer {
    pub fn new(program: HuleProgramAst) -> Analyzer {
        Analyzer {
            program
        }
    }

    pub fn get_function_by_name(&self, name: &str) -> Option<HuleFuncCall> {
        self.program.body.items
            .iter_function_calls()
            .find(|p| p.name == name)
    }

    pub fn contains_function(&self, name: &str) -> bool {
        self.get_function_by_name(name)
            .is_some()
    }

    fn analyze_unique_functions() -> Result<(), AnalyzerError> {
        Err(AnalyzerError::EntryPointMissing)
    }

    pub fn analyze(&mut self) {

    }
}