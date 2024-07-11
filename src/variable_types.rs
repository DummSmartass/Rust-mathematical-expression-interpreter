use std::sync::Arc;
use crate::basic_functions::BasicFunc;
use crate::custom_functions::CustomFunc;

/// Enum reprezentujący różne typy zmiennych
#[derive(Clone)]
pub enum VariableType {
    Value(f64),                                 // Prosta wartość liczby zmiennoprzecinkowej
    Variable(String),                           // Nazwa zmiennej
    BasicFuncWithVars(Arc<BasicFuncWithVars>),  // Funkcja podstawowa z zmiennymi
    CustomFuncWithVars(Arc<CustomFuncWithVars>),// Funkcja niestandardowa z zmiennymi
}

/// Struktura reprezentująca funkcję podstawową z jej zmiennymi
#[derive(Clone)]
pub struct BasicFuncWithVars {
    pub basic_func: BasicFunc,                  // Funkcja podstawowa
    pub func_variables: Vec<VariableType>,      // Lista zmiennych funkcji
}

impl BasicFuncWithVars {
    /// Tworzy nową instancję BasicFuncWithVars
    pub fn new(basic_func: BasicFunc, func_variables: Vec<VariableType>) -> Self {
        Self {
            basic_func,
            func_variables,
        }
    }
}

/// Struktura reprezentująca funkcję niestandardową z jej zmiennymi
#[derive(Clone)]
pub struct CustomFuncWithVars {
    pub custom_func: Arc<CustomFunc>,           // Funkcja niestandardowa
    pub func_variables: Vec<VariableType>,      // Lista zmiennych funkcji
}

impl CustomFuncWithVars {
    /// Tworzy nową instancję CustomFuncWithVars
    pub fn new(custom_func: Arc<CustomFunc>, func_variables: Vec<VariableType>) -> Self {
        Self {
            custom_func,
            func_variables,
        }
    }
}
