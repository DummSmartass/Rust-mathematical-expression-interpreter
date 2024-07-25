use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use crate::custom_functions::{CUSTOM_FUNC_MAP, CustomFunc, SAVE_FILE_NAME};

pub struct GlobalVariable {
    pub func: CustomFunc,
    pub variables: Vec<f64>,
    pub result: Vec<f64>,
    pub updated: bool,
}

impl GlobalVariable {
    pub fn new(func: CustomFunc, variables: Vec<f64>) -> Self {
        Self {
            func,
            variables,
            result: Vec::new(),
            updated: false,
        }
    }

    pub fn get(&mut self) -> Vec<f64> {
        if !self.updated {
            self.result = self.func.run(self.variables.clone());
            self.updated = true;
        }
        self.result.clone()
    }

    pub fn set_not_updated(&mut self) {
        self.updated = false;
    }
}

static GLOBAL_VARIABLES: Lazy<Mutex<HashMap<String, Arc<Mutex<GlobalVariable>>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub fn get_variable_by_name(name: String) -> Vec<f64> {
    let global_variables = GLOBAL_VARIABLES.lock().unwrap();
    let chosen_variable_arc = global_variables.get(&name).expect("Variable not found");
    let mut chosen_variable = chosen_variable_arc.lock().unwrap();
    chosen_variable.get()
}

pub fn create_global_variable(name: String, func: CustomFunc, variables: Vec<f64>) {
    let global_variable = Arc::new(Mutex::new(GlobalVariable::new(func, variables)));
    GLOBAL_VARIABLES.lock().unwrap().insert(name, global_variable);
}

pub unsafe fn create_global_variable_text(declaration: String, save_into_file: bool) {
    // Example: name=nameOfFunction(1,2,3,4)
    let parts: Vec<&str> = declaration.split('=').collect();
    let name = parts[0].trim().to_string();

    let binding = parts[1].replace(')', "");
    let function_parts = binding.split('(').collect::<Vec<&str>>();

    let func_arc = CUSTOM_FUNC_MAP.get(function_parts[0].trim()).expect("Function not found");

    let variables: Vec<f64> = function_parts[1]
        .trim()
        .split(',')
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect();

    let func = (**func_arc).clone();

    if save_into_file {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(SAVE_FILE_NAME)
            .expect("Unable to open file");
        writeln!(file, "{}", "V:".to_string()+&declaration).expect("Unable to write to file");
    }

    create_global_variable(name, func, variables);
}

pub fn set_all_not_updated() {
    let global_variables = GLOBAL_VARIABLES.lock().unwrap();
    for global_var_arc in global_variables.values() {
        let mut global_var = global_var_arc.lock().unwrap();
        global_var.set_not_updated();
    }
}

pub fn set_not_updated_by_name(name: &str) {
    let global_variables = GLOBAL_VARIABLES.lock().unwrap();
    if let Some(global_var_arc) = global_variables.get(name) {  // No need to use &name here
        let mut global_var = global_var_arc.lock().unwrap();
        global_var.set_not_updated();
    } else {
        panic!("Variable not found");
    }
}


