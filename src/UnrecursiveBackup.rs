use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Clone)]
enum VariableType {
    Value(f64),
    Variable(String),
    BasicFunWVariables(BasicFunWVariables),
    CustomFunWVariables(CustomFunWVariables),
}


type BasicFunc = fn(&[f64]) -> Vec<f64>;

fn sum(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Function 'a' expects exactly 2 arguments");
    }
    vec![args[0] + args[1]]
}

fn dif(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Function 'dif' expects exactly 2 arguments");
    }
    vec![args[0] - args[1]]
}

fn mul(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Function 'mul' expects exactly 2 arguments");
    }
    vec![args[0] * args[1]]
}

fn div(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Function 'div' expects exactly 2 arguments");
    }
    if args[1] == 0.0 {
        panic!("Division by zero is not allowed");
    }
    vec![args[0] / args[1]]
}

static FUNCTIONS: Lazy<HashMap<&'static str, BasicFunc>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("sum", sum as BasicFunc);
    map.insert("dif", dif as BasicFunc);
    map.insert("mul", mul as BasicFunc);
    map.insert("div", div as BasicFunc);
    map
});


#[derive(Clone)]
struct CustomFunc {
    primary_fun: BasicFunc,
    fun_variables: Vec<VariableType>,
    list_elements: Vec<Vec<i64>>,
    prov_variable_names: Vec<String>,
}

#[derive(Clone)]
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

#[derive(Clone)]
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

    fn process_variables(
        &self,
        mapped_prov_variables: &HashMap<String, f64>,
        fun_variables: &Vec<VariableType>,
    ) -> Vec<f64> {
        let mut processed_variables: Vec<f64> = Vec::new();

        for fun_variable in fun_variables {
            match fun_variable {
                VariableType::Value(value) => processed_variables.push(*value),
                VariableType::Variable(variable) => {
                    if let Some(value) = mapped_prov_variables.get(variable) {
                        processed_variables.push(*value);
                    }
                }
                VariableType::BasicFunWVariables(b_func) => {
                    let nested_processed_variables = self.process_variables(mapped_prov_variables, &b_func.fun_variables);
                    processed_variables.extend((b_func.basic_func)(&nested_processed_variables));
                }
                VariableType::CustomFunWVariables(c_func) => {
                    let nested_processed_variables = c_func.custom_func.process_variables(mapped_prov_variables, &c_func.fun_variables);
                    processed_variables.extend(c_func.custom_func.run(nested_processed_variables));
                }
            }
        }

        processed_variables
    }

    fn run(&self, prov_variable_values: Vec<f64>) -> Vec<f64> {
        let mapped_prov_variables: HashMap<String, f64> = self
            .prov_variable_names
            .iter()
            .zip(prov_variable_values.iter())
            .map(|(k, v)| (k.clone(), *v))
            .collect();

        let processed_variables = self.process_variables(&mapped_prov_variables, &self.fun_variables);

        println!("Processed variables: {:?}", processed_variables);
        (self.primary_fun)(&processed_variables)
    }
}

fn main() {
    let value1 = VariableType::Variable("x".to_string());
    let value2 = VariableType::Variable("y".to_string());
    let variables1 = vec![value1, value2];
    let my_instance1 = CustomFunc::new(
        sum,
        variables1.clone(), // Clone variables1 here
        vec![],
        vec![String::from("x"), String::from("y")],
    );

    let result = my_instance1.run(vec![1.0, 2.0]);
    println!("{:?}", result);

    let value1 = VariableType::Variable("x".to_string());
    let value2 = VariableType::Variable("y".to_string());
    let variables2 = vec![value1, value2];
    let my_instance2 = CustomFunc::new(
        dif,
        variables2.clone(), // Clone variables2 here
        vec![],
        vec![String::from("x"), String::from("y")],
    );
    let result = my_instance2.run(vec![1.0, 2.0]);
    println!("{:?}", result);

    let value1 = VariableType::CustomFunWVariables(CustomFunWVariables::new(my_instance1.clone(), variables1.clone()));
    let value2 = VariableType::CustomFunWVariables(CustomFunWVariables::new(my_instance2.clone(), variables2.clone()));
    let variables = vec![value1, value2];
    let my_instance3 = CustomFunc::new(
        mul,
        variables,
        vec![],
        vec![String::from("x"), String::from("y")],
    );
    let result = my_instance3.run(vec![1.0, 2.0]);
    println!("{:?}", result);


    let value1 = VariableType::CustomFunWVariables(CustomFunWVariables::new(my_instance3.clone(), variables1.clone()));
    let value2 = VariableType::Value(3.0);
    let variables = vec![value1, value2];
    let my_instance4 = CustomFunc::new(
        div,
        variables,
        vec![],
        vec![String::from("x"), String::from("y")],
    );
    let result = my_instance4.run(vec![1.0, 2.0]);
    println!("{:?}", result);

}

