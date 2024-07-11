mod basic_functions;
mod custom_functions;
mod variable_types;

use basic_functions::BasicFunc;
use custom_functions::CustomFunc;
use variable_types::{VariableType, CustomFunWVariables, BasicFunWVariables};
use std::sync::Arc;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::basic_functions::BASIC_FUNCTIONS;
use crate::custom_functions::CUSTOM_FUNC_MAP;

unsafe fn basic_divide<'a>(text: &'a str) -> VariableType {
    if let Some(pos) = text.find('(') {
        let function_name = &text[..pos];
        println!("function: {}", function_name);

        let args_str = &text[pos + 1..text.len() - 1];
        println!("args_str: {}", args_str);

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
            parsed_variables.push(basic_divide(&arg));
        }

        if let Some(&basic_func) = BASIC_FUNCTIONS.get(function_name) {
            println!("here1");
            VariableType::BasicFunWVariables(Arc::new(BasicFunWVariables::new(basic_func, parsed_variables)))
        } else if let Some(custom_func) = CUSTOM_FUNC_MAP.get(function_name) {
            println!("here2");
            VariableType::CustomFunWVariables(Arc::new(CustomFunWVariables::new(Arc::clone(custom_func), parsed_variables)))
        } else {
            panic!("Unknown function: {}", function_name);
        }



    } else {
        println!("Value: {}", text);
        if let Ok(num) = text.replace(")", "").parse::<f64>() {
            println!("here3");
            VariableType::Value(num)
        } else {
            println!("here4");
            VariableType::Variable(text.replace(")", "").to_string())
        }
    }
}


// fn main() {
//     // Parse the input text to create a variable type
//     //let result = basic_divide("sum(x,sum(sum(x,y),sum(sum(x,sum(x,y)),y)),multiply(x,y))");
//     let result = unsafe { basic_divide("sum(sum(1,sum(x,y)),multiply(x,y))") };
//
//     // Use variables x and y for testing
//     let x = 2.0;
//     let y = 3.0;
//
//     match result {
//         VariableType::BasicFunWVariables(basic_func_wrapper) => {
//             // Extract the basic function and its variables
//             let basic_func = basic_func_wrapper.basic_func;
//             let variables = basic_func_wrapper.func_variables.clone();
//
//             // Create a CustomFunc using the extracted basic function and variables
//             let custom_func = Arc::new(CustomFunc::new(
//                 basic_func,
//                 variables,
//                 vec![],  // Keeping this vector empty as per your instructions
//                 vec!["x".to_string(), "y".to_string()],
//             ));
//
//             // Run the custom function with test variables
//             let result = custom_func.run(vec![x, y]);
//             println!("Result of custom function: {:?}", result);
//
//             unsafe {
//                 CUSTOM_FUNC_MAP.insert("a".to_string(), custom_func);
//
//                 let result = unsafe { basic_divide("pass(a(x,y))") };
//
//                 match result {
//                     VariableType::BasicFunWVariables(basic_func_wrapper) => {
//                         // Extract the basic function and its variables
//                         let basic_func = basic_func_wrapper.basic_func;
//                         let variables = basic_func_wrapper.func_variables.clone();
//
//                         // Create a CustomFunc using the extracted basic function and variables
//                         let custom_func = Arc::new(CustomFunc::new(
//                             basic_func,
//                             variables,
//                             vec![],  // Keeping this vector empty as per your instructions
//                             vec!["x".to_string(), "y".to_string()],
//                         ));
//
//                         // Run the custom function with test variables
//                         let result = custom_func.run(vec![x, y]);
//                         println!("Result of custom function: {:?}", result);
//
//                     }
//                     VariableType::CustomFunWVariables(custom_func_wrapper) => {
//                         // Directly run the custom function if it is already wrapped in CustomFunWVariables
//                         let result = custom_func_wrapper.custom_func.run(vec![x, y]);
//                         println!("Result of custom function: {:?}", result);
//                     }
//                     _ => {
//                         println!("Expected a function, found something else.");
//                     }
//                 }
//
//             }
//
//         }
//         VariableType::CustomFunWVariables(custom_func_wrapper) => {
//             // Directly run the custom function if it is already wrapped in CustomFunWVariables
//             let result = custom_func_wrapper.custom_func.run(vec![x, y]);
//             println!("Result of custom function: {:?}", result);
//         }
//         _ => {
//             println!("Expected a function, found something else.");
//         }
//     }
// }

//LATER
//global variables
//why must I always use pass for nonbasic functions


fn main() {
    // Create a local HashMap to store instances of CustomFunc wrapped in Arc
    unsafe {
    let value2 = VariableType::Variable("y".to_string());
    let variables1 = vec![value2.clone(), value2.clone()];
    let my_instance1 = Arc::new(CustomFunc::new(
        BASIC_FUNCTIONS.get("sum").unwrap().clone(),
        variables1, // Pass variables1 without cloning
        vec!["y".to_string()],
    ));

    let result = my_instance1.run(vec![1.0, 2.0]);
    println!("{:?}", result);

    // Store my_instance1 in the map
    CUSTOM_FUNC_MAP.insert("my_instance1".to_string(), my_instance1.clone());

    let value1 = VariableType::BasicFunWVariables(Arc::new(BasicFunWVariables::new(BASIC_FUNCTIONS.get("get_series").unwrap().clone(), vec![VariableType::Value(2.0)])));
    let variables2 = vec![value1.clone()];
    let my_instance2 = Arc::new(CustomFunc::new(
        BASIC_FUNCTIONS.get("sum").unwrap().clone(),
        variables2, // Pass variables2 without cloning
        vec![],
    ));
    let result = my_instance2.run(vec![]);
    println!("{:?}", result);

    // Store my_instance2 in the map
    CUSTOM_FUNC_MAP.insert("my_instance2".to_string(), my_instance2.clone());

    // Retrieve instances from the map before creating new CustomFunc instances
    let instance1 = CUSTOM_FUNC_MAP.get("my_instance1").unwrap().clone();
    let instance2 = CUSTOM_FUNC_MAP.get("my_instance2").unwrap().clone();

    let value1 = VariableType::CustomFunWVariables(Arc::new(CustomFunWVariables::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
    let value2 = VariableType::CustomFunWVariables(Arc::new(CustomFunWVariables::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
    let variables = vec![value1.clone(), value2.clone()];
    let my_instance3 = Arc::new(CustomFunc::new(
        BASIC_FUNCTIONS.get("multiply").unwrap().clone(),
        variables,
        vec!["x".to_string()],
    ));
    let result = my_instance3.run(vec![1.0, 2.0]);
    println!("{:?}", result);

    // Store my_instance3 in the map
    CUSTOM_FUNC_MAP.insert("my_instance3".to_string(), my_instance3.clone());

    // Retrieve instance from the map before creating new CustomFunc instance
    let instance3 = CUSTOM_FUNC_MAP.get("my_instance3").unwrap().clone();

    let value1 = VariableType::CustomFunWVariables(Arc::new(CustomFunWVariables::new(instance3.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
    let value2 = VariableType::Value(3.0);
    let variables = vec![value1.clone(), value2.clone()];
    let my_instance4 = Arc::new(CustomFunc::new(
        BASIC_FUNCTIONS.get("divide").unwrap().clone(),
        variables,
        vec!["x".to_string()],
    ));
    let result = my_instance4.run(vec![1.0, 2.0]);
    println!("{:?}", result);

    // Store my_instance4 in the map
    CUSTOM_FUNC_MAP.insert("my_instance4".to_string(), my_instance4.clone());

    // Retrieve and run a function from the map
    if let Some(func) = CUSTOM_FUNC_MAP.get("my_instance4") {
        let result = func.run(vec![1.0, 2.0]);
        println!("Result of my_instance4: {:?}", result);
    } else {
        println!("Function not found");
    }
}
}
