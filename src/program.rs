use std::ops::Index;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct HuleProgram {
    functions: Vec<Box<HuleFunction>>,
}
    
impl HuleProgram {
    fn new(program: &Rc<HuleProgram>) -> HuleProgram {
        HuleProgram {
            functions: vec![],
        }
    }
}

#[derive(Clone)]
pub struct HuleFunction {
    program: Weak<HuleProgram>
}

impl HuleFunction {
    pub fn new(program: &Rc<HuleProgram>) -> HuleFunction {
        HuleFunction {
            program: Rc::downgrade(program),
        }
    }

    pub fn get_func_index(&self) -> i32 {
        // if let Some(program) = self.program.upgrade() {
        //     program.functions.iter().position(self).unwrap() as i32
        // } else {
        //     -1
        // }
        42
    }
}

struct HuleVariable {
    program: Weak<HuleProgram>
}