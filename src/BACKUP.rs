use std::collections::HashMap;
use std::io::Result;
use regex::Regex;
use once_cell::sync::Lazy;
use lazy_static::lazy_static;
use std::sync::RwLock;

type MathFunc = fn(&[String]) -> String;

fn a(args: &[String]) -> String {
    if (args.len() != 2) {
        panic!("Function 'a' expects exactly 2 arguments");
    }
    let result = args[0].parse::<f64>().unwrap() + args[1].parse::<f64>().unwrap();
    result.to_string()
}

fn b(args: &[String]) -> String {
    if (args.len() != 2) {
        panic!("Function 'b' expects exactly 2 arguments");
    }
    let result = args[0].parse::<f64>().unwrap() * args[1].parse::<f64>().unwrap();
    result.to_string()
}

fn c(args: &[String]) -> String {
    if (args.len() != 3) {
        panic!("Function 'c' expects exactly 3 arguments");
    }
    let result = args[0].parse::<f64>().unwrap() + args[1].parse::<f64>().unwrap() * args[2].parse::<f64>().unwrap();
    result.to_string()
}

fn e(_args: &[String]) -> String {
    vec!["1.0", "2.0", "3.0", "4.0", "5.0"].join(",")
}

fn max(args: &[String]) -> String {
    if args.is_empty() {
        panic!("Function 'max' expects at least one argument");
    }

    let mut max_value = args[0].parse::<f64>().unwrap();
    for arg in args.iter().skip(1) {
        let num = arg.parse::<f64>().unwrap();
        if num > max_value {
            max_value = num;
        }
    }

    max_value.to_string()
}


static FUNCTIONS: Lazy<HashMap<&str, MathFunc>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("sum", a as MathFunc);
    map.insert("b", b as MathFunc);
    map.insert("c", c as MathFunc);
    map.insert("e", e as MathFunc);
    map.insert("max", max as MathFunc);
    map
});

lazy_static! {
    static ref TEMP_FUNCTIONS: RwLock<HashMap<String, TempFun>> = RwLock::new(HashMap::new());
    static ref INTERPRET_REGEX: Regex = Regex::new(r"(\w+)\(([^()]*)\)(?:\[(.*)\])?").unwrap();
    static ref TEMP_FUNC_REGEXES: RwLock<HashMap<String, Regex>> = RwLock::new(HashMap::new());
}

pub struct TempFun {
    operations: String,
    variables: Vec<String>,
}

impl TempFun {
    pub fn new(operations: String, variables: Vec<String>) -> Self {
        TempFun { operations, variables }
    }
}

fn add_temp_fun(func: &str) {
    let mut func = func.to_string().replace(" ", "").replace("\t", "").replace("\n", "");
    let mut map = TEMP_FUNCTIONS.write().unwrap();

    let segments: Vec<&str> = func.split(|c| c == '=' || c == ';').collect();
    let name = segments.get(0).unwrap();
    let operations = segments.get(1).unwrap();
    let variable_string = segments.get(2).unwrap();

    let variables: Vec<String> = variable_string
        .trim_matches('(')
        .trim_matches(')')
        .split(',')
        .map(String::from)
        .collect();

    map.insert(name.to_string(), TempFun::new(operations.to_string(), variables));

    let mut regex_map = TEMP_FUNC_REGEXES.write().unwrap();
    let escaped_name = regex::escape(name);
    regex_map.insert(name.to_string(), Regex::new(&format!(r"{}[a-zA-Z0-9_]*(?:\((?:[^()]+|(?R))*\))", escaped_name)).unwrap());
}

fn interpret(input: &str) -> Result<String> {
    let mut input = input.to_string();

    while let Some(captures) = INTERPRET_REGEX.captures(&input) {
        while let Some(captures) = INTERPRET_REGEX.captures(&input) {
            let func_name = captures.get(1).unwrap().as_str();
            let args_str = captures.get(2).unwrap().as_str();
            let args: Vec<String> = if args_str.is_empty() {
                vec![]
            } else {
                args_str
                    .split(',')
                    .map(|s| interpret(s).unwrap_or_else(|_| s.to_string()))
                    .collect()
            };
            let result = match FUNCTIONS.get(func_name) {
                Some(func) => func(&args),
                None => break,
            };

            let selection = captures.get(3).map_or(result.clone(), |s| {
                s.as_str()
                    .split(',')
                    .map(|i| {
                        let index = i.parse::<usize>().unwrap();
                        let selected_element = result.split(',').collect::<Vec<&str>>()[index].to_string();
                        selected_element
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            });

            input = input.replacen(&captures[0], &selection, 1);
        }
        input = replace_temp_func_calls(input.as_str());
    }

    Ok(input)
}

fn replace_temp_func_calls(input: &str) -> String {
    let map = TEMP_FUNCTIONS.read().unwrap();
    let regex_map = TEMP_FUNC_REGEXES.read().unwrap();
    let mut output = input.to_string();

    let mut previous_output = String::new();
    while output != previous_output {
        previous_output = output.clone();
        for (name, re) in regex_map.iter() {
            output = re.replace_all(&output, |caps: &regex::Captures| {
                let cap = caps.get(0).unwrap().as_str();
                let temp_fun = map.get(name).unwrap();
                let args: Vec<&str> = cap[name.len() + 1..cap.len() - 1].split(',').collect();
                let mut operation = temp_fun.operations.clone();

                for (var, arg) in temp_fun.variables.iter().zip(args.iter()) {
                    let var_pattern = Regex::new(&format!(r"\b{}\b", regex::escape(var))).unwrap();
                    operation = var_pattern.replace_all(&operation, *arg).to_string();
                }
                operation
            }).to_string();
        }
    }

    output
}

fn main() -> Result<()> {
    add_temp_fun("examplexyz=max(z,y,z);(x,y,z)");
    add_temp_fun("exampleaaa2=examplexyz(z,y,z);(x,y,z)");
    add_temp_fun("noting=0;()");

    add_temp_fun("k=5;()");
    add_temp_fun("kk=3;()");

    let result = interpret("exampleaaa2(1,2,43)")?;

    println!("a{}", result);

    add_temp_fun("max_of_three=sum(x,y);(x,y,z)");


    let result1 = interpret("sum(max_of_three(1,2),1)")?;

    println!("b{}", result1);


    println!("d{}", result);




    add_temp_fun("n0=sum(x,y);(x,y)");
    add_temp_fun("n1=sum(n0(x,y),n0(x,y));(x,y)");
    add_temp_fun("n2=sum(n1(x,y),n1(x,y));(x,y)");
    add_temp_fun("n3=sum(n2(x,y),n2(x,y));(x,y)");
    add_temp_fun("n4=sum(n3(x,y),n3(x,y));(x,y)");
    add_temp_fun("n5=sum(n4(x,y),n4(x,y));(x,y)");
    add_temp_fun("n6=sum(n5(x,y),n5(x,y));(x,y)");
    add_temp_fun("n7=sum(n6(x,y),n6(x,y));(x,y)");
    add_temp_fun("n8=sum(n7(x,y),n7(x,y));(x,y)");
    add_temp_fun("n9=sum(n8(x,y),n8(x,y));(x,y)");
    add_temp_fun("n10=sum(n9(x,y),n9(x,y));(x,y)");
    add_temp_fun("n11=sum(n10(x,y),n10(x,y));(x,y)");
    add_temp_fun("n12=sum(n11(x,y),n11(x,y));(x,y)");
    add_temp_fun("n13=sum(n12(x,y),n12(x,y));(x,y)");
    add_temp_fun("n14=sum(n13(x,y),n13(x,y));(x,y)");
    add_temp_fun("n15=sum(n14(x,y),n14(x,y));(x,y)");
    add_temp_fun("n16=sum(n15(x,y),n15(x,y));(x,y)");
    add_temp_fun("n17=sum(n16(x,y),n16(x,y));(x,y)");
    add_temp_fun("n18=sum(n17(x,y),n17(x,y));(x,y)");
    add_temp_fun("n19=sum(n18(x,y),n18(x,y));(x,y)");
    add_temp_fun("n20=sum(n19(x,y),n19(x,y));(x,y)");


    //    let mut input = input.to_string().replace(" ", "").replace("\t", "").replace("\n", "");

    use std::time::Instant;

    let start = Instant::now(); // Start time measurement right before the code block

    let result = interpret("n1(1,1)")?;

    let duration = start.elapsed(); // End time measurement right after the code block

    println!("Time taken: {:?}", duration); // Print the time taken


    println!("{}", result);


    // add_temp_fun("temp1=sum(x,y);(x,y)");
    // add_temp_fun("temp2=sum(temp1(x,y),z);(x,y,z)");
    // add_temp_fun("temp3=sum(temp2(x,y,z),w);(x,y,z,w)");
    // add_temp_fun("temp4=sum(temp3(x,y,z,w),v);(x,y,z,w,v)");
    //
    // let result = interpret("temp4(1,2,3,4,5)")?;
    // println!("{}", result); // Expected output: 15


    add_temp_fun("temp1=sum(x,y);(x,y)");
    add_temp_fun("temp2=sum(temp1(x,y),z);(x,y,z)");
    add_temp_fun("temp3=sum(temp2(temp1(x,y),z,z),w);(x,y,z,w)");
    add_temp_fun("temp4=sum(temp3(temp2(temp1(x,y),z,z,v),w,v,v),v);(x,y,z,w,v)");

    let result = interpret("temp4(e())")?;
    println!("{}", result);

    let result = interpret("e()")?;
    println!("{}", result);

    let result = interpret("sum(1,2)")?;
    println!("{}", result);

    let result = interpret("sum(e()[0,0])")?;
    println!("{}", result);

    let result = interpret("max_of_three(1,2,3)")?;
    println!("{}", result);


    add_temp_fun("examplexyz=max(z,y,z);(x,y,z)");
    add_temp_fun("exampleaaa2=examplexyz(z,y,z);(x,y,z)");
    add_temp_fun("noting=0;()");

    add_temp_fun("k=5;()");
    add_temp_fun("kk=3;()");

    let result = interpret("exampleaaa2(1,2,43)")?;

    println!("a{}", result);

    add_temp_fun("max_of_three=sum(x,y);(x,y,z)");


    let result1 = interpret("sum(max_of_three(1,2),1)")?;

    println!("b{}", result1);


    println!("d{}", result);




    add_temp_fun("n0=sum(x,y);(x,y)");
    add_temp_fun("n1=sum(n0(x,y),n0(x,y));(x,y)");
    add_temp_fun("n2=sum(n1(x,y),n1(x,y));(x,y)");
    add_temp_fun("n3=sum(n2(x,y),n2(x,y));(x,y)");
    add_temp_fun("n4=sum(n3(x,y),n3(x,y));(x,y)");
    add_temp_fun("n5=sum(n4(x,y),n4(x,y));(x,y)");
    add_temp_fun("n6=sum(n5(x,y),n5(x,y));(x,y)");
    add_temp_fun("n7=sum(n6(x,y),n6(x,y));(x,y)");
    add_temp_fun("n8=sum(n7(x,y),n7(x,y));(x,y)");
    add_temp_fun("n9=sum(n8(x,y),n8(x,y));(x,y)");
    add_temp_fun("n10=sum(n9(x,y),n9(x,y));(x,y)");
    add_temp_fun("n11=sum(n10(x,y),n10(x,y));(x,y)");
    add_temp_fun("n12=sum(n11(x,y),n11(x,y));(x,y)");
    add_temp_fun("n13=sum(n12(x,y),n12(x,y));(x,y)");
    add_temp_fun("n14=sum(n13(x,y),n13(x,y));(x,y)");
    add_temp_fun("n15=sum(n14(x,y),n14(x,y));(x,y)");
    add_temp_fun("n16=sum(n15(x,y),n15(x,y));(x,y)");
    add_temp_fun("n17=sum(n16(x,y),n16(x,y));(x,y)");
    add_temp_fun("n18=sum(n17(x,y),n17(x,y));(x,y)");
    add_temp_fun("n19=sum(n18(x,y),n18(x,y));(x,y)");
    add_temp_fun("n20=sum(n19(x,y),n19(x,y));(x,y)");


    //    let mut input = input.to_string().replace(" ", "").replace("\t", "").replace("\n", "");


    let start = Instant::now(); // Start time measurement right before the code block

    let result = interpret("n10(1,1)")?;

    let duration = start.elapsed(); // End time measurement right after the code block

    println!("Time taken: {:?}", duration); // Print the time taken


    println!("{}", result);


    // add_temp_fun("temp1=sum(x,y);(x,y)");
    // add_temp_fun("temp2=sum(temp1(x,y),z);(x,y,z)");
    // add_temp_fun("temp3=sum(temp2(x,y,z),w);(x,y,z,w)");
    // add_temp_fun("temp4=sum(temp3(x,y,z,w),v);(x,y,z,w,v)");
    //
    // let result = interpret("temp4(1,2,3,4,5)")?;
    // println!("{}", result); // Expected output: 15


    add_temp_fun("temp1=sum(x,y);(x,y)");
    add_temp_fun("temp2=sum(temp1(x,y),z);(x,y,z)");
    add_temp_fun("temp3=sum(temp2(temp1(x,y),z,z),w);(x,y,z,w)");
    add_temp_fun("temp4=sum(temp3(temp2(temp1(x,y),z,z,v),w,v,v),v);(x,y,z,w,v)");

    let result = interpret("temp4(e())")?;
    println!("{}", result);

    let result = interpret("e()")?;
    println!("{}", result);

    let result = interpret("sum(1,2)")?;
    println!("{}", result);

    let result = interpret("sum(e()[0,0])")?;
    println!("{}", result);

    let result = interpret("max_of_three(1,2,3)")?;
    println!("{}", result);



    Ok(())
}


// get all regexes static
// optimize replace_temp_func_calls
// make variables
//
//