use std::string::String;
use std::collections::HashMap;
use std::str::FromStr;
use std::io;
use std::io::Result;
use regex::Regex;
use once_cell::sync::Lazy;
use lazy_static::lazy_static;
use std::sync::Mutex;

type MathFunc = fn(&[f64]) -> Vec<f64>;

fn a(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Function 'a' expects exactly 2 arguments");
    }
    let result = args[0] + args[1];
    vec![result]
}

fn b(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Function 'b' expects exactly 2 arguments");
    }
    let result = args[0] * args[1];
    vec![result]
}

fn c(args: &[f64]) -> Vec<f64> {
    if args.len() != 3 {
        panic!("Function 'c' expects exactly 3 arguments");
    }
    let result = args[0] + args[1] * args[2];
    vec![result]
}

fn e(_args: &[f64]) -> Vec<f64> {
    vec![1.0, 2.0, 3.0, 4.0, 5.0]
}

static FUNCTIONS: Lazy<HashMap<&str, MathFunc>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("a", a as MathFunc);
    map.insert("b", b as MathFunc);
    map.insert("c", c as MathFunc);
    map.insert("e", e as MathFunc);
    map
});

lazy_static! {
    static ref TEMP_FUNCTIONS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn interpret(input: &str) -> Result<Vec<f64>> {
    let re = Regex::new(r"(\w+)\(([^()]*)\)(?:\[(.*)\])?").unwrap();
    let mut input = input.to_string();

    while let Some(captures) = re.captures(&input) {
        let func_name = captures.get(1).unwrap().as_str();
        println!("1>{}", func_name);
        let args_str = captures.get(2).unwrap().as_str();
        println!("2>{}", args_str);
        let args: Vec<f64> = if args_str.is_empty() {
            vec![]
        } else {
            args_str
                .split(',')
                .map(|s| {
                    interpret(s)
                        .unwrap_or_else(|_| vec![f64::from_str(s).unwrap()])
                        .into_iter()
                        .next()
                        .unwrap()
                })
                .collect()
        };
        let result = match FUNCTIONS.get(func_name) {
            Some(func) => func(&args),
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid function name")),
        };

        let result_ref = &result;
        let selection = captures.get(3).map_or(result_ref.clone(), |s| {
            s.as_str()
                .split(',')
                .map(|i| {
                    let index = i.parse::<usize>().unwrap();
                    let selected_element = result_ref[index];
                    selected_element
                })
                .collect()
        });


        println!("4>{{ {} {}", selection.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "), "}}");
        let replace_str = selection
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",");
        input = input.replacen(&captures[0], &replace_str, 1);
    }

    Ok(input
        .split(',')
        .map(|s| f64::from_str(s).unwrap())
        .collect())
}

fn main() -> Result<()> {

    let mut map = TEMP_FUNCTIONS.lock().unwrap();
    map.insert(String::from("key"), String::from("value"));


    let result = interpret("a(1,a(a(a(1,1),a(4,9)),3))")?;
    for value in result {
       println!("{}", value);
    }

    Ok(())
}



//
// this kind of string to make a temporary function "doublex+y+1=a(a(x,x),a(y,1))(x,y)"   function string initializing
// syntax: "name=functionName(otherFunctionNamesAndSoOn(Variables Provided))(ALL VARIABLES USED IN)"
// you can use names of other temporary functions in temporay function function
// Than you can just use interpret("doublex+y+1(6,7)")
// It would work on string operation, so it would just detect temporary funcction by string operation taking this
// doublex+y+1=a(a(x,x),a(y,1)) from this (x,y)
// asociating this (x,y) to this (6,7)
// and suplementing in the interpreter this: interpret("doublex+y+1(6,7)") to this interpret("a(a(6,6),a(7,1))")(IT SHOULD BE DONE IN THE BEGINING OF INTERPRETER BASE ON SOME STATIC MAP OF TEMPORARY FUNCTIONS)
// KEEP IN MIND TO PUT IN PLACEPROCUSIONS TO NOT CHANGE PARTS THAT WHOULDNT BE CHANGED like if there was function max for two variables and somonde made 4max=max(max(x,y),max(z,k))(x,y,z,k) dont let 4max(1,2,3,4) turn into  ma1(ma1(1,2),ma1(3,4))
//
//
//
