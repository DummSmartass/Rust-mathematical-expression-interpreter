use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Typ dla funkcji podstawowej, która przyjmuje tablicę `f64` i zwraca tablicę `f64`
pub type BasicFunc = fn(&[f64]) -> Vec<f64>;

/// Funkcja sumująca dwa argumenty
fn sum(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Funkcja 'sum' oczekuje dokładnie 2 argumentów");
    }
    vec![args[0] + args[1]]
}

/// Funkcja obliczająca różnicę dwóch argumentów
fn difference(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Funkcja 'difference' oczekuje dokładnie 2 argumentów");
    }
    vec![args[0] - args[1]]
}

/// Funkcja mnożąca dwa argumenty
fn multiply(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Funkcja 'multiply' oczekuje dokładnie 2 argumentów");
    }
    vec![args[0] * args[1]]
}

/// Funkcja dzieląca pierwszy argument przez drugi
fn divide(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        panic!("Funkcja 'divide' oczekuje dokładnie 2 argumentów");
    }
    if args[1] == 0.0 {
        panic!("Dzielenie przez zero jest niedozwolone");
    }
    vec![args[0] / args[1]]
}

//

/// Funkcja zwracająca argumenty bez zmian
fn pass(args: &[f64]) -> Vec<f64> {
    args.to_vec()
}

/// Funkcja zwracająca serię liczb od 1 do podanego argumentu
fn get_series(args: &[f64]) -> Vec<f64> {
    if args.len() != 1 {
        panic!("Funkcja 'get_series' oczekuje dokładnie 1 argumentu");
    }
    let mut result = Vec::new();
    for i in 1..=args[0] as i32 {
        result.push(i as f64);
    }
    result
}

/// Statyczna mapa przechowująca wszystkie podstawowe funkcje
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

