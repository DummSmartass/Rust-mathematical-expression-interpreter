use std::collections::HashMap;
use once_cell::sync::Lazy;

pub type BasicFunc = fn(&[f64]) -> Vec<f64>;

fn sum(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Function 'sum' expects exactly 2 arguments");
    }
    vec![args[0] + args[1]]
}

fn difference(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Function 'difference' expects exactly 2 arguments");
    }
    vec![args[0] - args[1]]
}

fn multiply(args: &[f64]) -> Vec<f64> {
    println!("MULARGS:{:?}",args);
    if args.len() != 2 {
        panic!("Function 'multiply' expects exactly 2 arguments");
    }
    vec![args[0] * args[1]]
}

fn divide(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Function 'divide' expects exactly 2 arguments");
    }
    if args[1] == 0.0 {
        panic!("Division by zero is not allowed");
    }
    vec![args[0] / args[1]]
}

fn pass(args: &[f64]) -> Vec<f64> {
    args.to_vec()
}


fn get_series(args: &[f64]) -> Vec<f64> {
    if args.len() != 1 {
        panic!("Function 'get_series' expects exactly 1 argument");
    }
    let mut result = Vec::new();
    for i in 1..=args[0] as i32 {
        result.push(i as f64);
    }
    result
}

pub static BASIC_FUNCTIONS: Lazy<HashMap<&'static str, BasicFunc>> = Lazy::new(|| {
    let mut basic_functions = HashMap::new();
    basic_functions.insert("sum", sum as BasicFunc);
    basic_functions.insert("difference", difference as BasicFunc);
    basic_functions.insert("multiply", multiply as BasicFunc);
    basic_functions.insert("divide", divide as BasicFunc);
    basic_functions.insert("get_series", get_series as BasicFunc);
    basic_functions.insert("pass", pass as BasicFunc);
    basic_functions
});

// pub fn get_basic_functions() -> &'static HashMap<&'static str, BasicFunc> {
//     &BASIC_FUNCTIONS
// }
