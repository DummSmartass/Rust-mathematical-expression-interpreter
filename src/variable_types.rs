use crate::basic_functions::BasicFunc;
use crate::custom_functions::CustomFunc;


pub enum VariableType<'a> {
    Value(f64),
    Variable(&'a str),
    BasicFunWVariables(BasicFunWVariables<'a>),
    CustomFunWVariables(CustomFunWVariables<'a>),
}

pub struct BasicFunWVariables<'a> {
    pub basic_func: BasicFunc,
    pub func_variables: Vec<VariableType<'a>>,
}

impl<'a> BasicFunWVariables<'a> {
    pub fn new(basic_func: BasicFunc, func_variables: Vec<VariableType<'a>>) -> Self {
        Self {
            basic_func,
            func_variables,
        }
    }
}

pub struct CustomFunWVariables<'a> {
    pub custom_func: &'a CustomFunc<'a>,
    pub func_variables: Vec<VariableType<'a>>,
}

impl<'a> CustomFunWVariables<'a> {
    pub fn new(custom_func: &'a CustomFunc<'a>, func_variables: Vec<VariableType<'a>>) -> Self {
        Self {
            custom_func,
            func_variables,
        }
    }
}
