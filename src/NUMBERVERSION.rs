
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
    map.insert("sum", a as MathFunc);
    map.insert("b", b as MathFunc);
    map.insert("c", c as MathFunc);
    map.insert("e", e as MathFunc);
    map.insert("max", max as MathFunc);
    map
});

lazy_static! {
    static ref TEMP_FUNCTIONS: Mutex<HashMap<String, TempFun>> = Mutex::new(HashMap::new());
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


fn add_temp_fun(func: &str)
{
    let mut map = TEMP_FUNCTIONS.lock().unwrap();

    //doublex+y+1=a(a(x,x),a(y,1));(x,y)

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

    //println!("Name: {}", name);
    //println!("Operations: {}", operations);
    //println!("Variable String: {}", variable_string);
    //println!("Variables: {:?}", variables);

    map.insert(name.to_string(), TempFun::new(operations.to_string(), variables));

}


fn max(args: &[f64]) -> Vec<f64> {
    let max_value = args.iter().copied().reduce(f64::max);
    match max_value {
        Some(max_value) => args.iter()
            .copied()
            .filter(|&x| x.is_nan() || x == max_value)
            .collect(),
        None => Vec::new(),
    }
}



fn interpret(input: &str) -> Result<String> {
    let re = Regex::new(r"(\w+)\(([^()]*)\)(?:\[(.*)\])?").unwrap();
    let mut input = input.to_string().replace(" ", "").replace("\t", "").replace("\n", "");

    while let Some(captures) = re.captures(&input)
    {
        while let Some(captures) = re.captures(&input) {
            let func_name = captures.get(1).unwrap().as_str();
            let args_str = captures.get(2).unwrap().as_str();
            let args: Vec<f64> = if args_str.is_empty() {
                vec![]
            } else {
                args_str
                    .split(',')
                    .map(|s| {
                        //println!("Before mapping: >{}<", s);
                        interpret(s)
                            .unwrap_or_else(|_| s.to_string())
                            .parse::<f64>()
                            .unwrap()
                    })
                    .collect()
            };
            let result = match FUNCTIONS.get(func_name) {
                Some(func) => func(&args),
                None => {
                    // Return the input string as it is if the function does not exist
                    //return Ok(input);
                    //println!("HERE IDOT");
                    break;
                }
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

            let replace_str = selection
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",");
            input = input.replacen(&captures[0], &replace_str, 1);
            //println!(">1>>>>>{}", input);
        }
        //println!(">2>>>>>>{}", input);
        input = replace_temp_func_calls(input.as_str());
        //println!(">3>>>>>>{}", input);

    }

    Ok(input)
}



fn replace_temp_func_calls(input: &str) -> String {
    let map = TEMP_FUNCTIONS.lock().unwrap();
    let mut output = input.to_string();
    //println!("{}",output);


    let mut pattern_strings: Vec<String> = Vec::new();
    for (name, _) in map.iter() {
        let escaped_name = regex::escape(name);
        pattern_strings.push(format!(r"{}[a-zA-Z0-9_]*(?:\((?:[^()]+|(?R))*\))", escaped_name));
    }

    //println!("{:?}",pattern_strings);

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

    add_temp_fun("examplexyz=max(e(),sum(x,y),sum(1,6),sum(y,z),sum(z,x));(x,y,z)");
    add_temp_fun("examplexyz=max(z,y,z);(x,y,z)");

    add_temp_fun("exampleaaa2=examplexyz(z,y,z);(x,y,z)");
    add_temp_fun("noting=0;()");


    //let translated = replace_temp_func_calls("a(exampleaaa2(1,2,3),exampleaaa2(1,2,3))");
    //let translated = replace_temp_func_calls("examplexyz(1,2,a(1,2)))");
    //let translated = replace_temp_func_calls("examplexyz(3,4,1)");


    //println!("{}", translated);

    //let result = interpret("examplexyz(exampleaaa2(1,2,a(1,2)),2,a(1,2)),e(),examplexyz(1,2,a(1,2))")?;
    // let result = interpret("noting()")?;
    // //let result = interpret("a(a(1,a(3,5)),a(3,4))")?;
    //
    //
    // addTempFun("deepNest=exampleaaa2(exampleaaa2(exampleaaa2(x,y,z),y,z),y,z);(x,y,z)");
    // let result = interpret("deepNest(1,2,3)")?;
    //
    // addTempFun("nested1=max(x, y, z);(x,y,z)");
    // addTempFun("nested2=nested1(x, y, nested1(y, z, x));(x,y,z)");
    // let result = interpret("nested2(1, nested2(2,3,4), nested1(5,6,7))")?;



    // addTempFun("nested3=examplexyz(x, exampleaaa2(y, z, x), z);(x,y,z)");
    // let result = interpret("nested3(1, nested3(2,3,4), nested3(5,6,7))")?;
    // addTempFun("func1=max(x, y, z);(x,y,z)");
    // addTempFun("func2=examplexyz(func1(x, y, z), y, exampleaaa2(z, y, x));(x,y,z)");
    // addTempFun("func3=a(x, y);(x,y)");
    // addTempFun("func4=func3(func1(x, y, z), examplexyz(y, z, x));(x,y,z)");
    // addTempFun("func5=func4(exampleaaa2(x, y, z), func3(y, z), x);(x,y,z)");
    // addTempFun("func6=examplexyz(a(x, y), max(y, z, x), func1(x, y, z));(x,y,z)");
    // addTempFun("func7=func6(func3(x, y), exampleaaa2(y, z, x), max(z, y, x));(x,y,z)");
    // addTempFun("func8=exampleaaa2(func1(x, y, z), func3(y, z), examplexyz(z, x, y));(x,y,z)");
    // addTempFun("func9=func8(func4(x, y, z), examplexyz(y, z, x), func5(z, x, y));(x,y,z)");
    // addTempFun("func10=func9(func7(x, y, z), func6(y, z, x), func5(z, x, y));(x,y,z)");
    // let result = interpret("func10(func8(1,2,3), func9(4,5,6), func10(7,8,9))")?;


    // addTempFun("func1=max(x, y, z);(x,y,z)");
    // addTempFun("func2=examplexyz(func1(x, y, z), y, exampleaaa2(z, y, x));(x,y,z)");
    // addTempFun("func3=a(x, y);(x,y)");
    // addTempFun("func4=func3(func1(x, y, z), examplexyz(y, z, x));(x,y,z)");
    // addTempFun("func5=func4(exampleaaa2(x, y, z), func3(y, z), x);(x,y,z)");
    // addTempFun("func6=examplexyz(a(x, y), max(y, z, x), func1(x, y, z));(x,y,z)");
    // addTempFun("func7=func6(func3(x, y), exampleaaa2(y, z, x), max(z, y, x));(x,y,z)");
    // addTempFun("func8=exampleaaa2(func1(x, y, z), func3(y, z), examplexyz(z, x, y));(x,y,z)");
    // addTempFun("func9=func8(func4(x, y, z), examplexyz(y, z, x), func5(z, x, y));(x,y,z)");
    // addTempFun("func10=func9(func7(x, y, z), func6(y, z, x), func5(z, x, y));(x,y,z)");
    // let result = interpret("func10(func8(1,2,3), func9(4,5,6), func10(7,8,9))")?;
    //

    // addTempFun("y=10;(x,y)");
    // addTempFun("x,y=a(x,y);(x,y,z)");
    // addTempFun("r=a(x,y);(x,y,z)");
    // addTempFun("z=r(x,y(z,z),z);(z,x)");

    add_temp_fun("k=5;()");
    add_temp_fun("kk=3;()");

    let result = interpret("kk()")?;

    println!("{}", result);


    add_temp_fun("examplexyz=max(z,y,z);(x,y,z)");
    add_temp_fun("exampleaaa2=examplexyz(z,y,z);(x,y,z)");
    add_temp_fun("noting=0;()");

    add_temp_fun("k=5;()");
    add_temp_fun("kk=3;()");

    let result = interpret("exampleaaa2(1,2,43)")?;

    println!("a{}", result);

    add_temp_fun("max_of_three=sum(x,y);(x,y,z)");


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



//
// this kind of string to make a temporary function "doublex+y+1=a(a(x,x),a(y,1));(x,y)"   function string initializing
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