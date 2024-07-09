mod basic_functions;
mod custom_functions;
mod variable_types;

use basic_functions::{get_basic_functions, BasicFunc};
use custom_functions::CustomFunc;
use variable_types::{VariableType, CustomFunWVariables};

use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let value1 = VariableType::Variable("x");
    let value2 = VariableType::Variable("y");
    let variables1 = vec![&value1, &value2];
    let custom_func1 = CustomFunc::new(
        get_basic_functions().get("sum").unwrap().clone(),
        variables1,
        vec![],
        vec!["x", "y"],
    );

    let result = custom_func1.run(vec![1.0, 1.0]);
    println!("{:?}", result);

    let nested_value = VariableType::CustomFunWVariables(CustomFunWVariables::new(
        &custom_func1,
        vec![VariableType::Variable("x"), VariableType::Variable("y")],
    ));
    let nested_variables = vec![&nested_value, &nested_value];
    let nested_custom_func = CustomFunc::new(
        get_basic_functions().get("sum").unwrap().clone(),
        nested_variables,
        vec![],
        vec!["x", "y"],
    );

    let start = Instant::now();
    let result = nested_custom_func.run(vec![1.0, 1.0]);
    println!("{:?}", result);

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}
