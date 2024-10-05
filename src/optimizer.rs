/// hyperULE Compiler
/// Code Optimizer
///
/// Program Size Optimizer
/// - Minify / obfuscate variables
/// - Move duplicate constant values in to variables
/// - Remove duplicates from single-pass code generation like:
///   var + "" + var2 + ""    ->    var + "" + var2
/// - Remove dead code
///
/// RAM Optimizer:
/// - Move constant, single use variables in to constant expressions
///
/// Flash Memory Optimizer
/// -
///

use std::rc::{Rc, Weak};
use crate::program::HuleProgram;

pub enum OptimizerLevel {
    O0,
    O1,
    O2,
    O3
}


pub trait HuleOptimizer<T> {
    fn new(program: &Rc<HuleProgram>) -> T;
    fn optimize(&self, program: HuleProgram);
}

struct FirstLevelOptimizer {
    program: Weak<HuleProgram>
}

impl HuleOptimizer<FirstLevelOptimizer> for FirstLevelOptimizer {
    fn new(program: &Rc<HuleProgram>) -> FirstLevelOptimizer {
        FirstLevelOptimizer {
            program: Rc::downgrade(program)
        }
    }

    fn optimize(&self, program: HuleProgram) {

    }
}

struct SecondLevelOptimizer {
    first_level_optimizer: FirstLevelOptimizer
}

struct ThirdLevelOptimizer {
    first_level_optimizer: FirstLevelOptimizer,
    second_level_optimizer: SecondLevelOptimizer
}

impl HuleProgram {
    // pub fn optimize(self) -> HuleProgram {
    //     let result = *self.clone();
    //     result
    // }
}