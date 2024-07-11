use std::sync::Arc;
use crate::basic_functions::BasicFunc;
use crate::custom_functions::CustomFunc;

#[derive(Clone)]
pub enum VariableType {
    Value(f64),
    Variable(String),
    BasicFunWVariables(Arc<BasicFunWVariables>),
    CustomFunWVariables(Arc<CustomFunWVariables>),
}

#[derive(Clone)]
pub struct BasicFunWVariables {
    pub basic_func: BasicFunc,
    pub func_variables: Vec<VariableType>,
}

impl BasicFunWVariables {
    pub fn new(basic_func: BasicFunc, func_variables: Vec<VariableType>) -> Self {
        Self {
            basic_func,
            func_variables,
        }
    }
}

#[derive(Clone)]
pub struct CustomFunWVariables {
    pub custom_func: Arc<CustomFunc>,
    pub func_variables: Vec<VariableType>,
}

impl CustomFunWVariables {
    pub fn new(custom_func: Arc<CustomFunc>, func_variables: Vec<VariableType>) -> Self {
        Self {
            custom_func,
            func_variables,
        }
    }
}
