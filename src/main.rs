use std::vec::Vec;
use std::collections::HashMap;
use once_cell::sync::Lazy;

enum VariableType {
    Value(f64),
    Variable(String),
    BasicFunWVariables(BasicFunWVariables),
    CustomFunWVariables(CustomFunWVariables),
}

type BasicFunc = fn(&[f64]) -> Vec<f64>;

fn a(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Function 'a' expects exactly 2 arguments");
    }
    vec![args[0] + args[1]]
}

static FUNCTIONS: Lazy<HashMap<&'static str, BasicFunc>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("sum", a as BasicFunc);
    map
});

struct CustomFunc {
    primary_fun: BasicFunc,
    fun_variables: Vec<VariableType>,
    list_elements: Vec<Vec<i64>>,
    prov_variable_names: Vec<String>,
}

struct BasicFunWVariables {
    basic_func: BasicFunc,
    fun_variables: Vec<VariableType>,
}

impl BasicFunWVariables {
    fn new(basic_func: BasicFunc, fun_variables: Vec<VariableType>) -> Self {
        Self {
            basic_func,
            fun_variables,
        }
    }
}

struct CustomFunWVariables {
    custom_func: CustomFunc,
    fun_variables: Vec<VariableType>,
}

impl CustomFunWVariables {
    fn new(custom_func: CustomFunc, fun_variables: Vec<VariableType>) -> Self {
        Self {
            custom_func,
            fun_variables,
        }
    }
}

impl CustomFunc {
    fn new(
        primary_fun: BasicFunc,
        fun_variables: Vec<VariableType>,
        list_elements: Vec<Vec<i64>>,
        prov_variable_names: Vec<String>,
    ) -> CustomFunc {
        CustomFunc {
            primary_fun,
            fun_variables,
            list_elements,
            prov_variable_names,
        }
    }

    fn run(&self, prov_variable_values: Vec<f64>) -> Vec<f64> {
        let mapped_prov_variables: HashMap<String, f64> = self
            .prov_variable_names
            .iter()
            .zip(prov_variable_values.iter())
            .map(|(k, v)| (k.clone(), *v))
            .collect();

        let mut processed_variables: Vec<f64> = Vec::new();

        for fun_variable in &self.fun_variables {
            match fun_variable {
                VariableType::Value(value) => processed_variables.push(*value),
                VariableType::Variable(variable) => {
                    let value = mapped_prov_variables.get(variable).unwrap();
                    processed_variables.push(*value);
                }
                VariableType::BasicFunWVariables(b_func) => {
                    // Handle BasicFunWVariables here if necessary
                }
                VariableType::CustomFunWVariables(c_func) => {
                    // Handle CustomFunWVariables here if necessary
                }
            }
        }

        println!("Processed variables: {:?}", processed_variables);
        (self.primary_fun)(&processed_variables)
    }
}

fn main() {
    // Define some variables
    let value1 = VariableType::Value(3.14);
    let value2 = VariableType::Variable("x".to_string());

    // Create a vector of variables
    let variables = vec![value1, value2];

    // Create an instance of CustomFunc
    let my_instance = CustomFunc::new(
        a,
        variables,
        vec![vec![0], vec![0], vec![0]],
        vec![String::from("x"), String::from("y"), String::from("z")],
    );

    // Run the instance
    let result = my_instance.run(vec![1.0, 2.0, 3.0, 4.0]);
    println!("{:?}", result);
}
