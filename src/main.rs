mod basic_functions;
mod custom_functions;
mod variable_types;
mod GlobalVariables;

use basic_functions::BasicFunc;
use custom_functions::CustomFunc;
use variable_types::{VariableType, CustomFuncWithVars, BasicFuncWithVars};
use std::sync::Arc;
use std::time::Instant;
use crate::basic_functions::BASIC_FUNCTIONS;
use crate::custom_functions::{CUSTOM_FUNC_MAP, interpret};
use crate::GlobalVariables::{create_global_variable, create_global_variable_text, get_by_name};

fn main() {
    unsafe {
        interpret("a=sum(sum(1,sum(x,y)),multiply(x,y));x,y");
        // interpret("sum(1,1)");
         println!("RESULT: {:?}", CUSTOM_FUNC_MAP.get("a").unwrap().run(vec![1.0, 2.0]));

        let custom_func = CUSTOM_FUNC_MAP.get("a").unwrap().clone();

        // Create global variables
        //create_global_variable("global_var1".to_string(), GlobalVariable::new((**func_arc).clone(), vec![1.0, 2.0]);
        let declaration = "global_var2 = a(3.0, 4.0)".to_string();
        unsafe {
            create_global_variable_text(declaration);
        }

        // Retrieve and print the values of the global variables
        //println!("GLOBAL VAR1: {:?}", get_by_name("global_var1".to_string()));
        println!("GLOBAL VAR2: {:?}", get_by_name("global_var2".to_string()));

        interpret("b=sum(global_var2,x);x");
        println!("RESULT: {:?}", CUSTOM_FUNC_MAP.get("b").unwrap().run(vec![1.0]));


    }
}



//LATER
//global variables having as a base list of values , function and variables
//why must I always use pass for nonbasic functions
//external compiler of +,-,*,/,**,//,++,--
//Extrernal compiler of recursion, by string replacment in pre compilng
//cash
//precompilation into files
//LateX document prep

// // CORECTNESS TEST
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
//     let value1 = VariableType::BasicFuncWithVars(Arc::new(BasicFuncWithVars::new(BASIC_FUNCTIONS.get("get_series").unwrap().clone(), vec![VariableType::Value(2.0)])));
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
//     let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//     let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
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
//     let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance3.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
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

// //speed test
// fn main() {
//     unsafe {
//         let value1 = VariableType::Variable("x".to_string());
//         let value2 = VariableType::Variable("y".to_string());
//
//         let variables1 = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables1, // Pass variables1 without cloning
//             vec!["x".to_string(),"y".to_string()],
//         ));
//
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         println!("{:?}", result);
//
//
//
//         let instance1 = my_instance1.clone();
//         let instance2 = my_instance1.clone();
//
//         let value1 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance1.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let value2 = VariableType::CustomFuncWithVars(Arc::new(CustomFuncWithVars::new(instance2.clone(), vec![VariableType::Variable("x".to_string()), VariableType::Variable("y".to_string())])));
//         let variables = vec![value1.clone(), value2.clone()];
//         let my_instance1 = Arc::new(CustomFunc::new(
//             BASIC_FUNCTIONS.get("sum").unwrap().clone(),
//             variables,
//             vec!["x".to_string(),"y".to_string()],
//         ));
//
//
//
//
//         let now = Instant::now();
//         let result = my_instance1.run(vec![1.0, 1.0]);
//         let elapsed = now.elapsed();
//         println!("Time taken by function: {:.2?}", elapsed);
//         println!("{:?}", result);
//     }
//}

