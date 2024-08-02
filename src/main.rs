mod basic_functions;
mod custom_functions;
mod variable_types;
mod global_variables;
mod server;
mod client;

use basic_functions::BasicFunc;
use custom_functions::CustomFunc;
use variable_types::{VariableType, CustomFuncWithVars, BasicFuncWithVars};
use std::sync::Arc;
use std::time::Instant;
use crate::basic_functions::BASIC_FUNCTIONS;
use crate::custom_functions::{CUSTOM_FUNC_MAP, interpret, SAVE_FILE_NAME, set_save_file_name, run_custom_logic, load_remembered};
use crate::global_variables::{create_global_variable, create_global_variable_text, get_variable_by_name};

use std::fs::File;
use std::sync::Mutex;


// main.rs

use tokio::task;
use std::time::Duration;
use tokio::time;


use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io::{self, BufRead, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start the server in a separate task
    let server_handle = task::spawn(async {
        server::run_server().await.unwrap();
    });

    // Give the server a moment to start
    time::sleep(Duration::from_millis(300)).await;

    // Run the client with input from stdin
    println!("Running client:");
    run_client().await?;

    // Stop the server
    server_handle.abort();

    Ok(())
}

pub async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Read input from stdin
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);

    for line in reader.lines() {
        let text = line?;

        // Send the text
        stream.write_all(text.as_bytes()).await?;
        stream.write_all(b"\n").await?; // Use newline as delimiter

        // Read the reversed response
        let mut response = vec![0; 1024];
        let n = stream.read(&mut response).await?;
        let response_text = String::from_utf8_lossy(&response[..n]).trim_end().to_string();

        println!("Received reversed text: {}", response_text);
    }

    Ok(())
}



// fn main() {
//     unsafe {
//         load_remembered();
//         run_custom_logic();
//     }
// }




//LATER
// make global variables multi level?
//why must I always use pass for nonbasic functions
//external compiler of +,-,*,/,**,//,++,--
//Extrernal compiler of recursion, by string replacment in pre compilng
//precompilation into files



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

