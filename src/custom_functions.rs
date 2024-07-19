use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write; // Import the Write trait
use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::basic_functions::{BASIC_FUNCTIONS, BasicFunc};
use crate::global_variables::get_by_name;
use crate::variable_types::{VariableType, CustomFuncWithVars, BasicFuncWithVars};

/// Struktura reprezentująca niestandardową funkcję
#[derive(Clone)]
pub struct CustomFunc {
    primary_func: BasicFunc,                  // Główna funkcja podstawowa
    func_variables: Vec<VariableType>,        // Lista zmiennych funkcji
    provided_variable_names: Vec<String>,     // Nazwy dostarczonych zmiennych
}

/// Statyczna mapa przechowująca niestandardowe funkcje
pub(crate) static mut CUSTOM_FUNC_MAP: Lazy<HashMap<String, Arc<CustomFunc>>> = Lazy::new(|| {
    HashMap::new()
});

pub static mut SAVE_FILE_NAME: &str = "REMEMBERED.txt";

pub unsafe fn set_save_file_name(new_name: &str) {
    // Box the new name and leak it to get a static reference
    SAVE_FILE_NAME = Box::leak(new_name.to_string().into_boxed_str());
}

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
                    else
                    {
                        processed_variables.extend(get_by_name(variable.to_string()));
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

unsafe fn recursive_variable_creator(recipe: &str) -> VariableType {
    if let Some(pos) = recipe.find('(') {
        let function_name = &recipe[..pos];
        let args_str = &recipe[pos + 1..recipe.len() - 1];

        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut parentheses_count = 0;

        for c in args_str.chars() {
            match c {
                ',' if parentheses_count == 0 => {
                    args.push(current_arg.trim().to_string());
                    current_arg = String::new();
                }
                '(' => {
                    parentheses_count += 1;
                    current_arg.push(c);
                }
                ')' => {
                    parentheses_count -= 1;
                    current_arg.push(c);
                }
                _ => {
                    current_arg.push(c);
                }
            }
        }

        if !current_arg.trim().is_empty() {
            args.push(current_arg.trim().to_string());
        }

        let mut parsed_variables = Vec::new();
        for arg in args {
            parsed_variables.push(recursive_variable_creator(&arg));
        }

        if let Some(&basic_func) = BASIC_FUNCTIONS.get(function_name) {
            VariableType::BasicFuncWithVars(Arc::new(BasicFuncWithVars::new(basic_func, parsed_variables)))
        } else if let Some(custom_func) = CUSTOM_FUNC_MAP.get(function_name) {
            VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(Arc::clone(custom_func), parsed_variables)))
        } else {
            panic!("Unknown function: {}", function_name);
        }
    } else {
        if let Ok(num) = recipe.replace(")", "").parse::<f64>() {
            VariableType::Value(num)
        } else {
            VariableType::Variable(recipe.replace(")", "").to_string())
        }
    }
}

unsafe fn create_custom_function(
    name: Option<String>,
    recipe: &str,
    variable_names: Vec<String>
) -> Arc<CustomFunc> {
    let function_variable_type = recursive_variable_creator(recipe);

    if let VariableType::BasicFuncWithVars(basic_func_wrapper) = function_variable_type {
        let basic_func = basic_func_wrapper.basic_func;
        let variables = basic_func_wrapper.func_variables.clone();

        let custom_func = Arc::new(CustomFunc::new(
            basic_func,
            variables,
            variable_names,
        ));

        if let Some(name) = name {
            CUSTOM_FUNC_MAP.insert(name, custom_func.clone());
        }
        custom_func
    } else {
        panic!("Unexpected error in creating custom function");
    }
}

/// Funkcja interpretująca pełne równanie i tworząca niestandardową funkcję
pub unsafe fn interpret(full_equation: &str, save_into_file: bool) -> Arc<CustomFunc> {
    let parts: Vec<&str> = full_equation.split(|c: char| c == '=' || c == ';').collect();

    match parts.len() {
        3 => {
            let name = parts[0].trim().to_string();
            let recipe = format!("pass({})", parts[1].trim());
            let variable_names = parts[2].trim().split(',').map(String::from).collect();

            if save_into_file {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(SAVE_FILE_NAME)
                    .expect("Unable to open file");
                writeln!(file, "{}", full_equation).expect("Unable to write to file");
            }

            create_custom_function(Some(name), &recipe, variable_names)
        }
        2 if full_equation.contains('=') => {
            let name = parts[0].trim().to_string();
            let recipe = format!("pass({})", parts[1].trim());

            if save_into_file {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(SAVE_FILE_NAME)
                    .expect("Unable to open file");
                writeln!(file, "{}", full_equation).expect("Unable to write to file");
            }

            create_custom_function(Some(name), &recipe, Vec::new())
        }
        2 => {
            let recipe = format!("pass({})", parts[0].trim());
            let variable_names = parts[1].trim().split(',').map(String::from).collect();

            if save_into_file {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(SAVE_FILE_NAME)
                    .expect("Unable to open file");
                writeln!(file, "{}", full_equation).expect("Unable to write to file");
            }

            create_custom_function(None, &recipe, variable_names)
        }
        1 => {
            let recipe = format!("pass({})", parts[0].trim());

            if save_into_file {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(SAVE_FILE_NAME)
                    .expect("Unable to open file");
                writeln!(file, "{}", "V:".to_string()+&full_equation).expect("Unable to write to file");
            }

            create_custom_function(None, &recipe, Vec::new())
        }
        _ => panic!("Invalid equation format"),
    }
}

pub unsafe fn run_custom_logic() {
    // Example interpretations
    //interpret("a=sum(sum(1,sum(x,y)),multiply(x,y));x,y",true);
    // interpret("sum(1,1)");
    println!("RESULT: {:?}", CUSTOM_FUNC_MAP.get("a").unwrap().run(vec![1.0, 2.0]));

    let custom_func = CUSTOM_FUNC_MAP.get("a").unwrap().clone();

    // Create global variables
    //create_global_variable("global_var1".to_string(), GlobalVariable::new((**func_arc).clone(), vec![1.0, 2.0]);
    let declaration = "global_var2 = a(3.0, 4.0)".to_string();
    // unsafe {
    //     create_global_variable_text(declaration,true);
    // }

    // Retrieve and print the values of the global variables
    //println!("GLOBAL VAR1: {:?}", get_by_name("global_var1".to_string()));
    println!("GLOBAL VAR2: {:?}", get_by_name("global_var2".to_string()));

    //interpret("b=sum(global_var2,x);x",true);
    println!("RESULT: {:?}", CUSTOM_FUNC_MAP.get("b").unwrap().run(vec![1.0]));
}
