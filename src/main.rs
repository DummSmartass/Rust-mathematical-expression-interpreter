use std::collections::HashMap;
use std::io::Result;
use regex::Regex;
use once_cell::sync::Lazy;
use lazy_static::lazy_static;
use std::sync::RwLock;

type MathFunc = fn(&[String]) -> Vec<String>;

fn a(args: &[String]) -> Vec<String> {
    if args.len() != 2 {
        panic!("Function 'a' expects exactly 2 arguments");
    }
    let result = args[0].parse::<f64>().unwrap() + args[1].parse::<f64>().unwrap();
    vec![result.to_string()]
}

fn b(args: &[String]) -> Vec<String> {
    if args.len() != 2 {
        panic!("Function 'b' expects exactly 2 arguments");
    }
    let result = args[0].parse::<f64>().unwrap() * args[1].parse::<f64>().unwrap();
    vec![result.to_string()]
}

fn c(args: &[String]) -> Vec<String> {
    if args.len() != 3 {
        panic!("Function 'c' expects exactly 3 arguments");
    }
    let result = args[0].parse::<f64>().unwrap() + args[1].parse::<f64>().unwrap() * args[2].parse::<f64>().unwrap();
    vec![result.to_string()]
}

fn e(_args: &[String]) -> Vec<String> {
    vec!["1.0".to_string(), "2.0".to_string(), "3.0".to_string(), "4.0".to_string(), "5.0".to_string()]
}

fn max(args: &[String]) -> Vec<String> {
    let floats: Vec<f64> = args.iter().map(|x| x.parse::<f64>().unwrap()).collect();
    let max_value = floats.iter().copied().reduce(f64::max);
    match max_value {
        Some(max_value) => floats.iter()
            .filter(|&&x| x.is_nan() || x == max_value)
            .map(|&x| x.to_string())
            .collect(),
        None => Vec::new(),
    }
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
}


pub struct TempFun {
    operations: String,
    variables: Vec<String>,
}

impl TempFun {
    pub fn new(operations: String, variables: Vec<String>) -> Self {
        TempFun {
            operations,
            variables,
        }
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
}

fn interpret(input: &str) -> Result<String> {
    let re = Regex::new(r"(\w+)\(([^()]*)\)(?:\[(.*)\])?").unwrap();
    let mut input = input.to_string();//.replace(" ", "").replace("\t", "").replace("\n", "");

    while let Some(captures) = re.captures(&input) {
        while let Some(captures) = re.captures(&input) {
            let func_name = captures.get(1).unwrap().as_str();
            let args_str = captures.get(2).unwrap().as_str();
            let args: Vec<String> = if args_str.is_empty() {
                vec![]
            } else {
                args_str
                    .split(',')
                    .map(|s| {
                        interpret(s)
                            .unwrap_or_else(|_| s.to_string())
                    })
                    .collect()
            };
            let result = match FUNCTIONS.get(func_name) {
                Some(func) => func(&args),
                None => {
                    break;
                }
            };

            let result_ref = &result;
            let selection = captures.get(3).map_or(result_ref.clone(), |s| {
                s.as_str()
                    .split(',')
                    .map(|i| {
                        let index = i.parse::<usize>().unwrap();
                        let selected_element = result_ref[index].clone();
                        selected_element
                    })
                    .collect()
            });

            let replace_str = selection
                .into_iter()
                .collect::<Vec<String>>()
                .join(",");
            input = input.replacen(&captures[0], &replace_str, 1);
        }
        input = replace_temp_func_calls(input.as_str());
    }

    Ok(input)
}

//optimize
fn replace_temp_func_calls(input: &str) -> String {
    let map = TEMP_FUNCTIONS.read().unwrap();
    let mut output = input.to_string();

    let mut pattern_strings: Vec<String> = Vec::new();
    for (name, _) in map.iter() {
        let escaped_name = regex::escape(name);
        pattern_strings.push(format!(r"{}[a-zA-Z0-9_]*(?:\((?:[^()]+|(?R))*\))", escaped_name));
    }

    let combined_pattern = pattern_strings.join("|");
    let re = Regex::new(&combined_pattern).unwrap();

    let mut previous_output = String::new();
    while output != previous_output {
        previous_output = output.clone();
        output = re.replace_all(&output, |caps: &regex::Captures| {
            let cap = caps.get(0).unwrap().as_str();
            let name = cap.split('(').next().unwrap();
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

    output
}

fn main() -> Result<()> {
    add_temp_fun("examplexyz=max(e(),a(x,y),a(1,6),a(y,z),a(z,x));(x,y,z)");
    add_temp_fun("examplexyz=max(z,y,z);(x,y,z)");

    add_temp_fun("exampleaaa2=examplexyz(z,y,z);(x,y,z)");
    add_temp_fun("noting=0;()");

    add_temp_fun("k=5;()");
    add_temp_fun("kk=3;()");

    let result = interpret("exampleaaa2(1,2,43)")?;

    add_temp_fun("max_of_three=max(x,y,z);(x,y,z)");
    add_temp_fun("double=2*x;(x)");
    add_temp_fun("triple=3*x;(x)");
    add_temp_fun("average=(x+y+z)/3;(x,y,z)");

    let result1 = interpret("max_of_three(1,2,3)");
    let result2 = interpret("double(5)");
    let result3 = interpret("triple(7)");
    let result4 = interpret("average(1,2,3)");


    //println!("{}", result);

    add_temp_fun("n1=sum(x,y);(x,y)");
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
    add_temp_fun("n21=sum(n20(x,y),n20(x,y));(x,y)");
    add_temp_fun("n22=sum(n21(x,y),n21(x,y));(x,y)");
    add_temp_fun("n23=sum(n22(x,y),n22(x,y));(x,y)");
    add_temp_fun("n24=sum(n23(x,y),n23(x,y));(x,y)");
    add_temp_fun("n25=sum(n24(x,y),n24(x,y));(x,y)");
    add_temp_fun("n26=sum(n25(x,y),n25(x,y));(x,y)");
    add_temp_fun("n27=sum(n26(x,y),n26(x,y));(x,y)");
    add_temp_fun("n28=sum(n27(x,y),n27(x,y));(x,y)");
    add_temp_fun("n29=sum(n28(x,y),n28(x,y));(x,y)");
    add_temp_fun("n30=sum(n29(x,y),n29(x,y));(x,y)");
    add_temp_fun("n31=sum(n30(x,y),n30(x,y));(x,y)");
    add_temp_fun("n32=sum(n31(x,y),n31(x,y));(x,y)");
    add_temp_fun("n33=sum(n32(x,y),n32(x,y));(x,y)");
    add_temp_fun("n34=sum(n33(x,y),n33(x,y));(x,y)");
    add_temp_fun("n35=sum(n34(x,y),n34(x,y));(x,y)");
    add_temp_fun("n36=sum(n35(x,y),n35(x,y));(x,y)");
    add_temp_fun("n37=sum(n36(x,y),n36(x,y));(x,y)");
    add_temp_fun("n38=sum(n37(x,y),n37(x,y));(x,y)");
    add_temp_fun("n39=sum(n38(x,y),n38(x,y));(x,y)");
    add_temp_fun("n40=sum(n39(x,y),n39(x,y));(x,y)");
    add_temp_fun("n41=sum(n40(x,y),n40(x,y));(x,y)");
    add_temp_fun("n42=sum(n41(x,y),n41(x,y));(x,y)");
    add_temp_fun("n43=sum(n42(x,y),n42(x,y));(x,y)");
    add_temp_fun("n44=sum(n43(x,y),n43(x,y));(x,y)");
    add_temp_fun("n45=sum(n44(x,y),n44(x,y));(x,y)");
    add_temp_fun("n46=sum(n45(x,y),n45(x,y));(x,y)");
    add_temp_fun("n47=sum(n46(x,y),n46(x,y));(x,y)");
    add_temp_fun("n48=sum(n47(x,y),n47(x,y));(x,y)");
    add_temp_fun("n49=sum(n48(x,y),n48(x,y));(x,y)");
    add_temp_fun("n50=sum(n49(x,y),n49(x,y));(x,y)");

    //    let mut input = input.to_string().replace(" ", "").replace("\t", "").replace("\n", "");

    use std::time::Instant;

    let start = Instant::now(); // Start time measurement right before the code block

    //let result = interpret("n6(1,1)")?;

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
    println!("{}", result); // Expected output: 15

    Ok(())
}


// get all regexes static
// make variables
//
//