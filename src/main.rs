use std::collections::HashMap;
use once_cell::sync::Lazy;

enum VariableType<'a> {
    Value(f64),
    Variable(&'a str),
    BasicFunWVariables(BasicFunWVariables<'a>),
    CustomFunWVariables(CustomFunWVariables<'a>),
}

type BasicFunc = fn(&[f64]) -> Vec<f64>;

fn sum(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Function 'sum' expects exactly 2 arguments");
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

fn getSeries(args: &[f64]) -> Vec<f64> {
    if args.len() != 1 {
        panic!("Function 'getSeries' expects exactly 1 argument");
    }
    let mut result: Vec<f64> = vec![];

    for i in 1..=args[0] as i32 {
        result.push(i as f64);
    }
    result
}

static FUNCTIONS: Lazy<HashMap<&'static str, BasicFunc>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("sum", sum as BasicFunc);
    map.insert("dif", dif as BasicFunc);
    map.insert("mul", mul as BasicFunc);
    map.insert("div", div as BasicFunc);
    map.insert("getSeries", getSeries as BasicFunc);
    map
});

struct CustomFunc<'a> {
    primary_fun: BasicFunc,
    fun_variables: Vec<&'a VariableType<'a>>,
    picked_elements: Vec<Vec<usize>>,
    prov_variable_names: Vec<&'a str>,
}

struct BasicFunWVariables<'a> {
    basic_func: BasicFunc,
    fun_variables: Vec<VariableType<'a>>,
}

impl<'a> BasicFunWVariables<'a> {
    fn new(basic_func: BasicFunc, fun_variables: Vec<VariableType<'a>>) -> Self {
        Self {
            basic_func,
            fun_variables,
        }
    }
}

struct CustomFunWVariables<'a> {
    custom_func: &'a CustomFunc<'a>,
    fun_variables: Vec<VariableType<'a>>,
}

impl<'a> CustomFunWVariables<'a> {
    fn new(custom_func: &'a CustomFunc<'a>, fun_variables: Vec<VariableType<'a>>) -> Self {
        Self {
            custom_func,
            fun_variables,
        }
    }
}

impl<'a> CustomFunc<'a> {
    fn new(
        primary_fun: BasicFunc,
        fun_variables: Vec<&'a VariableType<'a>>,
        picked_elements: Vec<Vec<usize>>,
        prov_variable_names: Vec<&'a str>,
    ) -> CustomFunc<'a> {
        CustomFunc {
            primary_fun,
            fun_variables,
            picked_elements,
            prov_variable_names,
        }
    }

    fn process_variables(
        &self,
        mapped_prov_variables: &HashMap<&'a str, f64>,
        fun_variables: &[&VariableType<'a>],
    ) -> Vec<f64> {
        let mut processed_variables: Vec<f64> = Vec::new();

        for (i, fun_variable) in fun_variables.iter().enumerate() {
            match fun_variable {
                VariableType::Value(value) => processed_variables.push(*value),
                VariableType::Variable(variable) => {
                    if let Some(value) = mapped_prov_variables.get(variable) {
                        processed_variables.push(*value);
                    }
                }
                VariableType::BasicFunWVariables(b_func) => {
                    let nested_processed_variables = self.process_variables(mapped_prov_variables, &b_func.fun_variables.iter().collect::<Vec<_>>());
                    let results = (b_func.basic_func)(&nested_processed_variables);

                    if self.picked_elements.len() > i {
                        let mut picked_results: Vec<f64> = vec![];

                        for &j in &self.picked_elements[i] {
                            if let Some(&result) = results.get(j) {
                                picked_results.push(result);
                            }
                        }
                        processed_variables.extend(picked_results);
                    }
                    else
                    {
                        processed_variables.extend(results);
                    }
                }
                VariableType::CustomFunWVariables(c_func) => {
                    let nested_processed_variables = self.process_variables(mapped_prov_variables, &c_func.fun_variables.iter().collect::<Vec<_>>());
                    let results = (c_func.custom_func).run(nested_processed_variables);

                    if self.picked_elements.len() > i {
                        let mut picked_results: Vec<f64> = vec![];

                        for &j in &self.picked_elements[i] {
                            if let Some(&result) = results.get(j) {
                                picked_results.push(result);
                            }
                        }
                        processed_variables.extend(picked_results);
                    }
                    else
                    {
                        processed_variables.extend(results);
                    }
                }
            }
        }

        processed_variables
    }

    fn run(&self, prov_variable_values: Vec<f64>) -> Vec<f64> {
        let mapped_prov_variables: HashMap<&'a str, f64> = self
            .prov_variable_names
            .iter()
            .zip(prov_variable_values.iter())
            .map(|(&k, &v)| (k, v))
            .collect();

        let processed_variables = self.process_variables(&mapped_prov_variables, &self.fun_variables);

        println!("Processed variables: {:?}", processed_variables);
        (self.primary_fun)(&processed_variables)
    }
}

fn main() {
    let value2 = VariableType::Variable("y");
    let variables1 = vec![&value2, &value2];
    let my_instance1 = CustomFunc::new(
        sum,
        variables1, // Pass variables1 without cloning
        vec![],
        vec!["x", "y"],
    );

    let result = my_instance1.run(vec![1.0, 2.0]);
    println!("{:?}", result);

    let value1 = VariableType::BasicFunWVariables(BasicFunWVariables::new(getSeries, vec![VariableType::Value(5.0)]));
    let value2 = VariableType::Variable("y");
    let variables2 = vec![&value1, &value2];
    let my_instance2 = CustomFunc::new(
        dif,
        variables2, // Pass variables2 without cloning
        vec![vec![0]],
        vec!["x", "y"],
    );
    let result = my_instance2.run(vec![1.0, 2.0]);
    println!("{:?}", result);

    let value1 = VariableType::CustomFunWVariables(CustomFunWVariables::new(&my_instance1, vec![VariableType::Variable("x"), VariableType::Variable("y")]));
    let value2 = VariableType::CustomFunWVariables(CustomFunWVariables::new(&my_instance2, vec![VariableType::Variable("x"), VariableType::Variable("y")]));
    let variables = vec![&value1, &value2];
    let my_instance3 = CustomFunc::new(
        mul,
        variables,
        vec![],
        vec!["x", "y"],
    );
    let result = my_instance3.run(vec![1.0, 2.0]);
    println!("{:?}", result);

    let value1 = VariableType::CustomFunWVariables(CustomFunWVariables::new(&my_instance3, vec![VariableType::Variable("x"), VariableType::Variable("y")]));
    let value2 = VariableType::Value(3.0);
    let variables = vec![&value1, &value2];
    let my_instance4 = CustomFunc::new(
        div,
        variables,
        vec![],
        vec!["x", "y"],
    );
    let result = my_instance4.run(vec![1.0, 2.0]);
    println!("{:?}", result);
}
