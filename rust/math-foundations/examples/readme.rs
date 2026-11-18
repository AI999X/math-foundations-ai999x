//! Math Foundations: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p math-foundations-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&math_foundations_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "abs_linear_inequality", "inputs": {"a": "2", "b": "3", "c": "7", "rel": "<"}}));
    println!();
    show(json!({"key": "fraction_arithmetic", "inputs": {"a": "3", "b": "4", "c": "5", "d": "6", "op": "+"}}));
    println!();
    show(json!({"key": "complex_arithmetic", "inputs": {"a": "3", "b": "2", "c": "1", "d": "-1", "op": "/"}}));
    println!();
    show(json!({"key": "circle_from_general_form", "inputs": {"dd": "-4", "ee": "6", "ff": "-3"}}));
    println!();
    show(json!({"key": "distance_between_points", "inputs": {"x1": "1", "y1": "1", "x2": "4", "y2": "5"}}));
    println!();
    show(json!({"key": "counting_choices", "inputs": {"k": "3", "n": "10"}}));
    println!();
    show(json!({"key": "compound_interest", "inputs": {"t": "2", "principal": "1000", "r": "0.05", "n": "1"}}));
    println!();
    show(json!({"key": "doubling_time", "inputs": {"k": "0.1"}}));
    println!();
    show(json!({"key": "half_life_remaining", "inputs": {"n0": "80", "half_life": "5", "t": "15"}}));
    println!();
    show(json!({"key": "exponential_equation", "inputs": {"b": "2", "y": "32"}}));
    println!();
    show(json!({"key": "factor_monic_quadratic", "inputs": {"b": "5", "c": "6"}}));
    println!();
    show(json!({"key": "average_rate_of_change_definition", "inputs": {"x_start": "0", "f_x_start": "1", "x_end": "5", "f_x_end": "11"}}));
    println!();
    show(json!({"key": "linear_inequality", "inputs": {"a": "2", "b": "1", "c": "7"}}));
    println!();
    show(json!({"key": "abs_value_inequality_upper_bound", "inputs": {"c": "4", "d": "1"}}));
    println!();
    show(json!({"key": "linear_equation_root", "inputs": {"a": "3", "b": "6", "c": "0"}}));
}
