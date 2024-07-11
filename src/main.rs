mod basic_functions;
mod custom_functions;
mod variable_types;

use basic_functions::BasicFunc;
use custom_functions::CustomFunc;
use variable_types::{VariableType, CustomFuncWithVars, BasicFuncWithVars};
use std::sync::Arc;
use std::collections::HashMap;
use std::ptr::null;
use eval::Value;
use once_cell::sync::Lazy;
use crate::basic_functions::BASIC_FUNCTIONS;
use crate::custom_functions::CUSTOM_FUNC_MAP;

/// Funkcja rekurencyjna tworząca typ zmiennej na podstawie ciągu znaków
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

/// Funkcja interpretująca pełne równanie i tworząca niestandardową funkcję
unsafe fn interpret(full_equation: &str) -> Arc<CustomFunc> {
    let parts: Vec<&str> = full_equation.split(|c: char| c == '=' || c == ';').collect();
    let name: String;
    let recipe: String;
    let variable_names: Vec<String>;
    let function_variable_type: VariableType;

    if(full_equation.contains("=") & full_equation.contains(";"))
    {
        name = parts[0].trim().to_string();
        recipe = format!("pass({})", parts[1].trim());
        variable_names = parts[2].trim().split(',').map(String::from).collect();

        function_variable_type = recursive_variable_creator(&recipe);

        if let VariableType::BasicFuncWithVars(basic_func_wrapper) = function_variable_type {
            let basic_func = basic_func_wrapper.basic_func;
            let variables = basic_func_wrapper.func_variables.clone();

            let custom_func = Arc::new(CustomFunc::new(
                basic_func,
                variables,
                variable_names,
            ));

            CUSTOM_FUNC_MAP.insert(name, custom_func.clone());
            custom_func
        } else {
            panic!("Unexpected error1 in creating custom function");
        }
    }
    else if full_equation.contains("=")
    {
        name = parts[0].trim().to_string();
        recipe = format!("pass({})", parts[1].trim());
        variable_names = Vec::new();

        function_variable_type = recursive_variable_creator(&recipe);

        if let VariableType::BasicFuncWithVars(basic_func_wrapper) = function_variable_type {
            let basic_func = basic_func_wrapper.basic_func;
            let variables = basic_func_wrapper.func_variables.clone();

            let custom_func = Arc::new(CustomFunc::new(
                basic_func,
                variables,
                variable_names,
            ));

            CUSTOM_FUNC_MAP.insert(name, custom_func.clone());
            custom_func
        } else {
            panic!("Unexpected error2 in creating custom function");
        }
    }
    else if full_equation.contains(";")
    {
        recipe = format!("pass({})", parts[0].trim());
        variable_names = parts[1].trim().split(',').map(String::from).collect();

        function_variable_type = recursive_variable_creator(&recipe);

        if let VariableType::BasicFuncWithVars(basic_func_wrapper) = function_variable_type {
            let basic_func = basic_func_wrapper.basic_func;
            let variables = basic_func_wrapper.func_variables.clone();

            let custom_func = Arc::new(CustomFunc::new(
                basic_func,
                variables,
                variable_names,
            ));

            custom_func
        } else {
            panic!("Unexpected error3 in creating custom function");
        }
    }
    else
    {
        recipe = format!("pass({})", parts[0].trim());

        function_variable_type = recursive_variable_creator(&recipe);
        variable_names = Vec::new();

        if let VariableType::BasicFuncWithVars(basic_func_wrapper) = function_variable_type {
            let basic_func = basic_func_wrapper.basic_func;
            let variables = basic_func_wrapper.func_variables.clone();

            let custom_func = Arc::new(CustomFunc::new(
                basic_func,
                variables,
                variable_names,
            ));

            custom_func
        } else {
            panic!("Unexpected error4 in creating custom function");
        }
    }
}



fn main() {
    unsafe {
        //interpret("a=sum(sum(1,sum(x,y)),multiply(x,y));x,y");
        interpret("sum(1,1)");
        //println!("RESULT: {:?}", CUSTOM_FUNC_MAP.get("a").unwrap().run(vec![1.0, 2.0]));
    }
}


//LATER
//global variables
//why must I always use pass for nonbasic functions


// fn main() {
//     // Create a local HashMap to store instances of CustomFunc wrapped in Arc
//     unsafe {
//     let value2 = VariableType::Variable("y".to_string());
//     let variables1 = vec![value2.clone(), value2.clone()];
//     let my_instance1 = Arc::new(CustomFunc::new(
//         BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//         variables1, // Pass variables1 without cloning
//         vec!["y".to_string()],
//     ));
//
//     let result = my_instance1.run(vec![1.0, 2.0]);
//     println!("{:?}", result);
//
//     // Store my_instance1 in the map
//     CUSTOM_FUNC_MAP.insert("my_instance1".to_string(), my_instance1.clone());
//
//     let value1 = VariableType::BasicFunWVariables(Arc::new(BasicFunWVariables::new(BASIC_FUNCTIONS.get("get_series").unwrap().clone(), vec![VariableType::Value(2.0)])));
//     let variables2 = vec![value1.clone()];
//     let my_instance2 = Arc::new(CustomFunc::new(
//         BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//         variables2, // Pass variables2 without cloning
//         vec![],
//     ));
//     let result = my_instance2.run(vec![]);
//     println!("{:?}", result);
//
//     // Store my_instance2 in the map
//     CUSTOM_FUNC_MAP.insert("my_instance2".to_string(), my_instance2.clone());
//
//     // Retrieve instances from the map before creating new CustomFunc instances
//     let instance1 = CUSTOM_FUNC_MAP.get("my_instance1").unwrap().clone();
//     let instance2 = CUSTOM_FUNC_MAP.get("my_instance2").unwrap().clone();
//
//     let value1 = VariableType::CustomFunWVariables(Arc::new(CustomFunWVariables::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//     let value2 = VariableType::CustomFunWVariables(Arc::new(CustomFunWVariables::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//     let variables = vec![value1.clone(), value2.clone()];
//     let my_instance3 = Arc::new(CustomFunc::new(
//         BASIC_FUNCTIONS.get("multiply").unwrap().clone(),
//         variables,
//         vec!["x".to_string()],
//     ));
//     let result = my_instance3.run(vec![1.0, 2.0]);
//     println!("{:?}", result);
//
//     // Store my_instance3 in the map
//     CUSTOM_FUNC_MAP.insert("my_instance3".to_string(), my_instance3.clone());
//
//     // Retrieve instance from the map before creating new CustomFunc instance
//     let instance3 = CUSTOM_FUNC_MAP.get("my_instance3").unwrap().clone();
//
//     let value1 = VariableType::CustomFunWVariables(Arc::new(CustomFunWVariables::new(instance3.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//     let value2 = VariableType::Value(3.0);
//     let variables = vec![value1.clone(), value2.clone()];
//     let my_instance4 = Arc::new(CustomFunc::new(
//         BASIC_FUNCTIONS.get("divide").unwrap().clone(),
//         variables,
//         vec!["x".to_string()],
//     ));
//     let result = my_instance4.run(vec![1.0, 2.0]);
//     println!("{:?}", result);
//
//     // Store my_instance4 in the map
//     CUSTOM_FUNC_MAP.insert("my_instance4".to_string(), my_instance4.clone());
//
//     // Retrieve and run a function from the map
//     if let Some(func) = CUSTOM_FUNC_MAP.get("my_instance4") {
//         let result = func.run(vec![1.0, 2.0]);
//         println!("Result of my_instance4: {:?}", result);
//     } else {
//         println!("Function not found");
//     }
// }
// }
