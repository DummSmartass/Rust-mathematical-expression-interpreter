use std::vec::Vec;
use std::collections::HashMap;
use once_cell::sync::Lazy;

enum VariableType {
    value(f64),
    variable(String),
    basicFunWVariables(BasicFunWVariables),
    customFunWVariables(CustomFunWVariables)
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
    funVariables: Vec<VariableType>,
    listElements: Vec<Vec<i64>>,
    provVariableNames: Vec<String>,
    provVariableValues: Vec<f64>
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
    fn new(primary_fun: BasicFunc, funVariables: Vec<VariableType>,listElements: Vec<Vec<i64>>,provVariableNames: Vec<String>,    provVariableValues: Vec<f64>) -> CustomFunc
    {
        CustomFunc { primary_fun, funVariables,listElements,provVariableNames,provVariableValues}
    }

    fn run(&self) {
        let mapped_prov_variables: HashMap<String, f64> = self.provVariableNames.iter()
            .zip(self.provVariableValues.iter())
            .map(|(k, v)| (k.clone(), *v))
            .collect();


        let mut processed_variables: Vec<_> = Vec::<f64>::new();

        for funVariable in &self.funVariables
        {
            match funVariable {
                VariableType::value(value) => processed_variables.push(*value),
                VariableType::variable(variable) => {
                    let value = mapped_prov_variables.get(variable).unwrap();
                    processed_variables.push(*value);
                },
                VariableType::basicFunWVariables(bFunc) => {
                    //bFunc
                },
                VariableType::customFunWVariables(cFunc) => {
                    //cFunc
                },
            }
        }
    }
    //dodatkowa lista list określające liczności
}

fn main() {
    // Define some variables
    let value = VariableType::value(3.14);
    let variable = VariableType::variable("x".to_string());

    // Create an instance of BasicFunWVariables
    let basic_func = BasicFunWVariables::new(a, vec![]); // Pass your BasicFunc and Vec<VariableType> here

    // Create a variable of type VariableType::basicFunWVariables
    let basic_func_variable = VariableType::basicFunWVariables(basic_func);

    // Create a vector of variables
    let variables = vec![value, variable, basic_func_variable];

    let mut map = HashMap::new();
    map.insert("key1".to_string(), 1.0);
    map.insert("key2".to_string(), 2.0);

    // Create an instance of CustomFunc
    let my_instance = CustomFunc::new(
        a,
        variables,
        vec![vec![0], vec![0], vec![0]],
        vec![String::from("x"), String::from("y"), String::from("z")],
        vec![1.0,2.0,3.0,4.0]
    );

    // Run the instance
    my_instance.run();
}

