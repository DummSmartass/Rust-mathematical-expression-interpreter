use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::basic_functions::{BASIC_FUNCTIONS, BasicFunc};
use crate::variable_types::VariableType;

/// Struktura reprezentująca niestandardową funkcję
pub struct CustomFunc {
    primary_func: BasicFunc,                  // Główna funkcja podstawowa
    func_variables: Vec<VariableType>,        // Lista zmiennych funkcji
    provided_variable_names: Vec<String>,     // Nazwy dostarczonych zmiennych
}

/// Statyczna mapa przechowująca niestandardowe funkcje
pub(crate) static mut CUSTOM_FUNC_MAP: Lazy<HashMap<String, Arc<CustomFunc>>> = Lazy::new(|| {
    HashMap::new()
});

impl CustomFunc {
    /// Tworzy nową instancję CustomFunc
    pub fn new(
        primary_func: BasicFunc,
        func_variables: Vec<VariableType>,
        provided_variable_names: Vec<String>,
    ) -> CustomFunc {
        CustomFunc {
            primary_func,
            func_variables,
            provided_variable_names,
        }
    }

    /// Przetwarza zmienne funkcji, zamieniając je na wartości liczbowe
    fn process_variables(
        &self,
        mapped_provided_variables: &HashMap<String, f64>,
        func_variables: &[VariableType],
    ) -> Vec<f64> {
        let mut processed_variables = Vec::new();

        for (i, func_variable) in func_variables.iter().enumerate() {
            match func_variable {
                VariableType::Value(value) => processed_variables.push(*value),
                VariableType::Variable(variable) => {
                    if let Some(value) = mapped_provided_variables.get(variable) {
                        processed_variables.push(*value);
                    }
                }
                VariableType::BasicFuncWithVars(b_func) => {
                    let nested_processed_variables = self.process_variables(
                        mapped_provided_variables,
                        &b_func.func_variables,
                    );
                    let results = (b_func.basic_func)(&nested_processed_variables);
                    processed_variables.extend(results);
                }
                VariableType::CustomFuncWithVars(c_func) => {
                    let nested_processed_variables = self.process_variables(
                        mapped_provided_variables,
                        &c_func.func_variables,
                    );
                    let results = c_func.custom_func.run(nested_processed_variables);
                    processed_variables.extend(results);
                }
            }
        }

        processed_variables
    }

    /// Wykonuje niestandardową funkcję z dostarczonymi wartościami zmiennych
    pub fn run(&self, provided_variable_values: Vec<f64>) -> Vec<f64> {
        // Mapowanie dostarczonych zmiennych na ich wartości
        let mapped_provided_variables: HashMap<String, f64> = self
            .provided_variable_names
            .iter()
            .zip(provided_variable_values.iter())
            .map(|(k, &v)| (k.clone(), v))
            .collect();

        // Przetworzenie zmiennych funkcji
        let processed_variables = self.process_variables(&mapped_provided_variables, &self.func_variables);
        (self.primary_func)(&processed_variables)
    }
}

