use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::basic_functions::{BASIC_FUNCTIONS, BasicFunc};
use crate::variable_types::{VariableType, BasicFunWVariables, CustomFunWVariables};

pub struct CustomFunc {
    primary_func: BasicFunc,
    func_variables: Vec<VariableType>,
    provided_variable_names: Vec<String>,
}

pub(crate) static mut CUSTOM_FUNC_MAP: Lazy<HashMap<String, Arc<CustomFunc>>> = Lazy::new(|| {
    HashMap::new()
});

impl CustomFunc {
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
                VariableType::BasicFunWVariables(b_func) => {
                    let nested_processed_variables = self.process_variables(
                        mapped_provided_variables,
                        &b_func.func_variables,
                    );
                    let results = (b_func.basic_func)(&nested_processed_variables);

                    processed_variables.extend(results);
                }
                VariableType::CustomFunWVariables(c_func) => {
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

    pub fn run(&self, provided_variable_values: Vec<f64>) -> Vec<f64> {
        let mapped_provided_variables: HashMap<String, f64> = self
            .provided_variable_names
            .iter()
            .zip(provided_variable_values.iter())
            .map(|(k, &v)| (k.clone(), v))
            .collect();

        let processed_variables = self.process_variables(&mapped_provided_variables, &self.func_variables);
        (self.primary_func)(&processed_variables)
    }
}

// pub fn get_custom_functions() -> &'static HashMap<&'static str, CustomFunc> {
//     &CUSTOM_FUNC_MAP
// }