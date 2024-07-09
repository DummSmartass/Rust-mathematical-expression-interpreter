use std::collections::HashMap;
use std::rc::Rc;
use once_cell::sync::Lazy;

mod basic_functions;
mod custom_functions;
mod variable_types;

use basic_functions::{get_basic_functions, BasicFunc};
use custom_functions::CustomFunc;
use variable_types::{VariableType, CustomFunWVariables};
use crate::variable_types::BasicFunWVariables;
use crate::custom_functions::custom_func_map;


fn main() {
    // Create a HashMap to store instances of CustomFunc wrapped in Rc
    //let mut custom_func_map: HashMap<String, Rc<CustomFunc>> = HashMap::new();
    unsafe {
        let mut local_custom_func_map = custom_func_map.clone();

        let value2 = VariableType::Variable("y");
        let variables1 = vec![&value2, &value2];
        let my_instance1 = Rc::new(CustomFunc::new(
            get_basic_functions().get("sum").unwrap().clone(),
            variables1, // Pass variables1 without cloning
            vec![],
            vec!["y"],
        ));

        let result = my_instance1.run(vec![1.0, 2.0]);
        println!("{:?}", result);

        // Store my_instance1 in the map
        local_custom_func_map.insert("my_instance1".to_string(), my_instance1.clone());

        let value1 = VariableType::BasicFunWVariables(BasicFunWVariables::new(get_basic_functions().get("get_series").unwrap().clone(), vec![VariableType::Value(5.0)]));
        let variables2 = vec![&value1];
        let my_instance2 = Rc::new(CustomFunc::new(
            get_basic_functions().get("sum").unwrap().clone(),
            variables2, // Pass variables2 without cloning
            vec![vec![1, 0]],
            vec![],
        ));
        let result = my_instance2.run(vec![]);
        println!("{:?}", result);

        // Store my_instance2 in the map
        local_custom_func_map.insert("my_instance2".to_string(), my_instance2.clone());

        // Retrieve instances from the map before creating new CustomFunc instances
        let instance1 = local_custom_func_map.get("my_instance1").unwrap().clone();
        let instance2 = local_custom_func_map.get("my_instance2").unwrap().clone();

        let value1 = VariableType::CustomFunWVariables(CustomFunWVariables::new(&instance1, vec![VariableType::Variable("x"), VariableType::Variable("y")]));
        let value2 = VariableType::CustomFunWVariables(CustomFunWVariables::new(&instance2, vec![VariableType::Variable("x"), VariableType::Variable("y")]));
        let variables = vec![&value1, &value2];
        let my_instance3 = Rc::new(CustomFunc::new(
            get_basic_functions().get("multiply").unwrap().clone(),
            variables,
            vec![],
            vec!["x"],
        ));
        let result = my_instance3.run(vec![1.0, 2.0]);
        println!("{:?}", result);

        // Store my_instance3 in the map
        local_custom_func_map.insert("my_instance3".to_string(), my_instance3.clone());

        // Retrieve instance from the map before creating new CustomFunc instance
        let instance3 = local_custom_func_map.get("my_instance3").unwrap().clone();

        let value1 = VariableType::CustomFunWVariables(CustomFunWVariables::new(&instance3, vec![VariableType::Variable("x"), VariableType::Variable("y")]));
        let value2 = VariableType::Value(3.0);
        let variables = vec![&value1, &value2];
        let my_instance4 = Rc::new(CustomFunc::new(
            get_basic_functions().get("divide").unwrap().clone(),
            variables,
            vec![],
            vec!["x"],
        ));
        let result = my_instance4.run(vec![1.0, 2.0]);
        println!("{:?}", result);

        // Store my_instance4 in the map
        local_custom_func_map.insert("my_instance4".to_string(), my_instance4.clone());

        // Retrieve and run a function from the map
        if let Some(func) = local_custom_func_map.get("my_instance4") {
            let result = func.run(vec![1.0, 2.0]);
            println!("Result of my_instance4: {:?}", result);
        } else {
            println!("Function not found");
        }
    }
}
