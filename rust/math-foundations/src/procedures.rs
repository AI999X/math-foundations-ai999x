//! math_foundations: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/math_foundations/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "abs_linear_inequality",
    "binomial_coefficient_term",
    "complex_arithmetic",
    "complex_modulus_argument",
    "complex_nth_roots",
    "composite_function_value",
    "continuous_growth",
    "counting_choices",
    "cubic_roots",
    "de_moivre_power",
    "deg_to_rad",
    "distance_point_to_line",
    "domain_sqrt_over_linear",
    "doubling_time_percent",
    "ellipse_parameters",
    "evaluate_function",
    "exponential_equation_general",
    "factor_quadratic_integer",
    "fraction_arithmetic",
    "graph_transformation_point",
    "half_life_remaining",
    "heron_area",
    "hyperbola_parameters",
    "infinite_geometric_series",
    "inverse_linear_function",
    "law_of_cosines_sas",
    "law_of_cosines_sss",
    "law_of_sines_aas",
    "law_of_sines_ssa",
    "line_intercepts",
    "linear_system_3x3",
    "log_equation_linear_argument",
    "log_evaluate",
    "log_laws_combine",
    "order_of_operations",
    "parabola_focus_directrix",
    "partial_fractions_linear",
    "percent_change",
    "percent_of_number",
    "percent_of_what",
    "piecewise_evaluate",
    "polar_to_rectangular",
    "polynomial_long_division",
    "proportion_solve",
    "quadratic_discriminant",
    "quadratic_inequality",
    "quadratic_roots",
    "quartic_roots",
    "rad_to_deg",
    "radical_equation",
    "ratio_split",
    "rational_equation",
    "rational_roots",
    "rectangular_to_polar",
    "right_triangle_angle_side",
    "right_triangle_leg_hypotenuse",
    "right_triangle_legs",
    "round_decimal_places",
    "round_significant_figures",
    "scientific_notation",
    "synthetic_division",
    "trig_equation_interval",
    "trig_values_from_one",
    "unit_circle_exact",
    "unit_rate_compare",
    "vector_dot_angle",
    "vector_magnitude_direction",
    "what_percent",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "abs_linear_inequality" => {
            let r = f_abs_linear_inequality(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rel") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_absineq(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "binomial_coefficient_term" => {
            let r = f_binomial_coefficient_term(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_binom(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "complex_arithmetic" => {
            let r = f_complex_arithmetic(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("op") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_carith(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "complex_modulus_argument" => {
            let r = f_complex_modulus_argument(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_95(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "complex_nth_roots" => {
            let r = f_complex_nth_roots(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_nroots(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "composite_function_value" => {
            let r = f_composite_function_value(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_comp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "continuous_growth" => {
            let r = f_continuous_growth(match v.get("n0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_247(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "counting_choices" => {
            let r = f_counting_choices(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_counting(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cubic_roots" => {
            let r = f_cubic_roots(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_276(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "de_moivre_power" => {
            let r = f_de_moivre_power(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_demoivre(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "deg_to_rad" => {
            let r = f_deg_to_rad(match v.get("deg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_d2r(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "distance_point_to_line" => {
            let r = f_distance_point_to_line(match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_dist_line(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "domain_sqrt_over_linear" => {
            let r = f_domain_sqrt_over_linear(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("root_in_denominator") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Bool(false)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_domain(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "doubling_time_percent" => {
            let r = f_doubling_time_percent(match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_402(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ellipse_parameters" => {
            let r = f_ellipse_parameters(match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ellipse(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "evaluate_function" => {
            let r = f_evaluate_function(match v.get("expression") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_feval(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "exponential_equation_general" => {
            let r = f_exponential_equation_general(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_547(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "factor_quadratic_integer" => {
            let r = f_factor_quadratic_integer(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_factor(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fraction_arithmetic" => {
            let r = f_fraction_arithmetic(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("op") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_fraction(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graph_transformation_point" => {
            let r = f_graph_transformation_point(match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("B") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("k") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_627(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "half_life_remaining" => {
            let r = f_half_life_remaining(match v.get("n0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("half_life") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_641(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "heron_area" => {
            let r = f_heron_area(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_659(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hyperbola_parameters" => {
            let r = f_hyperbola_parameters(match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("orientation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_hyperbola(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "infinite_geometric_series" => {
            let r = f_infinite_geometric_series(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_691(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inverse_linear_function" => {
            let r = f_inverse_linear_function(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_inv(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "law_of_cosines_sas" => {
            let r = f_law_of_cosines_sas(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_718(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "law_of_cosines_sss" => {
            let r = f_law_of_cosines_sss(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_735(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "law_of_sines_aas" => {
            let r = f_law_of_sines_aas(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_aas(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "law_of_sines_ssa" => {
            let r = f_law_of_sines_ssa(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ssa(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "line_intercepts" => {
            let r = f_line_intercepts(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_811(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_system_3x3" => {
            let r = f_linear_system_3x3(match v.get("augmented") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_sys3(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "log_equation_linear_argument" => {
            let r = f_log_equation_linear_argument(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_logeq(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "log_evaluate" => {
            let r = f_log_evaluate(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_881(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "log_laws_combine" => {
            let r = f_log_laws_combine(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_886(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "order_of_operations" => {
            let r = f_order_of_operations(match v.get("expression") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ooo(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parabola_focus_directrix" => {
            let r = f_parabola_focus_directrix(match v.get("orientation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("k") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_parabola(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "partial_fractions_linear" => {
            let r = f_partial_fractions_linear(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_pf(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "percent_change" => {
            let r = f_percent_change(match v.get("old") { Some(x) => py::from_val(x), None => { return None } }, match v.get("new") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_pchange(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "percent_of_number" => {
            let r = f_percent_of_number(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1038(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "percent_of_what" => {
            let r = f_percent_of_what(match v.get("part") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1045(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "piecewise_evaluate" => {
            let r = f_piecewise_evaluate(match v.get("e1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("left_rel") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_pw(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polar_to_rectangular" => {
            let r = f_polar_to_rectangular(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1179(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polynomial_long_division" => {
            let r = f_polynomial_long_division(match v.get("dividend") { Some(x) => py::from_val(x), None => { return None } }, match v.get("divisor") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_longdiv(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "proportion_solve" => {
            let r = f_proportion_solve(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("position") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_prop(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quadratic_discriminant" => {
            let r = f_discriminant(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_discriminant(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quadratic_inequality" => {
            let r = f_quadratic_inequality(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rel") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_qineq(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quadratic_roots" => {
            let r = f_quadratic_roots(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_roots(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quartic_roots" => {
            let r = f_quartic_roots(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1325(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rad_to_deg" => {
            let r = f_rad_to_deg(match v.get("rad") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1338(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radical_equation" => {
            let r = f_radical_equation(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_radical(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ratio_split" => {
            let r = f_ratio_split(match v.get("total") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ratio(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rational_equation" => {
            let r = f_rational_equation(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_rateq(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rational_roots" => {
            let r = f_rational_roots(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ratroots(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rectangular_to_polar" => {
            let r = f_rectangular_to_polar(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1552(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "right_triangle_angle_side" => {
            let r = f_right_triangle_angle_side(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("side") { Some(x) => py::from_val(x), None => { return None } }, match v.get("which") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_rt_as(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "right_triangle_leg_hypotenuse" => {
            let r = f_right_triangle_leg_hypotenuse(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1581(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "right_triangle_legs" => {
            let r = f_right_triangle_legs(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_rt_legs(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "round_decimal_places" => {
            let r = f_round_decimal_places(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_round_dp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "round_significant_figures" => {
            let r = f_round_significant_figures(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_round_sf(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "scientific_notation" => {
            let r = f_scientific_notation(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_sci(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "synthetic_division" => {
            let r = f_synthetic_division(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_synth(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "trig_equation_interval" => {
            let r = f_trig_equation_interval(match v.get("func") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } }, match v.get("unit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("degrees")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_trigeq(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "trig_values_from_one" => {
            let r = f_trig_values_from_one(match v.get("func") { Some(x) => py::from_val(x), None => { return None } }, match v.get("value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quadrant") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_trigvals(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "unit_circle_exact" => {
            let r = f_unit_circle_exact(match v.get("deg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_unit(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "unit_rate_compare" => {
            let r = f_unit_rate_compare(match v.get("n1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("c2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1890(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vector_dot_angle" => {
            let r = f_vector_dot_angle(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_vec(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vector_magnitude_direction" => {
            let r = f_vector_magnitude_direction(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_magdir(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "what_percent" => {
            let r = f_what_percent(match v.get("part") { Some(x) => py::from_val(x), None => { return None } }, match v.get("whole") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1943(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_COUNTING_MAX_N() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Int(170i64)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__ARITH() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pyre::module_call("compile", &Py::str("^[\\d\\s.+\\-*/^()\\[\\]]+$"), &[], 0)?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__EXACT_SIN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(0i64), Py::tuple(vec![Py::str("0"), Py::Float(0.0)])), (Py::Int(30i64), Py::tuple(vec![Py::str("1/2"), Py::Float(0.5)])), (Py::Int(45i64), Py::tuple(vec![Py::str("sqrt(2)/2"), Py::Float(0.7071067811865476)])), (Py::Int(60i64), Py::tuple(vec![Py::str("sqrt(3)/2"), Py::Float(0.8660254037844386)])), (Py::Int(90i64), Py::tuple(vec![Py::str("1"), Py::Float(1.0)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__EXACT_TAN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(0i64), Py::tuple(vec![Py::str("0"), Py::Float(0.0)])), (Py::Int(30i64), Py::tuple(vec![Py::str("sqrt(3)/3"), Py::Float(0.5773502691896257)])), (Py::Int(45i64), Py::tuple(vec![Py::str("1"), Py::Float(1.0)])), (Py::Int(60i64), Py::tuple(vec![Py::str("sqrt(3)"), Py::Float(1.7320508075688772)])), (Py::Int(90i64), Py::tuple(vec![Py::str("undefined"), Py::None]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__FEXPR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pyre::module_call("compile", &Py::str("^[\\dx\\s.+\\-*/^()\\[\\]a-z]+$"), &[], 0)?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__FUNC_NAMES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("sqrt"), Py::str("abs"), Py::str("ln")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LETTERS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::str("abcde")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__MAX_DIGITS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Int(400i64)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__ORIENT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("vertical"), Py::str("horizontal")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__POSITIONS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("first numerator"), Py::str("first denominator"), Py::str("second numerator"), Py::str("second denominator")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TOK() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pyre::module_call("compile", &Py::str("\\s*(?:(\\d+(?:\\.\\d+)?)|([a-z_]+)|(\\*\\*|[-+*/()]))"), &[], 0)?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TRIG() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("sin"), Py::str("cos"), Py::str("tan")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__UNITS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("degrees"), Py::str("radians")])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__arg360(mut v_z: Py) -> R {
    let mut v_t: Py = Py::None;
    v_t = py::modulo(&py::math1("degrees", &py::atan2(&py::imag(&v_z.clone())?, &py::real(&v_z.clone())?)?)?, &Py::Float(360.0))?;
    return Ok((if ({ let t115 = v_t.clone(); Py::Bool({ let t116 = py::sub(&Py::Float(360.0), &Py::Float(1e-09))?; if py::gt(&t115, &t116)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_t.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__big(mut v_v: Py) -> R {
    if ({ let t1101 = Py::Bool(matches!(v_v.clone(), Py::Frac(_))); if !t1101.truthy() { t1101 } else { { let t1100 = { let t1096 = py::len(&Py::str(&py::to_str(&py::abs(&py::numerator(&v_v.clone())?)?)))?; Py::Bool({ let t1097 = c__MAX_DIGITS()?; if py::gt(&t1096, &t1097)? { true } else { false } }) }; if t1100.truthy() { t1100 } else { { let t1098 = py::len(&Py::str(&py::to_str(&py::denominator(&v_v.clone())?)))?; Py::Bool({ let t1099 = c__MAX_DIGITS()?; if py::gt(&t1098, &t1099)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the numbers become too large"))));
    }
    if ({ let t1102 = Py::Bool(matches!(v_v.clone(), Py::Float(_))); if !t1102.truthy() { t1102 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the value is not finite"))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__candidates_roots(mut v_co: Py) -> R {
    let mut v_cand: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_roots: Py = Py::None;
    v_co = Py::list((v_co.clone()).items()?);
    v_roots = py::set(&Py::list(vec![]))?;
    while ({ let t1479 = { let t1475 = py::len(&v_co.clone())?; Py::Bool({ let t1476 = Py::Int(1i64); if py::gt(&t1475, &t1476)? { true } else { false } }) }; if !t1479.truthy() { t1479 } else { { let t1477 = py::getitem(&v_co.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t1478 = Py::Int(0i64); if py::eq(&t1477, &t1478) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_roots.clone(), "add", &[py::fraction(&[Py::Int(0i64)])?])?;
        let _ = py::method(&v_co.clone(), "pop", &[])?;
    }
    if ({ let t1480 = py::len(&v_co.clone())?; Py::Bool({ let t1481 = Py::Int(1i64); if py::gt(&t1480, &t1481)? { true } else { false } }) }).truthy() {
        for t1485 in py::iter(&(f__divisors(py::int(&py::getitem(&v_co.clone(), &py::neg(&Py::Int(1i64))?)?)?)?))? {
            v_p = t1485;
            for t1486 in py::iter(&(f__divisors(py::int(&py::getitem(&v_co.clone(), &Py::Int(0i64))?)?)?))? {
                v_q = t1486;
                for t1487 in py::iter(&(Py::tuple(vec![py::fraction(&[v_p.clone(), v_q.clone()])?, py::fraction(&[py::neg(&v_p.clone())?, v_q.clone()])?])))? {
                    v_cand = t1487;
                    if ({ let t1493 = { let t1488 = v_cand.clone(); Py::Bool({ let t1489 = v_roots.clone(); if !py::contains(&t1489, &t1488)? { true } else { false } }) }; if !t1493.truthy() { t1493 } else { { let t1490 = f__horner(v_co.clone(), v_cand.clone())?; Py::Bool({ let t1491 = Py::Int(0i64); if py::eq(&t1490, &t1491) { true } else { false } }) } } }).truthy() {
                        let _ = py::method(&v_roots.clone(), "add", &[v_cand.clone()])?;
                    }
                }
            }
        }
    }
    return Ok(py::sorted(&v_roots.clone(), (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__canonical_roots(mut v_roots: Py) -> R {
    let mut v_cplx: Py = Py::None;
    let mut v_real: Py = Py::None;
    v_real = py::sorted(&{ let mut out = vec![]; for t1318 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t1318;
if ({ let t1319 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t1320 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::le(&t1319, &t1320)? { true } else { false } }) }).truthy() { out.push(py::float(&py::real(&v_r.clone())?)?); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
    v_cplx = py::sorted_key(&{ let mut out = vec![]; for t1321 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t1321;
if ({ let t1322 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t1323 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::gt(&t1322, &t1323)? { true } else { false } }) }).truthy() { out.push(py::complex(&[v_r.clone()])?); }} Py::list(out) }, &{ let v_real = v_real.clone(); let v_roots = v_roots.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::round(&py::real(&v_z.clone())?, Some((Py::Int(12i64)).i()?))?, py::neg(&py::imag(&v_z.clone())?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok(py::add(&v_real.clone(), &v_cplx.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_name: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t1294 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("a"), v_a.clone()]), Py::tuple(vec![Py::str("b"), v_b.clone()]), Py::tuple(vec![Py::str("c"), v_c.clone()])])))? {
        let t1296 = (t1294).items()?;
        if t1296.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_name = t1296[0].clone();
        v_v = t1296[1].clone();
        if (py::not(&{ let t1295 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1295.truthy() { t1295 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite real number".to_string()].concat()))));
        }
    }
    if ({ let t1297 = v_a.clone(); Py::Bool({ let t1298 = Py::Int(0i64); if py::eq(&t1297, &t1298) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0: with a = 0 the equation is linear, not quadratic"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__clean(mut v_q: Py) -> R {
    v_q = py::method(&py::method(&py::method(&py::method(&py::method(&py::method(&v_q.clone(), "strip", &[])?, "replace", &[Py::str("−"), Py::str("-")])?, "replace", &[Py::str("–"), Py::str("-")])?, "replace", &[Py::str("×"), Py::str("*")])?, "replace", &[Py::str("·"), Py::str("*")])?, "replace", &[Py::str("÷"), Py::str(" divided by ")])?;
    v_q = py::method(&py::method(&py::method(&py::method(&py::method(&v_q.clone(), "replace", &[Py::str("π"), Py::str("pi")])?, "replace", &[Py::str("√"), Py::str("sqrt")])?, "replace", &[Py::str("≤"), Py::str("<=")])?, "replace", &[Py::str("≥"), Py::str(">=")])?, "replace", &[Py::str("°"), Py::str(" degrees")])?;
    return Ok(py::method(&ax_engine::pyre::module_call("sub", &Py::str("\\s+"), &[Py::str(" "), v_q.clone(), Py::Int(0)], 0)?, "strip", &[])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cpow_exact(mut v_A: Py, mut v_B: Py, mut v_n: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_ri: Py = Py::None;
    let mut v_rr: Py = Py::None;
    if ({ let t297 = v_n.clone(); Py::Bool({ let t298 = Py::Int(0i64); if py::lt(&t297, &t298)? { true } else { false } }) }).truthy() {
        v_den = py::add(&py::mul(&v_A.clone(), &v_A.clone())?, &py::mul(&v_B.clone(), &v_B.clone())?)?;
        let t299 = (Py::tuple(vec![py::truediv(&v_A.clone(), &v_den.clone())?, py::truediv(&py::neg(&v_B.clone())?, &v_den.clone())?, py::neg(&v_n.clone())?])).items()?;
        if t299.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_A = t299[0].clone();
        v_B = t299[1].clone();
        v_n = t299[2].clone();
    }
    let t300 = (Py::tuple(vec![py::fraction(&[Py::Int(1i64)])?, py::fraction(&[Py::Int(0i64)])?])).items()?;
    if t300.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rr = t300[0].clone();
    v_ri = t300[1].clone();
    while (v_n.clone()).truthy() {
        if (py::bitand(&v_n.clone(), &Py::Int(1i64))?).truthy() {
            let t301 = (Py::tuple(vec![py::sub(&py::mul(&v_rr.clone(), &v_A.clone())?, &py::mul(&v_ri.clone(), &v_B.clone())?)?, py::add(&py::mul(&v_rr.clone(), &v_B.clone())?, &py::mul(&v_ri.clone(), &v_A.clone())?)?])).items()?;
            if t301.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_rr = t301[0].clone();
            v_ri = t301[1].clone();
        }
        let t302 = (Py::tuple(vec![py::sub(&py::mul(&v_A.clone(), &v_A.clone())?, &py::mul(&v_B.clone(), &v_B.clone())?)?, py::mul(&py::mul(&Py::Int(2i64), &v_A.clone())?, &v_B.clone())?])).items()?;
        if t302.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_A = t302[0].clone();
        v_B = t302[1].clone();
        v_n = py::rshift(&v_n, &Py::Int(1i64))?;
    }
    return Ok(Py::tuple(vec![v_rr.clone(), v_ri.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cross(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_op: Py) -> R {
    return Ok(py::getitem(&Py::dict(vec![(Py::str("+"), Py::tuple(vec![py::add(&py::mul(&v_a.clone(), &v_d.clone())?, &py::mul(&v_c.clone(), &v_b.clone())?)?, py::mul(&v_b.clone(), &v_d.clone())?])), (Py::str("-"), Py::tuple(vec![py::sub(&py::mul(&v_a.clone(), &v_d.clone())?, &py::mul(&v_c.clone(), &v_b.clone())?)?, py::mul(&v_b.clone(), &v_d.clone())?])), (Py::str("*"), Py::tuple(vec![py::mul(&v_a.clone(), &v_c.clone())?, py::mul(&v_b.clone(), &v_d.clone())?])), (Py::str("/"), Py::tuple(vec![py::mul(&v_a.clone(), &v_d.clone())?, py::mul(&v_b.clone(), &v_c.clone())?]))]), &v_op.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cross_mag(mut v_u: Py, mut v_v: Py) -> R {
    if ({ let t1914 = py::len(&v_u.clone())?; Py::Bool({ let t1915 = Py::Int(2i64); if py::eq(&t1914, &t1915) { true } else { false } }) }).truthy() {
        return Ok(py::abs(&py::sub(&py::mul(&py::getitem(&v_u.clone(), &Py::Int(0i64))?, &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_u.clone(), &Py::Int(1i64))?, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?)?)?);
    }
    return Ok(py::math1("sqrt", &py::add(&py::add(&py::pow(&py::sub(&py::mul(&py::getitem(&v_u.clone(), &Py::Int(1i64))?, &py::getitem(&v_v.clone(), &Py::Int(2i64))?)?, &py::mul(&py::getitem(&v_u.clone(), &Py::Int(2i64))?, &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?)?, &Py::Int(2i64))?, &py::pow(&py::sub(&py::mul(&py::getitem(&v_u.clone(), &Py::Int(2i64))?, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_u.clone(), &Py::Int(0i64))?, &py::getitem(&v_v.clone(), &Py::Int(2i64))?)?)?, &Py::Int(2i64))?)?, &py::pow(&py::sub(&py::mul(&py::getitem(&v_u.clone(), &Py::Int(0i64))?, &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_u.clone(), &Py::Int(1i64))?, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?)?, &Py::Int(2i64))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dec(mut v_x: Py) -> R {
    return Ok(py::fraction(&[Py::str(&py::repr(&py::float(&v_x.clone())?))])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__deg_sin(mut v_d: Py) -> R {
    return Ok(py::math1("sin", &py::math1("radians", &v_d.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__direction(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_r = py::hypot(&[v_x.clone(), v_y.clone()])?;
    v_t = py::math1("degrees", &py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&v_x.clone(), &v_r.clone())?], false)?], true)?)?)?;
    return Ok(py::modulo(&(if ({ let t1932 = v_y.clone(); Py::Bool({ let t1933 = Py::Int(0i64); if py::lt(&t1932, &t1933)? { true } else { false } }) }).truthy() { py::sub(&Py::Float(360.0), &v_t.clone())? } else { v_t.clone() }), &Py::Float(360.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__divisors(mut v_n: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_n = py::abs(&v_n.clone())?;
    v_out = py::set(&Py::list(vec![]))?;
    for t1482 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::isqrt(&v_n.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t1482;
        if ({ let t1483 = py::modulo(&v_n.clone(), &v_k.clone())?; Py::Bool({ let t1484 = Py::Int(0i64); if py::eq(&t1483, &t1484) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "update", &[Py::tuple(vec![v_k.clone(), py::floordiv(&v_n.clone(), &v_k.clone())?])])?;
        }
    }
    return Ok(py::sorted(&v_out.clone(), (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__domain_parts(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_strict: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi_br: Py = Py::None;
    let mut v_inside: Py = Py::None;
    let mut v_lo_br: Py = Py::None;
    let mut v_side: Py = Py::None;
    let mut v_text: Py = Py::None;
    let mut v_up: Py = Py::None;
    v_B = py::truediv(&py::neg(&f__fr(v_b.clone())?)?, &f__fr(v_a.clone())?)?;
    v_up = { let t359 = v_a.clone(); Py::Bool({ let t360 = Py::Int(0i64); if py::gt(&t359, &t360)? { true } else { false } }) };
    v_side = (if (v_up.clone()).truthy() { (if (v_strict.clone()).truthy() { Py::str(">") } else { Py::str(">=") }) } else { (if (v_strict.clone()).truthy() { Py::str("<") } else { Py::str("<=") }) });
    v_E = Py::None;
    if ({ let t361 = v_c.clone(); Py::Bool({ let t362 = Py::Int(0i64); if py::ne(&t361, &t362) { true } else { false } }) }).truthy() {
        v_e = py::truediv(&py::neg(&f__fr(v_d.clone())?)?, &f__fr(v_c.clone())?)?;
        v_inside = { let t370 = (if (v_up.clone()).truthy() { { let t363 = v_e.clone(); Py::Bool({ let t364 = v_B.clone(); if py::gt(&t363, &t364)? { true } else { false } }) } } else { { let t365 = v_e.clone(); Py::Bool({ let t366 = v_B.clone(); if py::lt(&t365, &t366)? { true } else { false } }) } }); if t370.truthy() { t370 } else { { let t369 = { let t367 = v_e.clone(); Py::Bool({ let t368 = v_B.clone(); if py::eq(&t367, &t368) { true } else { false } }) }; if !t369.truthy() { t369 } else { py::not(&v_strict.clone()) } } } };
        v_E = (if (v_inside.clone()).truthy() { v_e.clone() } else { Py::None });
    }
    v_f = { let v_B = v_B.clone(); let v_E = v_E.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_e = v_e.clone(); let v_inside = v_inside.clone(); let v_side = v_side.clone(); let v_strict = v_strict.clone(); let v_up = v_up.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::str(&[py::format(&py::float(&v_v.clone())?, ".10g")?].concat()));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t371 = (Py::tuple(vec![(if (v_strict.clone()).truthy() { Py::str("(") } else { Py::str("[") }), (if (v_strict.clone()).truthy() { Py::str("(") } else { Py::str("]") })])).items()?;
    if t371.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo_br = t371[0].clone();
    v_hi_br = t371[1].clone();
    if (v_up.clone()).truthy() {
        if ({ let t372 = v_E.clone(); Py::Bool({ let t373 = Py::None; if py::is(&t372, &t373) { true } else { false } }) }).truthy() {
            v_text = Py::str(&[py::format(&v_lo_br.clone(), "")?, py::format(&py::call(&v_f.clone(), &[v_B.clone()])?, "")?, ", inf)".to_string()].concat());
        } else {
            if ({ let t374 = v_E.clone(); Py::Bool({ let t375 = v_B.clone(); if py::eq(&t374, &t375) { true } else { false } }) }).truthy() {
                v_text = Py::str(&["(".to_string(), py::format(&py::call(&v_f.clone(), &[v_B.clone()])?, "")?, ", inf)".to_string()].concat());
            } else {
                v_text = Py::str(&[py::format(&v_lo_br.clone(), "")?, py::format(&py::call(&v_f.clone(), &[v_B.clone()])?, "")?, ", ".to_string(), py::format(&py::call(&v_f.clone(), &[v_E.clone()])?, "")?, ") U (".to_string(), py::format(&py::call(&v_f.clone(), &[v_E.clone()])?, "")?, ", inf)".to_string()].concat());
            }
        }
    } else {
        v_hi_br = (if (v_strict.clone()).truthy() { Py::str(")") } else { Py::str("]") });
        if ({ let t376 = v_E.clone(); Py::Bool({ let t377 = Py::None; if py::is(&t376, &t377) { true } else { false } }) }).truthy() {
            v_text = Py::str(&["(-inf, ".to_string(), py::format(&py::call(&v_f.clone(), &[v_B.clone()])?, "")?, py::format(&v_hi_br.clone(), "")?].concat());
        } else {
            if ({ let t378 = v_E.clone(); Py::Bool({ let t379 = v_B.clone(); if py::eq(&t378, &t379) { true } else { false } }) }).truthy() {
                v_text = Py::str(&["(-inf, ".to_string(), py::format(&py::call(&v_f.clone(), &[v_B.clone()])?, "")?, ")".to_string()].concat());
            } else {
                v_text = Py::str(&["(-inf, ".to_string(), py::format(&py::call(&v_f.clone(), &[v_E.clone()])?, "")?, ") U (".to_string(), py::format(&py::call(&v_f.clone(), &[v_E.clone()])?, "")?, ", ".to_string(), py::format(&py::call(&v_f.clone(), &[v_B.clone()])?, "")?, py::format(&v_hi_br.clone(), "")?].concat());
            }
        }
    }
    return Ok(Py::tuple(vec![v_B.clone(), v_side.clone(), v_E.clone(), v_text.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eval_pratt(mut v_text: Py, mut v_env: Py, mut v_var: Py, mut v_flat: Py) -> R {
    let mut v_lbp: Py = Py::None;
    let mut v_parse: Py = Py::None;
    let mut v_peek: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_take: Py = Py::None;
    let mut v_toks: Py = Py::None;
    let mut v_value: Py = Py::None;
    let c_nud = py::cell_new();
    v_env = { let t1059 = v_env.clone(); if t1059.truthy() { t1059 } else { Py::dict(vec![]) } };
    v_toks = f__tokens(f__prep_expr(v_text.clone(), v_var.clone())?)?;
    v_pos = Py::list(vec![Py::Int(0i64)]);
    v_lbp = (if (py::not(&v_flat.clone())).truthy() { Py::dict(vec![(Py::str("+"), Py::Int(10i64)), (Py::str("-"), Py::Int(10i64)), (Py::str("*"), Py::Int(20i64)), (Py::str("/"), Py::Int(20i64)), (Py::str("**"), Py::Int(30i64))]) } else { Py::dict(vec![(Py::str("+"), Py::Int(10i64)), (Py::str("-"), Py::Int(10i64)), (Py::str("*"), Py::Int(10i64)), (Py::str("/"), Py::Int(10i64)), (Py::str("**"), Py::Int(10i64))]) });
    v_peek = { let v_env = v_env.clone(); let v_flat = v_flat.clone(); let v_lbp = v_lbp.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let v_var = v_var.clone(); let c_nud = c_nud.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok((if ({ let t1071 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t1072 = py::len(&v_toks.clone())?; if py::lt(&t1071, &t1072)? { true } else { false } }) }).truthy() { py::getitem(&v_toks.clone(), &py::getitem(&v_pos.clone(), &Py::Int(0i64))?)? } else { Py::tuple(vec![Py::str("end"), Py::None]) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_take = { let v_env = v_env.clone(); let v_flat = v_flat.clone(); let v_lbp = v_lbp.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let v_var = v_var.clone(); let c_nud = c_nud.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = Py::None;
            v_t = py::call(&v_peek.clone(), &[])?;
            let t1073 = v_pos.clone(); let t1074 = Py::Int(0i64);
            py::setitem(&t1073, &t1074, py::iadd(&py::getitem(&t1073, &t1074)?, &Py::Int(1i64))?)?;
            return Ok(v_t.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let rec1075 = py::rec_cell();
    v_parse = { let v_env = v_env.clone(); let v_flat = v_flat.clone(); let v_lbp = v_lbp.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let v_var = v_var.clone(); let c_nud = c_nud.clone(); let rec1075 = rec1075.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_rbp: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_parse: Py = py::rec_get(&rec1075)?;
            let mut v_left: Py = Py::None;
            let mut v_p: Py = Py::None;
            let mut v_right: Py = Py::None;
            let mut v_t: Py = Py::None;
            v_t = py::call(&v_take.clone(), &[])?;
            v_left = py::call(&py::cell_get(&c_nud)?, &[v_t.clone()])?;
            while (Py::Bool(true)).truthy() {
                v_p = py::call(&v_peek.clone(), &[])?;
                if ({ let t1083 = { let t1076 = py::getitem(&v_p.clone(), &Py::Int(0i64))?; Py::Bool({ let t1077 = Py::str("op"); if py::ne(&t1076, &t1077) { true } else { false } }) }; if t1083.truthy() { t1083 } else { { let t1082 = { let t1078 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t1079 = v_lbp.clone(); if !py::contains(&t1079, &t1078)? { true } else { false } }) }; if t1082.truthy() { t1082 } else { { let t1080 = py::getitem(&v_lbp.clone(), &py::getitem(&v_p.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1081 = v_rbp.clone(); if py::le(&t1080, &t1081)? { true } else { false } }) } } } } }).truthy() {
                    break;
                }
                let _ = py::call(&v_take.clone(), &[])?;
                if ({ let t1084 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t1085 = Py::str("**"); if py::eq(&t1084, &t1085) { true } else { false } }) }).truthy() {
                    v_right = py::call(&v_parse.clone(), &[(if (py::not(&v_flat.clone())).truthy() { Py::Int(29i64) } else { Py::Int(10i64) })])?;
                    v_left = f__pow(v_left.clone(), (if (Py::Bool(matches!(v_right.clone(), Py::Frac(_)))).truthy() { v_right.clone() } else { py::fraction(&[v_right.clone()])? }))?;
                    continue;
                }
                v_right = py::call(&v_parse.clone(), &[py::getitem(&v_lbp.clone(), &py::getitem(&v_p.clone(), &Py::Int(1i64))?)?])?;
                if ({ let t1119 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t1120 = Py::str("+"); if py::eq(&t1119, &t1120) { true } else { false } }) }).truthy() {
                    v_left = f__big(py::add(&v_left.clone(), &v_right.clone())?)?;
                } else {
                    if ({ let t1121 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t1122 = Py::str("-"); if py::eq(&t1121, &t1122) { true } else { false } }) }).truthy() {
                        v_left = f__big(py::sub(&v_left.clone(), &v_right.clone())?)?;
                    } else {
                        if ({ let t1123 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t1124 = Py::str("*"); if py::eq(&t1123, &t1124) { true } else { false } }) }).truthy() {
                            v_left = f__big(py::mul(&v_left.clone(), &v_right.clone())?)?;
                        } else {
                            if ({ let t1125 = v_right.clone(); Py::Bool({ let t1126 = Py::Int(0i64); if py::eq(&t1125, &t1126) { true } else { false } }) }).truthy() {
                                return Err(py::value_error(py::to_str(&Py::str("division by zero"))));
                            }
                            v_left = f__big(py::truediv(&v_left.clone(), &v_right.clone())?)?;
                        }
                    }
                }
            }
            return Ok(v_left.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec1075, &v_parse);
    py::cell_set(&c_nud, { let v_env = v_env.clone(); let v_flat = v_flat.clone(); let v_lbp = v_lbp.clone(); let v_parse = v_parse.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let v_var = v_var.clone(); let c_nud = c_nud.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_inner: Py = Py::None;
            let mut v_kind: Py = Py::None;
            let mut v_v: Py = Py::None;
            let mut v_x: Py = Py::None;
            let t1127 = (v_t.clone()).items()?;
            if t1127.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_kind = t1127[0].clone();
            v_v = t1127[1].clone();
            if ({ let t1128 = v_kind.clone(); Py::Bool({ let t1129 = Py::str("num"); if py::eq(&t1128, &t1129) { true } else { false } }) }).truthy() {
                return Ok(py::fraction(&[v_v.clone()])?);
            }
            if ({ let t1130 = v_kind.clone(); Py::Bool({ let t1131 = Py::str("name"); if py::eq(&t1130, &t1131) { true } else { false } }) }).truthy() {
                if ({ let t1132 = v_v.clone(); Py::Bool({ let t1133 = v_env.clone(); if py::contains(&t1133, &t1132)? { true } else { false } }) }).truthy() {
                    v_x = py::getitem(&v_env.clone(), &v_v.clone())?;
                    return Ok((if (Py::Bool(matches!(v_x.clone(), Py::Float(_)))).truthy() { py::fraction(&[Py::str(&py::repr(&py::float(&v_x.clone())?))])? } else { py::fraction(&[v_x.clone()])? }));
                }
                if ({ let t1134 = v_v.clone(); Py::Bool({ let t1135 = c__FUNC_NAMES()?; if py::contains(&t1135, &t1134)? { true } else { false } }) }).truthy() {
                    if ({ let t1136 = py::call(&v_take.clone(), &[])?; Py::Bool({ let t1137 = Py::tuple(vec![Py::str("op"), Py::str("(")]); if py::ne(&t1136, &t1137) { true } else { false } }) }).truthy() {
                        return Err(py::value_error(py::to_str(&Py::str("a function needs parentheses"))));
                    }
                    v_inner = py::call(&v_parse.clone(), &[Py::Int(0i64)])?;
                    if ({ let t1138 = py::call(&v_take.clone(), &[])?; Py::Bool({ let t1139 = Py::tuple(vec![Py::str("op"), Py::str(")")]); if py::ne(&t1138, &t1139) { true } else { false } }) }).truthy() {
                        return Err(py::value_error(py::to_str(&Py::str("unbalanced parentheses"))));
                    }
                    return Ok(f__fn(v_v.clone(), v_inner.clone())?);
                }
                return Err(py::value_error(py::to_str(&Py::str(&["unknown name ".to_string(), py::format(&Py::str(&py::repr(&v_v.clone())), "")?].concat()))));
            }
            if ({ let t1148 = v_t.clone(); Py::Bool({ let t1149 = Py::tuple(vec![Py::str("op"), Py::str("(")]); if py::eq(&t1148, &t1149) { true } else { false } }) }).truthy() {
                v_inner = py::call(&v_parse.clone(), &[Py::Int(0i64)])?;
                if ({ let t1150 = py::call(&v_take.clone(), &[])?; Py::Bool({ let t1151 = Py::tuple(vec![Py::str("op"), Py::str(")")]); if py::ne(&t1150, &t1151) { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("unbalanced parentheses"))));
                }
                return Ok(v_inner.clone());
            }
            if ({ let t1152 = v_t.clone(); Py::Bool({ let t1153 = Py::tuple(vec![Py::str("op"), Py::str("-")]); if py::eq(&t1152, &t1153) { true } else { false } }) }).truthy() {
                return Ok(py::neg(&py::call(&v_parse.clone(), &[(if (py::not(&v_flat.clone())).truthy() { Py::Int(25i64) } else { Py::Int(30i64) })])?)?);
            }
            if ({ let t1154 = v_t.clone(); Py::Bool({ let t1155 = Py::tuple(vec![Py::str("op"), Py::str("+")]); if py::eq(&t1154, &t1155) { true } else { false } }) }).truthy() {
                return Ok(py::call(&v_parse.clone(), &[(if (py::not(&v_flat.clone())).truthy() { Py::Int(25i64) } else { Py::Int(30i64) })])?);
            }
            return Err(py::value_error(py::to_str(&Py::str("cannot read the expression"))));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) });
    v_value = py::call(&v_parse.clone(), &[Py::Int(0i64)])?;
    if ({ let t1156 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t1157 = py::len(&v_toks.clone())?; if py::ne(&t1156, &t1157) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("cannot read the expression (unexpected text at the end)"))));
    }
    return Ok(v_value.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__exact_outputs(mut v_v: Py) -> R {
    if (Py::Bool(matches!(v_v.clone(), Py::Frac(_)))).truthy() {
        return Ok(Py::dict(vec![(Py::str("value"), py::float(&v_v.clone())?), (Py::str("numerator"), py::float(&py::numerator(&v_v.clone())?)?), (Py::str("denominator"), py::float(&py::denominator(&v_v.clone())?)?)]));
    }
    return Ok(Py::dict(vec![(Py::str("value"), py::float(&v_v.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__exact_text_value(mut v_t: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_sign: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t1847 = v_t.clone(); Py::Bool({ let t1848 = Py::str("undefined"); if py::eq(&t1847, &t1848) { true } else { false } }) }).truthy() {
        return Ok(Py::None);
    }
    v_sign = (if (py::method(&v_t.clone(), "startswith", &[Py::str("-")])?).truthy() { py::neg(&Py::Int(1i64))? } else { Py::Int(1i64) });
    v_t = py::method(&v_t.clone(), "lstrip", &[Py::str("-")])?;
    v_m = ax_engine::pyre::module_call("fullmatch", &Py::str("(?:sqrt\\((\\d)\\)|(\\d))(?:/(\\d))?"), &[v_t.clone()], 0)?;
    v_v = py::truediv(&(if (py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?).truthy() { py::math1("sqrt", &py::int(&py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?)?)? } else { py::int(&py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?)? }), &(if (py::method(&v_m.clone(), "group", &[Py::Int(3i64)])?).truthy() { py::int(&py::method(&v_m.clone(), "group", &[Py::Int(3i64)])?)? } else { Py::Int(1i64) }))?;
    return Ok(py::mul(&v_sign.clone(), &v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__finite(mut v_kw: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t1937 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t1940 = (t1937).items()?;
        if t1940.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1940[0].clone();
        v_v = t1940[1].clone();
        if ({ let t1939 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t1939.truthy() { t1939 } else { { let t1938 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t1938.truthy() { t1938 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must be a finite number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fn(mut v_name: Py, mut v_v: Py) -> R {
    if ({ let t1140 = v_name.clone(); Py::Bool({ let t1141 = Py::str("sqrt"); if py::eq(&t1140, &t1141) { true } else { false } }) }).truthy() {
        return Ok(f__sqrt(v_v.clone())?);
    }
    if ({ let t1142 = v_name.clone(); Py::Bool({ let t1143 = Py::str("abs"); if py::eq(&t1142, &t1143) { true } else { false } }) }).truthy() {
        return Ok(py::abs(&v_v.clone())?);
    }
    if ({ let t1144 = v_name.clone(); Py::Bool({ let t1145 = Py::str("ln"); if py::eq(&t1144, &t1145) { true } else { false } }) }).truthy() {
        if ({ let t1146 = v_v.clone(); Py::Bool({ let t1147 = Py::Int(0i64); if py::le(&t1146, &t1147)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("logarithm of a number that is not positive"))));
        }
        return Ok(py::math1("log", &v_v.clone())?);
    }
    return Err(py::value_error(py::to_str(&Py::str(&["unknown function ".to_string(), py::format(&v_name.clone(), "")?].concat()))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fr(mut v_v: Py) -> R {
    return Ok(py::fraction(&[Py::str(&py::repr(&py::float(&v_v.clone())?))])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fval(mut v_expression: Py, mut v_x: Py) -> R {
    if ({ let t1058 = py::not(&Py::Bool(matches!(v_expression.clone(), Py::Str(_)))); if t1058.truthy() { t1058 } else { py::not(&py::method(&c__FEXPR()?, "match", &[py::method(&f__clean(v_expression.clone())?, "lower", &[])?])?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the function must be written with numbers, x, + - * / ^, parentheses, sqrt, abs or ln"))));
    }
    if (py::not(&Py::Bool(matches!(v_x.clone(), Py::Frac(_))))).truthy() {
        let _ = f__finite(Py::dict(vec![(Py::str("x"), v_x.clone())]))?;
    }
    return Ok(f__eval_pratt(v_expression.clone(), Py::dict(vec![(Py::str("x"), v_x.clone())]), Py::str("x"), Py::Bool(false))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gauss(mut v_rows: Py) -> R {
    let mut v_col: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_piv: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_n = py::len(&v_rows.clone())?;
    v_m = { let mut out = vec![]; for t818 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t818;
out.push({ let mut out = vec![]; for t819 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t819;
out.push(f__fr(v_v.clone())?); } Py::list(out) }); } Py::list(out) };
    for t820 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_col = t820;
        v_piv = (match ('g821: { let mut out = vec![]; for t822 in py::range_iter(&[(v_col.clone()).i()?, (v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t822;
if ({ let t823 = py::getitem(&py::getitem(&v_m.clone(), &v_r.clone())?, &v_col.clone())?; Py::Bool({ let t824 = Py::Int(0i64); if py::ne(&t823, &t824) { true } else { false } }) }).truthy() { out.push(v_r.clone()); break 'g821 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
        if ({ let t825 = v_piv.clone(); Py::Bool({ let t826 = Py::None; if py::is(&t825, &t826) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the system has no unique solution (its determinant is 0)"))));
        }
        let t827 = (Py::tuple(vec![py::getitem(&v_m.clone(), &v_piv.clone())?, py::getitem(&v_m.clone(), &v_col.clone())?])).items()?;
        if t827.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_m.clone(), &v_col.clone(), t827[0].clone())?;
        py::setitem(&v_m.clone(), &v_piv.clone(), t827[1].clone())?;
        for t828 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_r = t828;
            if ({ let t833 = { let t829 = v_r.clone(); Py::Bool({ let t830 = v_col.clone(); if py::ne(&t829, &t830) { true } else { false } }) }; if !t833.truthy() { t833 } else { { let t831 = py::getitem(&py::getitem(&v_m.clone(), &v_r.clone())?, &v_col.clone())?; Py::Bool({ let t832 = Py::Int(0i64); if py::ne(&t831, &t832) { true } else { false } }) } } }).truthy() {
                v_f = py::truediv(&py::getitem(&py::getitem(&v_m.clone(), &v_r.clone())?, &v_col.clone())?, &py::getitem(&py::getitem(&v_m.clone(), &v_col.clone())?, &v_col.clone())?)?;
                py::setitem(&v_m.clone(), &v_r.clone(), { let mut out = vec![]; for t834 in py::iter(&(py::zip(&[py::getitem(&v_m.clone(), &v_r.clone())?, py::getitem(&v_m.clone(), &v_col.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t835 = (t834).items()?;
if t835.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t835[0].clone();
v_b = t835[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_f.clone(), &v_b.clone())?)?); } Py::list(out) })?;
            }
        }
    }
    return Ok({ let mut out = vec![]; for t836 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t836;
out.push(py::truediv(&py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_n.clone())?, &py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_i.clone())?)?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__horner(mut v_co: Py, mut v_x: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = py::fraction(&[Py::Int(0i64)])?;
    for t1492 in py::iter(&(v_co.clone()))? {
        v_c = t1492;
        v_v = py::add(&py::mul(&v_v.clone(), &v_x.clone())?, &v_c.clone())?;
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hyp_point(mut v_i: Py, mut v_t: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let t681 = (Py::tuple(vec![py::math1("sqrt", &py::getitem(&v_i.clone(), &Py::str("A"))?)?, py::math1("sqrt", &py::getitem(&v_i.clone(), &Py::str("B"))?)?])).items()?;
    if t681.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t681[0].clone();
    v_b = t681[1].clone();
    if ({ let t682 = py::getitem(&v_i.clone(), &Py::str("orientation"))?; Py::Bool({ let t683 = Py::str("horizontal"); if py::eq(&t682, &t683) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![py::add(&py::getitem(&v_i.clone(), &Py::str("h"))?, &py::mul(&v_a.clone(), &py::math1("cosh", &v_t.clone())?)?)?, py::add(&py::getitem(&v_i.clone(), &Py::str("k"))?, &py::mul(&v_b.clone(), &py::math1("sinh", &v_t.clone())?)?)?]));
    }
    return Ok(Py::tuple(vec![py::add(&py::getitem(&v_i.clone(), &Py::str("h"))?, &py::mul(&v_b.clone(), &py::math1("sinh", &v_t.clone())?)?)?, py::add(&py::getitem(&v_i.clone(), &Py::str("k"))?, &py::mul(&v_a.clone(), &py::math1("cosh", &v_t.clone())?)?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ineq_text(mut v_lo: Py, mut v_hi: Py, mut v_rel: Py, mut v_between: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_gt: Py = Py::None;
    let mut v_lt: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_f = { let v_between = v_between.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_rel = v_rel.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::str(&[py::format(&v_v.clone(), ".10g")?].concat()));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    if (v_between.clone()).truthy() {
        v_r = (if ({ let t1275 = v_rel.clone(); Py::Bool({ let t1276 = Py::tuple(vec![Py::str("<"), Py::str(">")]); if py::contains(&t1276, &t1275)? { true } else { false } }) }).truthy() { Py::str("<") } else { Py::str("<=") });
        return Ok(Py::str(&[py::format(&py::call(&v_f.clone(), &[v_lo.clone()])?, "")?, " ".to_string(), py::format(&v_r.clone(), "")?, " x ".to_string(), py::format(&v_r.clone(), "")?, " ".to_string(), py::format(&py::call(&v_f.clone(), &[v_hi.clone()])?, "")?].concat()));
    }
    let t1279 = ((if ({ let t1277 = v_rel.clone(); Py::Bool({ let t1278 = Py::tuple(vec![Py::str("<"), Py::str(">")]); if py::contains(&t1278, &t1277)? { true } else { false } }) }).truthy() { Py::tuple(vec![Py::str("<"), Py::str(">")]) } else { Py::tuple(vec![Py::str("<="), Py::str(">=")]) })).items()?;
    if t1279.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lt = t1279[0].clone();
    v_gt = t1279[1].clone();
    return Ok(Py::str(&["x ".to_string(), py::format(&v_lt.clone(), "")?, " ".to_string(), py::format(&py::call(&v_f.clone(), &[v_lo.clone()])?, "")?, " or x ".to_string(), py::format(&v_gt.clone(), "")?, " ".to_string(), py::format(&py::call(&v_f.clone(), &[v_hi.clone()])?, "")?].concat()));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__poly_roots(mut v_coeffs: Py) -> R {
    let mut v_name: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t1312 in py::iter(&(py::zip(&[c__LETTERS()?, v_coeffs.clone()])?))? {
        let t1314 = (t1312).items()?;
        if t1314.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_name = t1314[0].clone();
        v_v = t1314[1].clone();
        if (py::not(&{ let t1313 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1313.truthy() { t1313 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite real number".to_string()].concat()))));
        }
    }
    if ({ let t1315 = py::getitem(&v_coeffs.clone(), &Py::Int(0i64))?; Py::Bool({ let t1316 = Py::Int(0i64); if py::eq(&t1315, &t1316) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["a must not be 0: with a = 0 the equation has a lower degree".to_string()].concat()))));
    }
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_roots = ax_engine::pymp::call("polyroots", &[{ let mut out = vec![]; for t1317 in py::iter(&(v_coeffs.clone()))? { let mut v_c: Py; 
v_c = t1317;
out.push(ax_engine::pymp::call("mpf", &[Py::str(&py::to_str(&v_c.clone()))], &[])?); } Py::list(out) }], &[(String::from("maxsteps"), Py::Int(200i64)), (String::from("extraprec"), Py::Int(60i64))])?;
    }
    return Ok(f__canonical_roots({ let mut out = vec![]; for t1324 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t1324;
out.push(py::complex(&[v_r.clone()])?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pow(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_r: Py = Py::None;
    if ({ let t1088 = Py::Bool(matches!(v_b.clone(), Py::Frac(_))); if !t1088.truthy() { t1088 } else { { let t1086 = py::denominator(&v_b.clone())?; Py::Bool({ let t1087 = Py::Int(1i64); if py::eq(&t1086, &t1087) { true } else { false } }) } } }).truthy() {
        if ({ let t1089 = py::abs(&v_b.clone())?; Py::Bool({ let t1090 = Py::Int(64i64); if py::gt(&t1089, &t1090)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("exponents are limited to 64 in size"))));
        }
        if ({ let t1095 = { let t1091 = v_a.clone(); Py::Bool({ let t1092 = Py::Int(0i64); if py::eq(&t1091, &t1092) { true } else { false } }) }; if !t1095.truthy() { t1095 } else { { let t1093 = v_b.clone(); Py::Bool({ let t1094 = Py::Int(0i64); if py::lt(&t1093, &t1094)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("division by zero (zero to a negative power)"))));
        }
        return Ok((if (Py::Bool(matches!(v_a.clone(), Py::Frac(_)))).truthy() { f__big(py::pow(&v_a.clone(), &py::int(&v_b.clone())?)?)? } else { f__big(py::pow(&py::float(&v_a.clone())?, &py::int(&v_b.clone())?)?)? }));
    }
    if ({ let t1108 = Py::Bool(matches!(v_b.clone(), Py::Frac(_))); if !t1108.truthy() { t1108 } else { { let t1107 = { let t1103 = py::denominator(&v_b.clone())?; Py::Bool({ let t1104 = Py::Int(2i64); if py::eq(&t1103, &t1104) { true } else { false } }) }; if !t1107.truthy() { t1107 } else { { let t1105 = v_a.clone(); Py::Bool({ let t1106 = Py::Int(0i64); if py::ge(&t1105, &t1106)? { true } else { false } }) } } } } }).truthy() {
        v_r = f__sqrt((if (Py::Bool(matches!(v_a.clone(), Py::Frac(_)))).truthy() { v_a.clone() } else { py::fraction(&[v_a.clone()])? }))?;
        return Ok(f__pow(v_r.clone(), py::fraction(&[py::numerator(&v_b.clone())?])?)?);
    }
    if ({ let t1117 = v_a.clone(); Py::Bool({ let t1118 = Py::Int(0i64); if py::le(&t1117, &t1118)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a non-integer power of a number that is not positive"))));
    }
    return Ok(f__big(py::pow(&py::float(&v_a.clone())?, &py::float(&v_b.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__prep_expr(mut v_text: Py, mut v_var: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = py::method(&py::method(&py::method(&py::method(&py::method(&f__clean(v_text.clone())?, "lower", &[])?, "replace", &[Py::str("^"), Py::str("**")])?, "replace", &[Py::str("["), Py::str("(")])?, "replace", &[Py::str("]"), Py::str(")")])?, "replace", &[Py::str(" divided by "), Py::str("/")])?;
    v_s = ax_engine::pyre::module_call("sub", &Py::str("\\bsqrt\\s*(\\d+(?:\\.\\d+)?)"), &[Py::str("sqrt(\\1)"), v_s.clone(), Py::Int(0)], 0)?;
    v_s = ax_engine::pyre::module_call("sub", &Py::str("(\\d)\\s*(?=[a-z(])"), &[Py::str("\\1*"), v_s.clone(), Py::Int(0)], 0)?;
    v_s = ax_engine::pyre::module_call("sub", &Py::str("\\)\\s*(?=[a-z0-9(])"), &[Py::str(")*"), v_s.clone(), Py::Int(0)], 0)?;
    if (v_var.clone()).truthy() {
        v_s = ax_engine::pyre::module_call("sub", &Py::str(&["\\b".to_string(), py::format(&v_var.clone(), "")?, "\\s*(?=\\()".to_string()].concat()), &[Py::str(&[py::format(&v_var.clone(), "")?, "*".to_string()].concat()), v_s.clone(), Py::Int(0)], 0)?;
    }
    return Ok(v_s.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__prop_terms(mut v_p: Py, mut v_q: Py, mut v_r: Py, mut v_position: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_t = Py::list(vec![v_p.clone(), v_q.clone(), v_r.clone()]);
    v_k = py::method(&c__POSITIONS()?, "index", &[v_position.clone()])?;
    return Ok(py::add(&py::add(&py::slice(&v_t.clone(), None, Some((v_k.clone()).i()?), None)?, &Py::list(vec![Py::None]))?, &py::slice(&v_t.clone(), Some((v_k.clone()).i()?), None, None)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_x: Py, mut v_y: Py) -> R {
    let t1922 = (Py::tuple(vec![py::complex(&[v_x.clone()])?, py::complex(&[v_y.clone()])?])).items()?;
    if t1922.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1922[0].clone();
    v_y = t1922[1].clone();
    return Ok(py::truediv(&py::abs(&py::sub(&v_x.clone(), &v_y.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_x.clone())?, py::abs(&v_y.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__roots_out(mut v_roots: Py) -> R {
    return Ok({ let mut out = vec![]; for t1310 in py::iter(&(py::enumerate(&v_roots.clone(), 0)?))? { let mut v_i: Py; let mut v_r: Py; 
let t1311 = (t1310).items()?;
if t1311.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1311[0].clone();
v_r = t1311[1].clone();
out.push((Py::str(&["x".to_string(), py::format(&py::add(&v_i.clone(), &Py::Int(1i64))?, "")?].concat()), (if (Py::Bool(matches!(v_r.clone(), Py::Complex(..)))).truthy() { v_r.clone() } else { py::float(&v_r.clone())? }))); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__round_half_up(mut v_q: Py, mut v_n: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_scale: Py = Py::None;
    v_scale = py::pow(&py::fraction(&[Py::Int(10i64)])?, &v_n.clone())?;
    v_m = py::mul(&py::abs(&v_q.clone())?, &v_scale.clone())?;
    v_k = py::math1("floor", &py::add(&v_m.clone(), &py::fraction(&[Py::Int(1i64), Py::Int(2i64)])?)?)?;
    return Ok(py::truediv(&(if ({ let t1623 = v_q.clone(); Py::Bool({ let t1624 = Py::Int(0i64); if py::ge(&t1623, &t1624)? { true } else { false } }) }).truthy() { v_k.clone() } else { py::neg(&v_k.clone())? }), &v_scale.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sig_exponent(mut v_q: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    v_a = py::abs(&v_q.clone())?;
    v_e = py::sub(&py::len(&Py::str(&py::to_str(&py::numerator(&v_a.clone())?)))?, &py::len(&Py::str(&py::to_str(&py::denominator(&v_a.clone())?)))?)?;
    while ({ let t1646 = py::pow(&py::fraction(&[Py::Int(10i64)])?, &v_e.clone())?; Py::Bool({ let t1647 = v_a.clone(); if py::gt(&t1646, &t1647)? { true } else { false } }) }).truthy() {
        v_e = py::sub(&v_e, &Py::Int(1i64))?;
    }
    while ({ let t1648 = py::pow(&py::fraction(&[Py::Int(10i64)])?, &py::add(&v_e.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1649 = v_a.clone(); if py::le(&t1648, &t1649)? { true } else { false } }) }).truthy() {
        v_e = py::iadd(&v_e, &Py::Int(1i64))?;
    }
    return Ok(v_e.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sign_root_expr(mut v_p: Py, mut v_q: Py, mut v_D: Py) -> R {
    let mut v_big: Py = Py::None;
    if ({ let t1373 = { let t1369 = v_q.clone(); Py::Bool({ let t1370 = Py::Int(0i64); if py::eq(&t1369, &t1370) { true } else { false } }) }; if t1373.truthy() { t1373 } else { { let t1371 = v_D.clone(); Py::Bool({ let t1372 = Py::Int(0i64); if py::eq(&t1371, &t1372) { true } else { false } }) } } }).truthy() {
        return Ok(py::sub(&{ let t1374 = v_p.clone(); Py::Bool({ let t1375 = Py::Int(0i64); if py::gt(&t1374, &t1375)? { true } else { false } }) }, &{ let t1376 = v_p.clone(); Py::Bool({ let t1377 = Py::Int(0i64); if py::lt(&t1376, &t1377)? { true } else { false } }) })?);
    }
    if ({ let t1382 = { let t1378 = v_p.clone(); Py::Bool({ let t1379 = Py::Int(0i64); if py::ge(&t1378, &t1379)? { true } else { false } }) }; if !t1382.truthy() { t1382 } else { { let t1380 = v_q.clone(); Py::Bool({ let t1381 = Py::Int(0i64); if py::gt(&t1380, &t1381)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Int(1i64));
    }
    if ({ let t1387 = { let t1383 = v_p.clone(); Py::Bool({ let t1384 = Py::Int(0i64); if py::le(&t1383, &t1384)? { true } else { false } }) }; if !t1387.truthy() { t1387 } else { { let t1385 = v_q.clone(); Py::Bool({ let t1386 = Py::Int(0i64); if py::lt(&t1385, &t1386)? { true } else { false } }) } } }).truthy() {
        return Ok(py::neg(&Py::Int(1i64))?);
    }
    v_big = py::sub(&py::mul(&v_p.clone(), &v_p.clone())?, &py::mul(&py::mul(&v_q.clone(), &v_q.clone())?, &v_D.clone())?)?;
    if ({ let t1388 = v_big.clone(); Py::Bool({ let t1389 = Py::Int(0i64); if py::eq(&t1388, &t1389) { true } else { false } }) }).truthy() {
        return Ok(Py::Int(0i64));
    }
    return Ok((if ({ let t1390 = v_big.clone(); Py::Bool({ let t1391 = Py::Int(0i64); if py::gt(&t1390, &t1391)? { true } else { false } }) }).truthy() { py::sub(&{ let t1392 = v_p.clone(); Py::Bool({ let t1393 = Py::Int(0i64); if py::gt(&t1392, &t1393)? { true } else { false } }) }, &{ let t1394 = v_p.clone(); Py::Bool({ let t1395 = Py::Int(0i64); if py::lt(&t1394, &t1395)? { true } else { false } }) })? } else { py::sub(&{ let t1396 = v_q.clone(); Py::Bool({ let t1397 = Py::Int(0i64); if py::gt(&t1396, &t1397)? { true } else { false } }) }, &{ let t1398 = v_q.clone(); Py::Bool({ let t1399 = Py::Int(0i64); if py::lt(&t1398, &t1399)? { true } else { false } }) })? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__signed(mut v_text_val: Py, mut v_sign: Py) -> R {
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t1827 = (v_text_val.clone()).items()?;
    if t1827.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t1827[0].clone();
    v_v = t1827[1].clone();
    if ({ let t1828 = v_v.clone(); Py::Bool({ let t1829 = Py::None; if py::is(&t1828, &t1829) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![v_t.clone(), Py::None]));
    }
    if ({ let t1834 = { let t1830 = v_sign.clone(); Py::Bool({ let t1831 = Py::Int(0i64); if py::lt(&t1830, &t1831)? { true } else { false } }) }; if !t1834.truthy() { t1834 } else { { let t1832 = v_v.clone(); Py::Bool({ let t1833 = Py::Int(0i64); if py::ne(&t1832, &t1833) { true } else { false } }) } } }).truthy() {
        return Ok(Py::tuple(vec![py::add(&Py::str("-"), &v_t.clone())?, py::neg(&v_v.clone())?]));
    }
    return Ok(Py::tuple(vec![v_t.clone(), py::add(&v_v.clone(), &Py::Float(0.0))?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sqrt(mut v_v: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    if ({ let t1109 = v_v.clone(); Py::Bool({ let t1110 = Py::Int(0i64); if py::lt(&t1109, &t1110)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("square root of a negative number"))));
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Frac(_)))).truthy() {
        let t1111 = (Py::tuple(vec![py::isqrt(&py::numerator(&v_v.clone())?)?, py::isqrt(&py::denominator(&v_v.clone())?)?])).items()?;
        if t1111.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t1111[0].clone();
        v_d = t1111[1].clone();
        if ({ let t1116 = { let t1112 = py::mul(&v_n.clone(), &v_n.clone())?; Py::Bool({ let t1113 = py::numerator(&v_v.clone())?; if py::eq(&t1112, &t1113) { true } else { false } }) }; if !t1116.truthy() { t1116 } else { { let t1114 = py::mul(&v_d.clone(), &v_d.clone())?; Py::Bool({ let t1115 = py::denominator(&v_v.clone())?; if py::eq(&t1114, &t1115) { true } else { false } }) } } }).truthy() {
            return Ok(py::fraction(&[v_n.clone(), v_d.clone()])?);
        }
    }
    return Ok(py::math1("sqrt", &v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__strip(mut v_co: Py) -> R {
    v_co = Py::list((v_co.clone()).items()?);
    while ({ let t1661 = { let t1657 = py::len(&v_co.clone())?; Py::Bool({ let t1658 = Py::Int(1i64); if py::gt(&t1657, &t1658)? { true } else { false } }) }; if !t1661.truthy() { t1661 } else { { let t1659 = py::getitem(&v_co.clone(), &Py::Int(0i64))?; Py::Bool({ let t1660 = Py::Int(0i64); if py::eq(&t1659, &t1660) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_co.clone(), "pop", &[Py::Int(0i64)])?;
    }
    return Ok(v_co.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tokens(mut v_s: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let t1060 = (Py::tuple(vec![Py::list(vec![]), Py::Int(0i64)])).items()?;
    if t1060.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_out = t1060[0].clone();
    v_pos = t1060[1].clone();
    v_s = py::method(&v_s.clone(), "strip", &[])?;
    while ({ let t1069 = v_pos.clone(); Py::Bool({ let t1070 = py::len(&v_s.clone())?; if py::lt(&t1069, &t1070)? { true } else { false } }) }).truthy() {
        v_m = py::method(&c__TOK()?, "match", &[v_s.clone(), v_pos.clone()])?;
        if ({ let t1063 = py::not(&v_m.clone()); if t1063.truthy() { t1063 } else { { let t1061 = py::method(&v_m.clone(), "end", &[])?; Py::Bool({ let t1062 = v_pos.clone(); if py::eq(&t1061, &t1062) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the expression at ".to_string(), py::format(&Py::str(&py::repr(&py::slice(&v_s.clone(), Some((v_pos.clone()).i()?), Some((py::add(&v_pos.clone(), &Py::Int(10i64))?).i()?), None)?)), "")?].concat()))));
        }
        let _ = py::method(&v_out.clone(), "append", &[(if (py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?).truthy() { Py::tuple(vec![Py::str("num"), py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?]) } else { (if (py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?).truthy() { Py::tuple(vec![Py::str("name"), py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?]) } else { Py::tuple(vec![Py::str("op"), py::method(&v_m.clone(), "group", &[Py::Int(3i64)])?]) }) })])?;
        v_pos = py::method(&v_m.clone(), "end", &[])?;
        while ({ let t1068 = { let t1064 = v_pos.clone(); Py::Bool({ let t1065 = py::len(&v_s.clone())?; if py::lt(&t1064, &t1065)? { true } else { false } }) }; if !t1068.truthy() { t1068 } else { { let t1066 = py::getitem(&v_s.clone(), &v_pos.clone())?; Py::Bool({ let t1067 = Py::str(" "); if py::eq(&t1066, &t1067) { true } else { false } }) } } }).truthy() {
            v_pos = py::iadd(&v_pos, &Py::Int(1i64))?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tri_check(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone())]))?;
    if ({ let t733 = { let t723 = py::minmax(&[v_a.clone(), v_b.clone(), v_c.clone()], false)?; Py::Bool({ let t724 = Py::Int(0i64); if py::le(&t723, &t724)? { true } else { false } }) }; if t733.truthy() { t733 } else { { let t732 = { let t725 = py::add(&v_a.clone(), &v_b.clone())?; Py::Bool({ let t726 = v_c.clone(); if py::le(&t725, &t726)? { true } else { false } }) }; if t732.truthy() { t732 } else { { let t731 = { let t727 = py::add(&v_a.clone(), &v_c.clone())?; Py::Bool({ let t728 = v_b.clone(); if py::le(&t727, &t728)? { true } else { false } }) }; if t731.truthy() { t731 } else { { let t729 = py::add(&v_b.clone(), &v_c.clone())?; Py::Bool({ let t730 = v_a.clone(); if py::le(&t729, &t730)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("these lengths do not form a triangle (each side must be shorter than the sum of the other two)"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__trig_f(mut v_func: Py) -> R {
    return Ok(py::getitem(&Py::dict(vec![(Py::str("sin"), py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("sin() takes one argument".into()))?; py::math1("sin", &x) })), (Py::str("cos"), py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("cos() takes one argument".into()))?; py::math1("cos", &x) })), (Py::str("tan"), py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("tan() takes one argument".into()))?; py::math1("tan", &x) }))]), &v_func.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_aas(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cosA: Py = Py::None;
    let t750 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_i.clone(), &Py::str("B"))?, py::getitem(&v_i.clone(), &Py::str("a"))?])).items()?;
    if t750.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t750[0].clone();
    v_B = t750[1].clone();
    v_a = t750[2].clone();
    let t751 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("b"))?, py::getitem(&v_o.clone(), &Py::str("c"))?, py::getitem(&v_o.clone(), &Py::str("C"))?])).items()?;
    if t751.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b = t751[0].clone();
    v_c = t751[1].clone();
    v_C = t751[2].clone();
    v_cosA = py::math1("cos", &py::math1("radians", &v_A.clone())?)?;
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::add(&v_A.clone(), &v_B.clone())?, &v_C.clone())?, &Py::Int(180i64))?)?, &Py::Int(180i64))?, &py::truediv(&py::abs(&py::sub(&py::mul(&v_a.clone(), &v_a.clone())?, &py::sub(&py::add(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_b.clone())?, &v_c.clone())?, &v_cosA.clone())?)?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&v_a.clone(), &v_a.clone())?, py::add(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&v_b.clone(), &f__deg_sin(v_A.clone())?)?, &py::mul(&v_a.clone(), &f__deg_sin(v_B.clone())?)?)?)?, &py::minmax(&[Py::Float(1.0), v_a.clone(), v_b.clone()], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_absineq(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t17 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])).items()?;
    if t17.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t17[0].clone();
    v_b = t17[1].clone();
    v_c = t17[2].clone();
    let t18 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("lower"))?, py::getitem(&v_o.clone(), &Py::str("upper"))?])).items()?;
    if t18.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t18[0].clone();
    v_hi = t18[1].clone();
    v_r = py::add(&py::truediv(&py::abs(&py::sub(&py::abs(&py::add(&py::mul(&v_a.clone(), &v_lo.clone())?, &v_b.clone())?)?, &v_c.clone())?)?, &py::minmax(&[Py::Float(1.0), v_c.clone()], true)?)?, &py::truediv(&py::abs(&py::sub(&py::abs(&py::add(&py::mul(&v_a.clone(), &v_hi.clone())?, &v_b.clone())?)?, &v_c.clone())?)?, &py::minmax(&[Py::Float(1.0), v_c.clone()], true)?)?)?;
    v_r = py::iadd(&v_r, &py::truediv(&py::abs(&py::add(&py::add(&py::mul(&v_a.clone(), &v_lo.clone())?, &v_b.clone())?, &py::add(&py::mul(&v_a.clone(), &v_hi.clone())?, &v_b.clone())?)?)?, &py::minmax(&[Py::Float(1.0), v_c.clone()], true)?)?)?;
    v_r = py::iadd(&v_r, &py::add(&{ let t19 = v_lo.clone(); Py::Bool({ let t20 = v_hi.clone(); if py::gt(&t19, &t20)? { true } else { false } }) }, &{ let t21 = py::getitem(&v_o.clone(), &Py::str("region"))?; Py::Bool({ let t22 = (if ({ let t23 = py::getitem(&v_i.clone(), &Py::str("rel"))?; Py::Bool({ let t24 = Py::tuple(vec![Py::str("<"), Py::str("<=")]); if py::contains(&t24, &t23)? { true } else { false } }) }).truthy() { Py::str("between") } else { Py::str("outside") }); if py::ne(&t21, &t22) { true } else { false } }) })?)?;
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_binom(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_co: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pas: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t39 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?])).items()?;
    if t39.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t39[0].clone();
    v_k = t39[1].clone();
    v_co = Py::list(vec![py::fraction(&[Py::Int(1i64)])?]);
    let t40 = (Py::tuple(vec![f__fr(py::getitem(&v_i.clone(), &Py::str("a"))?)?, f__fr(py::getitem(&v_i.clone(), &Py::str("b"))?)?])).items()?;
    if t40.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t40[0].clone();
    v_B = t40[1].clone();
    for t41 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t41;
        v_co = { let mut out = vec![]; for t42 in py::range_iter(&[(py::add(&py::len(&v_co.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t42;
out.push(py::add(&(if ({ let t43 = v_j.clone(); Py::Bool({ let t44 = py::len(&v_co.clone())?; if py::lt(&t43, &t44)? { true } else { false } }) }).truthy() { py::mul(&py::getitem(&v_co.clone(), &v_j.clone())?, &v_B.clone())? } else { Py::Int(0i64) }), &(if ({ let t45 = v_j.clone(); Py::Bool({ let t46 = Py::Int(1i64); if py::ge(&t45, &t46)? { true } else { false } }) }).truthy() { py::mul(&py::getitem(&v_co.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?, &v_A.clone())? } else { Py::Int(0i64) }))?); } Py::list(out) };
    }
    v_want = (if ({ let t47 = v_k.clone(); Py::Bool({ let t48 = py::len(&v_co.clone())?; if py::lt(&t47, &t48)? { true } else { false } }) }).truthy() { py::getitem(&v_co.clone(), &v_k.clone())? } else { py::fraction(&[Py::Int(0i64)])? });
    v_pas = Py::list(vec![Py::Int(1i64)]);
    for t49 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t49;
        v_pas = { let mut out = vec![]; for t50 in py::iter(&(py::zip(&[py::add(&Py::list(vec![Py::Int(0i64)]), &v_pas.clone())?, py::add(&v_pas.clone(), &Py::list(vec![Py::Int(0i64)]))?])?))? { let mut v_x: Py; let mut v_y: Py; 
let t51 = (t50).items()?;
if t51.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t51[0].clone();
v_y = t51[1].clone();
out.push(py::add(&v_x.clone(), &v_y.clone())?); } Py::list(out) };
    }
    return Ok(py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("coefficient"))?, py::float(&v_want.clone())?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("binomial"))?, (if ({ let t53 = v_k.clone(); Py::Bool({ let t54 = v_n.clone(); if py::le(&t53, &t54)? { true } else { false } }) }).truthy() { py::getitem(&v_pas.clone(), &v_k.clone())? } else { Py::Int(0i64) }))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_carith(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_op: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t77 = (Py::tuple(vec![py::complex(&[py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?])?, py::complex(&[py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("d"))?])?, py::complex(&[py::getitem(&v_o.clone(), &Py::str("re"))?, py::getitem(&v_o.clone(), &Py::str("im"))?])?])).items()?;
    if t77.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_z = t77[0].clone();
    v_w = t77[1].clone();
    v_r = t77[2].clone();
    v_op = py::getitem(&v_i.clone(), &Py::str("op"))?;
    if ({ let t78 = v_op.clone(); Py::Bool({ let t79 = Py::str("+"); if py::eq(&t78, &t79) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::abs(&py::sub(&v_r.clone(), &py::add(&v_z.clone(), &v_w.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_z.clone())?, py::abs(&v_w.clone())?], true)?)?);
    }
    if ({ let t80 = v_op.clone(); Py::Bool({ let t81 = Py::str("-"); if py::eq(&t80, &t81) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::abs(&py::sub(&v_r.clone(), &py::sub(&v_z.clone(), &v_w.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_z.clone())?, py::abs(&v_w.clone())?], true)?)?);
    }
    if ({ let t82 = v_op.clone(); Py::Bool({ let t83 = Py::str("*"); if py::eq(&t82, &t83) { true } else { false } }) }).truthy() {
        return Ok((if (v_w.clone()).truthy() { py::truediv(&py::abs(&py::sub(&py::truediv(&v_r.clone(), &v_w.clone())?, &v_z.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_z.clone())?], true)?)? } else { py::abs(&v_r.clone())? }));
    }
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&v_r.clone(), &v_w.clone())?, &v_z.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_z.clone())?, py::abs(&py::mul(&v_r.clone(), &v_w.clone())?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_comp(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_first: Py = Py::None;
    let mut v_inner: Py = Py::None;
    let mut v_second: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t239 = ((if ({ let t237 = py::getitem(&v_i.clone(), &Py::str("order"))?; Py::Bool({ let t238 = Py::str("f(g(x))"); if py::eq(&t237, &t238) { true } else { false } }) }).truthy() { Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("g"))?, py::getitem(&v_i.clone(), &Py::str("f"))?]) } else { Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("g"))?]) })).items()?;
    if t239.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_first = t239[0].clone();
    v_second = t239[1].clone();
    v_inner = f__eval_pratt(v_first.clone(), Py::dict(vec![(Py::str("x"), py::getitem(&v_i.clone(), &Py::str("x"))?)]), Py::str("x"), Py::Bool(false))?;
    v_v = f__eval_pratt(v_second.clone(), Py::dict(vec![(Py::str("x"), (if (Py::Bool(matches!(v_inner.clone(), Py::Frac(_)))).truthy() { v_inner.clone() } else { py::float(&v_inner.clone())? }))]), Py::str("x"), Py::Bool(false))?;
    return Ok(py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("inner"))?, py::float(&v_inner.clone())?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("value"))?, py::float(&v_v.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_d2r(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_r = py::add(&py::truediv(&py::abs(&py::sub(&py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("radians"))?, &Py::Int(180i64))?, &Py::Float(std::f64::consts::PI))?, &py::getitem(&v_i.clone(), &Py::str("deg"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("deg"))?)?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("pi_multiple"))?, &Py::Int(180i64))?, &py::getitem(&v_i.clone(), &Py::str("deg"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("deg"))?)?], true)?)?)?;
    if (py::getitem(&v_o.clone(), &Py::str("exact"))?).truthy() {
        v_m = ax_engine::pyre::module_call("fullmatch", &Py::str("(-?)(\\d*)pi(?:/(\\d+))?|0"), &[py::getitem(&v_o.clone(), &Py::str("exact"))?], 0)?;
        if (py::not(&v_m.clone())).truthy() {
            return Ok(py::add(&Py::Float(1.0), &v_r.clone())?);
        }
        v_v = (if ({ let t333 = py::getitem(&v_o.clone(), &Py::str("exact"))?; Py::Bool({ let t334 = Py::str("0"); if py::eq(&t333, &t334) { true } else { false } }) }).truthy() { py::fraction(&[Py::Int(0i64)])? } else { py::fraction(&[py::mul(&(if (py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?).truthy() { py::neg(&Py::Int(1i64))? } else { Py::Int(1i64) }), &py::int(&{ let t335 = py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?; if t335.truthy() { t335 } else { Py::Int(1i64) } })?)?, py::int(&{ let t336 = py::method(&v_m.clone(), "group", &[Py::Int(3i64)])?; if t336.truthy() { t336 } else { Py::Int(1i64) } })?])? });
        v_r = py::iadd(&v_r, &py::float(&py::abs(&py::sub(&py::mul(&v_v.clone(), &Py::Int(180i64))?, &f__fr(py::getitem(&v_i.clone(), &Py::str("deg"))?)?)?)?)?)?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_demoivre(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_mod: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_ri: Py = Py::None;
    let mut v_rr: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    let t310 = (Py::tuple(vec![f__fr(py::getitem(&v_i.clone(), &Py::str("a"))?)?, f__fr(py::getitem(&v_i.clone(), &Py::str("b"))?)?])).items()?;
    if t310.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t310[0].clone();
    v_B = t310[1].clone();
    if ({ let t311 = v_n.clone(); Py::Bool({ let t312 = Py::Int(0i64); if py::lt(&t311, &t312)? { true } else { false } }) }).truthy() {
        v_den = py::add(&py::mul(&v_A.clone(), &v_A.clone())?, &py::mul(&v_B.clone(), &v_B.clone())?)?;
        let t313 = (Py::tuple(vec![py::truediv(&v_A.clone(), &v_den.clone())?, py::truediv(&py::neg(&v_B.clone())?, &v_den.clone())?])).items()?;
        if t313.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_A = t313[0].clone();
        v_B = t313[1].clone();
    }
    let t314 = (Py::tuple(vec![py::fraction(&[Py::Int(1i64)])?, py::fraction(&[Py::Int(0i64)])?])).items()?;
    if t314.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rr = t314[0].clone();
    v_ri = t314[1].clone();
    for t315 in py::range_iter(&[(py::abs(&v_n.clone())?).i()?]) {
        v__ = t315;
        let t316 = (Py::tuple(vec![py::sub(&py::mul(&v_rr.clone(), &v_A.clone())?, &py::mul(&v_ri.clone(), &v_B.clone())?)?, py::add(&py::mul(&v_rr.clone(), &v_B.clone())?, &py::mul(&v_ri.clone(), &v_A.clone())?)?])).items()?;
        if t316.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_rr = t316[0].clone();
        v_ri = t316[1].clone();
    }
    v_r = py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("re"))?, &py::float(&v_rr.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::float(&v_rr.clone())?)?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("im"))?, &py::float(&v_ri.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::float(&v_ri.clone())?)?], true)?)?)?;
    v_mod = py::hypot(&[py::float(&v_rr.clone())?, py::float(&v_ri.clone())?])?;
    v_r = py::iadd(&v_r, &f__rel(py::getitem(&v_o.clone(), &Py::str("modulus"))?, v_mod.clone())?)?;
    if (v_mod.clone()).truthy() {
        v_z = py::mul(&py::getitem(&v_o.clone(), &Py::str("modulus"))?, &py::complex(&[py::math1("cos", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("argument_deg"))?)?)?, py::math1("sin", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("argument_deg"))?)?)?])?)?;
        v_r = py::iadd(&v_r, &py::truediv(&py::abs(&py::sub(&v_z.clone(), &py::complex(&[py::getitem(&v_o.clone(), &Py::str("re"))?, py::getitem(&v_o.clone(), &Py::str("im"))?])?)?)?, &py::minmax(&[Py::Float(1.0), v_mod.clone()], true)?)?)?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_dist_line(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_fx: Py = Py::None;
    let mut v_fy: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_y0: Py = Py::None;
    let t346 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("x0"))?, py::getitem(&v_i.clone(), &Py::str("y0"))?])).items()?;
    if t346.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_a = t346[0].clone();
    v_b = t346[1].clone();
    v_c = t346[2].clone();
    v_x0 = t346[3].clone();
    v_y0 = t346[4].clone();
    v_n = py::hypot(&[v_a.clone(), v_b.clone()])?;
    v_t = py::truediv(&py::neg(&py::add(&py::add(&py::mul(&v_a.clone(), &v_x0.clone())?, &py::mul(&v_b.clone(), &v_y0.clone())?)?, &v_c.clone())?)?, &py::mul(&v_n.clone(), &v_n.clone())?)?;
    let t347 = (Py::tuple(vec![py::add(&v_x0.clone(), &py::mul(&v_a.clone(), &v_t.clone())?)?, py::add(&v_y0.clone(), &py::mul(&v_b.clone(), &v_t.clone())?)?])).items()?;
    if t347.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t347[0].clone();
    v_fy = t347[1].clone();
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("distance"))?, &py::hypot(&[py::sub(&v_fx.clone(), &v_x0.clone())?, py::sub(&v_fy.clone(), &v_y0.clone())?])?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_o.clone(), &Py::str("distance"))?], true)?)?, &py::truediv(&py::abs(&py::add(&py::add(&py::mul(&v_a.clone(), &v_fx.clone())?, &py::mul(&v_b.clone(), &v_fy.clone())?)?, &v_c.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_c.clone())?, py::abs(&py::mul(&v_a.clone(), &v_fx.clone())?)?, py::abs(&py::mul(&v_b.clone(), &v_fy.clone())?)?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_domain(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_side: Py = Py::None;
    let mut v_text: Py = Py::None;
    let t383 = (f__domain_parts(py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("d"))?, Py::Bool((py::method(&v_i.clone(), "get", &[Py::str("root_in_denominator")])?).truthy()))?).items()?;
    if t383.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_B = t383[0].clone();
    v_side = t383[1].clone();
    v_E = t383[2].clone();
    v_text = t383[3].clone();
    v_r = py::truediv(&py::abs(&py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_o.clone(), &Py::str("boundary"))?)?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("b"))?)?, py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_o.clone(), &Py::str("boundary"))?)?)?], true)?)?;
    v_r = py::iadd(&v_r, &py::add(&{ let t384 = py::getitem(&v_o.clone(), &Py::str("side"))?; Py::Bool({ let t385 = v_side.clone(); if py::ne(&t384, &t385) { true } else { false } }) }, &{ let t386 = py::getitem(&v_o.clone(), &Py::str("domain"))?; Py::Bool({ let t387 = v_text.clone(); if py::ne(&t386, &t387) { true } else { false } }) })?)?;
    if ({ let t388 = { let t390 = v_E.clone(); Py::Bool({ let t391 = Py::None; if py::is(&t390, &t391) { true } else { false } }) }; Py::Bool({ let t389 = { let t392 = py::getitem(&v_o.clone(), &Py::str("excluded"))?; Py::Bool({ let t393 = Py::None; if py::is(&t392, &t393) { true } else { false } }) }; if py::ne(&t388, &t389) { true } else { false } }) }).truthy() {
        return Ok(py::add(&v_r.clone(), &Py::Float(1.0))?);
    }
    if ({ let t394 = v_E.clone(); Py::Bool({ let t395 = Py::None; if !py::is(&t394, &t395) { true } else { false } }) }).truthy() {
        v_r = py::iadd(&v_r, &py::truediv(&py::abs(&py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("c"))?, &py::getitem(&v_o.clone(), &Py::str("excluded"))?)?, &py::getitem(&v_i.clone(), &Py::str("d"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("d"))?)?], true)?)?)?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ellipse(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_px: Py = Py::None;
    let mut v_py: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t420 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("h"))?, py::getitem(&v_i.clone(), &Py::str("k"))?, py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_i.clone(), &Py::str("B"))?])).items()?;
    if t420.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_h = t420[0].clone();
    v_k = t420[1].clone();
    v_A = t420[2].clone();
    v_B = t420[3].clone();
    v_r = py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("c"))?, &Py::Int(2i64))?, &py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("a"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("b"))?, &Py::Int(2i64))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::pow(&py::getitem(&v_o.clone(), &Py::str("a"))?, &Py::Int(2i64))?], true)?)?, &f__rel(py::pow(&py::getitem(&v_o.clone(), &Py::str("a"))?, &Py::Int(2i64))?, py::minmax(&[v_A.clone(), v_B.clone()], true)?)?)?, &f__rel(py::pow(&py::getitem(&v_o.clone(), &Py::str("b"))?, &Py::Int(2i64))?, py::minmax(&[v_A.clone(), v_B.clone()], false)?)?)?;
    v_r = py::iadd(&v_r, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("eccentricity"))?, &py::getitem(&v_o.clone(), &Py::str("a"))?)?, py::getitem(&v_o.clone(), &Py::str("c"))?)?)?;
    for t422 in py::iter(&(Py::tuple(vec![Py::Float(0.3), Py::Float(1.7), Py::Float(4.0)])))? {
        v_t = t422;
        let t423 = (Py::tuple(vec![py::add(&v_h.clone(), &py::mul(&py::math1("sqrt", &v_A.clone())?, &py::math1("cos", &v_t.clone())?)?)?, py::add(&v_k.clone(), &py::mul(&py::math1("sqrt", &v_B.clone())?, &py::math1("sin", &v_t.clone())?)?)?])).items()?;
        if t423.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_px = t423[0].clone();
        v_py = t423[1].clone();
        v_s = py::add(&py::hypot(&[py::sub(&v_px.clone(), &py::getitem(&v_o.clone(), &Py::str("f1x"))?)?, py::sub(&v_py.clone(), &py::getitem(&v_o.clone(), &Py::str("f1y"))?)?])?, &py::hypot(&[py::sub(&v_px.clone(), &py::getitem(&v_o.clone(), &Py::str("f2x"))?)?, py::sub(&v_py.clone(), &py::getitem(&v_o.clone(), &Py::str("f2y"))?)?])?)?;
        v_r = py::iadd(&v_r, &py::truediv(&py::abs(&py::sub(&v_s.clone(), &py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("a"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_o.clone(), &Py::str("a"))?], true)?)?)?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_factor(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t573 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("g"))?, py::getitem(&v_o.clone(), &Py::str("p"))?, py::getitem(&v_o.clone(), &Py::str("q"))?, py::getitem(&v_o.clone(), &Py::str("r"))?, py::getitem(&v_o.clone(), &Py::str("s"))?])).items()?;
    if t573.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_g = t573[0].clone();
    v_p = t573[1].clone();
    v_q = t573[2].clone();
    v_r = t573[3].clone();
    v_s = t573[4].clone();
    if ({ let t582 = Py::Bool(({ let mut out = vec![]; for t574 in py::iter(&(Py::tuple(vec![v_g.clone(), v_p.clone(), v_q.clone(), v_r.clone(), v_s.clone()])))? { let mut v_v: Py; 
v_v = t574;
out.push({ let t575 = v_v.clone(); Py::Bool({ let t576 = py::int(&v_v.clone())?; if py::ne(&t575, &t576) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t582.truthy() { t582 } else { { let t581 = { let t577 = v_p.clone(); Py::Bool({ let t578 = Py::Int(0i64); if py::le(&t577, &t578)? { true } else { false } }) }; if t581.truthy() { t581 } else { { let t579 = v_r.clone(); Py::Bool({ let t580 = Py::Int(0i64); if py::le(&t579, &t580)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t590 = { let t583 = py::gcd(&py::int(&v_p.clone())?, &py::int(&v_q.clone())?)?; Py::Bool({ let t584 = Py::Int(1i64); if py::ne(&t583, &t584) { true } else { false } }) }; if t590.truthy() { t590 } else { { let t589 = { let t585 = py::gcd(&py::int(&v_r.clone())?, &py::int(&v_s.clone())?)?; Py::Bool({ let t586 = Py::Int(1i64); if py::ne(&t585, &t586) { true } else { false } }) }; if t589.truthy() { t589 } else { { let t587 = py::fraction(&[py::int(&py::neg(&v_q.clone())?)?, py::int(&v_p.clone())?])?; Py::Bool({ let t588 = py::fraction(&[py::int(&py::neg(&v_s.clone())?)?, py::int(&v_r.clone())?])?; if py::gt(&t587, &t588)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_want = Py::tuple(vec![py::mul(&py::mul(&v_g.clone(), &v_p.clone())?, &v_r.clone())?, py::mul(&v_g.clone(), &py::add(&py::mul(&v_p.clone(), &v_s.clone())?, &py::mul(&v_q.clone(), &v_r.clone())?)?)?, py::mul(&py::mul(&v_g.clone(), &v_q.clone())?, &v_s.clone())?]);
    return Ok(py::minmax(&[{ let mut out = vec![]; for t591 in py::iter(&(py::zip(&[v_want.clone(), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])])?))? { let mut v_u: Py; let mut v_v: Py; 
let t592 = (t591).items()?;
if t592.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t592[0].clone();
v_v = t592[1].clone();
out.push(py::truediv(&py::abs(&py::sub(&v_u.clone(), &v_v.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_v.clone())?], true)?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_feval(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = f__eval_pratt(py::getitem(&v_i.clone(), &Py::str("expression"))?, Py::dict(vec![(Py::str("x"), py::getitem(&v_i.clone(), &Py::str("x"))?)]), Py::str("x"), Py::Bool(false))?;
    v_r = f__rel(py::getitem(&v_o.clone(), &Py::str("value"))?, py::float(&v_v.clone())?)?;
    if ({ let t530 = Py::Bool(matches!(v_v.clone(), Py::Frac(_))); if !t530.truthy() { t530 } else { py::method(&v_o.clone(), "get", &[Py::str("denominator")])? } }).truthy() {
        v_r = py::iadd(&v_r, &py::float(&py::truediv(&py::abs(&py::sub(&py::fraction(&[py::int(&py::getitem(&v_o.clone(), &Py::str("numerator"))?)?, py::int(&py::getitem(&v_o.clone(), &Py::str("denominator"))?)?])?, &v_v.clone())?)?, &py::minmax(&[Py::Int(1i64), py::abs(&v_v.clone())?], true)?)?)?)?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_hyperbola(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_px: Py = Py::None;
    let mut v_py: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_r = py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("c"))?, &Py::Int(2i64))?, &py::getitem(&v_i.clone(), &Py::str("A"))?)?, &py::getitem(&v_i.clone(), &Py::str("B"))?)?)?, &py::minmax(&[Py::Float(1.0), py::pow(&py::getitem(&v_o.clone(), &Py::str("c"))?, &Py::Int(2i64))?], true)?)?, &f__rel(py::pow(&py::getitem(&v_o.clone(), &Py::str("a"))?, &Py::Int(2i64))?, py::getitem(&v_i.clone(), &Py::str("A"))?)?)?, &f__rel(py::pow(&py::getitem(&v_o.clone(), &Py::str("b"))?, &Py::Int(2i64))?, py::getitem(&v_i.clone(), &Py::str("B"))?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("eccentricity"))?, &py::getitem(&v_o.clone(), &Py::str("a"))?)?, py::getitem(&v_o.clone(), &Py::str("c"))?)?)?;
    v_r = py::iadd(&v_r, &f__rel(py::getitem(&v_o.clone(), &Py::str("asymptote_slope"))?, (if ({ let t678 = py::getitem(&v_i.clone(), &Py::str("orientation"))?; Py::Bool({ let t679 = Py::str("horizontal"); if py::eq(&t678, &t679) { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&v_o.clone(), &Py::str("b"))?, &py::getitem(&v_o.clone(), &Py::str("a"))?)? } else { py::truediv(&py::getitem(&v_o.clone(), &Py::str("a"))?, &py::getitem(&v_o.clone(), &Py::str("b"))?)? }))?)?;
    for t680 in py::iter(&(Py::tuple(vec![Py::Float(0.4), Py::Float(1.1), py::neg(&Py::Float(2.0))?])))? {
        v_t = t680;
        let t684 = (f__hyp_point(v_i.clone(), v_t.clone())?).items()?;
        if t684.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_px = t684[0].clone();
        v_py = t684[1].clone();
        v_d = py::abs(&py::sub(&py::hypot(&[py::sub(&v_px.clone(), &py::getitem(&v_o.clone(), &Py::str("f1x"))?)?, py::sub(&v_py.clone(), &py::getitem(&v_o.clone(), &Py::str("f1y"))?)?])?, &py::hypot(&[py::sub(&v_px.clone(), &py::getitem(&v_o.clone(), &Py::str("f2x"))?)?, py::sub(&v_py.clone(), &py::getitem(&v_o.clone(), &Py::str("f2y"))?)?])?)?)?;
        v_r = py::iadd(&v_r, &py::truediv(&py::abs(&py::sub(&v_d.clone(), &py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("a"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_o.clone(), &Py::str("a"))?], true)?)?)?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_inv(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t700 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?])).items()?;
    if t700.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t700[0].clone();
    v_b = t700[1].clone();
    v_r = py::add(&py::abs(&py::sub(&py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("inv_slope"))?)?, &Py::Int(1i64))?)?, &py::truediv(&py::abs(&py::add(&py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("inv_intercept"))?)?, &v_b.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_b.clone())?], true)?)?)?;
    if ({ let t701 = py::method(&v_i.clone(), "get", &[Py::str("y")])?; Py::Bool({ let t702 = Py::None; if !py::is(&t701, &t702) { true } else { false } }) }).truthy() {
        v_r = py::iadd(&v_r, &(if ({ let t703 = Py::str("value"); Py::Bool({ let t704 = v_o.clone(); if py::contains(&t704, &t703)? { true } else { false } }) }).truthy() { py::truediv(&py::abs(&py::sub(&py::add(&py::mul(&v_a.clone(), &py::method(&v_o.clone(), "get", &[Py::str("value"), py::float(&Py::str("inf"))?])?)?, &v_b.clone())?, &py::getitem(&v_i.clone(), &Py::str("y"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("y"))?)?, py::abs(&v_b.clone())?], true)?)? } else { Py::Float(1.0) }))?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_logeq(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_arg: Py = Py::None;
    v_arg = py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("c"))?, &py::getitem(&v_o.clone(), &Py::str("x"))?)?, &py::getitem(&v_i.clone(), &Py::str("d"))?)?;
    if ({ let t853 = v_arg.clone(); Py::Bool({ let t854 = Py::Int(0i64); if py::le(&t853, &t854)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::truediv(&py::abs(&py::sub(&py::truediv(&py::math1("log", &v_arg.clone())?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("k"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("k"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_longdiv(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_a_i: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_b_i: Py = Py::None;
    let mut v_back: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pad: Py = Py::None;
    let mut v_prod: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let t1207 = (Py::tuple(vec![f__strip({ let mut out = vec![]; for t1205 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("dividend"))?))? { let mut v_c: Py; 
v_c = t1205;
out.push(py::float(&v_c.clone())?); } Py::list(out) })?, f__strip({ let mut out = vec![]; for t1206 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("divisor"))?))? { let mut v_c: Py; 
v_c = t1206;
out.push(py::float(&v_c.clone())?); } Py::list(out) })?])).items()?;
    if t1207.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1207[0].clone();
    v_d = t1207[1].clone();
    let t1208 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("quotient"))?, py::getitem(&v_o.clone(), &Py::str("remainder"))?])).items()?;
    if t1208.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t1208[0].clone();
    v_r = t1208[1].clone();
    if ({ let t1211 = { let t1209 = py::len(&v_r.clone())?; Py::Bool({ let t1210 = py::len(&v_d.clone())?; if py::ge(&t1209, &t1210)? { true } else { false } }) }; if !t1211.truthy() { t1211 } else { Py::Bool((py::slice(&v_r.clone(), None, Some((py::add(&py::sub(&py::len(&v_r.clone())?, &py::len(&v_d.clone())?)?, &Py::Int(1i64))?).i()?), None)?).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_prod = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&py::add(&py::len(&v_q.clone())?, &py::len(&v_d.clone())?)?, &Py::Int(1i64))?)?;
    for t1212 in py::iter(&(py::enumerate(&v_q.clone(), 0)?))? {
        let t1217 = (t1212).items()?;
        if t1217.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a_i = t1217[0].clone();
        v_a = t1217[1].clone();
        for t1213 in py::iter(&(py::enumerate(&v_d.clone(), 0)?))? {
            let t1216 = (t1213).items()?;
            if t1216.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_b_i = t1216[0].clone();
            v_b = t1216[1].clone();
            let t1214 = v_prod.clone(); let t1215 = py::add(&v_a_i.clone(), &v_b_i.clone())?;
            py::setitem(&t1214, &t1215, py::iadd(&py::getitem(&t1214, &t1215)?, &py::mul(&v_a.clone(), &v_b.clone())?)?)?;
        }
    }
    v_n = py::minmax(&[py::len(&v_prod.clone())?, py::len(&v_r.clone())?, py::len(&v_p.clone())?], true)?;
    v_pad = { let v_a = v_a.clone(); let v_a_i = v_a_i.clone(); let v_b = v_b.clone(); let v_b_i = v_b_i.clone(); let v_d = v_d.clone(); let v_i = v_i.clone(); let v_n = v_n.clone(); let v_o = v_o.clone(); let v_p = v_p.clone(); let v_prod = v_prod.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_n.clone(), &py::len(&v_v.clone())?)?)?, &Py::list((v_v.clone()).items()?))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_back = { let mut out = vec![]; for t1218 in py::iter(&(py::zip(&[py::call(&v_pad.clone(), &[v_prod.clone()])?, py::call(&v_pad.clone(), &[v_r.clone()])?])?))? { let mut v_u: Py; let mut v_v: Py; 
let t1219 = (t1218).items()?;
if t1219.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t1219[0].clone();
v_v = t1219[1].clone();
out.push(py::add(&v_u.clone(), &v_v.clone())?); } Py::list(out) };
    v_scale = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t1220 in py::iter(&(v_p.clone()))? { let mut v_c: Py; 
v_c = t1220;
out.push(py::abs(&v_c.clone())?); } Py::list(out) }], true)?, py::minmax(&[{ let mut out = vec![]; for t1221 in py::iter(&(v_prod.clone()))? { let mut v_v: Py; 
v_v = t1221;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?], true)?;
    return Ok(py::truediv(&py::minmax(&[{ let mut out = vec![]; for t1222 in py::iter(&(py::zip(&[v_back.clone(), py::call(&v_pad.clone(), &[v_p.clone()])?])?))? { let mut v_u: Py; let mut v_v: Py; 
let t1223 = (t1222).items()?;
if t1223.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t1223[0].clone();
v_v = t1223[1].clone();
out.push(py::abs(&py::sub(&v_u.clone(), &v_v.clone())?)?); } Py::list(out) }], true)?, &v_scale.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_magdir(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_t = py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("direction_deg"))?)?;
    if (py::not(&{ let t1934 = Py::Int(0i64); Py::Bool({ let t1935 = py::getitem(&v_o.clone(), &Py::str("direction_deg"))?; if py::le(&t1934, &t1935)? { { let t1936 = Py::Int(360i64); if py::lt(&t1935, &t1936)? { true } else { false } } } else { false } }) })).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::getitem(&v_o.clone(), &Py::str("magnitude"))?;
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&py::mul(&v_r.clone(), &v_r.clone())?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("x"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("y"))?, &Py::Int(2i64))?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&v_r.clone(), &v_r.clone())?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&v_r.clone(), &py::math1("cos", &v_t.clone())?)?, &py::getitem(&v_i.clone(), &Py::str("x"))?)?)?, &py::minmax(&[Py::Float(1.0), v_r.clone()], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&v_r.clone(), &py::math1("sin", &v_t.clone())?)?, &py::getitem(&v_i.clone(), &Py::str("y"))?)?)?, &py::minmax(&[Py::Float(1.0), v_r.clone()], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_nroots(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_args: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t117 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::complex(&[py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?])?])).items()?;
    if t117.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t117[0].clone();
    v_z = t117[1].clone();
    v_rs = py::getitem(&v_o.clone(), &Py::str("roots"))?;
    if ({ let t122 = { let t118 = py::len(&v_rs.clone())?; Py::Bool({ let t119 = v_n.clone(); if py::ne(&t118, &t119) { true } else { false } }) }; if t122.truthy() { t122 } else { { let t120 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t121 = v_n.clone(); if py::ne(&t120, &t121) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_args = { let mut out = vec![]; for t123 in py::iter(&(v_rs.clone()))? { let mut v_w: Py; 
v_w = t123;
out.push(f__arg360(v_w.clone())?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t124 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t124;
out.push({ let t125 = py::getitem(&v_args.clone(), &v_j.clone())?; Py::Bool({ let t126 = py::getitem(&v_args.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?; if py::ge(&t125, &t126)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = f__rel(py::pow(&py::getitem(&v_o.clone(), &Py::str("modulus"))?, &v_n.clone())?, py::abs(&v_z.clone())?)?;
    for t128 in py::iter(&(v_rs.clone()))? {
        v_w = t128;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::pow(&v_w.clone(), &v_n.clone())?, &v_z.clone())?)?, &py::abs(&v_z.clone())?)?, py::truediv(&py::abs(&py::sub(&py::abs(&v_w.clone())?, &py::getitem(&v_o.clone(), &Py::str("modulus"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("modulus"))?)?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ooo(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = f__eval_pratt(py::getitem(&v_i.clone(), &Py::str("expression"))?, Py::None, Py::None, Py::Bool(false))?;
    v_r = f__rel(py::getitem(&v_o.clone(), &Py::str("value"))?, py::float(&v_v.clone())?)?;
    if (Py::Bool(matches!(v_v.clone(), Py::Frac(_)))).truthy() {
        if ({ let t992 = { let t988 = py::method(&v_o.clone(), "get", &[Py::str("numerator")])?; Py::Bool({ let t989 = Py::None; if py::is(&t988, &t989) { true } else { false } }) }; if t992.truthy() { t992 } else { { let t990 = py::getitem(&v_o.clone(), &Py::str("denominator"))?; Py::Bool({ let t991 = Py::Int(0i64); if py::le(&t990, &t991)? { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        v_r = py::iadd(&v_r, &py::truediv(&py::abs(&py::sub(&py::fraction(&[py::int(&py::getitem(&v_o.clone(), &Py::str("numerator"))?)?, py::int(&py::getitem(&v_o.clone(), &Py::str("denominator"))?)?])?, &v_v.clone())?)?, &py::minmax(&[Py::Int(1i64), py::abs(&v_v.clone())?], true)?)?)?;
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_parabola(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d_line: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_px: Py = Py::None;
    let mut v_py: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1003 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("m"))?, py::method(&v_i.clone(), "get", &[Py::str("h"), Py::Float(0.0)])?, py::method(&v_i.clone(), "get", &[Py::str("k"), Py::Float(0.0)])?])).items()?;
    if t1003.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1003[0].clone();
    v_h = t1003[1].clone();
    v_k = t1003[2].clone();
    v_worst = py::truediv(&py::abs(&py::sub(&py::mul(&Py::Int(4i64), &py::getitem(&v_o.clone(), &Py::str("p"))?)?, &v_m.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_m.clone())?], true)?)?;
    for t1004 in py::iter(&(Py::tuple(vec![Py::Float(0.5), Py::Float(1.0), Py::Float(2.0), py::neg(&Py::Float(3.0))?])))? {
        v_t = t1004;
        if ({ let t1005 = py::getitem(&v_i.clone(), &Py::str("orientation"))?; Py::Bool({ let t1006 = Py::str("vertical"); if py::eq(&t1005, &t1006) { true } else { false } }) }).truthy() {
            let t1007 = (Py::tuple(vec![py::add(&v_h.clone(), &v_t.clone())?, py::add(&v_k.clone(), &py::truediv(&py::mul(&v_t.clone(), &v_t.clone())?, &v_m.clone())?)?])).items()?;
            if t1007.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_px = t1007[0].clone();
            v_py = t1007[1].clone();
            v_d_line = py::abs(&py::sub(&v_py.clone(), &py::getitem(&v_o.clone(), &Py::str("directrix"))?)?)?;
        } else {
            let t1008 = (Py::tuple(vec![py::add(&v_h.clone(), &py::truediv(&py::mul(&v_t.clone(), &v_t.clone())?, &v_m.clone())?)?, py::add(&v_k.clone(), &v_t.clone())?])).items()?;
            if t1008.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_px = t1008[0].clone();
            v_py = t1008[1].clone();
            v_d_line = py::abs(&py::sub(&v_px.clone(), &py::getitem(&v_o.clone(), &Py::str("directrix"))?)?)?;
        }
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::hypot(&[py::sub(&v_px.clone(), &py::getitem(&v_o.clone(), &Py::str("focus_x"))?)?, py::sub(&v_py.clone(), &py::getitem(&v_o.clone(), &Py::str("focus_y"))?)?])?, &v_d_line.clone())?)?, &py::minmax(&[Py::Float(1.0), v_d_line.clone()], true)?)?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pchange(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_want_dir: Py = Py::None;
    v_want_dir = (if ({ let t1027 = py::getitem(&v_i.clone(), &Py::str("new"))?; Py::Bool({ let t1028 = py::getitem(&v_i.clone(), &Py::str("old"))?; if py::gt(&t1027, &t1028)? { true } else { false } }) }).truthy() { Py::str("increase") } else { (if ({ let t1029 = py::getitem(&v_i.clone(), &Py::str("new"))?; Py::Bool({ let t1030 = py::getitem(&v_i.clone(), &Py::str("old"))?; if py::lt(&t1029, &t1030)? { true } else { false } }) }).truthy() { Py::str("decrease") } else { Py::str("no change") }) });
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("percent_change"))?, &py::getitem(&v_i.clone(), &Py::str("old"))?)?, &py::mul(&Py::Int(100i64), &py::sub(&py::getitem(&v_i.clone(), &Py::str("new"))?, &py::getitem(&v_i.clone(), &Py::str("old"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&Py::Int(100i64), &py::sub(&py::getitem(&v_i.clone(), &Py::str("new"))?, &py::getitem(&v_i.clone(), &Py::str("old"))?)?)?)?, py::abs(&py::mul(&py::getitem(&v_o.clone(), &Py::str("percent_change"))?, &py::getitem(&v_i.clone(), &Py::str("old"))?)?)?], true)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("difference"))?, py::sub(&py::getitem(&v_i.clone(), &Py::str("new"))?, &py::getitem(&v_i.clone(), &Py::str("old"))?)?)?)?, &{ let t1032 = py::getitem(&v_o.clone(), &Py::str("direction"))?; Py::Bool({ let t1033 = v_want_dir.clone(); if py::ne(&t1032, &t1033) { true } else { false } }) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pf(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let t1015 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("B"))?])).items()?;
    if t1015.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1015[0].clone();
    v_B = t1015[1].clone();
    let t1016 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("p"))?, py::getitem(&v_i.clone(), &Py::str("q"))?, py::getitem(&v_i.clone(), &Py::str("r1"))?, py::getitem(&v_i.clone(), &Py::str("r2"))?])).items()?;
    if t1016.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1016[0].clone();
    v_q = t1016[1].clone();
    v_r1 = t1016[2].clone();
    v_r2 = t1016[3].clone();
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::add(&v_A.clone(), &v_B.clone())?, &v_p.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_p.clone())?, py::abs(&v_A.clone())?, py::abs(&v_B.clone())?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::sub(&py::mul(&py::neg(&v_A.clone())?, &v_r2.clone())?, &py::mul(&v_B.clone(), &v_r1.clone())?)?, &v_q.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_q.clone())?, py::abs(&py::mul(&v_A.clone(), &v_r2.clone())?)?, py::abs(&py::mul(&v_B.clone(), &v_r1.clone())?)?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_poly(mut v_inputs: Py, mut v_outputs: Py, mut v_n: Py) -> R {
    let mut v_co: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_co = { let mut out = vec![]; for t1326 in py::iter(&(py::slice(&c__LETTERS()?, None, Some((py::add(&v_n.clone(), &Py::Int(1i64))?).i()?), None)?))? { let mut v_k: Py; 
v_k = t1326;
out.push(py::getitem(&v_inputs.clone(), &v_k.clone())?); } Py::list(out) };
    v_worst = Py::Float(0.0);
    for t1327 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t1327;
        v_x = py::complex(&[py::getitem(&v_outputs.clone(), &Py::str(&["x".to_string(), py::format(&py::add(&v_i.clone(), &Py::Int(1i64))?, "")?].concat()))?])?;
        v_terms = { let mut out = vec![]; for t1328 in py::iter(&(py::enumerate(&v_co.clone(), 0)?))? { let mut v_c: Py; let mut v_j: Py; 
let t1329 = (t1328).items()?;
if t1329.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t1329[0].clone();
v_c = t1329[1].clone();
out.push(py::mul(&v_c.clone(), &py::pow(&v_x.clone(), &py::sub(&v_n.clone(), &v_j.clone())?)?)?); } Py::list(out) };
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sum(&v_terms.clone(), None)?)?, &{ let t1331 = py::sum(&{ let mut out = vec![]; for t1330 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t1330;
out.push(py::abs(&v_t.clone())?); } Py::list(out) }, None)?; if t1331.truthy() { t1331 } else { Py::Float(1.0) } })?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_prop(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_t = f__prop_terms(py::getitem(&v_i.clone(), &Py::str("p"))?, py::getitem(&v_i.clone(), &Py::str("q"))?, py::getitem(&v_i.clone(), &Py::str("r"))?, py::getitem(&v_i.clone(), &Py::str("position"))?)?;
    py::setitem(&v_t.clone(), &py::method(&v_t.clone(), "index", &[Py::None])?, py::getitem(&v_o.clone(), &Py::str("x"))?)?;
    let t1249 = (v_t.clone()).items()?;
    if t1249.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t1249[0].clone();
    v_d1 = t1249[1].clone();
    v_n2 = t1249[2].clone();
    v_d2 = t1249[3].clone();
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&v_n1.clone(), &v_d2.clone())?, &py::mul(&v_n2.clone(), &v_d1.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&v_n1.clone(), &v_d2.clone())?)?, py::abs(&py::mul(&v_n2.clone(), &v_d1.clone())?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pw(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_first: Py = Py::None;
    v_first = (if ({ let t1159 = py::getitem(&v_i.clone(), &Py::str("left_rel"))?; Py::Bool({ let t1160 = Py::str("<"); if py::eq(&t1159, &t1160) { true } else { false } }) }).truthy() { { let t1161 = py::getitem(&v_i.clone(), &Py::str("x"))?; Py::Bool({ let t1162 = py::getitem(&v_i.clone(), &Py::str("c"))?; if py::lt(&t1161, &t1162)? { true } else { false } }) } } else { { let t1163 = py::getitem(&v_i.clone(), &Py::str("x"))?; Py::Bool({ let t1164 = py::getitem(&v_i.clone(), &Py::str("c"))?; if py::le(&t1163, &t1164)? { true } else { false } }) } });
    if ({ let t1165 = py::getitem(&v_o.clone(), &Py::str("piece"))?; Py::Bool({ let t1166 = (if (v_first.clone()).truthy() { Py::Float(1.0) } else { Py::Float(2.0) }); if py::ne(&t1165, &t1166) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("value"))?, py::float(&f__eval_pratt((if (v_first.clone()).truthy() { py::getitem(&v_i.clone(), &Py::str("e1"))? } else { py::getitem(&v_i.clone(), &Py::str("e2"))? }), Py::dict(vec![(Py::str("x"), py::getitem(&v_i.clone(), &Py::str("x"))?)]), Py::str("x"), Py::Bool(false))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_qineq(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1280 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])).items()?;
    if t1280.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1280[0].clone();
    v_b = t1280[1].clone();
    v_c = t1280[2].clone();
    v_worst = Py::Float(0.0);
    for t1281 in py::iter(&(Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("r1"))?, py::getitem(&v_o.clone(), &Py::str("r2"))?])))? {
        v_x = t1281;
        v_t = Py::list(vec![py::mul(&py::mul(&v_a.clone(), &v_x.clone())?, &v_x.clone())?, py::mul(&v_b.clone(), &v_x.clone())?, v_c.clone()]);
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sum(&v_t.clone(), None)?)?, &{ let t1283 = py::sum(&{ let mut out = vec![]; for t1282 in py::iter(&(v_t.clone()))? { let mut v_v: Py; 
v_v = t1282;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }, None)?; if t1283.truthy() { t1283 } else { Py::Float(1.0) } })?], true)?;
    }
    v_worst = py::iadd(&v_worst, &py::truediv(&py::abs(&py::add(&py::add(&py::getitem(&v_o.clone(), &Py::str("r1"))?, &py::getitem(&v_o.clone(), &Py::str("r2"))?)?, &py::truediv(&v_b.clone(), &v_a.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::truediv(&v_b.clone(), &v_a.clone())?)?], true)?)?)?;
    v_worst = py::iadd(&v_worst, &py::add(&{ let t1284 = py::getitem(&v_o.clone(), &Py::str("r1"))?; Py::Bool({ let t1285 = py::getitem(&v_o.clone(), &Py::str("r2"))?; if py::ge(&t1284, &t1285)? { true } else { false } }) }, &{ let t1286 = py::getitem(&v_o.clone(), &Py::str("region"))?; Py::Bool({ let t1287 = (if ({ let t1288 = { let t1290 = py::getitem(&v_i.clone(), &Py::str("rel"))?; Py::Bool({ let t1291 = Py::tuple(vec![Py::str("<"), Py::str("<=")]); if py::contains(&t1291, &t1290)? { true } else { false } }) }; Py::Bool({ let t1289 = { let t1292 = v_a.clone(); Py::Bool({ let t1293 = Py::Int(0i64); if py::gt(&t1292, &t1293)? { true } else { false } }) }; if py::eq(&t1288, &t1289) { true } else { false } }) }).truthy() { Py::str("between") } else { Py::str("outside") }); if py::ne(&t1286, &t1287) { true } else { false } }) })?)?;
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_radical(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_lhs: Py = Py::None;
    let mut v_lhs2: Py = Py::None;
    let mut v_n_roots: Py = Py::None;
    let mut v_qa: Py = Py::None;
    let mut v_qb: Py = Py::None;
    let mut v_qc: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1404 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("d"))?])).items()?;
    if t1404.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1404[0].clone();
    v_b = t1404[1].clone();
    v_c = t1404[2].clone();
    v_d = t1404[3].clone();
    if ({ let t1405 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t1406 = py::len(&py::getitem(&v_o.clone(), &Py::str("solutions"))?)?; if py::ne(&t1405, &t1406) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    for t1407 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("solutions"))?))? {
        v_x = t1407;
        v_lhs2 = py::add(&py::mul(&v_a.clone(), &v_x.clone())?, &v_b.clone())?;
        if ({ let t1408 = v_lhs2.clone(); Py::Bool({ let t1409 = py::mul(&py::neg(&Py::Float(1e-12))?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&v_a.clone(), &v_x.clone())?)?, py::abs(&v_b.clone())?], true)?)?; if py::lt(&t1408, &t1409)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
        v_lhs = py::math1("sqrt", &py::minmax(&[Py::Float(0.0), v_lhs2.clone()], true)?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&v_lhs.clone(), &py::add(&py::mul(&v_c.clone(), &v_x.clone())?, &v_d.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_lhs.clone())?, py::abs(&py::mul(&v_c.clone(), &v_x.clone())?)?, py::abs(&v_d.clone())?], true)?)?], true)?;
    }
    let t1410 = (Py::tuple(vec![py::mul(&v_c.clone(), &v_c.clone())?, py::sub(&py::mul(&py::mul(&Py::Int(2i64), &v_c.clone())?, &v_d.clone())?, &v_a.clone())?, py::sub(&py::mul(&v_d.clone(), &v_d.clone())?, &v_b.clone())?])).items()?;
    if t1410.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_qa = t1410[0].clone();
    v_qb = t1410[1].clone();
    v_qc = t1410[2].clone();
    v_n_roots = (if ({ let t1411 = v_qa.clone(); Py::Bool({ let t1412 = Py::Int(0i64); if py::eq(&t1411, &t1412) { true } else { false } }) }).truthy() { (if ({ let t1413 = v_qb.clone(); Py::Bool({ let t1414 = Py::Int(0i64); if py::ne(&t1413, &t1414) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }) } else { py::call(&{ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_i = v_i.clone(); let v_lhs = v_lhs.clone(); let v_lhs2 = v_lhs2.clone(); let v_o = v_o.clone(); let v_qa = v_qa.clone(); let v_qb = v_qb.clone(); let v_qc = v_qc.clone(); let v_worst = v_worst.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_D: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t1415 = v_D.clone(); Py::Bool({ let t1416 = Py::Int(0i64); if py::lt(&t1415, &t1416)? { true } else { false } }) }).truthy() { Py::Int(0i64) } else { (if ({ let t1417 = v_D.clone(); Py::Bool({ let t1418 = Py::Int(0i64); if py::eq(&t1417, &t1418) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(2i64) }) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::sub(&py::pow(&f__fr(v_qb.clone())?, &Py::Int(2i64))?, &py::mul(&py::mul(&Py::Int(4i64), &f__fr(v_qa.clone())?)?, &f__fr(v_qc.clone())?)?)?])? });
    if ({ let t1419 = py::add(&py::len(&py::getitem(&v_o.clone(), &Py::str("solutions"))?)?, &py::len(&py::method(&v_o.clone(), "get", &[Py::str("extraneous"), Py::list(vec![])])?)?)?; Py::Bool({ let t1420 = v_n_roots.clone(); if py::ne(&t1419, &t1420) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_rateq(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = py::getitem(&v_o.clone(), &Py::str("x"))?;
    let t1454 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("d"))?])).items()?;
    if t1454.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1454[0].clone();
    v_b = t1454[1].clone();
    v_c = t1454[2].clone();
    v_d = t1454[3].clone();
    if ({ let t1459 = { let t1455 = py::add(&v_x.clone(), &v_b.clone())?; Py::Bool({ let t1456 = Py::Int(0i64); if py::eq(&t1455, &t1456) { true } else { false } }) }; if t1459.truthy() { t1459 } else { { let t1457 = py::add(&v_x.clone(), &v_d.clone())?; Py::Bool({ let t1458 = Py::Int(0i64); if py::eq(&t1457, &t1458) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&v_a.clone(), &py::add(&v_x.clone(), &v_d.clone())?)?, &py::mul(&v_c.clone(), &py::add(&v_x.clone(), &v_b.clone())?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&v_a.clone(), &py::add(&v_x.clone(), &v_d.clone())?)?)?, py::abs(&py::mul(&v_c.clone(), &py::add(&v_x.clone(), &v_b.clone())?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ratio(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_got: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_parts: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_parts = py::add(&Py::list(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?]), &(if ({ let t1433 = py::method(&v_i.clone(), "get", &[Py::str("c")])?; Py::Bool({ let t1434 = Py::None; if !py::is(&t1433, &t1434) { true } else { false } }) }).truthy() { Py::list(vec![py::getitem(&v_i.clone(), &Py::str("c"))?]) } else { Py::list(vec![]) }))?;
    v_got = { let mut out = vec![]; for t1435 in py::range_iter(&[(py::len(&v_parts.clone())?).i()?]) { let mut v_k: Py; 
v_k = t1435;
out.push(py::method(&v_o.clone(), "get", &[Py::str(&["part".to_string(), py::format(&py::add(&v_k.clone(), &Py::Int(1i64))?, "")?].concat())])?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t1436 in py::iter(&(v_got.clone()))? { let mut v_g: Py; 
v_g = t1436;
out.push({ let t1437 = v_g.clone(); Py::Bool({ let t1438 = Py::None; if py::is(&t1437, &t1438) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = f__rel(py::sum(&v_got.clone(), None)?, py::getitem(&v_i.clone(), &Py::str("total"))?)?;
    for t1440 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_parts.clone())?).i()?]) {
        v_k = t1440;
        v_r = py::iadd(&v_r, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_got.clone(), &v_k.clone())?, &py::getitem(&v_parts.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_got.clone(), &Py::Int(0i64))?, &py::getitem(&v_parts.clone(), &v_k.clone())?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&py::getitem(&v_got.clone(), &Py::Int(0i64))?, &py::getitem(&v_parts.clone(), &v_k.clone())?)?)?, py::abs(&py::mul(&py::getitem(&v_got.clone(), &v_k.clone())?, &py::getitem(&v_parts.clone(), &Py::Int(0i64))?)?)?], true)?)?)?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ratroots(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_co: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_ints: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rest: Py = Py::None;
    v_co = { let mut out = vec![]; for t1497 in py::iter(&(f__strip({ let mut out = vec![]; for t1496 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("coefficients"))?))? { let mut v_c: Py; 
v_c = t1496;
out.push(py::float(&v_c.clone())?); } Py::list(out) })?))? { let mut v_c: Py; 
v_c = t1497;
out.push(py::int(&v_c.clone())?); } Py::list(out) };
    v_ex = { let mut out = vec![]; for t1498 in py::iter(&(py::method(&v_o.clone(), "get", &[Py::str("roots_exact"), Py::list(vec![])])?))? { let mut v_s: Py; 
v_s = t1498;
out.push(py::fraction(&[v_s.clone()])?); } Py::list(out) };
    if ({ let t1506 = { let t1499 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t1500 = py::len(&v_ex.clone())?; if py::ne(&t1499, &t1500) { true } else { false } }) }; if t1506.truthy() { t1506 } else { { let t1505 = { let t1501 = py::len(&py::getitem(&v_o.clone(), &Py::str("roots"))?)?; Py::Bool({ let t1502 = py::len(&v_ex.clone())?; if py::ne(&t1501, &t1502) { true } else { false } }) }; if t1505.truthy() { t1505 } else { { let t1503 = py::len(&py::set(&v_ex.clone())?)?; Py::Bool({ let t1504 = py::len(&v_ex.clone())?; if py::ne(&t1503, &t1504) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t1515 = Py::Bool(({ let mut out = vec![]; for t1507 in py::iter(&(v_ex.clone()))? { let mut v_r: Py; 
v_r = t1507;
out.push({ let t1508 = f__horner(v_co.clone(), v_r.clone())?; Py::Bool({ let t1509 = Py::Int(0i64); if py::ne(&t1508, &t1509) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t1515.truthy() { t1515 } else { Py::Bool(({ let mut out = vec![]; for t1510 in py::iter(&(py::zip(&[v_ex.clone(), py::getitem(&v_o.clone(), &Py::str("roots"))?])?))? { let mut v_r: Py; let mut v_v: Py; 
let t1511 = (t1510).items()?;
if t1511.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t1511[0].clone();
v_v = t1511[1].clone();
out.push({ let t1512 = f__rel(py::float(&v_r.clone())?, v_v.clone())?; Py::Bool({ let t1513 = Py::Float(1e-15); if py::gt(&t1512, &t1513)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_rest = { let mut out = vec![]; for t1516 in py::iter(&(v_co.clone()))? { let mut v_c: Py; 
v_c = t1516;
out.push(py::fraction(&[v_c.clone()])?); } Py::list(out) };
    for t1517 in py::iter(&(v_ex.clone()))? {
        v_r = t1517;
        while ({ let t1523 = { let t1519 = py::len(&v_rest.clone())?; Py::Bool({ let t1520 = Py::Int(1i64); if py::gt(&t1519, &t1520)? { true } else { false } }) }; if !t1523.truthy() { t1523 } else { { let t1521 = f__horner(v_rest.clone(), v_r.clone())?; Py::Bool({ let t1522 = Py::Int(0i64); if py::eq(&t1521, &t1522) { true } else { false } }) } } }).truthy() {
            v_out = Py::list(vec![py::getitem(&v_rest.clone(), &Py::Int(0i64))?]);
            for t1518 in py::iter(&(py::slice(&v_rest.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
                v_c = t1518;
                let _ = py::method(&v_out.clone(), "append", &[py::add(&v_c.clone(), &py::mul(&py::getitem(&v_out.clone(), &py::neg(&Py::Int(1i64))?)?, &v_r.clone())?)?])?;
            }
            v_rest = py::slice(&v_out.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?;
        }
    }
    if ({ let t1524 = py::len(&v_rest.clone())?; Py::Bool({ let t1525 = Py::Int(1i64); if py::gt(&t1524, &t1525)? { true } else { false } }) }).truthy() {
        v_den = py::gcd_lcm("lcm", &({ let mut v: Vec<Py> = vec![]; v.extend(({ let mut out = vec![]; for t1526 in py::iter(&(v_rest.clone()))? { let mut v_f: Py; 
v_f = t1526;
out.push(py::denominator(&v_f.clone())?); } Py::list(out) }).items()?); v })[..])?;
        v_ints = { let mut out = vec![]; for t1527 in py::iter(&(v_rest.clone()))? { let mut v_f: Py; 
v_f = t1527;
out.push(py::int(&py::mul(&v_f.clone(), &v_den.clone())?)?); } Py::list(out) };
        v_g = py::gcd_lcm("gcd", &({ let mut v: Vec<Py> = vec![]; v.extend((v_ints.clone()).items()?); v })[..])?;
        v_ints = { let mut out = vec![]; for t1528 in py::iter(&(v_ints.clone()))? { let mut v_v: Py; 
v_v = t1528;
out.push(py::floordiv(&v_v.clone(), &v_g.clone())?); } Py::list(out) };
        while ({ let t1533 = { let t1529 = py::len(&v_ints.clone())?; Py::Bool({ let t1530 = Py::Int(1i64); if py::gt(&t1529, &t1530)? { true } else { false } }) }; if !t1533.truthy() { t1533 } else { { let t1531 = py::getitem(&v_ints.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t1532 = Py::Int(0i64); if py::eq(&t1531, &t1532) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        for t1534 in py::iter(&(f__divisors(py::getitem(&v_ints.clone(), &py::neg(&Py::Int(1i64))?)?)?))? {
            v_p = t1534;
            for t1535 in py::iter(&(f__divisors(py::getitem(&v_ints.clone(), &Py::Int(0i64))?)?))? {
                v_q = t1535;
                if ({ let t1540 = { let t1536 = f__horner(v_ints.clone(), py::fraction(&[v_p.clone(), v_q.clone()])?)?; Py::Bool({ let t1537 = Py::Int(0i64); if py::eq(&t1536, &t1537) { true } else { false } }) }; if t1540.truthy() { t1540 } else { { let t1538 = f__horner(v_ints.clone(), py::fraction(&[py::neg(&v_p.clone())?, v_q.clone()])?)?; Py::Bool({ let t1539 = Py::Int(0i64); if py::eq(&t1538, &t1539) { true } else { false } }) } } }).truthy() {
                    return Ok(Py::Float(1.0));
                }
            }
        }
    }
    return Ok(Py::Float(0.0));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_round_dp(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_gap: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_unit: Py = Py::None;
    let t1601 = (Py::tuple(vec![f__dec(py::getitem(&v_i.clone(), &Py::str("x"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?])).items()?;
    if t1601.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t1601[0].clone();
    v_n = t1601[1].clone();
    v_r = f__dec(py::getitem(&v_o.clone(), &Py::str("rounded"))?)?;
    v_unit = (if ({ let t1602 = v_n.clone(); Py::Bool({ let t1603 = Py::Int(0i64); if py::ge(&t1602, &t1603)? { true } else { false } }) }).truthy() { py::fraction(&[Py::Int(1i64), py::pow(&Py::Int(10i64), &v_n.clone())?])? } else { py::fraction(&[py::pow(&Py::Int(10i64), &py::neg(&v_n.clone())?)?])? });
    if ({ let t1604 = py::denominator(&py::truediv(&v_r.clone(), &v_unit.clone())?)?; Py::Bool({ let t1605 = Py::Int(1i64); if py::ne(&t1604, &t1605) { true } else { false } }) }).truthy() {
        return Ok(py::add(&Py::Float(1.0), &py::float(&py::abs(&py::sub(&v_r.clone(), &v_q.clone())?)?)?)?);
    }
    v_gap = py::abs(&py::sub(&v_r.clone(), &v_q.clone())?)?;
    if ({ let t1613 = { let t1606 = v_gap.clone(); Py::Bool({ let t1607 = py::truediv(&v_unit.clone(), &Py::Int(2i64))?; if py::gt(&t1606, &t1607)? { true } else { false } }) }; if t1613.truthy() { t1613 } else { { let t1612 = { let t1608 = v_gap.clone(); Py::Bool({ let t1609 = py::truediv(&v_unit.clone(), &Py::Int(2i64))?; if py::eq(&t1608, &t1609) { true } else { false } }) }; if !t1612.truthy() { t1612 } else { { let t1610 = py::abs(&v_r.clone())?; Py::Bool({ let t1611 = py::abs(&v_q.clone())?; if py::lt(&t1610, &t1611)? { true } else { false } }) } } } } }).truthy() {
        return Ok(py::add(&Py::Float(1.0), &py::float(&v_gap.clone())?)?);
    }
    return Ok(Py::Float(0.0));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_round_sf(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_gap: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_places: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_unit: Py = Py::None;
    let t1629 = (Py::tuple(vec![f__dec(py::getitem(&v_i.clone(), &Py::str("x"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?])).items()?;
    if t1629.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t1629[0].clone();
    v_n = t1629[1].clone();
    v_r = f__dec(py::getitem(&v_o.clone(), &Py::str("rounded"))?)?;
    v_places = py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &f__sig_exponent(v_q.clone())?)?;
    v_unit = py::pow(&py::fraction(&[Py::Int(10i64)])?, &py::neg(&v_places.clone())?)?;
    if ({ let t1630 = py::denominator(&py::truediv(&v_r.clone(), &v_unit.clone())?)?; Py::Bool({ let t1631 = Py::Int(1i64); if py::ne(&t1630, &t1631) { true } else { false } }) }).truthy() {
        return Ok(py::add(&Py::Float(1.0), &py::float(&py::abs(&py::sub(&v_r.clone(), &v_q.clone())?)?)?)?);
    }
    v_gap = py::abs(&py::sub(&v_r.clone(), &v_q.clone())?)?;
    if ({ let t1639 = { let t1632 = v_gap.clone(); Py::Bool({ let t1633 = py::truediv(&v_unit.clone(), &Py::Int(2i64))?; if py::gt(&t1632, &t1633)? { true } else { false } }) }; if t1639.truthy() { t1639 } else { { let t1638 = { let t1634 = v_gap.clone(); Py::Bool({ let t1635 = py::truediv(&v_unit.clone(), &Py::Int(2i64))?; if py::eq(&t1634, &t1635) { true } else { false } }) }; if !t1638.truthy() { t1638 } else { { let t1636 = py::abs(&v_r.clone())?; Py::Bool({ let t1637 = py::abs(&v_q.clone())?; if py::lt(&t1636, &t1637)? { true } else { false } }) } } } } }).truthy() {
        return Ok(py::add(&Py::Float(1.0), &py::float(&v_gap.clone())?)?);
    }
    return Ok(Py::Float(0.0));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_rt_as(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_given: Py = Py::None;
    let t1572 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("opposite"))?, py::getitem(&v_o.clone(), &Py::str("adjacent"))?, py::getitem(&v_o.clone(), &Py::str("hypotenuse"))?])).items()?;
    if t1572.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1572[0].clone();
    v_b = t1572[1].clone();
    v_c = t1572[2].clone();
    v_given = py::getitem(&Py::dict(vec![(Py::str("hypotenuse"), v_c.clone()), (Py::str("opposite"), v_a.clone()), (Py::str("adjacent"), v_b.clone())]), &py::getitem(&v_i.clone(), &Py::str("which"))?)?;
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&v_c.clone(), &v_c.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&v_c.clone(), &v_c.clone())?], true)?)?, &py::truediv(&py::abs(&py::sub(&v_a.clone(), &py::mul(&v_c.clone(), &py::math1("sin", &py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("A"))?)?)?)?)?)?, &py::minmax(&[Py::Float(1.0), v_c.clone()], true)?)?)?, &f__rel(v_given.clone(), py::getitem(&v_i.clone(), &Py::str("side"))?)?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("B"))?, &py::getitem(&v_i.clone(), &Py::str("A"))?)?, &Py::Int(90i64))?)?, &Py::Int(90i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_rt_legs(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let t1591 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?])).items()?;
    if t1591.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1591[0].clone();
    v_b = t1591[1].clone();
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("c"))?, &Py::Int(2i64))?, &py::mul(&v_a.clone(), &v_a.clone())?)?, &py::mul(&v_b.clone(), &v_b.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::math1("tan", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("A"))?)?)?, &v_b.clone())?, &v_a.clone())?)?, &py::minmax(&[Py::Float(1.0), v_a.clone()], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("A"))?, &py::getitem(&v_o.clone(), &Py::str("B"))?)?, &Py::Int(90i64))?)?, &Py::Int(90i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_sci(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t1650 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("mantissa"))?, py::getitem(&v_o.clone(), &Py::str("exponent"))?])).items()?;
    if t1650.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1650[0].clone();
    v_e = t1650[1].clone();
    if ({ let t1656 = { let t1651 = v_e.clone(); Py::Bool({ let t1652 = py::int(&v_e.clone())?; if py::ne(&t1651, &t1652) { true } else { false } }) }; if t1656.truthy() { t1656 } else { py::not(&{ let t1653 = Py::Int(1i64); Py::Bool({ let t1654 = py::abs(&v_m.clone())?; if py::le(&t1653, &t1654)? { { let t1655 = Py::Int(10i64); if py::lt(&t1654, &t1655)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&py::truediv(&py::abs(&py::sub(&py::mul(&f__dec(v_m.clone())?, &py::pow(&py::fraction(&[Py::Int(10i64)])?, &py::int(&v_e.clone())?)?)?, &f__dec(py::getitem(&v_i.clone(), &Py::str("x"))?)?)?)?, &py::minmax(&[py::fraction(&[Py::Int(1i64)])?, py::abs(&f__dec(py::getitem(&v_i.clone(), &Py::str("x"))?)?)?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ssa(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t777 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("A"))?])).items()?;
    if t777.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t777[0].clone();
    v_b = t777[1].clone();
    v_A = t777[2].clone();
    v_n = py::int(&py::getitem(&v_o.clone(), &Py::str("count"))?)?;
    if ({ let t789 = { let t778 = v_n.clone(); Py::Bool({ let t779 = py::getitem(&v_o.clone(), &Py::str("count"))?; if py::ne(&t778, &t779) { true } else { false } }) }; if t789.truthy() { t789 } else { { let t788 = { let t780 = v_n.clone(); Py::Bool({ let t781 = Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)]); if !py::contains(&t781, &t780)? { true } else { false } }) }; if t788.truthy() { t788 } else { { let t787 = Py::Bool(({ let mut out = vec![]; for t782 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t782;
out.push({ let t783 = Py::str(&["B".to_string(), py::format(&v_k.clone(), "")?].concat()); Py::Bool({ let t784 = v_o.clone(); if !py::contains(&t784, &t783)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t787.truthy() { t787 } else { { let t785 = Py::str(&["B".to_string(), py::format(&py::add(&v_n.clone(), &Py::Int(1i64))?, "")?].concat()); Py::Bool({ let t786 = v_o.clone(); if py::contains(&t786, &t785)? { true } else { false } }) } } } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    for t790 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t790;
        let t791 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str(&["B".to_string(), py::format(&v_k.clone(), "")?].concat()))?, py::getitem(&v_o.clone(), &Py::str(&["C".to_string(), py::format(&v_k.clone(), "")?].concat()))?, py::getitem(&v_o.clone(), &Py::str(&["c".to_string(), py::format(&v_k.clone(), "")?].concat()))?])).items()?;
        if t791.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_B = t791[0].clone();
        v_C = t791[1].clone();
        v_c = t791[2].clone();
        v_worst = py::iadd(&v_worst, &py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::add(&v_A.clone(), &v_B.clone())?, &v_C.clone())?, &Py::Int(180i64))?)?, &Py::Int(180i64))?, &py::truediv(&py::abs(&py::sub(&py::mul(&v_a.clone(), &v_a.clone())?, &py::sub(&py::add(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_b.clone())?, &v_c.clone())?, &py::math1("cos", &py::math1("radians", &v_A.clone())?)?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&v_a.clone(), &v_a.clone())?, py::mul(&v_b.clone(), &v_b.clone())?], true)?)?)?)?;
        v_worst = py::iadd(&v_worst, &py::truediv(&py::abs(&py::sub(&py::mul(&v_b.clone(), &f__deg_sin(v_A.clone())?)?, &py::mul(&v_a.clone(), &f__deg_sin(v_B.clone())?)?)?)?, &py::minmax(&[Py::Float(1.0), v_a.clone(), v_b.clone()], true)?)?)?;
    }
    let t792 = (Py::tuple(vec![py::mul(&py::mul(&py::neg(&Py::Int(2i64))?, &v_b.clone())?, &py::math1("cos", &py::math1("radians", &v_A.clone())?)?)?, py::sub(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&v_a.clone(), &v_a.clone())?)?])).items()?;
    if t792.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t792[0].clone();
    v_q = t792[1].clone();
    v_disc = py::sub(&py::mul(&v_p.clone(), &v_p.clone())?, &py::mul(&Py::Int(4i64), &v_q.clone())?)?;
    if ({ let t793 = py::abs(&v_disc.clone())?; Py::Bool({ let t794 = py::mul(&Py::Float(1e-11), &py::minmax(&[Py::Float(1.0), py::mul(&v_p.clone(), &v_p.clone())?, py::abs(&py::mul(&Py::Int(4i64), &v_q.clone())?)?], true)?)?; if py::le(&t793, &t794)? { true } else { false } }) }).truthy() {
        v_disc = Py::Float(0.0);
    }
    v_roots = (if ({ let t795 = v_disc.clone(); Py::Bool({ let t796 = Py::Int(0i64); if py::lt(&t795, &t796)? { true } else { false } }) }).truthy() { Py::list(vec![]) } else { Py::list(vec![py::truediv(&py::sub(&py::neg(&v_p.clone())?, &py::math1("sqrt", &v_disc.clone())?)?, &Py::Int(2i64))?, py::truediv(&py::add(&py::neg(&v_p.clone())?, &py::math1("sqrt", &v_disc.clone())?)?, &Py::Int(2i64))?]) });
    v_want = py::len(&py::set(&{ let mut out = vec![]; for t797 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t797;
if ({ let t798 = v_r.clone(); Py::Bool({ let t799 = Py::Float(1e-09); if py::gt(&t798, &t799)? { true } else { false } }) }).truthy() { out.push(py::round(&v_r.clone(), Some((Py::Int(9i64)).i()?))?); }} Py::list(out) })?)?;
    return Ok(py::add(&v_worst.clone(), &{ let t800 = v_want.clone(); Py::Bool({ let t801 = v_n.clone(); if py::ne(&t800, &t801) { true } else { false } }) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_synth(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_back: Py = Py::None;
    let mut v_co: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rem: Py = Py::None;
    let mut v_scale: Py = Py::None;
    v_co = f__strip({ let mut out = vec![]; for t1671 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("coefficients"))?))? { let mut v_c: Py; 
v_c = t1671;
out.push(py::float(&v_c.clone())?); } Py::list(out) })?;
    let t1672 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("quotient"))?, py::getitem(&v_o.clone(), &Py::str("remainder"))?, py::getitem(&v_i.clone(), &Py::str("r"))?])).items()?;
    if t1672.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t1672[0].clone();
    v_rem = t1672[1].clone();
    v_r = t1672[2].clone();
    if ({ let t1673 = py::len(&v_q.clone())?; Py::Bool({ let t1674 = py::sub(&py::len(&v_co.clone())?, &Py::Int(1i64))?; if py::ne(&t1673, &t1674) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_back = py::add(&py::add(&Py::list(vec![py::getitem(&v_q.clone(), &Py::Int(0i64))?]), &{ let mut out = vec![]; for t1675 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_q.clone())?).i()?]) { let mut v_k: Py; 
v_k = t1675;
out.push(py::sub(&py::getitem(&v_q.clone(), &v_k.clone())?, &py::mul(&v_r.clone(), &py::getitem(&v_q.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?)?); } Py::list(out) })?, &Py::list(vec![py::sub(&v_rem.clone(), &py::mul(&v_r.clone(), &py::getitem(&v_q.clone(), &py::neg(&Py::Int(1i64))?)?)?)?]))?;
    v_scale = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t1676 in py::iter(&(v_co.clone()))? { let mut v_c: Py; 
v_c = t1676;
out.push(py::abs(&v_c.clone())?); } Py::list(out) }], true)?, py::mul(&py::minmax(&[{ let mut out = vec![]; for t1677 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t1677;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?], true)?;
    return Ok(py::truediv(&py::minmax(&[{ let mut out = vec![]; for t1678 in py::iter(&(py::zip(&[v_back.clone(), v_co.clone()])?))? { let mut v_u: Py; let mut v_v: Py; 
let t1679 = (t1678).items()?;
if t1679.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t1679[0].clone();
v_v = t1679[1].clone();
out.push(py::abs(&py::sub(&v_u.clone(), &v_v.clone())?)?); } Py::list(out) }], true)?, &v_scale.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_sys3(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_worst = Py::Float(0.0);
    for t838 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("augmented"))?))? {
        v_r = t838;
        v_terms = Py::list(vec![py::mul(&py::getitem(&v_r.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::str("x"))?)?, py::mul(&py::getitem(&v_r.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::str("y"))?)?, py::mul(&py::getitem(&v_r.clone(), &Py::Int(2i64))?, &py::getitem(&v_o.clone(), &Py::str("z"))?)?]);
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::sum(&v_terms.clone(), None)?, &py::getitem(&v_r.clone(), &Py::Int(3i64))?)?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::sum(&{ let mut out = vec![]; for t839 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t839;
out.push(py::abs(&v_t.clone())?); } Py::list(out) }, None)?, &py::abs(&py::getitem(&v_r.clone(), &Py::Int(3i64))?)?)?], true)?)?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_trigeq(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_func: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_per: Py = Py::None;
    let mut v_rad: Py = Py::None;
    let mut v_sols: Py = Py::None;
    let mut v_unit: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1717 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("func"))?, py::getitem(&v_i.clone(), &Py::str("k"))?, py::int(&py::method(&v_i.clone(), "get", &[Py::str("n"), Py::Int(1i64)])?)?])).items()?;
    if t1717.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_func = t1717[0].clone();
    v_k = t1717[1].clone();
    v_n = t1717[2].clone();
    v_unit = py::method(&v_i.clone(), "get", &[Py::str("unit"), Py::str("degrees")])?;
    v_per = (if ({ let t1718 = v_func.clone(); Py::Bool({ let t1719 = Py::str("tan"); if py::eq(&t1718, &t1719) { true } else { false } }) }).truthy() { Py::Int(2i64) } else { (if ({ let t1720 = py::abs(&v_k.clone())?; Py::Bool({ let t1721 = Py::Int(1i64); if py::lt(&t1720, &t1721)? { true } else { false } }) }).truthy() { Py::Int(2i64) } else { (if ({ let t1722 = py::abs(&v_k.clone())?; Py::Bool({ let t1723 = Py::Int(1i64); if py::eq(&t1722, &t1723) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }) }) });
    v_sols = py::getitem(&v_o.clone(), &Py::str("solutions"))?;
    if ({ let t1728 = { let t1724 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t1725 = py::len(&v_sols.clone())?; if py::ne(&t1724, &t1725) { true } else { false } }) }; if t1728.truthy() { t1728 } else { { let t1726 = py::len(&v_sols.clone())?; Py::Bool({ let t1727 = py::mul(&v_per.clone(), &v_n.clone())?; if py::ne(&t1726, &t1727) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_P = (if ({ let t1729 = v_unit.clone(); Py::Bool({ let t1730 = Py::str("degrees"); if py::eq(&t1729, &t1730) { true } else { false } }) }).truthy() { Py::Float(360.0) } else { py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))? });
    v_worst = Py::Float(0.0);
    for t1731 in py::iter(&(py::enumerate(&v_sols.clone(), 0)?))? {
        let t1741 = (t1731).items()?;
        if t1741.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t1741[0].clone();
        v_x = t1741[1].clone();
        if ({ let t1738 = py::not(&{ let t1732 = Py::Int(0i64); Py::Bool({ let t1733 = v_x.clone(); if py::le(&t1732, &t1733)? { { let t1734 = v_P.clone(); if py::lt(&t1733, &t1734)? { true } else { false } } } else { false } }) }); if t1738.truthy() { t1738 } else { { let t1737 = v_j.clone(); if !t1737.truthy() { t1737 } else { { let t1735 = v_x.clone(); Py::Bool({ let t1736 = py::add(&py::getitem(&v_sols.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?, &py::mul(&Py::Float(1e-12), &v_P.clone())?)?; if py::le(&t1735, &t1736)? { true } else { false } }) } } } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        v_rad = (if ({ let t1739 = v_unit.clone(); Py::Bool({ let t1740 = Py::str("degrees"); if py::eq(&t1739, &t1740) { true } else { false } }) }).truthy() { py::math1("radians", &v_x.clone())? } else { v_x.clone() });
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::call(&f__trig_f(v_func.clone())?, &[py::mul(&v_n.clone(), &v_rad.clone())?])?, &v_k.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_k.clone())?], true)?)?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_trigvals(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_given: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_q = py::int(&py::getitem(&v_i.clone(), &Py::str("quadrant"))?)?;
    let t1797 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("sin"))?, py::getitem(&v_o.clone(), &Py::str("cos"))?])).items()?;
    if t1797.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1797[0].clone();
    v_c = t1797[1].clone();
    if ({ let t1810 = { let t1798 = { let t1800 = v_s.clone(); Py::Bool({ let t1801 = Py::Int(0i64); if py::gt(&t1800, &t1801)? { true } else { false } }) }; Py::Bool({ let t1799 = { let t1802 = v_q.clone(); Py::Bool({ let t1803 = Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)]); if py::contains(&t1803, &t1802)? { true } else { false } }) }; if py::ne(&t1798, &t1799) { true } else { false } }) }; if t1810.truthy() { t1810 } else { { let t1804 = { let t1806 = v_c.clone(); Py::Bool({ let t1807 = Py::Int(0i64); if py::gt(&t1806, &t1807)? { true } else { false } }) }; Py::Bool({ let t1805 = { let t1808 = v_q.clone(); Py::Bool({ let t1809 = Py::tuple(vec![Py::Int(1i64), Py::Int(4i64)]); if py::contains(&t1809, &t1808)? { true } else { false } }) }; if py::ne(&t1804, &t1805) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_given = py::getitem(&Py::dict(vec![(Py::str("sin"), v_s.clone()), (Py::str("cos"), v_c.clone()), (Py::str("tan"), py::getitem(&v_o.clone(), &Py::str("tan"))?)]), &py::getitem(&v_i.clone(), &Py::str("func"))?)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::abs(&py::sub(&py::add(&py::mul(&v_s.clone(), &v_s.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?, &Py::Int(1i64))?)?, &f__rel(v_given.clone(), py::getitem(&v_i.clone(), &Py::str("value"))?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("tan"))?, &v_c.clone())?, v_s.clone())?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("sin2"))?, &py::mul(&py::mul(&Py::Int(2i64), &v_s.clone())?, &v_c.clone())?)?)?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("cos2"))?, &py::sub(&py::mul(&v_c.clone(), &v_c.clone())?, &py::mul(&v_s.clone(), &v_s.clone())?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_unit(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tv: Py = Py::None;
    v_t = py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("deg"))?)?;
    v_r = py::add(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("sin"))?, &py::math1("sin", &v_t.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("cos"))?, &py::math1("cos", &v_t.clone())?)?)?)?;
    if ({ let t1838 = { let t1840 = py::getitem(&v_o.clone(), &Py::str("tan"))?; Py::Bool({ let t1841 = Py::None; if py::is(&t1840, &t1841) { true } else { false } }) }; Py::Bool({ let t1839 = { let t1842 = py::abs(&py::math1("cos", &v_t.clone())?)?; Py::Bool({ let t1843 = Py::Float(1e-12); if py::lt(&t1842, &t1843)? { true } else { false } }) }; if py::ne(&t1838, &t1839) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t1844 = py::getitem(&v_o.clone(), &Py::str("tan"))?; Py::Bool({ let t1845 = Py::None; if !py::is(&t1844, &t1845) { true } else { false } }) }).truthy() {
        v_r = py::iadd(&v_r, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("tan"))?, &py::math1("tan", &v_t.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::math1("tan", &v_t.clone())?)?], true)?)?)?;
    }
    for t1846 in py::iter(&(Py::tuple(vec![Py::str("sin"), Py::str("cos"), Py::str("tan")])))? {
        v_k = t1846;
        v_tv = f__exact_text_value(py::getitem(&v_o.clone(), &py::add(&v_k.clone(), &Py::str("_exact"))?)?)?;
        if ({ let t1860 = { let t1849 = { let t1851 = v_tv.clone(); Py::Bool({ let t1852 = Py::None; if py::is(&t1851, &t1852) { true } else { false } }) }; Py::Bool({ let t1850 = { let t1853 = py::getitem(&v_o.clone(), &v_k.clone())?; Py::Bool({ let t1854 = Py::None; if py::is(&t1853, &t1854) { true } else { false } }) }; if py::ne(&t1849, &t1850) { true } else { false } }) }; if t1860.truthy() { t1860 } else { { let t1859 = { let t1855 = v_tv.clone(); Py::Bool({ let t1856 = Py::None; if !py::is(&t1855, &t1856) { true } else { false } }) }; if !t1859.truthy() { t1859 } else { { let t1857 = py::abs(&py::sub(&v_tv.clone(), &py::getitem(&v_o.clone(), &v_k.clone())?)?)?; Py::Bool({ let t1858 = Py::Float(1e-15); if py::gt(&t1857, &t1858)? { true } else { false } }) } } } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_vec(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_dot: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_mv: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t1916 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("u"))?, py::getitem(&v_i.clone(), &Py::str("v"))?])).items()?;
    if t1916.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t1916[0].clone();
    v_v = t1916[1].clone();
    let t1919 = (Py::tuple(vec![py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t1917 in py::iter(&(v_u.clone()))? { let mut v_a: Py; 
v_a = t1917;
out.push(py::mul(&v_a.clone(), &v_a.clone())?); } Py::list(out) }, None)?)?, py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t1918 in py::iter(&(v_v.clone()))? { let mut v_b: Py; 
v_b = t1918;
out.push(py::mul(&v_b.clone(), &v_b.clone())?); } Py::list(out) }, None)?)?])).items()?;
    if t1919.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mu = t1919[0].clone();
    v_mv = t1919[1].clone();
    v_dot = py::sum(&{ let mut out = vec![]; for t1920 in py::iter(&(py::zip(&[v_u.clone(), v_v.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1921 = (t1920).items()?;
if t1921.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1921[0].clone();
v_b = t1921[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?;
    v_t = py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("angle_deg"))?)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("dot"))?, v_dot.clone())?, &f__rel(py::getitem(&v_o.clone(), &Py::str("mag_u"))?, v_mu.clone())?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("mag_v"))?, v_mv.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&py::math1("cos", &v_t.clone())?, &v_mu.clone())?, &v_mv.clone())?, &v_dot.clone())?)?, &py::minmax(&[Py::Float(1.0), py::mul(&v_mu.clone(), &v_mv.clone())?], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&py::math1("sin", &v_t.clone())?, &v_mu.clone())?, &v_mv.clone())?, &f__cross_mag(v_u.clone(), v_v.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&v_mu.clone(), &v_mv.clone())?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__whole(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t253 = { let t249 = py::float(&v_v.clone())?; Py::Bool({ let t250 = py::int(&py::float(&v_v.clone())?)?; if py::ne(&t249, &t250) { true } else { false } }) }; if t253.truthy() { t253 } else { { let t251 = py::float(&v_v.clone())?; Py::Bool({ let t252 = Py::Int(0i64); if py::lt(&t251, &t252)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number >= 0".to_string()].concat()))));
    }
    return Ok(py::int(&py::float(&v_v.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_abs_linear_inequality(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_rel: Py) -> R {
    let mut v_between: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone())]))?;
    if ({ let t5 = v_a.clone(); Py::Bool({ let t6 = Py::Int(0i64); if py::eq(&t5, &t6) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0 (no x inside the absolute value)"))));
    }
    if ({ let t7 = v_c.clone(); Py::Bool({ let t8 = Py::Int(0i64); if py::le(&t7, &t8)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("c must be positive (with c <= 0 the solution set is empty, a point or every x)"))));
    }
    let t9 = (py::sorted(&Py::list(vec![py::truediv(&py::sub(&py::neg(&v_c.clone())?, &v_b.clone())?, &v_a.clone())?, py::truediv(&py::sub(&v_c.clone(), &v_b.clone())?, &v_a.clone())?]), (Py::Bool(false)).truthy())?).items()?;
    if t9.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t9[0].clone();
    v_hi = t9[1].clone();
    v_between = { let t10 = v_rel.clone(); Py::Bool({ let t11 = Py::tuple(vec![Py::str("<"), Py::str("<=")]); if py::contains(&t11, &t10)? { true } else { false } }) };
    return Ok(Py::dict(vec![(Py::str("lower"), v_lo.clone()), (Py::str("upper"), v_hi.clone()), (Py::str("region"), (if (v_between.clone()).truthy() { Py::str("between") } else { Py::str("outside") })), (Py::str("solution"), f__ineq_text(v_lo.clone(), v_hi.clone(), v_rel.clone(), v_between.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_binomial_coefficient_term(mut v_a: Py, mut v_b: Py, mut v_n: Py, mut v_k: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone())]))?;
    let t29 = (Py::tuple(vec![py::int(&v_n.clone())?, py::int(&v_k.clone())?])).items()?;
    if t29.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t29[0].clone();
    v_k = t29[1].clone();
    if ({ let t35 = py::not(&{ let t30 = Py::Int(0i64); Py::Bool({ let t31 = v_n.clone(); if py::le(&t30, &t31)? { { let t32 = Py::Int(60i64); if py::le(&t31, &t32)? { true } else { false } } } else { false } }) }); if t35.truthy() { t35 } else { { let t33 = v_k.clone(); Py::Bool({ let t34 = Py::Int(0i64); if py::lt(&t33, &t34)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be a whole number from 0 to 60 and k a whole number >= 0"))));
    }
    if ({ let t36 = v_k.clone(); Py::Bool({ let t37 = v_n.clone(); if py::gt(&t36, &t37)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("coefficient"), Py::Float(0.0)), (Py::str("binomial"), Py::Float(0.0))]));
    }
    let t38 = (Py::tuple(vec![f__fr(v_a.clone())?, f__fr(v_b.clone())?])).items()?;
    if t38.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t38[0].clone();
    v_B = t38[1].clone();
    return Ok(Py::dict(vec![(Py::str("coefficient"), py::float(&py::mul(&py::mul(&py::comb(&v_n.clone(), &v_k.clone())?, &py::pow(&v_A.clone(), &v_k.clone())?)?, &py::pow(&v_B.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?)?)?), (Py::str("binomial"), py::float(&py::comb(&v_n.clone(), &v_k.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_complex_arithmetic(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_op: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_im: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("d"), v_d.clone())]))?;
    let t59 = (Py::tuple(vec![py::complex(&[v_a.clone(), v_b.clone()])?, py::complex(&[v_c.clone(), v_d.clone()])?])).items()?;
    if t59.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_z = t59[0].clone();
    v_w = t59[1].clone();
    if ({ let t64 = { let t60 = v_op.clone(); Py::Bool({ let t61 = Py::str("/"); if py::eq(&t60, &t61) { true } else { false } }) }; if !t64.truthy() { t64 } else { { let t62 = v_w.clone(); Py::Bool({ let t63 = Py::Int(0i64); if py::eq(&t62, &t63) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("division by 0"))));
    }
    let t66 = ({ let mut out = vec![]; for t65 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()])))? { let mut v_v: Py; 
v_v = t65;
out.push(f__fr(v_v.clone())?); } Py::list(out) }).items()?;
    if t66.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t66[0].clone();
    v_B = t66[1].clone();
    v_C = t66[2].clone();
    v_D = t66[3].clone();
    if ({ let t67 = v_op.clone(); Py::Bool({ let t68 = Py::str("+"); if py::eq(&t67, &t68) { true } else { false } }) }).truthy() {
        let t69 = (Py::tuple(vec![py::add(&v_A.clone(), &v_C.clone())?, py::add(&v_B.clone(), &v_D.clone())?])).items()?;
        if t69.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_re_ = t69[0].clone();
        v_im = t69[1].clone();
    } else {
        if ({ let t70 = v_op.clone(); Py::Bool({ let t71 = Py::str("-"); if py::eq(&t70, &t71) { true } else { false } }) }).truthy() {
            let t72 = (Py::tuple(vec![py::sub(&v_A.clone(), &v_C.clone())?, py::sub(&v_B.clone(), &v_D.clone())?])).items()?;
            if t72.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_re_ = t72[0].clone();
            v_im = t72[1].clone();
        } else {
            if ({ let t73 = v_op.clone(); Py::Bool({ let t74 = Py::str("*"); if py::eq(&t73, &t74) { true } else { false } }) }).truthy() {
                let t75 = (Py::tuple(vec![py::sub(&py::mul(&v_A.clone(), &v_C.clone())?, &py::mul(&v_B.clone(), &v_D.clone())?)?, py::add(&py::mul(&v_A.clone(), &v_D.clone())?, &py::mul(&v_B.clone(), &v_C.clone())?)?])).items()?;
                if t75.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_re_ = t75[0].clone();
                v_im = t75[1].clone();
            } else {
                v_den = py::add(&py::mul(&v_C.clone(), &v_C.clone())?, &py::mul(&v_D.clone(), &v_D.clone())?)?;
                let t76 = (Py::tuple(vec![py::truediv(&py::add(&py::mul(&v_A.clone(), &v_C.clone())?, &py::mul(&v_B.clone(), &v_D.clone())?)?, &v_den.clone())?, py::truediv(&py::sub(&py::mul(&v_B.clone(), &v_C.clone())?, &py::mul(&v_A.clone(), &v_D.clone())?)?, &v_den.clone())?])).items()?;
                if t76.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_re_ = t76[0].clone();
                v_im = t76[1].clone();
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("re"), py::float(&v_re_.clone())?), (Py::str("im"), py::float(&v_im.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_complex_modulus_argument(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone())]))?;
    if ({ let t92 = { let t88 = v_a.clone(); Py::Bool({ let t89 = Py::Int(0i64); if py::eq(&t88, &t89) { true } else { false } }) }; if !t92.truthy() { t92 } else { { let t90 = v_b.clone(); Py::Bool({ let t91 = Py::Int(0i64); if py::eq(&t90, &t91) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("0 has modulus 0 and no argument"))));
    }
    v_r = py::hypot(&[v_a.clone(), v_b.clone()])?;
    v_t = py::math1("degrees", &py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&v_a.clone(), &v_r.clone())?], false)?], true)?)?)?;
    v_t = (if ({ let t93 = v_b.clone(); Py::Bool({ let t94 = Py::Int(0i64); if py::lt(&t93, &t94)? { true } else { false } }) }).truthy() { py::neg(&v_t.clone())? } else { v_t.clone() });
    return Ok(Py::dict(vec![(Py::str("modulus"), v_r.clone()), (Py::str("argument_deg"), v_t.clone()), (Py::str("argument_rad"), py::math1("radians", &v_t.clone())?), (Py::str("polar"), Py::str(&[py::format(&v_r.clone(), ".10g")?, "(cos(".to_string(), py::format(&v_t.clone(), ".10g")?, " deg) + i sin(".to_string(), py::format(&v_t.clone(), ".10g")?, " deg))".to_string()].concat()))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_complex_nth_roots(mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_snap: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone())]))?;
    v_n = py::int(&v_n.clone())?;
    if (py::not(&{ let t103 = Py::Int(2i64); Py::Bool({ let t104 = v_n.clone(); if py::le(&t103, &t104)? { { let t105 = Py::Int(12i64); if py::le(&t104, &t105)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be a whole number from 2 to 12"))));
    }
    if ({ let t110 = { let t106 = v_a.clone(); Py::Bool({ let t107 = Py::Int(0i64); if py::eq(&t106, &t107) { true } else { false } }) }; if !t110.truthy() { t110 } else { { let t108 = v_b.clone(); Py::Bool({ let t109 = Py::Int(0i64); if py::eq(&t108, &t109) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("0 has only the root 0"))));
    }
    v_r = py::pow(&py::hypot(&[v_a.clone(), v_b.clone()])?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?;
    v_t = py::modulo(&py::atan2(&v_b.clone(), &v_a.clone())?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?;
    v_roots = { let mut out = vec![]; for t111 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t111;
out.push(py::mul(&v_r.clone(), &py::complex(&[py::math1("cos", &py::truediv(&py::add(&v_t.clone(), &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?)?, &v_n.clone())?)?, py::math1("sin", &py::truediv(&py::add(&v_t.clone(), &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?)?, &v_n.clone())?)?])?)?); } Py::list(out) };
    v_snap = { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_n = v_n.clone(); let v_r = v_r.clone(); let v_roots = v_roots.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t112 = py::abs(&v_x.clone())?; Py::Bool({ let t113 = py::mul(&Py::Float(1e-12), &v_r.clone())?; if py::lt(&t112, &t113)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_roots = { let mut out = vec![]; for t114 in py::iter(&(v_roots.clone()))? { let mut v_z: Py; 
v_z = t114;
out.push(py::complex(&[py::call(&v_snap.clone(), &[py::real(&v_z.clone())?])?, py::call(&v_snap.clone(), &[py::imag(&v_z.clone())?])?])?); } Py::list(out) };
    let _ = py::method_sort(&v_roots.clone(), Some(&py::func(|a: &[Py]| -> R { f__arg360(a.get(0).cloned().unwrap_or(Py::None)) })), false)?;
    return Ok(Py::dict(vec![(Py::str("modulus"), v_r.clone()), (Py::str("count"), py::float(&v_n.clone())?), (Py::str("roots"), v_roots.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_composite_function_value(mut v_f: Py, mut v_g: Py, mut v_x: Py, mut v_order: Py) -> R {
    let mut v_exc: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_inner: Py = Py::None;
    let mut v_second: Py = Py::None;
    let mut v_value: Py = Py::None;
    let t131 = ((if ({ let t129 = v_order.clone(); Py::Bool({ let t130 = Py::str("f(g(x))"); if py::eq(&t129, &t130) { true } else { false } }) }).truthy() { Py::tuple(vec![v_g.clone(), v_f.clone()]) } else { Py::tuple(vec![v_f.clone(), v_g.clone()]) })).items()?;
    if t131.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_first = t131[0].clone();
    v_second = t131[1].clone();
    let t236: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_inner = f__fval(v_first.clone(), v_x.clone())?;
            v_value = f__fval(v_second.clone(), (if (py::not(&Py::Bool(matches!(v_inner.clone(), Py::Frac(_))))).truthy() { py::float(&v_inner.clone())? } else { v_inner.clone() }))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t236 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            v_exc = Py::str(&match &e { PyErr::Value(m) => m.clone(), other => format!("{:?}", other) });
            return Err(py::value_error(py::to_str(&Py::str(&["the composition is not defined at x = ".to_string(), py::format(&v_x.clone(), "g")?, ": ".to_string(), py::format(&v_exc.clone(), "")?].concat()))));
        }
        Err(e) => return Err(e),
    }
    return Ok(Py::dict(vec![(Py::str("inner"), py::float(&v_inner.clone())?), (Py::str("value"), py::float(&v_value.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_continuous_growth(mut v_n0: Py, mut v_k: Py, mut v_t: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("n0"), v_n0.clone()), (Py::str("k"), v_k.clone()), (Py::str("t"), v_t.clone())]))?;
    if ({ let t245 = py::abs(&py::mul(&v_k.clone(), &v_t.clone())?)?; Py::Bool({ let t246 = Py::Int(700i64); if py::gt(&t245, &t246)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("k*t is too large for a finite result"))));
    }
    return Ok(Py::dict(vec![(Py::str("amount"), py::mul(&v_n0.clone(), &py::math1("exp", &py::mul(&v_k.clone(), &v_t.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_counting_choices(mut v_n: Py, mut v_k: Py) -> R {
    let t254 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"))?, f__whole(v_k.clone(), Py::str("k"))?])).items()?;
    if t254.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t254[0].clone();
    v_k = t254[1].clone();
    if ({ let t255 = v_k.clone(); Py::Bool({ let t256 = v_n.clone(); if py::gt(&t255, &t256)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("k must not exceed n"))));
    }
    if ({ let t257 = v_n.clone(); Py::Bool({ let t258 = c_COUNTING_MAX_N()?; if py::gt(&t257, &t258)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["n must be at most ".to_string(), py::format(&c_COUNTING_MAX_N()?, "")?, " (the result would not fit in a floating-point number)".to_string()].concat()))));
    }
    return Ok(Py::dict(vec![(Py::str("combinations"), py::float(&py::comb(&v_n.clone(), &v_k.clone())?)?), (Py::str("permutations"), py::float(&py::perm(&v_n.clone(), &v_k.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cubic_roots(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py) -> R {
    return Ok(f__roots_out(f__poly_roots(Py::list(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()]))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_de_moivre_power(mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_im: Py = Py::None;
    let mut v_mod: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let mut v_ri: Py = Py::None;
    let mut v_rr: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone())]))?;
    v_n = py::int(&v_n.clone())?;
    if ({ let t287 = py::abs(&v_n.clone())?; Py::Bool({ let t288 = Py::Int(40i64); if py::gt(&t287, &t288)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the power is limited to 40 in size"))));
    }
    if ({ let t296 = { let t289 = v_a.clone(); Py::Bool({ let t290 = Py::Int(0i64); if py::eq(&t289, &t290) { true } else { false } }) }; if !t296.truthy() { t296 } else { { let t295 = { let t291 = v_b.clone(); Py::Bool({ let t292 = Py::Int(0i64); if py::eq(&t291, &t292) { true } else { false } }) }; if !t295.truthy() { t295 } else { { let t293 = v_n.clone(); Py::Bool({ let t294 = Py::Int(0i64); if py::le(&t293, &t294)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("0 to a power that is not positive is undefined"))));
    }
    let t303 = (f__cpow_exact(f__fr(v_a.clone())?, f__fr(v_b.clone())?, v_n.clone())?).items()?;
    if t303.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rr = t303[0].clone();
    v_ri = t303[1].clone();
    let t304 = (Py::tuple(vec![py::float(&v_rr.clone())?, py::float(&v_ri.clone())?])).items()?;
    if t304.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t304[0].clone();
    v_im = t304[1].clone();
    let t309 = (Py::tuple(vec![(if ({ let t305 = py::abs(&v_re_.clone())?; Py::Bool({ let t306 = Py::Float(1e-12); if py::lt(&t305, &t306)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_re_.clone() }), (if ({ let t307 = py::abs(&v_im.clone())?; Py::Bool({ let t308 = Py::Float(1e-12); if py::lt(&t307, &t308)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_im.clone() })])).items()?;
    if t309.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t309[0].clone();
    v_im = t309[1].clone();
    v_mod = py::hypot(&[v_re_.clone(), v_im.clone()])?;
    return Ok(Py::dict(vec![(Py::str("re"), v_re_.clone()), (Py::str("im"), v_im.clone()), (Py::str("modulus"), v_mod.clone()), (Py::str("argument_deg"), (if (v_mod.clone()).truthy() { py::math1("degrees", &py::atan2(&v_im.clone(), &v_re_.clone())?)? } else { Py::Float(0.0) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_deg_to_rad(mut v_deg: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_exact: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_n: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("deg"), v_deg.clone())]))?;
    v_f = py::truediv(&f__fr(v_deg.clone())?, &Py::Int(180i64))?;
    v_exact = Py::str("");
    if ({ let t322 = py::denominator(&v_f.clone())?; Py::Bool({ let t323 = Py::Int(360i64); if py::le(&t322, &t323)? { true } else { false } }) }).truthy() {
        let t324 = (Py::tuple(vec![py::numerator(&v_f.clone())?, py::denominator(&v_f.clone())?])).items()?;
        if t324.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t324[0].clone();
        v_d = t324[1].clone();
        v_exact = (if ({ let t325 = v_n.clone(); Py::Bool({ let t326 = Py::Int(0i64); if py::eq(&t325, &t326) { true } else { false } }) }).truthy() { Py::str("0") } else { py::add(&py::add(&py::add(&(if ({ let t327 = v_n.clone(); Py::Bool({ let t328 = Py::Int(0i64); if py::lt(&t327, &t328)? { true } else { false } }) }).truthy() { Py::str("-") } else { Py::str("") }), &(if ({ let t329 = py::abs(&v_n.clone())?; Py::Bool({ let t330 = Py::Int(1i64); if py::eq(&t329, &t330) { true } else { false } }) }).truthy() { Py::str("") } else { Py::str(&py::to_str(&py::abs(&v_n.clone())?)) }))?, &Py::str("pi"))?, &(if ({ let t331 = v_d.clone(); Py::Bool({ let t332 = Py::Int(1i64); if py::eq(&t331, &t332) { true } else { false } }) }).truthy() { Py::str("") } else { Py::str(&["/".to_string(), py::format(&v_d.clone(), "")?].concat()) }))? });
    }
    return Ok(Py::dict(vec![(Py::str("radians"), py::truediv(&py::mul(&v_deg.clone(), &Py::Float(std::f64::consts::PI))?, &Py::Int(180i64))?), (Py::str("pi_multiple"), py::float(&v_f.clone())?), (Py::str("exact"), v_exact.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_discriminant(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_nature: Py = Py::None;
    let _ = f__check(v_a.clone(), v_b.clone(), v_c.clone())?;
    v_d = py::sub(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_a.clone())?, &v_c.clone())?)?;
    v_nature = (if ({ let t1255 = v_d.clone(); Py::Bool({ let t1256 = Py::Int(0i64); if py::gt(&t1255, &t1256)? { true } else { false } }) }).truthy() { Py::str("two distinct real roots") } else { (if ({ let t1257 = v_d.clone(); Py::Bool({ let t1258 = Py::Int(0i64); if py::eq(&t1257, &t1258) { true } else { false } }) }).truthy() { Py::str("one repeated real root") } else { Py::str("two complex conjugate roots") }) });
    return Ok(Py::dict(vec![(Py::str("discriminant"), v_d.clone()), (Py::str("nature"), v_nature.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_distance_point_to_line(mut v_x0: Py, mut v_y0: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("x0"), v_x0.clone()), (Py::str("y0"), v_y0.clone()), (Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone())]))?;
    if ({ let t345 = { let t341 = v_a.clone(); Py::Bool({ let t342 = Py::Int(0i64); if py::eq(&t341, &t342) { true } else { false } }) }; if !t345.truthy() { t345 } else { { let t343 = v_b.clone(); Py::Bool({ let t344 = Py::Int(0i64); if py::eq(&t343, &t344) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a and b must not both be 0 (that is not a line)"))));
    }
    return Ok(Py::dict(vec![(Py::str("distance"), py::truediv(&py::abs(&py::add(&py::add(&py::mul(&v_a.clone(), &v_x0.clone())?, &py::mul(&v_b.clone(), &v_y0.clone())?)?, &v_c.clone())?)?, &py::hypot(&[v_a.clone(), v_b.clone()])?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_domain_sqrt_over_linear(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_root_in_denominator: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_side: Py = Py::None;
    let mut v_text: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("d"), v_d.clone())]))?;
    if ({ let t352 = v_a.clone(); Py::Bool({ let t353 = Py::Int(0i64); if py::eq(&t352, &t353) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0 (no x under the root)"))));
    }
    if ({ let t358 = { let t354 = v_c.clone(); Py::Bool({ let t355 = Py::Int(0i64); if py::eq(&t354, &t355) { true } else { false } }) }; if !t358.truthy() { t358 } else { { let t356 = v_d.clone(); Py::Bool({ let t357 = Py::Int(0i64); if py::eq(&t356, &t357) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the denominator is 0 for every x"))));
    }
    let t380 = (f__domain_parts(v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone(), Py::Bool((v_root_in_denominator.clone()).truthy()))?).items()?;
    if t380.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_B = t380[0].clone();
    v_side = t380[1].clone();
    v_E = t380[2].clone();
    v_text = t380[3].clone();
    return Ok(Py::dict(vec![(Py::str("boundary"), py::float(&v_B.clone())?), (Py::str("side"), v_side.clone()), (Py::str("excluded"), (if ({ let t381 = v_E.clone(); Py::Bool({ let t382 = Py::None; if !py::is(&t381, &t382) { true } else { false } }) }).truthy() { py::float(&v_E.clone())? } else { Py::None })), (Py::str("domain"), v_text.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_doubling_time_percent(mut v_r: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("r"), v_r.clone())]))?;
    if ({ let t400 = v_r.clone(); Py::Bool({ let t401 = Py::Int(0i64); if py::le(&t400, &t401)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the growth rate must be positive"))));
    }
    return Ok(Py::dict(vec![(Py::str("doubling_time"), py::truediv(&py::math1("log", &Py::Int(2i64))?, &py::math1("log1p", &py::truediv(&v_r.clone(), &Py::Int(100i64))?)?)?), (Py::str("rule_of_70_estimate"), py::truediv(&Py::Int(70i64), &v_r.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ellipse_parameters(mut v_h: Py, mut v_k: Py, mut v_A: Py, mut v_B: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("h"), v_h.clone()), (Py::str("k"), v_k.clone()), (Py::str("A"), v_A.clone()), (Py::str("B"), v_B.clone())]))?;
    if ({ let t412 = { let t408 = v_A.clone(); Py::Bool({ let t409 = Py::Int(0i64); if py::le(&t408, &t409)? { true } else { false } }) }; if t412.truthy() { t412 } else { { let t410 = v_B.clone(); Py::Bool({ let t411 = Py::Int(0i64); if py::le(&t410, &t411)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("both denominators must be positive"))));
    }
    if ({ let t413 = v_A.clone(); Py::Bool({ let t414 = v_B.clone(); if py::eq(&t413, &t414) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("equal denominators give a circle (no distinct foci)"))));
    }
    let t415 = (Py::tuple(vec![py::math1("sqrt", &py::minmax(&[v_A.clone(), v_B.clone()], true)?)?, py::math1("sqrt", &py::minmax(&[v_A.clone(), v_B.clone()], false)?)?])).items()?;
    if t415.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t415[0].clone();
    v_b = t415[1].clone();
    v_c = py::math1("sqrt", &py::abs(&py::sub(&v_A.clone(), &v_B.clone())?)?)?;
    if ({ let t416 = v_A.clone(); Py::Bool({ let t417 = v_B.clone(); if py::gt(&t416, &t417)? { true } else { false } }) }).truthy() {
        v_f = Py::list(vec![Py::tuple(vec![py::sub(&v_h.clone(), &v_c.clone())?, v_k.clone()]), Py::tuple(vec![py::add(&v_h.clone(), &v_c.clone())?, v_k.clone()])]);
    } else {
        v_f = Py::list(vec![Py::tuple(vec![v_h.clone(), py::sub(&v_k.clone(), &v_c.clone())?]), Py::tuple(vec![v_h.clone(), py::add(&v_k.clone(), &v_c.clone())?])]);
    }
    return Ok(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("eccentricity"), py::truediv(&v_c.clone(), &v_a.clone())?), (Py::str("f1x"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?), (Py::str("f1y"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?), (Py::str("f2x"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?), (Py::str("f2y"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?), (Py::str("orientation"), (if ({ let t418 = v_A.clone(); Py::Bool({ let t419 = v_B.clone(); if py::gt(&t418, &t419)? { true } else { false } }) }).truthy() { Py::str("horizontal") } else { Py::str("vertical") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_evaluate_function(mut v_expression: Py, mut v_x: Py) -> R {
    let mut v_exc: Py = Py::None;
    let t528: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            return Ok(Some(f__exact_outputs(f__fval(v_expression.clone(), v_x.clone())?)?));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t528 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            v_exc = Py::str(&match &e { PyErr::Value(m) => m.clone(), other => format!("{:?}", other) });
            return Err(py::value_error(py::to_str(&Py::str(&["f(".to_string(), py::format(&v_x.clone(), "g")?, ") is not defined: ".to_string(), py::format(&v_exc.clone(), "")?].concat()))));
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_exponential_equation_general(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_k: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("d"), v_d.clone()), (Py::str("k"), v_k.clone())]))?;
    if ({ let t539 = { let t535 = v_a.clone(); Py::Bool({ let t536 = Py::Int(0i64); if py::eq(&t535, &t536) { true } else { false } }) }; if t539.truthy() { t539 } else { { let t537 = v_c.clone(); Py::Bool({ let t538 = Py::Int(0i64); if py::eq(&t537, &t538) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a and c must not be 0"))));
    }
    if ({ let t544 = { let t540 = v_b.clone(); Py::Bool({ let t541 = Py::Int(0i64); if py::le(&t540, &t541)? { true } else { false } }) }; if t544.truthy() { t544 } else { { let t542 = v_b.clone(); Py::Bool({ let t543 = Py::Int(1i64); if py::eq(&t542, &t543) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the base must be positive and not 1"))));
    }
    if ({ let t545 = py::truediv(&v_k.clone(), &v_a.clone())?; Py::Bool({ let t546 = Py::Int(0i64); if py::le(&t545, &t546)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no real solution: b^(cx + d) is positive, so k/a must be positive"))));
    }
    return Ok(Py::dict(vec![(Py::str("x"), py::truediv(&py::sub(&py::truediv(&py::math1("log", &py::truediv(&v_k.clone(), &v_a.clone())?)?, &py::math1("log", &v_b.clone())?)?, &v_d.clone())?, &v_c.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_factor_quadratic_integer(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_fac: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_text: Py = Py::None;
    let t548 = (Py::tuple(vec![py::int(&v_a.clone())?, py::int(&v_b.clone())?, py::int(&v_c.clone())?])).items()?;
    if t548.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t548[0].clone();
    v_b = t548[1].clone();
    v_c = t548[2].clone();
    if ({ let t549 = v_a.clone(); Py::Bool({ let t550 = Py::Int(0i64); if py::eq(&t549, &t550) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0 (not a quadratic)"))));
    }
    v_g = py::mul(&py::gcd(&py::gcd(&v_a.clone(), &v_b.clone())?, &v_c.clone())?, &(if ({ let t551 = v_a.clone(); Py::Bool({ let t552 = Py::Int(0i64); if py::gt(&t551, &t552)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? }))?;
    let t553 = (Py::tuple(vec![py::floordiv(&v_a.clone(), &v_g.clone())?, py::floordiv(&v_b.clone(), &v_g.clone())?, py::floordiv(&v_c.clone(), &v_g.clone())?])).items()?;
    if t553.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t553[0].clone();
    v_B = t553[1].clone();
    v_C = t553[2].clone();
    v_D = py::sub(&py::mul(&v_B.clone(), &v_B.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_A.clone())?, &v_C.clone())?)?;
    if ({ let t558 = { let t554 = v_D.clone(); Py::Bool({ let t555 = Py::Int(0i64); if py::lt(&t554, &t555)? { true } else { false } }) }; if t558.truthy() { t558 } else { { let t556 = py::pow(&py::isqrt(&v_D.clone())?, &Py::Int(2i64))?; Py::Bool({ let t557 = v_D.clone(); if py::ne(&t556, &t557) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this quadratic does not factor over the integers (its discriminant is not a perfect square)"))));
    }
    v_s = py::isqrt(&v_D.clone())?;
    let t559 = (Py::tuple(vec![py::fraction(&[py::sub(&py::neg(&v_B.clone())?, &v_s.clone())?, py::mul(&Py::Int(2i64), &v_A.clone())?])?, py::fraction(&[py::add(&py::neg(&v_B.clone())?, &v_s.clone())?, py::mul(&Py::Int(2i64), &v_A.clone())?])?])).items()?;
    if t559.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t559[0].clone();
    v_r2 = t559[1].clone();
    let t560 = (Py::tuple(vec![py::denominator(&v_r1.clone())?, py::neg(&py::numerator(&v_r1.clone())?)?, py::denominator(&v_r2.clone())?, py::neg(&py::numerator(&v_r2.clone())?)?])).items()?;
    if t560.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t560[0].clone();
    v_q = t560[1].clone();
    v_r = t560[2].clone();
    v_t = t560[3].clone();
    if ({ let t561 = py::mul(&v_p.clone(), &v_r.clone())?; Py::Bool({ let t562 = v_A.clone(); if py::ne(&t561, &t562) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("internal: the factors do not reproduce the leading coefficient"))));
    }
    v_fac = { let v_A = v_A.clone(); let v_B = v_B.clone(); let v_C = v_C.clone(); let v_D = v_D.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_g = v_g.clone(); let v_p = v_p.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); let v_s = v_s.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_m: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_n: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::add(&(if ({ let t563 = v_m.clone(); Py::Bool({ let t564 = Py::Int(1i64); if py::eq(&t563, &t564) { true } else { false } }) }).truthy() { Py::str("x") } else { Py::str(&[py::format(&v_m.clone(), "")?, "x".to_string()].concat()) }), &(if ({ let t565 = v_n.clone(); Py::Bool({ let t566 = Py::Int(0i64); if py::gt(&t565, &t566)? { true } else { false } }) }).truthy() { Py::str(&[" + ".to_string(), py::format(&v_n.clone(), "")?].concat()) } else { (if ({ let t567 = v_n.clone(); Py::Bool({ let t568 = Py::Int(0i64); if py::lt(&t567, &t568)? { true } else { false } }) }).truthy() { Py::str(&[" - ".to_string(), py::format(&py::neg(&v_n.clone())?, "")?].concat()) } else { Py::str("") }) }))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_text = py::add(&(if ({ let t569 = v_g.clone(); Py::Bool({ let t570 = Py::Int(1i64); if py::eq(&t569, &t570) { true } else { false } }) }).truthy() { Py::str("") } else { (if ({ let t571 = v_g.clone(); Py::Bool({ let t572 = py::neg(&Py::Int(1i64))?; if py::eq(&t571, &t572) { true } else { false } }) }).truthy() { Py::str("-") } else { Py::str(&[py::format(&v_g.clone(), "")?].concat()) }) }), &Py::str(&["(".to_string(), py::format(&py::call(&v_fac.clone(), &[v_p.clone(), v_q.clone()])?, "")?, ")(".to_string(), py::format(&py::call(&v_fac.clone(), &[v_r.clone(), v_t.clone()])?, "")?, ")".to_string()].concat()))?;
    return Ok(Py::dict(vec![(Py::str("g"), py::float(&v_g.clone())?), (Py::str("p"), py::float(&v_p.clone())?), (Py::str("q"), py::float(&v_q.clone())?), (Py::str("r"), py::float(&v_r.clone())?), (Py::str("s"), py::float(&v_t.clone())?), (Py::str("factored"), v_text.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fraction_arithmetic(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_op: Py) -> R {
    let mut v_f1: Py = Py::None;
    let mut v_f2: Py = Py::None;
    let mut v_r: Py = Py::None;
    if ({ let t597 = { let t593 = v_b.clone(); Py::Bool({ let t594 = Py::Int(0i64); if py::eq(&t593, &t594) { true } else { false } }) }; if t597.truthy() { t597 } else { { let t595 = v_d.clone(); Py::Bool({ let t596 = Py::Int(0i64); if py::eq(&t595, &t596) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a denominator must not be 0"))));
    }
    let t598 = (Py::tuple(vec![py::fraction(&[py::int(&v_a.clone())?, py::int(&v_b.clone())?])?, py::fraction(&[py::int(&v_c.clone())?, py::int(&v_d.clone())?])?])).items()?;
    if t598.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f1 = t598[0].clone();
    v_f2 = t598[1].clone();
    if ({ let t603 = { let t599 = v_op.clone(); Py::Bool({ let t600 = Py::str("/"); if py::eq(&t599, &t600) { true } else { false } }) }; if !t603.truthy() { t603 } else { { let t601 = v_f2.clone(); Py::Bool({ let t602 = Py::Int(0i64); if py::eq(&t601, &t602) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("division by the fraction 0"))));
    }
    v_r = py::getitem(&Py::dict(vec![(Py::str("+"), py::add(&v_f1.clone(), &v_f2.clone())?), (Py::str("-"), py::sub(&v_f1.clone(), &v_f2.clone())?), (Py::str("*"), py::mul(&v_f1.clone(), &v_f2.clone())?), (Py::str("/"), (if (v_f2.clone()).truthy() { py::truediv(&v_f1.clone(), &v_f2.clone())? } else { Py::None }))]), &v_op.clone())?;
    return Ok(Py::dict(vec![(Py::str("numerator"), py::float(&py::numerator(&v_r.clone())?)?), (Py::str("denominator"), py::float(&py::denominator(&v_r.clone())?)?), (Py::str("value"), py::float(&v_r.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_graph_transformation_point(mut v_x0: Py, mut v_y0: Py, mut v_A: Py, mut v_B: Py, mut v_h: Py, mut v_k: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("x0"), v_x0.clone()), (Py::str("y0"), v_y0.clone()), (Py::str("A"), v_A.clone()), (Py::str("B"), v_B.clone()), (Py::str("h"), v_h.clone()), (Py::str("k"), v_k.clone())]))?;
    if ({ let t626 = { let t622 = v_A.clone(); Py::Bool({ let t623 = Py::Int(0i64); if py::eq(&t622, &t623) { true } else { false } }) }; if t626.truthy() { t626 } else { { let t624 = v_B.clone(); Py::Bool({ let t625 = Py::Int(0i64); if py::eq(&t624, &t625) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A and B must not be 0 (the graph would collapse)"))));
    }
    return Ok(Py::dict(vec![(Py::str("x_new"), py::add(&py::truediv(&v_x0.clone(), &v_B.clone())?, &v_h.clone())?), (Py::str("y_new"), py::add(&py::mul(&v_A.clone(), &v_y0.clone())?, &v_k.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_half_life_remaining(mut v_n0: Py, mut v_half_life: Py, mut v_t: Py) -> R {
    let mut v_frac: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("n0"), v_n0.clone()), (Py::str("half_life"), v_half_life.clone()), (Py::str("t"), v_t.clone())]))?;
    if ({ let t640 = { let t633 = v_n0.clone(); Py::Bool({ let t634 = Py::Int(0i64); if py::le(&t633, &t634)? { true } else { false } }) }; if t640.truthy() { t640 } else { { let t639 = { let t635 = v_half_life.clone(); Py::Bool({ let t636 = Py::Int(0i64); if py::le(&t635, &t636)? { true } else { false } }) }; if t639.truthy() { t639 } else { { let t637 = v_t.clone(); Py::Bool({ let t638 = Py::Int(0i64); if py::lt(&t637, &t638)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the amount and the half-life must be positive and the time not negative"))));
    }
    v_frac = py::pow(&Py::Float(0.5), &py::truediv(&v_t.clone(), &v_half_life.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("remaining"), py::mul(&v_n0.clone(), &v_frac.clone())?), (Py::str("fraction"), v_frac.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_heron_area(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_area: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__tri_check(v_a.clone(), v_b.clone(), v_c.clone())?;
    v_s = py::truediv(&py::add(&py::add(&v_a.clone(), &v_b.clone())?, &v_c.clone())?, &Py::Int(2i64))?;
    let t658 = (py::sorted(&Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone()]), (Py::Bool(true)).truthy())?).items()?;
    if t658.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t658[0].clone();
    v_y = t658[1].clone();
    v_z = t658[2].clone();
    v_area = py::mul(&Py::Float(0.25), &py::math1("sqrt", &py::mul(&py::mul(&py::mul(&py::add(&v_x.clone(), &py::add(&v_y.clone(), &v_z.clone())?)?, &py::sub(&v_z.clone(), &py::sub(&v_x.clone(), &v_y.clone())?)?)?, &py::add(&v_z.clone(), &py::sub(&v_x.clone(), &v_y.clone())?)?)?, &py::add(&v_x.clone(), &py::sub(&v_y.clone(), &v_z.clone())?)?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("area"), v_area.clone()), (Py::str("s"), v_s.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hyperbola_parameters(mut v_h: Py, mut v_k: Py, mut v_A: Py, mut v_B: Py, mut v_orientation: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("h"), v_h.clone()), (Py::str("k"), v_k.clone()), (Py::str("A"), v_A.clone()), (Py::str("B"), v_B.clone())]))?;
    if ({ let t669 = { let t665 = v_A.clone(); Py::Bool({ let t666 = Py::Int(0i64); if py::le(&t665, &t666)? { true } else { false } }) }; if t669.truthy() { t669 } else { { let t667 = v_B.clone(); Py::Bool({ let t668 = Py::Int(0i64); if py::le(&t667, &t668)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("both denominators must be positive"))));
    }
    if ({ let t670 = v_orientation.clone(); Py::Bool({ let t671 = c__ORIENT()?; if !py::contains(&t671, &t670)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("orientation is horizontal or vertical"))));
    }
    let t672 = (Py::tuple(vec![py::math1("sqrt", &v_A.clone())?, py::math1("sqrt", &v_B.clone())?])).items()?;
    if t672.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t672[0].clone();
    v_b = t672[1].clone();
    v_c = py::math1("sqrt", &py::add(&v_A.clone(), &v_B.clone())?)?;
    v_f = (if ({ let t673 = v_orientation.clone(); Py::Bool({ let t674 = Py::str("horizontal"); if py::eq(&t673, &t674) { true } else { false } }) }).truthy() { Py::list(vec![Py::tuple(vec![py::sub(&v_h.clone(), &v_c.clone())?, v_k.clone()]), Py::tuple(vec![py::add(&v_h.clone(), &v_c.clone())?, v_k.clone()])]) } else { Py::list(vec![Py::tuple(vec![v_h.clone(), py::sub(&v_k.clone(), &v_c.clone())?]), Py::tuple(vec![v_h.clone(), py::add(&v_k.clone(), &v_c.clone())?])]) });
    return Ok(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("eccentricity"), py::truediv(&v_c.clone(), &v_a.clone())?), (Py::str("asymptote_slope"), (if ({ let t675 = v_orientation.clone(); Py::Bool({ let t676 = Py::str("horizontal"); if py::eq(&t675, &t676) { true } else { false } }) }).truthy() { py::truediv(&v_b.clone(), &v_a.clone())? } else { py::truediv(&v_a.clone(), &v_b.clone())? })), (Py::str("f1x"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?), (Py::str("f1y"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?), (Py::str("f2x"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?), (Py::str("f2y"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_infinite_geometric_series(mut v_a: Py, mut v_r: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("r"), v_r.clone())]))?;
    if (py::not(&{ let t689 = py::abs(&v_r.clone())?; Py::Bool({ let t690 = Py::Int(1i64); if py::lt(&t689, &t690)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the series diverges unless |r| < 1"))));
    }
    return Ok(Py::dict(vec![(Py::str("sum"), py::truediv(&v_a.clone(), &py::sub(&Py::Int(1i64), &v_r.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inverse_linear_function(mut v_a: Py, mut v_b: Py, mut v_y: Py) -> R {
    let mut v_out: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone())]))?;
    if ({ let t696 = v_a.clone(); Py::Bool({ let t697 = Py::Int(0i64); if py::eq(&t696, &t697) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0 (a constant function has no inverse)"))));
    }
    v_out = Py::dict(vec![(Py::str("inv_slope"), py::truediv(&Py::Int(1i64), &v_a.clone())?), (Py::str("inv_intercept"), py::truediv(&py::neg(&v_b.clone())?, &v_a.clone())?)]);
    if ({ let t698 = v_y.clone(); Py::Bool({ let t699 = Py::None; if !py::is(&t698, &t699) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::dict(vec![(Py::str("y"), v_y.clone())]))?;
        py::setitem(&v_out.clone(), &Py::str("value"), py::truediv(&py::sub(&v_y.clone(), &v_b.clone())?, &v_a.clone())?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_law_of_cosines_sas(mut v_b: Py, mut v_c: Py, mut v_A: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_a: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("A"), v_A.clone())]))?;
    if ({ let t717 = { let t709 = v_b.clone(); Py::Bool({ let t710 = Py::Int(0i64); if py::le(&t709, &t710)? { true } else { false } }) }; if t717.truthy() { t717 } else { { let t716 = { let t711 = v_c.clone(); Py::Bool({ let t712 = Py::Int(0i64); if py::le(&t711, &t712)? { true } else { false } }) }; if t716.truthy() { t716 } else { py::not(&{ let t713 = Py::Int(0i64); Py::Bool({ let t714 = v_A.clone(); if py::lt(&t713, &t714)? { { let t715 = Py::Int(180i64); if py::lt(&t714, &t715)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the sides must be positive and A between 0 and 180 degrees"))));
    }
    v_a = py::math1("sqrt", &py::sub(&py::add(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_b.clone())?, &v_c.clone())?, &py::math1("cos", &py::math1("radians", &v_A.clone())?)?)?)?)?;
    v_B = py::math1("degrees", &py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&py::sub(&py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_c.clone())?)?], false)?], true)?)?)?;
    return Ok(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("B"), v_B.clone()), (Py::str("C"), py::sub(&py::sub(&Py::Float(180.0), &v_A.clone())?, &v_B.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_law_of_cosines_sss(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_ang: Py = Py::None;
    let _ = f__tri_check(v_a.clone(), v_b.clone(), v_c.clone())?;
    v_ang = { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_z: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::math1("degrees", &py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&py::sub(&py::add(&py::mul(&v_y.clone(), &v_y.clone())?, &py::mul(&v_z.clone(), &v_z.clone())?)?, &py::mul(&v_x.clone(), &v_x.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_y.clone())?, &v_z.clone())?)?], false)?], true)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t734 = (Py::tuple(vec![py::call(&v_ang.clone(), &[v_a.clone(), v_b.clone(), v_c.clone()])?, py::call(&v_ang.clone(), &[v_b.clone(), v_a.clone(), v_c.clone()])?])).items()?;
    if t734.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t734[0].clone();
    v_B = t734[1].clone();
    return Ok(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("B"), v_B.clone()), (Py::str("C"), py::sub(&py::sub(&Py::Float(180.0), &v_A.clone())?, &v_B.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_law_of_sines_aas(mut v_A: Py, mut v_B: Py, mut v_a: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("B"), v_B.clone()), (Py::str("a"), v_a.clone())]))?;
    if ({ let t747 = { let t740 = v_A.clone(); Py::Bool({ let t741 = Py::Int(0i64); if py::le(&t740, &t741)? { true } else { false } }) }; if t747.truthy() { t747 } else { { let t746 = { let t742 = v_B.clone(); Py::Bool({ let t743 = Py::Int(0i64); if py::le(&t742, &t743)? { true } else { false } }) }; if t746.truthy() { t746 } else { { let t744 = py::add(&v_A.clone(), &v_B.clone())?; Py::Bool({ let t745 = Py::Int(180i64); if py::ge(&t744, &t745)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two angles must be positive with a sum below 180 degrees"))));
    }
    if ({ let t748 = v_a.clone(); Py::Bool({ let t749 = Py::Int(0i64); if py::le(&t748, &t749)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the side must be positive"))));
    }
    v_C = py::sub(&py::sub(&Py::Float(180.0), &v_A.clone())?, &v_B.clone())?;
    v_k = py::truediv(&v_a.clone(), &f__deg_sin(v_A.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("b"), py::mul(&v_k.clone(), &f__deg_sin(v_B.clone())?)?), (Py::str("c"), py::mul(&v_k.clone(), &f__deg_sin(v_C.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_law_of_sines_ssa(mut v_a: Py, mut v_b: Py, mut v_A: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_B1: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sols: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("A"), v_A.clone())]))?;
    if ({ let t764 = { let t756 = v_a.clone(); Py::Bool({ let t757 = Py::Int(0i64); if py::le(&t756, &t757)? { true } else { false } }) }; if t764.truthy() { t764 } else { { let t763 = { let t758 = v_b.clone(); Py::Bool({ let t759 = Py::Int(0i64); if py::le(&t758, &t759)? { true } else { false } }) }; if t763.truthy() { t763 } else { py::not(&{ let t760 = Py::Int(0i64); Py::Bool({ let t761 = v_A.clone(); if py::lt(&t760, &t761)? { { let t762 = Py::Int(180i64); if py::lt(&t761, &t762)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the sides must be positive and A between 0 and 180 degrees"))));
    }
    v_s = py::truediv(&py::mul(&v_b.clone(), &f__deg_sin(v_A.clone())?)?, &v_a.clone())?;
    if ({ let t765 = v_s.clone(); Py::Bool({ let t766 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::gt(&t765, &t766)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no triangle exists: side a is shorter than the height b*sin(A)"))));
    }
    v_sols = Py::list(vec![]);
    v_B1 = (if ({ let t767 = v_s.clone(); Py::Bool({ let t768 = py::sub(&Py::Int(1i64), &Py::Float(1e-12))?; if py::ge(&t767, &t768)? { true } else { false } }) }).truthy() { Py::Float(90.0) } else { py::math1("degrees", &py::math1("asin", &v_s.clone())?)? });
    for t771 in py::iter(&((if ({ let t769 = v_B1.clone(); Py::Bool({ let t770 = Py::Float(90.0); if py::eq(&t769, &t770) { true } else { false } }) }).truthy() { Py::list(vec![v_B1.clone()]) } else { Py::list(vec![v_B1.clone(), py::sub(&Py::Float(180.0), &v_B1.clone())?]) })))? {
        v_B = t771;
        v_C = py::sub(&py::sub(&Py::Float(180.0), &v_A.clone())?, &v_B.clone())?;
        if ({ let t772 = v_C.clone(); Py::Bool({ let t773 = Py::Float(1e-09); if py::gt(&t772, &t773)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_sols.clone(), "append", &[Py::tuple(vec![v_B.clone(), v_C.clone(), py::truediv(&py::mul(&v_a.clone(), &f__deg_sin(v_C.clone())?)?, &f__deg_sin(v_A.clone())?)?])])?;
        }
    }
    if (py::not(&v_sols.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no triangle exists with these measurements"))));
    }
    v_out = Py::dict(vec![(Py::str("count"), py::float(&py::len(&v_sols.clone())?)?)]);
    for t774 in py::iter(&(py::enumerate(&v_sols.clone(), (Py::Int(1i64)).i()?)?))? {
        let t775 = (t774).items()?;
        if t775.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t775[0].clone();
        let t776 = (t775[1].clone()).items()?;
        if t776.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_B = t776[0].clone();
        v_C = t776[1].clone();
        v_c = t776[2].clone();
        let _ = py::method(&v_out.clone(), "update", &[Py::dict(vec![(Py::str(&["B".to_string(), py::format(&v_k.clone(), "")?].concat()), v_B.clone()), (Py::str(&["C".to_string(), py::format(&v_k.clone(), "")?].concat()), v_C.clone()), (Py::str(&["c".to_string(), py::format(&v_k.clone(), "")?].concat()), v_c.clone())])])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_intercepts(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone())]))?;
    if ({ let t810 = { let t806 = v_a.clone(); Py::Bool({ let t807 = Py::Int(0i64); if py::eq(&t806, &t807) { true } else { false } }) }; if t810.truthy() { t810 } else { { let t808 = v_b.clone(); Py::Bool({ let t809 = Py::Int(0i64); if py::eq(&t808, &t809) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a and b must both be nonzero (otherwise the line is parallel to an axis and misses one of them)"))));
    }
    return Ok(Py::dict(vec![(Py::str("x_intercept"), py::truediv(&v_c.clone(), &v_a.clone())?), (Py::str("y_intercept"), py::truediv(&v_c.clone(), &v_b.clone())?), (Py::str("slope"), py::truediv(&py::neg(&v_a.clone())?, &v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_system_3x3(mut v_augmented: Py) -> R {
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    if ({ let t817 = { let t812 = py::len(&v_augmented.clone())?; Py::Bool({ let t813 = Py::Int(3i64); if py::ne(&t812, &t813) { true } else { false } }) }; if t817.truthy() { t817 } else { Py::Bool(({ let mut out = vec![]; for t814 in py::iter(&(v_augmented.clone()))? { let mut v_r: Py; 
v_r = t814;
out.push({ let t815 = py::len(&v_r.clone())?; Py::Bool({ let t816 = Py::Int(4i64); if py::ne(&t815, &t816) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the augmented matrix must have 3 rows of 4 numbers (a, b, c | d)"))));
    }
    let t837 = (f__gauss(v_augmented.clone())?).items()?;
    if t837.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t837[0].clone();
    v_y = t837[1].clone();
    v_z = t837[2].clone();
    return Ok(Py::dict(vec![(Py::str("x"), py::float(&v_x.clone())?), (Py::str("y"), py::float(&v_y.clone())?), (Py::str("z"), py::float(&v_z.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_log_equation_linear_argument(mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_k: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("d"), v_d.clone()), (Py::str("k"), v_k.clone())]))?;
    if ({ let t848 = { let t844 = v_b.clone(); Py::Bool({ let t845 = Py::Int(0i64); if py::le(&t844, &t845)? { true } else { false } }) }; if t848.truthy() { t848 } else { { let t846 = v_b.clone(); Py::Bool({ let t847 = Py::Int(1i64); if py::eq(&t846, &t847) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the base must be positive and not 1"))));
    }
    if ({ let t849 = v_c.clone(); Py::Bool({ let t850 = Py::Int(0i64); if py::eq(&t849, &t850) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("c must not be 0 (no x inside the logarithm)"))));
    }
    if ({ let t851 = py::abs(&py::mul(&v_k.clone(), &py::math1("log", &v_b.clone())?)?)?; Py::Bool({ let t852 = Py::Int(700i64); if py::gt(&t851, &t852)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the solution is too large for a finite number"))));
    }
    return Ok(Py::dict(vec![(Py::str("x"), py::truediv(&py::sub(&py::pow(&v_b.clone(), &v_k.clone())?, &v_d.clone())?, &v_c.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_log_evaluate(mut v_b: Py, mut v_x: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("b"), v_b.clone()), (Py::str("x"), v_x.clone())]))?;
    if ({ let t863 = { let t859 = v_b.clone(); Py::Bool({ let t860 = Py::Int(0i64); if py::le(&t859, &t860)? { true } else { false } }) }; if t863.truthy() { t863 } else { { let t861 = v_b.clone(); Py::Bool({ let t862 = Py::Int(1i64); if py::eq(&t861, &t862) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the base must be positive and not 1"))));
    }
    if ({ let t864 = v_x.clone(); Py::Bool({ let t865 = Py::Int(0i64); if py::le(&t864, &t865)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the logarithm is defined only for positive numbers"))));
    }
    v_v = (if ({ let t866 = v_b.clone(); Py::Bool({ let t867 = Py::Int(2i64); if py::eq(&t866, &t867) { true } else { false } }) }).truthy() { py::truediv(&py::math1("log2", &v_x.clone())?, &py::math1("log2", &v_b.clone())?)? } else { (if ({ let t868 = v_b.clone(); Py::Bool({ let t869 = Py::Int(10i64); if py::eq(&t868, &t869) { true } else { false } }) }).truthy() { py::truediv(&py::math1("log10", &v_x.clone())?, &py::math1("log10", &v_b.clone())?)? } else { py::truediv(&py::math1("log", &v_x.clone())?, &py::math1("log", &v_b.clone())?)? }) });
    v_n = py::round(&v_v.clone(), None)?;
    if ({ let t880 = { let t870 = py::abs(&py::sub(&v_v.clone(), &v_n.clone())?)?; Py::Bool({ let t871 = Py::Float(1e-12); if py::lt(&t870, &t871)? { true } else { false } }) }; if !t880.truthy() { t880 } else { { let t879 = { let t872 = v_b.clone(); Py::Bool({ let t873 = py::int(&v_b.clone())?; if py::eq(&t872, &t873) { true } else { false } }) }; if !t879.truthy() { t879 } else { { let t878 = { let t874 = py::abs(&v_n.clone())?; Py::Bool({ let t875 = Py::Int(60i64); if py::le(&t874, &t875)? { true } else { false } }) }; if !t878.truthy() { t878 } else { { let t876 = py::pow(&py::fraction(&[py::int(&v_b.clone())?])?, &v_n.clone())?; Py::Bool({ let t877 = f__fr(v_x.clone())?; if py::eq(&t876, &t877) { true } else { false } }) } } } } } } }).truthy() {
        v_v = py::float(&v_n.clone())?;
    }
    return Ok(Py::dict(vec![(Py::str("value"), v_v.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_log_laws_combine(mut v_u: Py, mut v_v: Py, mut v_p: Py, mut v_q: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("u"), v_u.clone()), (Py::str("v"), v_v.clone()), (Py::str("p"), v_p.clone()), (Py::str("q"), v_q.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("value"), py::add(&py::mul(&v_p.clone(), &v_u.clone())?, &py::mul(&v_q.clone(), &v_v.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_order_of_operations(mut v_expression: Py) -> R {
    if ({ let t887 = py::not(&Py::Bool(matches!(v_expression.clone(), Py::Str(_)))); if t887.truthy() { t887 } else { py::not(&py::method(&c__ARITH()?, "match", &[py::method(&f__clean(v_expression.clone())?, "replace", &[Py::str(" divided by "), Py::str("/")])?])?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("only numbers, + - * / ^ and parentheses are allowed"))));
    }
    return Ok(f__exact_outputs(f__eval_pratt(v_expression.clone(), Py::None, Py::None, Py::Bool(false))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parabola_focus_directrix(mut v_orientation: Py, mut v_m: Py, mut v_h: Py, mut v_k: Py) -> R {
    let mut v_p: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("h"), v_h.clone()), (Py::str("k"), v_k.clone())]))?;
    if ({ let t997 = v_m.clone(); Py::Bool({ let t998 = Py::Int(0i64); if py::eq(&t997, &t998) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("m must not be 0 (no parabola)"))));
    }
    if ({ let t999 = v_orientation.clone(); Py::Bool({ let t1000 = c__ORIENT()?; if !py::contains(&t1000, &t999)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("orientation is vertical or horizontal"))));
    }
    v_p = py::truediv(&v_m.clone(), &Py::Int(4i64))?;
    if ({ let t1001 = v_orientation.clone(); Py::Bool({ let t1002 = Py::str("vertical"); if py::eq(&t1001, &t1002) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("p"), v_p.clone()), (Py::str("focus_x"), v_h.clone()), (Py::str("focus_y"), py::add(&v_k.clone(), &v_p.clone())?), (Py::str("directrix"), py::sub(&v_k.clone(), &v_p.clone())?), (Py::str("vertex_x"), v_h.clone()), (Py::str("vertex_y"), v_k.clone())]));
    }
    return Ok(Py::dict(vec![(Py::str("p"), v_p.clone()), (Py::str("focus_x"), py::add(&v_h.clone(), &v_p.clone())?), (Py::str("focus_y"), v_k.clone()), (Py::str("directrix"), py::sub(&v_h.clone(), &v_p.clone())?), (Py::str("vertex_x"), v_h.clone()), (Py::str("vertex_y"), v_k.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_partial_fractions_linear(mut v_p: Py, mut v_q: Py, mut v_r1: Py, mut v_r2: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("p"), v_p.clone()), (Py::str("q"), v_q.clone()), (Py::str("r1"), v_r1.clone()), (Py::str("r2"), v_r2.clone())]))?;
    if ({ let t1013 = v_r1.clone(); Py::Bool({ let t1014 = v_r2.clone(); if py::eq(&t1013, &t1014) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two linear factors must be different (repeated factors need another form)"))));
    }
    return Ok(Py::dict(vec![(Py::str("A"), py::truediv(&py::add(&py::mul(&v_p.clone(), &v_r1.clone())?, &v_q.clone())?, &py::sub(&v_r1.clone(), &v_r2.clone())?)?), (Py::str("B"), py::truediv(&py::add(&py::mul(&v_p.clone(), &v_r2.clone())?, &v_q.clone())?, &py::sub(&v_r2.clone(), &v_r1.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_percent_change(mut v_old: Py, mut v_new: Py) -> R {
    let mut v_pc: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("old"), v_old.clone()), (Py::str("new"), v_new.clone())]))?;
    if ({ let t1021 = v_old.clone(); Py::Bool({ let t1022 = Py::Int(0i64); if py::le(&t1021, &t1022)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the original value must be positive (a percent change is measured from it)"))));
    }
    v_pc = py::truediv(&py::mul(&Py::Int(100i64), &py::sub(&v_new.clone(), &v_old.clone())?)?, &v_old.clone())?;
    return Ok(Py::dict(vec![(Py::str("percent_change"), v_pc.clone()), (Py::str("difference"), py::sub(&v_new.clone(), &v_old.clone())?), (Py::str("direction"), (if ({ let t1023 = v_new.clone(); Py::Bool({ let t1024 = v_old.clone(); if py::gt(&t1023, &t1024)? { true } else { false } }) }).truthy() { Py::str("increase") } else { (if ({ let t1025 = v_new.clone(); Py::Bool({ let t1026 = v_old.clone(); if py::lt(&t1025, &t1026)? { true } else { false } }) }).truthy() { Py::str("decrease") } else { Py::str("no change") }) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_percent_of_number(mut v_p: Py, mut v_x: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("p"), v_p.clone()), (Py::str("x"), v_x.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("result"), py::truediv(&py::mul(&v_p.clone(), &v_x.clone())?, &Py::Int(100i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_percent_of_what(mut v_part: Py, mut v_p: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("part"), v_part.clone()), (Py::str("p"), v_p.clone())]))?;
    if ({ let t1043 = v_p.clone(); Py::Bool({ let t1044 = Py::Int(0i64); if py::eq(&t1043, &t1044) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("p must not be 0 (0% of every number is 0)"))));
    }
    return Ok(Py::dict(vec![(Py::str("whole"), py::truediv(&py::mul(&v_part.clone(), &Py::Int(100i64))?, &v_p.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_piecewise_evaluate(mut v_e1: Py, mut v_left_rel: Py, mut v_c: Py, mut v_e2: Py, mut v_x: Py) -> R {
    let mut v_exc: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("c"), v_c.clone()), (Py::str("x"), v_x.clone())]))?;
    if ({ let t1050 = v_left_rel.clone(); Py::Bool({ let t1051 = Py::tuple(vec![Py::str("<"), Py::str("<=")]); if !py::contains(&t1051, &t1050)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the first piece must be for x < c or x <= c"))));
    }
    v_first = (if ({ let t1052 = v_left_rel.clone(); Py::Bool({ let t1053 = Py::str("<"); if py::eq(&t1052, &t1053) { true } else { false } }) }).truthy() { { let t1054 = v_x.clone(); Py::Bool({ let t1055 = v_c.clone(); if py::lt(&t1054, &t1055)? { true } else { false } }) } } else { { let t1056 = v_x.clone(); Py::Bool({ let t1057 = v_c.clone(); if py::le(&t1056, &t1057)? { true } else { false } }) } });
    let t1158: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_v = f__fval((if (v_first.clone()).truthy() { v_e1.clone() } else { v_e2.clone() }), v_x.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1158 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            v_exc = Py::str(&match &e { PyErr::Value(m) => m.clone(), other => format!("{:?}", other) });
            return Err(py::value_error(py::to_str(&Py::str(&["f(".to_string(), py::format(&v_x.clone(), "g")?, ") is not defined: ".to_string(), py::format(&v_exc.clone(), "")?].concat()))));
        }
        Err(e) => return Err(e),
    }
    return Ok(Py::dict(vec![(Py::str("value"), py::float(&v_v.clone())?), (Py::str("piece"), (if (v_first.clone()).truthy() { Py::Float(1.0) } else { Py::Float(2.0) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polar_to_rectangular(mut v_r: Py, mut v_theta: Py, mut v_unit: Py) -> R {
    let mut v_snap: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("r"), v_r.clone()), (Py::str("theta"), v_theta.clone())]))?;
    if ({ let t1172 = v_unit.clone(); Py::Bool({ let t1173 = c__UNITS()?; if !py::contains(&t1173, &t1172)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angle unit is degrees or radians"))));
    }
    v_t = (if ({ let t1174 = v_unit.clone(); Py::Bool({ let t1175 = Py::str("degrees"); if py::eq(&t1174, &t1175) { true } else { false } }) }).truthy() { py::math1("radians", &v_theta.clone())? } else { v_theta.clone() });
    let t1176 = (Py::tuple(vec![py::mul(&v_r.clone(), &py::math1("cos", &v_t.clone())?)?, py::mul(&v_r.clone(), &py::math1("sin", &v_t.clone())?)?])).items()?;
    if t1176.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1176[0].clone();
    v_y = t1176[1].clone();
    v_snap = { let v_r = v_r.clone(); let v_t = v_t.clone(); let v_theta = v_theta.clone(); let v_unit = v_unit.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t1177 = py::abs(&v_v.clone())?; Py::Bool({ let t1178 = py::mul(&Py::Float(1e-15), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::lt(&t1177, &t1178)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_v.clone() }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("x"), py::call(&v_snap.clone(), &[v_x.clone()])?), (Py::str("y"), py::call(&v_snap.clone(), &[v_y.clone()])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polynomial_long_division(mut v_dividend: Py, mut v_divisor: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rem: Py = Py::None;
    let t1189 = (Py::tuple(vec![f__strip({ let mut out = vec![]; for t1187 in py::iter(&(v_dividend.clone()))? { let mut v_c: Py; 
v_c = t1187;
out.push(f__fr(v_c.clone())?); } Py::list(out) })?, f__strip({ let mut out = vec![]; for t1188 in py::iter(&(v_divisor.clone()))? { let mut v_c: Py; 
v_c = t1188;
out.push(f__fr(v_c.clone())?); } Py::list(out) })?])).items()?;
    if t1189.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1189[0].clone();
    v_d = t1189[1].clone();
    if ({ let t1190 = py::getitem(&v_d.clone(), &Py::Int(0i64))?; Py::Bool({ let t1191 = Py::Int(0i64); if py::eq(&t1190, &t1191) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the divisor must not be the zero polynomial"))));
    }
    if ({ let t1192 = py::len(&v_d.clone())?; Py::Bool({ let t1193 = Py::Int(2i64); if py::lt(&t1192, &t1193)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the divisor must have degree at least 1"))));
    }
    if ({ let t1194 = py::len(&v_p.clone())?; Py::Bool({ let t1195 = py::len(&v_d.clone())?; if py::lt(&t1194, &t1195)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("quotient"), Py::list(vec![Py::Float(0.0)])), (Py::str("remainder"), { let mut out = vec![]; for t1196 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t1196;
out.push(py::float(&v_v.clone())?); } Py::list(out) })]));
    }
    let t1197 = (Py::tuple(vec![Py::list((v_p.clone()).items()?), Py::list(vec![])])).items()?;
    if t1197.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rem = t1197[0].clone();
    v_q = t1197[1].clone();
    for t1198 in py::range_iter(&[(py::add(&py::sub(&py::len(&v_p.clone())?, &py::len(&v_d.clone())?)?, &Py::Int(1i64))?).i()?]) {
        v_k = t1198;
        v_f = py::truediv(&py::getitem(&v_rem.clone(), &v_k.clone())?, &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?;
        let _ = py::method(&v_q.clone(), "append", &[v_f.clone()])?;
        for t1199 in py::range_iter(&[(py::len(&v_d.clone())?).i()?]) {
            v_j = t1199;
            let t1200 = v_rem.clone(); let t1201 = py::add(&v_k.clone(), &v_j.clone())?;
            py::setitem(&t1200, &t1201, py::sub(&py::getitem(&t1200, &t1201)?, &py::mul(&v_f.clone(), &py::getitem(&v_d.clone(), &v_j.clone())?)?)?)?;
        }
    }
    v_r = { let t1202 = f__strip(py::slice(&v_rem.clone(), Some((py::add(&py::sub(&py::len(&v_p.clone())?, &py::len(&v_d.clone())?)?, &Py::Int(1i64))?).i()?), None, None)?)?; if t1202.truthy() { t1202 } else { Py::list(vec![py::fraction(&[Py::Int(0i64)])?]) } };
    return Ok(Py::dict(vec![(Py::str("quotient"), { let mut out = vec![]; for t1203 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t1203;
out.push(py::float(&v_v.clone())?); } Py::list(out) }), (Py::str("remainder"), { let mut out = vec![]; for t1204 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t1204;
out.push(py::float(&v_v.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_proportion_solve(mut v_p: Py, mut v_q: Py, mut v_r: Py, mut v_position: Py) -> R {
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("p"), v_p.clone()), (Py::str("q"), v_q.clone()), (Py::str("r"), v_r.clone())]))?;
    let t1228 = (f__prop_terms(v_p.clone(), v_q.clone(), v_r.clone(), v_position.clone())?).items()?;
    if t1228.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t1228[0].clone();
    v_d1 = t1228[1].clone();
    v_n2 = t1228[2].clone();
    v_d2 = t1228[3].clone();
    if ({ let t1233 = { let t1229 = v_d1.clone(); Py::Bool({ let t1230 = Py::Int(0i64); if py::eq(&t1229, &t1230) { true } else { false } }) }; if t1233.truthy() { t1233 } else { { let t1231 = v_d2.clone(); Py::Bool({ let t1232 = Py::Int(0i64); if py::eq(&t1231, &t1232) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a denominator must not be 0"))));
    }
    v_k = py::method(&c__POSITIONS()?, "index", &[v_position.clone()])?;
    if ({ let t1234 = v_k.clone(); Py::Bool({ let t1235 = Py::Int(0i64); if py::eq(&t1234, &t1235) { true } else { false } }) }).truthy() {
        v_x = py::truediv(&py::mul(&v_n2.clone(), &v_d1.clone())?, &v_d2.clone())?;
    } else {
        if ({ let t1236 = v_k.clone(); Py::Bool({ let t1237 = Py::Int(1i64); if py::eq(&t1236, &t1237) { true } else { false } }) }).truthy() {
            if ({ let t1238 = v_n2.clone(); Py::Bool({ let t1239 = Py::Int(0i64); if py::eq(&t1238, &t1239) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("no solution: the unknown denominator would have to make a nonzero ratio equal to 0"))));
            }
            v_x = py::truediv(&py::mul(&v_n1.clone(), &v_d2.clone())?, &v_n2.clone())?;
        } else {
            if ({ let t1240 = v_k.clone(); Py::Bool({ let t1241 = Py::Int(2i64); if py::eq(&t1240, &t1241) { true } else { false } }) }).truthy() {
                v_x = py::truediv(&py::mul(&v_n1.clone(), &v_d2.clone())?, &v_d1.clone())?;
            } else {
                if ({ let t1242 = v_n1.clone(); Py::Bool({ let t1243 = Py::Int(0i64); if py::eq(&t1242, &t1243) { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("no solution: the unknown denominator would have to make a nonzero ratio equal to 0"))));
                }
                v_x = py::truediv(&py::mul(&v_n2.clone(), &v_d1.clone())?, &v_n1.clone())?;
            }
        }
    }
    if ({ let t1248 = { let t1244 = v_k.clone(); Py::Bool({ let t1245 = Py::tuple(vec![Py::Int(1i64), Py::Int(3i64)]); if py::contains(&t1245, &t1244)? { true } else { false } }) }; if !t1248.truthy() { t1248 } else { { let t1246 = v_x.clone(); Py::Bool({ let t1247 = Py::Int(0i64); if py::eq(&t1246, &t1247) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no solution (the unknown denominator would be 0)"))));
    }
    return Ok(Py::dict(vec![(Py::str("x"), v_x.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_quadratic_inequality(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_rel: Py) -> R {
    let mut v_between: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone())]))?;
    if ({ let t1264 = v_a.clone(); Py::Bool({ let t1265 = Py::Int(0i64); if py::eq(&t1264, &t1265) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0 (not a quadratic)"))));
    }
    v_disc = py::sub(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_a.clone())?, &v_c.clone())?)?;
    if ({ let t1266 = v_disc.clone(); Py::Bool({ let t1267 = Py::Int(0i64); if py::le(&t1266, &t1267)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("refused: the quadratic has no two distinct real roots (this capability answers that case only)"))));
    }
    v_q = py::truediv(&py::neg(&py::add(&v_b.clone(), &py::copysign(&py::math1("sqrt", &v_disc.clone())?, &v_b.clone())?)?)?, &Py::Int(2i64))?;
    let t1268 = (py::sorted(&Py::list(vec![py::truediv(&v_q.clone(), &v_a.clone())?, py::truediv(&v_c.clone(), &v_q.clone())?]), (Py::Bool(false)).truthy())?).items()?;
    if t1268.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t1268[0].clone();
    v_r2 = t1268[1].clone();
    v_between = { let t1269 = { let t1271 = v_rel.clone(); Py::Bool({ let t1272 = Py::tuple(vec![Py::str("<"), Py::str("<=")]); if py::contains(&t1272, &t1271)? { true } else { false } }) }; Py::Bool({ let t1270 = { let t1273 = v_a.clone(); Py::Bool({ let t1274 = Py::Int(0i64); if py::gt(&t1273, &t1274)? { true } else { false } }) }; if py::eq(&t1269, &t1270) { true } else { false } }) };
    return Ok(Py::dict(vec![(Py::str("r1"), v_r1.clone()), (Py::str("r2"), v_r2.clone()), (Py::str("region"), (if (v_between.clone()).truthy() { Py::str("between") } else { Py::str("outside") })), (Py::str("solution"), f__ineq_text(v_r1.clone(), v_r2.clone(), v_rel.clone(), v_between.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_quadratic_roots(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_disc: Py = Py::None;
    let mut v_im: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__check(v_a.clone(), v_b.clone(), v_c.clone())?;
    v_disc = py::sub(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_a.clone())?, &v_c.clone())?)?;
    if ({ let t1299 = v_disc.clone(); Py::Bool({ let t1300 = Py::Int(0i64); if py::gt(&t1299, &t1300)? { true } else { false } }) }).truthy() {
        v_q = py::truediv(&py::neg(&py::add(&v_b.clone(), &py::copysign(&py::math1("sqrt", &v_disc.clone())?, &v_b.clone())?)?)?, &Py::Int(2i64))?;
        v_r = (if ({ let t1301 = v_q.clone(); Py::Bool({ let t1302 = Py::Int(0i64); if py::ne(&t1301, &t1302) { true } else { false } }) }).truthy() { py::sorted(&Py::list(vec![py::truediv(&v_q.clone(), &v_a.clone())?, py::truediv(&v_c.clone(), &v_q.clone())?]), (Py::Bool(false)).truthy())? } else { Py::list(vec![Py::Float(0.0), Py::Float(0.0)]) });
        return Ok(Py::dict(vec![(Py::str("x1"), py::getitem(&v_r.clone(), &Py::Int(0i64))?), (Py::str("x2"), py::getitem(&v_r.clone(), &Py::Int(1i64))?), (Py::str("discriminant"), v_disc.clone())]));
    }
    if ({ let t1303 = v_disc.clone(); Py::Bool({ let t1304 = Py::Int(0i64); if py::eq(&t1303, &t1304) { true } else { false } }) }).truthy() {
        v_x = py::truediv(&py::neg(&v_b.clone())?, &py::mul(&Py::Int(2i64), &v_a.clone())?)?;
        return Ok(Py::dict(vec![(Py::str("x1"), v_x.clone()), (Py::str("x2"), v_x.clone()), (Py::str("discriminant"), v_disc.clone())]));
    }
    let t1305 = (Py::tuple(vec![py::add(&py::truediv(&py::neg(&v_b.clone())?, &py::mul(&Py::Int(2i64), &v_a.clone())?)?, &Py::Float(0.0))?, py::truediv(&py::math1("sqrt", &py::neg(&v_disc.clone())?)?, &py::mul(&Py::Int(2i64), &py::abs(&v_a.clone())?)?)?])).items()?;
    if t1305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t1305[0].clone();
    v_im = t1305[1].clone();
    return Ok(Py::dict(vec![(Py::str("x1"), py::complex(&[v_re_.clone(), v_im.clone()])?), (Py::str("x2"), py::complex(&[v_re_.clone(), py::neg(&v_im.clone())?])?), (Py::str("discriminant"), v_disc.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_quartic_roots(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_e: Py) -> R {
    return Ok(f__roots_out(f__poly_roots(Py::list(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone(), v_e.clone()]))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rad_to_deg(mut v_rad: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_snapped: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("rad"), v_rad.clone())]))?;
    v_d = py::truediv(&py::mul(&v_rad.clone(), &Py::Int(180i64))?, &Py::Float(std::f64::consts::PI))?;
    v_snapped = py::round(&v_d.clone(), Some((Py::Int(9i64)).i()?))?;
    if ({ let t1336 = py::abs(&py::sub(&v_d.clone(), &v_snapped.clone())?)?; Py::Bool({ let t1337 = py::mul(&Py::Float(1e-11), &py::minmax(&[Py::Float(1.0), py::abs(&v_d.clone())?], true)?)?; if py::le(&t1336, &t1337)? { true } else { false } }) }).truthy() {
        v_d = py::add(&v_snapped.clone(), &Py::Float(0.0))?;
    }
    return Ok(Py::dict(vec![(Py::str("degrees"), v_d.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radical_equation(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py) -> R {
    let mut v_A0: Py = Py::None;
    let mut v_B0: Py = Py::None;
    let mut v_C0: Py = Py::None;
    let mut v_D0: Py = Py::None;
    let mut v_bad: Py = Py::None;
    let mut v_cands: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_ext: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ok: Py = Py::None;
    let mut v_qa: Py = Py::None;
    let mut v_qb: Py = Py::None;
    let mut v_qc: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sols: Py = Py::None;
    let mut v_valid: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xv: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("d"), v_d.clone())]))?;
    let t1344 = ({ let mut out = vec![]; for t1343 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()])))? { let mut v_v: Py; 
v_v = t1343;
out.push(f__fr(v_v.clone())?); } Py::list(out) }).items()?;
    if t1344.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A0 = t1344[0].clone();
    v_B0 = t1344[1].clone();
    v_C0 = t1344[2].clone();
    v_D0 = t1344[3].clone();
    if ({ let t1345 = v_A0.clone(); Py::Bool({ let t1346 = Py::Int(0i64); if py::eq(&t1345, &t1346) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0 (no x under the root)"))));
    }
    let t1347 = (Py::tuple(vec![py::mul(&v_C0.clone(), &v_C0.clone())?, py::sub(&py::mul(&py::mul(&Py::Int(2i64), &v_C0.clone())?, &v_D0.clone())?, &v_A0.clone())?, py::sub(&py::mul(&v_D0.clone(), &v_D0.clone())?, &v_B0.clone())?])).items()?;
    if t1347.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_qa = t1347[0].clone();
    v_qb = t1347[1].clone();
    v_qc = t1347[2].clone();
    v_cands = Py::list(vec![]);
    if ({ let t1348 = v_qa.clone(); Py::Bool({ let t1349 = Py::Int(0i64); if py::eq(&t1348, &t1349) { true } else { false } }) }).truthy() {
        v_x = py::truediv(&py::neg(&v_qc.clone())?, &v_qb.clone())?;
        let _ = py::method(&v_cands.clone(), "append", &[v_x.clone()])?;
        v_valid = (if ({ let t1350 = py::add(&py::mul(&v_C0.clone(), &v_x.clone())?, &v_D0.clone())?; Py::Bool({ let t1351 = Py::Int(0i64); if py::ge(&t1350, &t1351)? { true } else { false } }) }).truthy() { Py::list(vec![v_x.clone()]) } else { Py::list(vec![]) });
        v_bad = (if (v_valid.clone()).truthy() { Py::list(vec![]) } else { Py::list(vec![v_x.clone()]) });
        let t1354 = (Py::tuple(vec![{ let mut out = vec![]; for t1352 in py::iter(&(v_valid.clone()))? { let mut v_v: Py; 
v_v = t1352;
out.push(py::float(&v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t1353 in py::iter(&(v_bad.clone()))? { let mut v_v: Py; 
v_v = t1353;
out.push(py::float(&v_v.clone())?); } Py::list(out) }])).items()?;
        if t1354.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sols = t1354[0].clone();
        v_ext = t1354[1].clone();
    } else {
        v_disc = py::sub(&py::mul(&v_qb.clone(), &v_qb.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_qa.clone())?, &v_qc.clone())?)?;
        let t1355 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
        if t1355.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sols = t1355[0].clone();
        v_ext = t1355[1].clone();
        if ({ let t1356 = v_disc.clone(); Py::Bool({ let t1357 = Py::Int(0i64); if py::ge(&t1356, &t1357)? { true } else { false } }) }).truthy() {
            let t1358 = (Py::tuple(vec![py::isqrt(&py::numerator(&v_disc.clone())?)?, py::isqrt(&py::denominator(&v_disc.clone())?)?])).items()?;
            if t1358.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_n = t1358[0].clone();
            v_m = t1358[1].clone();
            for t1359 in py::iter(&(Py::tuple(vec![py::neg(&Py::Int(1i64))?, Py::Int(1i64)])))? {
                v_s = t1359;
                if ({ let t1364 = { let t1360 = py::mul(&v_n.clone(), &v_n.clone())?; Py::Bool({ let t1361 = py::numerator(&v_disc.clone())?; if py::eq(&t1360, &t1361) { true } else { false } }) }; if !t1364.truthy() { t1364 } else { { let t1362 = py::mul(&v_m.clone(), &v_m.clone())?; Py::Bool({ let t1363 = py::denominator(&v_disc.clone())?; if py::eq(&t1362, &t1363) { true } else { false } }) } } }).truthy() {
                    v_x = py::truediv(&py::add(&py::neg(&v_qb.clone())?, &py::mul(&v_s.clone(), &py::fraction(&[v_n.clone(), v_m.clone()])?)?)?, &py::mul(&Py::Int(2i64), &v_qa.clone())?)?;
                    v_ok = { let t1365 = py::add(&py::mul(&v_C0.clone(), &v_x.clone())?, &v_D0.clone())?; Py::Bool({ let t1366 = Py::Int(0i64); if py::ge(&t1365, &t1366)? { true } else { false } }) };
                    v_xv = py::float(&v_x.clone())?;
                } else {
                    v_ok = { let t1367 = f__sign_root_expr(py::add(&py::mul(&py::neg(&v_C0.clone())?, &v_qb.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_qa.clone())?, &v_D0.clone())?)?, py::mul(&v_s.clone(), &v_C0.clone())?, v_disc.clone())?; Py::Bool({ let t1368 = Py::Int(0i64); if py::ge(&t1367, &t1368)? { true } else { false } }) };
                    v_xv = py::truediv(&py::add(&py::float(&py::neg(&v_qb.clone())?)?, &py::mul(&v_s.clone(), &py::math1("sqrt", &py::float(&v_disc.clone())?)?)?)?, &py::float(&py::mul(&Py::Int(2i64), &v_qa.clone())?)?)?;
                }
                let _ = py::method(&(if (v_ok.clone()).truthy() { v_sols.clone() } else { v_ext.clone() }), "append", &[v_xv.clone()])?;
            }
            if ({ let t1400 = v_disc.clone(); Py::Bool({ let t1401 = Py::Int(0i64); if py::eq(&t1400, &t1401) { true } else { false } }) }).truthy() {
                let t1402 = (Py::tuple(vec![py::sorted(&py::set(&v_sols.clone())?, (Py::Bool(false)).truthy())?, py::sorted(&py::set(&v_ext.clone())?, (Py::Bool(false)).truthy())?])).items()?;
                if t1402.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_sols = t1402[0].clone();
                v_ext = t1402[1].clone();
            }
        }
    }
    let t1403 = (Py::tuple(vec![py::sorted(&v_sols.clone(), (Py::Bool(false)).truthy())?, py::sorted(&v_ext.clone(), (Py::Bool(false)).truthy())?])).items()?;
    if t1403.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sols = t1403[0].clone();
    v_ext = t1403[1].clone();
    return Ok(Py::dict(vec![(Py::str("count"), py::float(&py::len(&v_sols.clone())?)?), (Py::str("solutions"), v_sols.clone()), (Py::str("extraneous"), v_ext.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ratio_split(mut v_total: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_parts: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("total"), v_total.clone()), (Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone())]))?;
    v_parts = py::add(&Py::list(vec![v_a.clone(), v_b.clone()]), &(if ({ let t1425 = v_c.clone(); Py::Bool({ let t1426 = Py::None; if !py::is(&t1425, &t1426) { true } else { false } }) }).truthy() { Py::list(vec![v_c.clone()]) } else { Py::list(vec![]) }))?;
    if (Py::Bool(({ let mut out = vec![]; for t1427 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t1427;
out.push({ let t1430 = py::not(&py::math1("isfinite", &v_p.clone())?); if t1430.truthy() { t1430 } else { { let t1428 = v_p.clone(); Py::Bool({ let t1429 = Py::Int(0i64); if py::le(&t1428, &t1429)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every term of the ratio must be positive"))));
    }
    v_s = py::sum(&v_parts.clone(), None)?;
    return Ok({ let mut out = vec![]; for t1431 in py::iter(&(py::enumerate(&v_parts.clone(), 0)?))? { let mut v_i: Py; let mut v_p: Py; 
let t1432 = (t1431).items()?;
if t1432.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1432[0].clone();
v_p = t1432[1].clone();
out.push((Py::str(&["part".to_string(), py::format(&py::add(&v_i.clone(), &Py::Int(1i64))?, "")?].concat()), py::truediv(&py::mul(&v_total.clone(), &v_p.clone())?, &v_s.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rational_equation(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("d"), v_d.clone())]))?;
    let t1446 = ({ let mut out = vec![]; for t1445 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()])))? { let mut v_v: Py; 
v_v = t1445;
out.push(f__fr(v_v.clone())?); } Py::list(out) }).items()?;
    if t1446.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1446[0].clone();
    v_B = t1446[1].clone();
    v_C = t1446[2].clone();
    v_D = t1446[3].clone();
    if ({ let t1447 = v_A.clone(); Py::Bool({ let t1448 = v_C.clone(); if py::eq(&t1447, &t1448) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no unique solution (equal numerators: no solution or the same expression on both sides)"))));
    }
    v_x = py::truediv(&py::sub(&py::mul(&v_C.clone(), &v_B.clone())?, &py::mul(&v_A.clone(), &v_D.clone())?)?, &py::sub(&v_A.clone(), &v_C.clone())?)?;
    if ({ let t1453 = { let t1449 = v_x.clone(); Py::Bool({ let t1450 = py::neg(&v_B.clone())?; if py::eq(&t1449, &t1450) { true } else { false } }) }; if t1453.truthy() { t1453 } else { { let t1451 = v_x.clone(); Py::Bool({ let t1452 = py::neg(&v_D.clone())?; if py::eq(&t1451, &t1452) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["no solution: the only candidate x = ".to_string(), py::format(&py::float(&v_x.clone())?, "g")?, " makes a denominator 0 (extraneous)".to_string()].concat()))));
    }
    return Ok(Py::dict(vec![(Py::str("x"), py::float(&v_x.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rational_roots(mut v_coefficients: Py) -> R {
    let mut v_co: Py = Py::None;
    let mut v_roots: Py = Py::None;
    v_co = f__strip({ let mut out = vec![]; for t1465 in py::iter(&(v_coefficients.clone()))? { let mut v_c: Py; 
v_c = t1465;
out.push(py::float(&v_c.clone())?); } Py::list(out) })?;
    if (Py::Bool(({ let mut out = vec![]; for t1466 in py::iter(&(v_co.clone()))? { let mut v_c: Py; 
v_c = t1466;
out.push({ let t1467 = v_c.clone(); Py::Bool({ let t1468 = py::int(&v_c.clone())?; if py::ne(&t1467, &t1468) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the coefficients must be whole numbers"))));
    }
    v_co = { let mut out = vec![]; for t1469 in py::iter(&(v_co.clone()))? { let mut v_c: Py; 
v_c = t1469;
out.push(py::int(&v_c.clone())?); } Py::list(out) };
    if ({ let t1470 = py::len(&v_co.clone())?; Py::Bool({ let t1471 = Py::Int(2i64); if py::lt(&t1470, &t1471)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the polynomial must have degree at least 1"))));
    }
    if ({ let t1472 = py::minmax(&[{ let mut out = vec![]; for t1474 in py::iter(&(v_co.clone()))? { let mut v_c: Py; 
v_c = t1474;
out.push(py::abs(&v_c.clone())?); } Py::list(out) }], true)?; Py::Bool({ let t1473 = py::pow(&Py::Int(10i64), &Py::Int(7i64))?; if py::gt(&t1472, &t1473)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("coefficients are limited to 10^7 in size"))));
    }
    v_roots = f__candidates_roots(v_co.clone())?;
    return Ok(Py::dict(vec![(Py::str("count"), py::float(&py::len(&v_roots.clone())?)?), (Py::str("roots"), { let mut out = vec![]; for t1494 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t1494;
out.push(py::float(&v_r.clone())?); } Py::list(out) }), (Py::str("roots_exact"), { let mut out = vec![]; for t1495 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t1495;
out.push(Py::str(&py::to_str(&v_r.clone()))); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rectangular_to_polar(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("x"), v_x.clone()), (Py::str("y"), v_y.clone())]))?;
    if ({ let t1549 = { let t1545 = v_x.clone(); Py::Bool({ let t1546 = Py::Int(0i64); if py::eq(&t1545, &t1546) { true } else { false } }) }; if !t1549.truthy() { t1549 } else { { let t1547 = v_y.clone(); Py::Bool({ let t1548 = Py::Int(0i64); if py::eq(&t1547, &t1548) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the origin has r = 0 and no defined angle"))));
    }
    v_d = f__direction(v_x.clone(), v_y.clone())?;
    return Ok(Py::dict(vec![(Py::str("r"), py::hypot(&[v_x.clone(), v_y.clone()])?), (Py::str("theta_deg"), v_d.clone()), (Py::str("theta_rad"), py::math1("radians", &v_d.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_right_triangle_angle_side(mut v_A: Py, mut v_side: Py, mut v_which: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("side"), v_side.clone())]))?;
    if (py::not(&{ let t1560 = Py::Int(0i64); Py::Bool({ let t1561 = v_A.clone(); if py::lt(&t1560, &t1561)? { { let t1562 = Py::Int(90i64); if py::lt(&t1561, &t1562)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the acute angle must be between 0 and 90 degrees"))));
    }
    if ({ let t1563 = v_side.clone(); Py::Bool({ let t1564 = Py::Int(0i64); if py::le(&t1563, &t1564)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the side must be positive"))));
    }
    v_t = py::math1("radians", &v_A.clone())?;
    if ({ let t1565 = v_which.clone(); Py::Bool({ let t1566 = Py::str("hypotenuse"); if py::eq(&t1565, &t1566) { true } else { false } }) }).truthy() {
        v_c = v_side.clone();
        let t1567 = (Py::tuple(vec![py::mul(&v_c.clone(), &py::math1("sin", &v_t.clone())?)?, py::mul(&v_c.clone(), &py::math1("cos", &v_t.clone())?)?])).items()?;
        if t1567.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1567[0].clone();
        v_b = t1567[1].clone();
    } else {
        if ({ let t1568 = v_which.clone(); Py::Bool({ let t1569 = Py::str("opposite"); if py::eq(&t1568, &t1569) { true } else { false } }) }).truthy() {
            v_a = v_side.clone();
            let t1570 = (Py::tuple(vec![py::truediv(&v_a.clone(), &py::math1("sin", &v_t.clone())?)?, py::truediv(&v_a.clone(), &py::math1("tan", &v_t.clone())?)?])).items()?;
            if t1570.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t1570[0].clone();
            v_b = t1570[1].clone();
        } else {
            v_b = v_side.clone();
            let t1571 = (Py::tuple(vec![py::truediv(&v_b.clone(), &py::math1("cos", &v_t.clone())?)?, py::mul(&v_b.clone(), &py::math1("tan", &v_t.clone())?)?])).items()?;
            if t1571.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t1571[0].clone();
            v_a = t1571[1].clone();
        }
    }
    return Ok(Py::dict(vec![(Py::str("opposite"), v_a.clone()), (Py::str("adjacent"), v_b.clone()), (Py::str("hypotenuse"), v_c.clone()), (Py::str("B"), py::sub(&Py::Float(90.0), &v_A.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_right_triangle_leg_hypotenuse(mut v_a: Py, mut v_c: Py) -> R {
    let mut v_A: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("c"), v_c.clone())]))?;
    if (py::not(&{ let t1578 = Py::Int(0i64); Py::Bool({ let t1579 = v_a.clone(); if py::lt(&t1578, &t1579)? { { let t1580 = v_c.clone(); if py::lt(&t1579, &t1580)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the leg must be positive and shorter than the hypotenuse"))));
    }
    v_A = py::math1("degrees", &py::math1("asin", &py::truediv(&v_a.clone(), &v_c.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("b"), py::math1("sqrt", &py::mul(&py::sub(&v_c.clone(), &v_a.clone())?, &py::add(&v_c.clone(), &v_a.clone())?)?)?), (Py::str("A"), v_A.clone()), (Py::str("B"), py::sub(&Py::Float(90.0), &v_A.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_right_triangle_legs(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_A: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone())]))?;
    if ({ let t1590 = { let t1586 = v_a.clone(); Py::Bool({ let t1587 = Py::Int(0i64); if py::le(&t1586, &t1587)? { true } else { false } }) }; if t1590.truthy() { t1590 } else { { let t1588 = v_b.clone(); Py::Bool({ let t1589 = Py::Int(0i64); if py::le(&t1588, &t1589)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the legs must be positive"))));
    }
    v_A = py::math1("degrees", &py::atan2(&v_a.clone(), &v_b.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("c"), py::hypot(&[v_a.clone(), v_b.clone()])?), (Py::str("A"), v_A.clone()), (Py::str("B"), py::sub(&Py::Float(90.0), &v_A.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_round_decimal_places(mut v_x: Py, mut v_n: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("x"), v_x.clone())]))?;
    v_n = py::int(&v_n.clone())?;
    if (py::not(&{ let t1596 = py::neg(&Py::Int(9i64))?; Py::Bool({ let t1597 = v_n.clone(); if py::le(&t1596, &t1597)? { { let t1598 = Py::Int(12i64); if py::le(&t1597, &t1598)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of decimal places must be between -9 and 12"))));
    }
    return Ok(Py::dict(vec![(Py::str("rounded"), py::float(&f__round_half_up(f__dec(v_x.clone())?, v_n.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_round_significant_figures(mut v_x: Py, mut v_n: Py) -> R {
    let mut v_q: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("x"), v_x.clone())]))?;
    v_n = py::int(&v_n.clone())?;
    if ({ let t1618 = v_x.clone(); Py::Bool({ let t1619 = Py::Int(0i64); if py::eq(&t1618, &t1619) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("0 has no significant figures to round to"))));
    }
    if (py::not(&{ let t1620 = Py::Int(1i64); Py::Bool({ let t1621 = v_n.clone(); if py::le(&t1620, &t1621)? { { let t1622 = Py::Int(15i64); if py::le(&t1621, &t1622)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of significant figures must be between 1 and 15"))));
    }
    v_q = f__dec(v_x.clone())?;
    return Ok(Py::dict(vec![(Py::str("rounded"), py::float(&f__round_half_up(v_q.clone(), py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &f__sig_exponent(v_q.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_scientific_notation(mut v_x: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("x"), v_x.clone())]))?;
    if ({ let t1644 = v_x.clone(); Py::Bool({ let t1645 = Py::Int(0i64); if py::eq(&t1644, &t1645) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("0 has no scientific notation with a leading digit from 1 to 9"))));
    }
    v_q = f__dec(v_x.clone())?;
    v_e = f__sig_exponent(v_q.clone())?;
    return Ok(Py::dict(vec![(Py::str("mantissa"), py::float(&py::truediv(&v_q.clone(), &py::pow(&py::fraction(&[Py::Int(10i64)])?, &v_e.clone())?)?)?), (Py::str("exponent"), py::float(&v_e.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_synthetic_division(mut v_coefficients: Py, mut v_r: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_co: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_co = f__strip({ let mut out = vec![]; for t1662 in py::iter(&(v_coefficients.clone()))? { let mut v_c: Py; 
v_c = t1662;
out.push(f__fr(v_c.clone())?); } Py::list(out) })?;
    let _ = f__finite(Py::dict(vec![(Py::str("r"), v_r.clone())]))?;
    if ({ let t1667 = py::len(&v_co.clone())?; Py::Bool({ let t1668 = Py::Int(2i64); if py::lt(&t1667, &t1668)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the dividend must have degree at least 1"))));
    }
    v_R = f__fr(v_r.clone())?;
    v_out = Py::list(vec![py::getitem(&v_co.clone(), &Py::Int(0i64))?]);
    for t1669 in py::iter(&(py::slice(&v_co.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
        v_c = t1669;
        let _ = py::method(&v_out.clone(), "append", &[py::add(&v_c.clone(), &py::mul(&py::getitem(&v_out.clone(), &py::neg(&Py::Int(1i64))?)?, &v_R.clone())?)?])?;
    }
    return Ok(Py::dict(vec![(Py::str("quotient"), { let mut out = vec![]; for t1670 in py::iter(&(py::slice(&v_out.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?))? { let mut v_v: Py; 
v_v = t1670;
out.push(py::float(&v_v.clone())?); } Py::list(out) }), (Py::str("remainder"), py::float(&py::getitem(&v_out.clone(), &py::neg(&Py::Int(1i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_trig_equation_interval(mut v_func: Py, mut v_k: Py, mut v_n: Py, mut v_unit: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_base: Py = Py::None;
    let mut v_sols: Py = Py::None;
    let mut v_us: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("k"), v_k.clone())]))?;
    v_n = py::int(&v_n.clone())?;
    if ({ let t1688 = { let t1684 = v_func.clone(); Py::Bool({ let t1685 = c__TRIG()?; if !py::contains(&t1685, &t1684)? { true } else { false } }) }; if t1688.truthy() { t1688 } else { { let t1686 = v_unit.clone(); Py::Bool({ let t1687 = c__UNITS()?; if !py::contains(&t1687, &t1686)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("func is sin, cos or tan; unit is degrees or radians"))));
    }
    if (py::not(&{ let t1689 = Py::Int(1i64); Py::Bool({ let t1690 = v_n.clone(); if py::le(&t1689, &t1690)? { { let t1691 = Py::Int(12i64); if py::le(&t1690, &t1691)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the multiple n of x must be a whole number from 1 to 12"))));
    }
    if ({ let t1692 = v_func.clone(); Py::Bool({ let t1693 = Py::str("tan"); if py::eq(&t1692, &t1693) { true } else { false } }) }).truthy() {
        v_base = Py::list(vec![py::math1("degrees", &py::math1("atan", &v_k.clone())?)?, py::add(&py::math1("degrees", &py::math1("atan", &v_k.clone())?)?, &Py::Int(180i64))?]);
    } else {
        if ({ let t1694 = py::abs(&v_k.clone())?; Py::Bool({ let t1695 = Py::Int(1i64); if py::gt(&t1694, &t1695)? { true } else { false } }) }).truthy() {
            v_base = Py::list(vec![]);
        } else {
            if ({ let t1696 = v_func.clone(); Py::Bool({ let t1697 = Py::str("sin"); if py::eq(&t1696, &t1697) { true } else { false } }) }).truthy() {
                v_a = (if ({ let t1698 = v_k.clone(); Py::Bool({ let t1699 = Py::Int(1i64); if py::eq(&t1698, &t1699) { true } else { false } }) }).truthy() { Py::Float(90.0) } else { (if ({ let t1700 = v_k.clone(); Py::Bool({ let t1701 = py::neg(&Py::Int(1i64))?; if py::eq(&t1700, &t1701) { true } else { false } }) }).truthy() { py::neg(&Py::Float(90.0))? } else { py::math1("degrees", &py::math1("asin", &v_k.clone())?)? }) });
                v_base = Py::list(vec![v_a.clone(), py::sub(&Py::Int(180i64), &v_a.clone())?]);
            } else {
                v_a = (if ({ let t1702 = v_k.clone(); Py::Bool({ let t1703 = Py::Int(1i64); if py::eq(&t1702, &t1703) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { (if ({ let t1704 = v_k.clone(); Py::Bool({ let t1705 = py::neg(&Py::Int(1i64))?; if py::eq(&t1704, &t1705) { true } else { false } }) }).truthy() { Py::Float(180.0) } else { py::math1("degrees", &py::math1("acos", &v_k.clone())?)? }) });
                v_base = Py::list(vec![v_a.clone(), py::neg(&v_a.clone())?]);
            }
        }
    }
    v_us = Py::list(vec![]);
    for t1706 in py::iter(&(v_base.clone()))? {
        v_b = t1706;
        v_v = py::modulo(&v_b.clone(), &Py::Float(360.0))?;
        if ({ let t1707 = v_v.clone(); Py::Bool({ let t1708 = py::sub(&Py::Float(360.0), &Py::Float(1e-09))?; if py::gt(&t1707, &t1708)? { true } else { false } }) }).truthy() {
            v_v = Py::Float(0.0);
        }
        if (py::not(&Py::Bool(({ let mut out = vec![]; for t1709 in py::iter(&(v_us.clone()))? { let mut v_w: Py; 
v_w = t1709;
out.push({ let t1710 = py::abs(&py::sub(&v_v.clone(), &v_w.clone())?)?; Py::Bool({ let t1711 = Py::Float(1e-09); if py::lt(&t1710, &t1711)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())))).truthy() {
            let _ = py::method(&v_us.clone(), "append", &[v_v.clone()])?;
        }
    }
    v_sols = py::sorted(&{ let mut out = vec![]; for t1712 in py::iter(&(v_us.clone()))? { let mut v_u: Py; 
v_u = t1712;
for t1713 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_m: Py; 
v_m = t1713;
out.push(py::truediv(&py::add(&v_u.clone(), &py::mul(&Py::Float(360.0), &v_m.clone())?)?, &v_n.clone())?); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
    if ({ let t1714 = v_unit.clone(); Py::Bool({ let t1715 = Py::str("radians"); if py::eq(&t1714, &t1715) { true } else { false } }) }).truthy() {
        v_sols = { let mut out = vec![]; for t1716 in py::iter(&(v_sols.clone()))? { let mut v_s: Py; 
v_s = t1716;
out.push(py::math1("radians", &v_s.clone())?); } Py::list(out) };
    }
    return Ok(Py::dict(vec![(Py::str("count"), py::float(&py::len(&v_sols.clone())?)?), (Py::str("solutions"), v_sols.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_trig_values_from_one(mut v_func: Py, mut v_value: Py, mut v_quadrant: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_cos_sign: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sin_sign: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("value"), v_value.clone())]))?;
    v_q = py::int(&v_quadrant.clone())?;
    if ({ let t1750 = { let t1746 = v_func.clone(); Py::Bool({ let t1747 = c__TRIG()?; if !py::contains(&t1747, &t1746)? { true } else { false } }) }; if t1750.truthy() { t1750 } else { { let t1748 = v_q.clone(); Py::Bool({ let t1749 = Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64), Py::Int(4i64)]); if !py::contains(&t1749, &t1748)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("func is sin, cos or tan; the quadrant is 1, 2, 3 or 4"))));
    }
    let t1755 = (Py::tuple(vec![(if ({ let t1751 = v_q.clone(); Py::Bool({ let t1752 = Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)]); if py::contains(&t1752, &t1751)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? }), (if ({ let t1753 = v_q.clone(); Py::Bool({ let t1754 = Py::tuple(vec![Py::Int(1i64), Py::Int(4i64)]); if py::contains(&t1754, &t1753)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? })])).items()?;
    if t1755.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sin_sign = t1755[0].clone();
    v_cos_sign = t1755[1].clone();
    if ({ let t1761 = { let t1756 = v_func.clone(); Py::Bool({ let t1757 = Py::tuple(vec![Py::str("sin"), Py::str("cos")]); if py::contains(&t1757, &t1756)? { true } else { false } }) }; if !t1761.truthy() { t1761 } else { py::not(&{ let t1758 = Py::Int(0i64); Py::Bool({ let t1759 = py::abs(&v_value.clone())?; if py::lt(&t1758, &t1759)? { { let t1760 = Py::Int(1i64); if py::lt(&t1759, &t1760)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a sine or cosine strictly inside a quadrant lies strictly between -1 and 1 and is not 0"))));
    }
    if ({ let t1766 = { let t1762 = v_func.clone(); Py::Bool({ let t1763 = Py::str("tan"); if py::eq(&t1762, &t1763) { true } else { false } }) }; if !t1766.truthy() { t1766 } else { { let t1764 = v_value.clone(); Py::Bool({ let t1765 = Py::Int(0i64); if py::eq(&t1764, &t1765) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("tan = 0 is on an axis, not inside a quadrant"))));
    }
    if ({ let t1767 = v_func.clone(); Py::Bool({ let t1768 = Py::str("sin"); if py::eq(&t1767, &t1768) { true } else { false } }) }).truthy() {
        if ({ let t1769 = { let t1771 = v_value.clone(); Py::Bool({ let t1772 = Py::Int(0i64); if py::gt(&t1771, &t1772)? { true } else { false } }) }; Py::Bool({ let t1770 = { let t1773 = v_sin_sign.clone(); Py::Bool({ let t1774 = Py::Int(0i64); if py::gt(&t1773, &t1774)? { true } else { false } }) }; if py::ne(&t1769, &t1770) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["sine is ".to_string(), py::format(&(if ({ let t1775 = v_sin_sign.clone(); Py::Bool({ let t1776 = Py::Int(0i64); if py::gt(&t1775, &t1776)? { true } else { false } }) }).truthy() { Py::str("positive") } else { Py::str("negative") }), "")?, " in quadrant ".to_string(), py::format(&v_q.clone(), "")?].concat()))));
        }
        let t1777 = (Py::tuple(vec![v_value.clone(), py::mul(&v_cos_sign.clone(), &py::math1("sqrt", &py::mul(&py::sub(&Py::Int(1i64), &v_value.clone())?, &py::add(&Py::Int(1i64), &v_value.clone())?)?)?)?])).items()?;
        if t1777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s = t1777[0].clone();
        v_c = t1777[1].clone();
    } else {
        if ({ let t1778 = v_func.clone(); Py::Bool({ let t1779 = Py::str("cos"); if py::eq(&t1778, &t1779) { true } else { false } }) }).truthy() {
            if ({ let t1780 = { let t1782 = v_value.clone(); Py::Bool({ let t1783 = Py::Int(0i64); if py::gt(&t1782, &t1783)? { true } else { false } }) }; Py::Bool({ let t1781 = { let t1784 = v_cos_sign.clone(); Py::Bool({ let t1785 = Py::Int(0i64); if py::gt(&t1784, &t1785)? { true } else { false } }) }; if py::ne(&t1780, &t1781) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&["cosine is ".to_string(), py::format(&(if ({ let t1786 = v_cos_sign.clone(); Py::Bool({ let t1787 = Py::Int(0i64); if py::gt(&t1786, &t1787)? { true } else { false } }) }).truthy() { Py::str("positive") } else { Py::str("negative") }), "")?, " in quadrant ".to_string(), py::format(&v_q.clone(), "")?].concat()))));
            }
            let t1788 = (Py::tuple(vec![v_value.clone(), py::mul(&v_sin_sign.clone(), &py::math1("sqrt", &py::mul(&py::sub(&Py::Int(1i64), &v_value.clone())?, &py::add(&Py::Int(1i64), &v_value.clone())?)?)?)?])).items()?;
            if t1788.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t1788[0].clone();
            v_s = t1788[1].clone();
        } else {
            if ({ let t1789 = { let t1791 = v_value.clone(); Py::Bool({ let t1792 = Py::Int(0i64); if py::gt(&t1791, &t1792)? { true } else { false } }) }; Py::Bool({ let t1790 = { let t1793 = py::mul(&v_sin_sign.clone(), &v_cos_sign.clone())?; Py::Bool({ let t1794 = Py::Int(0i64); if py::gt(&t1793, &t1794)? { true } else { false } }) }; if py::ne(&t1789, &t1790) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&["tangent is ".to_string(), py::format(&(if ({ let t1795 = py::mul(&v_sin_sign.clone(), &v_cos_sign.clone())?; Py::Bool({ let t1796 = Py::Int(0i64); if py::gt(&t1795, &t1796)? { true } else { false } }) }).truthy() { Py::str("positive") } else { Py::str("negative") }), "")?, " in quadrant ".to_string(), py::format(&v_q.clone(), "")?].concat()))));
            }
            v_c = py::truediv(&v_cos_sign.clone(), &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_value.clone(), &v_value.clone())?)?)?)?;
            v_s = py::mul(&v_value.clone(), &v_c.clone())?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("sin"), v_s.clone()), (Py::str("cos"), v_c.clone()), (Py::str("tan"), py::truediv(&v_s.clone(), &v_c.clone())?), (Py::str("sin2"), py::mul(&py::mul(&Py::Int(2i64), &v_s.clone())?, &v_c.clone())?), (Py::str("cos2"), py::mul(&py::sub(&v_c.clone(), &v_s.clone())?, &py::add(&v_c.clone(), &v_s.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_unit_circle_exact(mut v_deg: Py) -> R {
    let mut v_cs: Py = Py::None;
    let mut v_ct: Py = Py::None;
    let mut v_cv: Py = Py::None;
    let mut v_ref: Py = Py::None;
    let mut v_ss: Py = Py::None;
    let mut v_st: Py = Py::None;
    let mut v_sv: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tt: Py = Py::None;
    let mut v_tv: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("deg"), v_deg.clone())]))?;
    if ({ let t1819 = { let t1816 = v_deg.clone(); Py::Bool({ let t1817 = py::int(&v_deg.clone())?; if py::ne(&t1816, &t1817) { true } else { false } }) }; if t1819.truthy() { t1819 } else { { let t1818 = py::modulo(&py::int(&v_deg.clone())?, &Py::Int(30i64))?; if !t1818.truthy() { t1818 } else { py::modulo(&py::int(&v_deg.clone())?, &Py::Int(45i64))? } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("exact values are given for multiples of 30 or 45 degrees (pi/6 or pi/4)"))));
    }
    v_t = py::modulo(&py::int(&v_deg.clone())?, &Py::Int(360i64))?;
    let t1826 = ((if ({ let t1820 = v_t.clone(); Py::Bool({ let t1821 = Py::Int(90i64); if py::le(&t1820, &t1821)? { true } else { false } }) }).truthy() { Py::tuple(vec![v_t.clone(), Py::Int(1i64), Py::Int(1i64)]) } else { (if ({ let t1822 = v_t.clone(); Py::Bool({ let t1823 = Py::Int(180i64); if py::le(&t1822, &t1823)? { true } else { false } }) }).truthy() { Py::tuple(vec![py::sub(&Py::Int(180i64), &v_t.clone())?, Py::Int(1i64), py::neg(&Py::Int(1i64))?]) } else { (if ({ let t1824 = v_t.clone(); Py::Bool({ let t1825 = Py::Int(270i64); if py::le(&t1824, &t1825)? { true } else { false } }) }).truthy() { Py::tuple(vec![py::sub(&v_t.clone(), &Py::Int(180i64))?, py::neg(&Py::Int(1i64))?, py::neg(&Py::Int(1i64))?]) } else { Py::tuple(vec![py::sub(&Py::Int(360i64), &v_t.clone())?, py::neg(&Py::Int(1i64))?, Py::Int(1i64)]) }) }) })).items()?;
    if t1826.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ref = t1826[0].clone();
    v_ss = t1826[1].clone();
    v_cs = t1826[2].clone();
    let t1835 = (f__signed(py::getitem(&c__EXACT_SIN()?, &v_ref.clone())?, v_ss.clone())?).items()?;
    if t1835.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_st = t1835[0].clone();
    v_sv = t1835[1].clone();
    let t1836 = (f__signed(py::getitem(&c__EXACT_SIN()?, &py::sub(&Py::Int(90i64), &v_ref.clone())?)?, v_cs.clone())?).items()?;
    if t1836.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ct = t1836[0].clone();
    v_cv = t1836[1].clone();
    let t1837 = (f__signed(py::getitem(&c__EXACT_TAN()?, &v_ref.clone())?, py::mul(&v_ss.clone(), &v_cs.clone())?)?).items()?;
    if t1837.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tt = t1837[0].clone();
    v_tv = t1837[1].clone();
    return Ok(Py::dict(vec![(Py::str("sin"), v_sv.clone()), (Py::str("cos"), v_cv.clone()), (Py::str("tan"), v_tv.clone()), (Py::str("sin_exact"), v_st.clone()), (Py::str("cos_exact"), v_ct.clone()), (Py::str("tan_exact"), v_tt.clone()), (Py::str("reference_angle"), py::float(&v_ref.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_unit_rate_compare(mut v_n1: Py, mut v_c1: Py, mut v_n2: Py, mut v_c2: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("n1"), v_n1.clone()), (Py::str("c1"), v_c1.clone())]))?;
    if ({ let t1869 = { let t1865 = v_n1.clone(); Py::Bool({ let t1866 = Py::Int(0i64); if py::le(&t1865, &t1866)? { true } else { false } }) }; if t1869.truthy() { t1869 } else { { let t1867 = v_c1.clone(); Py::Bool({ let t1868 = Py::Int(0i64); if py::lt(&t1867, &t1868)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("quantities must be positive and amounts not negative"))));
    }
    v_out = Py::dict(vec![(Py::str("unit_rate_1"), py::truediv(&v_c1.clone(), &v_n1.clone())?)]);
    if ({ let t1874 = { let t1870 = v_n2.clone(); Py::Bool({ let t1871 = Py::None; if py::is(&t1870, &t1871) { true } else { false } }) }; if !t1874.truthy() { t1874 } else { { let t1872 = v_c2.clone(); Py::Bool({ let t1873 = Py::None; if py::is(&t1872, &t1873) { true } else { false } }) } } }).truthy() {
        return Ok(v_out.clone());
    }
    if ({ let t1879 = { let t1875 = v_n2.clone(); Py::Bool({ let t1876 = Py::None; if py::is(&t1875, &t1876) { true } else { false } }) }; if t1879.truthy() { t1879 } else { { let t1877 = v_c2.clone(); Py::Bool({ let t1878 = Py::None; if py::is(&t1877, &t1878) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the second offer needs both its quantity and its amount"))));
    }
    let _ = f__finite(Py::dict(vec![(Py::str("n2"), v_n2.clone()), (Py::str("c2"), v_c2.clone())]))?;
    if ({ let t1884 = { let t1880 = v_n2.clone(); Py::Bool({ let t1881 = Py::Int(0i64); if py::le(&t1880, &t1881)? { true } else { false } }) }; if t1884.truthy() { t1884 } else { { let t1882 = v_c2.clone(); Py::Bool({ let t1883 = Py::Int(0i64); if py::lt(&t1882, &t1883)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("quantities must be positive and amounts not negative"))));
    }
    py::setitem(&v_out.clone(), &Py::str("unit_rate_2"), py::truediv(&v_c2.clone(), &v_n2.clone())?)?;
    let t1885 = (Py::tuple(vec![py::truediv(&py::fraction(&[Py::str(&py::repr(&py::float(&v_c1.clone())?))])?, &py::fraction(&[Py::str(&py::repr(&py::float(&v_n1.clone())?))])?)?, py::truediv(&py::fraction(&[Py::str(&py::repr(&py::float(&v_c2.clone())?))])?, &py::fraction(&[Py::str(&py::repr(&py::float(&v_n2.clone())?))])?)?])).items()?;
    if t1885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1885[0].clone();
    v_b = t1885[1].clone();
    py::setitem(&v_out.clone(), &Py::str("better"), (if ({ let t1886 = v_a.clone(); Py::Bool({ let t1887 = v_b.clone(); if py::lt(&t1886, &t1887)? { true } else { false } }) }).truthy() { Py::str("first") } else { (if ({ let t1888 = v_b.clone(); Py::Bool({ let t1889 = v_a.clone(); if py::lt(&t1888, &t1889)? { true } else { false } }) }).truthy() { Py::str("second") } else { Py::str("equal") }) }))?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vector_dot_angle(mut v_u: Py, mut v_v: Py) -> R {
    let mut v_dot: Py = Py::None;
    let mut v_w: Py = Py::None;
    if ({ let t1905 = { let t1901 = py::len(&v_u.clone())?; Py::Bool({ let t1902 = py::len(&v_v.clone())?; if py::ne(&t1901, &t1902) { true } else { false } }) }; if t1905.truthy() { t1905 } else { { let t1903 = py::len(&v_u.clone())?; Py::Bool({ let t1904 = Py::tuple(vec![Py::Int(2i64), Py::Int(3i64)]); if !py::contains(&t1904, &t1903)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("two vectors of the same dimension (2 or 3)"))));
    }
    for t1906 in py::iter(&(Py::tuple(vec![v_u.clone(), v_v.clone()])))? {
        v_w = t1906;
        if (Py::Bool(({ let mut out = vec![]; for t1907 in py::iter(&(v_w.clone()))? { let mut v_c: Py; 
v_c = t1907;
out.push({ let t1908 = v_c.clone(); Py::Bool({ let t1909 = Py::Int(0i64); if py::eq(&t1908, &t1909) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the zero vector has no direction, so the angle is not defined"))));
        }
    }
    v_dot = py::fsum(&{ let mut out = vec![]; for t1910 in py::iter(&(py::zip(&[v_u.clone(), v_v.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1911 = (t1910).items()?;
if t1911.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1911[0].clone();
v_b = t1911[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("dot"), v_dot.clone()), (Py::str("mag_u"), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t1912 in py::iter(&(v_u.clone()))? { let mut v_a: Py; 
v_a = t1912;
out.push(py::mul(&v_a.clone(), &v_a.clone())?); } Py::list(out) })?)?), (Py::str("mag_v"), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t1913 in py::iter(&(v_v.clone()))? { let mut v_b: Py; 
v_b = t1913;
out.push(py::mul(&v_b.clone(), &v_b.clone())?); } Py::list(out) })?)?), (Py::str("angle_deg"), py::math1("degrees", &py::atan2(&f__cross_mag(v_u.clone(), v_v.clone())?, &v_dot.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vector_magnitude_direction(mut v_x: Py, mut v_y: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("x"), v_x.clone()), (Py::str("y"), v_y.clone())]))?;
    if ({ let t1931 = { let t1927 = v_x.clone(); Py::Bool({ let t1928 = Py::Int(0i64); if py::eq(&t1927, &t1928) { true } else { false } }) }; if !t1931.truthy() { t1931 } else { { let t1929 = v_y.clone(); Py::Bool({ let t1930 = Py::Int(0i64); if py::eq(&t1929, &t1930) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the zero vector has no direction"))));
    }
    return Ok(Py::dict(vec![(Py::str("magnitude"), py::hypot(&[v_x.clone(), v_y.clone()])?), (Py::str("direction_deg"), f__direction(v_x.clone(), v_y.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_counting(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let t259 = (Py::tuple(vec![py::int(&py::getitem(&v_inputs.clone(), &Py::str("n"))?)?, py::int(&py::getitem(&v_inputs.clone(), &Py::str("k"))?)?])).items()?;
    if t259.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t259[0].clone();
    v_k = t259[1].clone();
    let t260 = (Py::tuple(vec![py::fraction(&[py::getitem(&v_outputs.clone(), &Py::str("combinations"))?])?, py::fraction(&[py::getitem(&v_outputs.clone(), &Py::str("permutations"))?])?])).items()?;
    if t260.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t260[0].clone();
    v_p = t260[1].clone();
    v_rel = { let v_c = v_c.clone(); let v_inputs = v_inputs.clone(); let v_k = v_k.clone(); let v_n = v_n.clone(); let v_outputs = v_outputs.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &py::minmax(&[Py::Int(1i64), py::abs(&v_b.clone())?], true)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(py::minmax(&[py::call(&v_rel.clone(), &[py::mul(&v_c.clone(), &py::factorial(&v_k.clone())?)?, v_p.clone()])?, py::call(&v_rel.clone(), &[py::mul(&v_p.clone(), &py::factorial(&py::sub(&v_n.clone(), &v_k.clone())?)?)?, py::factorial(&v_n.clone())?])?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_discriminant(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let t1259 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("a"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?, py::getitem(&v_inputs.clone(), &Py::str("c"))?])).items()?;
    if t1259.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1259[0].clone();
    v_b = t1259[1].clone();
    v_c = t1259[2].clone();
    return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("discriminant"))?, &py::sub(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_a.clone())?, &v_c.clone())?)?)?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::mul(&v_b.clone(), &v_b.clone())?, &py::abs(&py::mul(&py::mul(&Py::Int(4i64), &v_a.clone())?, &v_c.clone())?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_fraction(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_num: Py = Py::None;
    let t604 = (f__cross(py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("c"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("d"))?)?, py::getitem(&v_i.clone(), &Py::str("op"))?)?).items()?;
    if t604.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t604[0].clone();
    v_m = t604[1].clone();
    let t605 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("numerator"))?, py::getitem(&v_o.clone(), &Py::str("denominator"))?])).items()?;
    if t605.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t605[0].clone();
    v_den = t605[1].clone();
    if ({ let t616 = { let t606 = v_den.clone(); Py::Bool({ let t607 = Py::Int(0i64); if py::le(&t606, &t607)? { true } else { false } }) }; if t616.truthy() { t616 } else { { let t615 = { let t608 = v_num.clone(); Py::Bool({ let t609 = py::int(&v_num.clone())?; if py::ne(&t608, &t609) { true } else { false } }) }; if t615.truthy() { t615 } else { { let t614 = { let t610 = v_den.clone(); Py::Bool({ let t611 = py::int(&v_den.clone())?; if py::ne(&t610, &t611) { true } else { false } }) }; if t614.truthy() { t614 } else { { let t612 = py::gcd(&py::int(&v_num.clone())?, &py::int(&v_den.clone())?)?; Py::Bool({ let t613 = Py::Int(1i64); if py::ne(&t612, &t613) { true } else { false } }) } } } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&v_num.clone(), &v_m.clone())?, &py::mul(&v_n.clone(), &v_den.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&v_n.clone(), &v_den.clone())?)?], true)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("value"))?, py::truediv(&v_num.clone(), &v_den.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_roots(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1306 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("a"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?, py::getitem(&v_inputs.clone(), &Py::str("c"))?])).items()?;
    if t1306.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1306[0].clone();
    v_b = t1306[1].clone();
    v_c = t1306[2].clone();
    v_worst = Py::Float(0.0);
    for t1307 in py::iter(&(Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("x1"))?, py::getitem(&v_outputs.clone(), &Py::str("x2"))?])))? {
        v_x = t1307;
        v_terms = Py::list(vec![py::mul(&py::mul(&v_a.clone(), &v_x.clone())?, &v_x.clone())?, py::mul(&v_b.clone(), &v_x.clone())?, v_c.clone()]);
        v_scale = { let t1309 = py::sum(&{ let mut out = vec![]; for t1308 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t1308;
out.push(py::abs(&v_t.clone())?); } Py::list(out) }, None)?; if t1309.truthy() { t1309 } else { Py::Float(1.0) } };
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sum(&v_terms.clone(), None)?)?, &v_scale.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_what_percent(mut v_part: Py, mut v_whole: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("part"), v_part.clone()), (Py::str("whole"), v_whole.clone())]))?;
    if ({ let t1941 = v_whole.clone(); Py::Bool({ let t1942 = Py::Int(0i64); if py::eq(&t1941, &t1942) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the whole must not be 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("percent"), py::truediv(&py::mul(&Py::Int(100i64), &v_part.clone())?, &v_whole.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1038(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("result"))?, &Py::Int(100i64))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::getitem(&v_i.clone(), &Py::str("x"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::getitem(&v_i.clone(), &Py::str("x"))?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1045(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("whole"))?, &py::getitem(&v_i.clone(), &Py::str("p"))?)?, &py::mul(&Py::Int(100i64), &py::getitem(&v_i.clone(), &Py::str("part"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&Py::Int(100i64), &py::getitem(&v_i.clone(), &Py::str("part"))?)?)?, py::abs(&py::mul(&py::getitem(&v_o.clone(), &Py::str("whole"))?, &py::getitem(&v_i.clone(), &Py::str("p"))?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1179(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("x"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("r"))?, &py::math1("cos", &v_t.clone())?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("r"))?)?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("y"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("r"))?, &py::math1("sin", &v_t.clone())?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("r"))?)?], true)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[(if ({ let t1180 = py::getitem(&v_i.clone(), &Py::str("unit"))?; Py::Bool({ let t1181 = Py::str("degrees"); if py::eq(&t1180, &t1181) { true } else { false } }) }).truthy() { py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("theta"))?)? } else { py::getitem(&v_i.clone(), &Py::str("theta"))? })])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1325(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_poly(v_i.clone(), v_o.clone(), Py::Int(4i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1338(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("degrees"))?, &Py::Float(std::f64::consts::PI))?, &Py::Int(180i64))?, &py::getitem(&v_i.clone(), &Py::str("rad"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("rad"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1552(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&f__verify_magdir(v_i.clone(), Py::dict(vec![(Py::str("magnitude"), py::getitem(&v_o.clone(), &Py::str("r"))?), (Py::str("direction_deg"), py::getitem(&v_o.clone(), &Py::str("theta_deg"))?)]))?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("theta_rad"))?, &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("theta_deg"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1581(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::pow(&py::getitem(&v_o.clone(), &Py::str("b"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("a"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Int(2i64))?)?)?, &py::minmax(&[Py::Float(1.0), py::pow(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Int(2i64))?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::math1("sin", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("A"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_i.clone(), &Py::str("a"))?], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("A"))?, &py::getitem(&v_o.clone(), &Py::str("B"))?)?, &Py::Int(90i64))?)?, &Py::Int(90i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1890(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_cmp: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_r = py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("unit_rate_1"))?, &py::getitem(&v_i.clone(), &Py::str("n1"))?)?, &py::getitem(&v_i.clone(), &Py::str("c1"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("c1"))?)?], true)?)?;
    if ({ let t1891 = py::method(&v_i.clone(), "get", &[Py::str("n2")])?; Py::Bool({ let t1892 = Py::None; if !py::is(&t1891, &t1892) { true } else { false } }) }).truthy() {
        if ({ let t1893 = Py::str("unit_rate_2"); Py::Bool({ let t1894 = v_o.clone(); if !py::contains(&t1894, &t1893)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
        v_r = py::iadd(&v_r, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("unit_rate_2"))?, &py::getitem(&v_i.clone(), &Py::str("n2"))?)?, &py::getitem(&v_i.clone(), &Py::str("c2"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("c2"))?)?], true)?)?)?;
        v_cmp = py::sub(&py::mul(&py::fraction(&[Py::str(&py::repr(&py::float(&py::getitem(&v_i.clone(), &Py::str("c1"))?)?))])?, &py::fraction(&[Py::str(&py::repr(&py::float(&py::getitem(&v_i.clone(), &Py::str("n2"))?)?))])?)?, &py::mul(&py::fraction(&[Py::str(&py::repr(&py::float(&py::getitem(&v_i.clone(), &Py::str("c2"))?)?))])?, &py::fraction(&[Py::str(&py::repr(&py::float(&py::getitem(&v_i.clone(), &Py::str("n1"))?)?))])?)?)?;
        v_r = py::iadd(&v_r, &{ let t1895 = py::method(&v_o.clone(), "get", &[Py::str("better")])?; Py::Bool({ let t1896 = (if ({ let t1897 = v_cmp.clone(); Py::Bool({ let t1898 = Py::Int(0i64); if py::lt(&t1897, &t1898)? { true } else { false } }) }).truthy() { Py::str("first") } else { (if ({ let t1899 = v_cmp.clone(); Py::Bool({ let t1900 = Py::Int(0i64); if py::gt(&t1899, &t1900)? { true } else { false } }) }).truthy() { Py::str("second") } else { Py::str("equal") }) }); if py::ne(&t1895, &t1896) { true } else { false } }) })?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1943(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("percent"))?, &py::getitem(&v_i.clone(), &Py::str("whole"))?)?, &py::mul(&Py::Int(100i64), &py::getitem(&v_i.clone(), &Py::str("part"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&Py::Int(100i64), &py::getitem(&v_i.clone(), &Py::str("part"))?)?)?, py::abs(&py::mul(&py::getitem(&v_o.clone(), &Py::str("percent"))?, &py::getitem(&v_i.clone(), &Py::str("whole"))?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_247(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("amount"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("n0"))?, &py::math1("exp", &py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &py::getitem(&v_i.clone(), &Py::str("t"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_276(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_poly(v_i.clone(), v_o.clone(), Py::Int(3i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_402(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::pow(&py::add(&Py::Int(1i64), &py::truediv(&py::getitem(&v_i.clone(), &Py::str("r"))?, &Py::Int(100i64))?)?, &py::getitem(&v_o.clone(), &Py::str("doubling_time"))?)?, &Py::Int(2i64))?)?, &Py::Int(2i64))?, &f__rel(py::getitem(&v_o.clone(), &Py::str("rule_of_70_estimate"))?, py::truediv(&Py::Int(70i64), &py::getitem(&v_i.clone(), &Py::str("r"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_547(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("c"))?, &py::getitem(&v_o.clone(), &Py::str("x"))?)?, &py::getitem(&v_i.clone(), &Py::str("d"))?)?)?)?, &py::getitem(&v_i.clone(), &Py::str("k"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("k"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_627(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::method(&v_i.clone(), "get", &[Py::str("B"), Py::Float(1.0)])?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("x_new"))?, &py::method(&v_i.clone(), "get", &[Py::str("h"), Py::Float(0.0)])?)?)?, &py::getitem(&v_i.clone(), &Py::str("x0"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("x0"))?)?], true)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("y_new"))?, py::add(&py::mul(&py::method(&v_i.clone(), "get", &[Py::str("A"), Py::Float(1.0)])?, &py::getitem(&v_i.clone(), &Py::str("y0"))?)?, &py::method(&v_i.clone(), "get", &[Py::str("k"), Py::Float(0.0)])?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_641(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("remaining"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("n0"))?, &py::pow(&Py::Int(2i64), &py::truediv(&py::neg(&py::getitem(&v_i.clone(), &Py::str("t"))?)?, &py::getitem(&v_i.clone(), &Py::str("half_life"))?)?)?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("fraction"))?, &py::getitem(&v_i.clone(), &Py::str("n0"))?)?, py::getitem(&v_o.clone(), &Py::str("remaining"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_659(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("area"))?, &Py::Int(2i64))?, &py::mul(&py::mul(&py::mul(&v_s.clone(), &py::sub(&v_s.clone(), &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, &py::sub(&v_s.clone(), &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?, &py::sub(&v_s.clone(), &py::getitem(&v_i.clone(), &Py::str("c"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::pow(&py::getitem(&v_o.clone(), &Py::str("area"))?, &Py::Int(2i64))?], true)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("s"))?, v_s.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::truediv(&py::add(&py::add(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &Py::Int(2i64))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_691(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("sum"))?, &py::sub(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("r"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("a"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_718(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("a"))?, &Py::Int(2i64))?, &py::sub(&py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("b"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::math1("cos", &py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("A"))?)?)?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("b"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Int(2i64))?)?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("a"))?, &f__deg_sin(py::getitem(&v_o.clone(), &Py::str("B"))?)?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("b"))?, &f__deg_sin(py::getitem(&v_i.clone(), &Py::str("A"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_o.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::add(&py::getitem(&v_i.clone(), &Py::str("A"))?, &py::getitem(&v_o.clone(), &Py::str("B"))?)?, &py::getitem(&v_o.clone(), &Py::str("C"))?)?, &Py::Int(180i64))?)?, &Py::Int(180i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_735(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&py::mul(&py::math1("cos", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("A"))?)?)?, &Py::Int(2i64))?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::sub(&py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("b"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("a"))?, &Py::Int(2i64))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&py::mul(&py::math1("cos", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("B"))?)?)?, &Py::Int(2i64))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::sub(&py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("a"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("b"))?, &Py::Int(2i64))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::add(&py::getitem(&v_o.clone(), &Py::str("A"))?, &py::getitem(&v_o.clone(), &Py::str("B"))?)?, &py::getitem(&v_o.clone(), &Py::str("C"))?)?, &Py::Int(180i64))?)?, &Py::Int(180i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_811(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::add(&py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_o.clone(), &Py::str("x_intercept"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?)?, &py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_o.clone(), &Py::str("y_intercept"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("c"))?)?], true)?)?, &py::truediv(&py::abs(&py::add(&py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("a"))?)?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_881(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::math1("log", &py::getitem(&v_i.clone(), &Py::str("x"))?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("value"))?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::math1("log", &py::getitem(&v_i.clone(), &Py::str("x"))?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_886(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("value"))?, &py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::getitem(&v_i.clone(), &Py::str("u"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("q"))?, &py::getitem(&v_i.clone(), &Py::str("v"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::getitem(&v_i.clone(), &Py::str("u"))?)?)?, &py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("q"))?, &py::getitem(&v_i.clone(), &Py::str("v"))?)?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_95(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("modulus"))?, &py::math1("cos", &v_t.clone())?)?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_o.clone(), &Py::str("modulus"))?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("modulus"))?, &py::math1("sin", &v_t.clone())?)?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_o.clone(), &Py::str("modulus"))?], true)?)?)?, &py::not(&{ let t96 = py::neg(&Py::Int(180i64))?; Py::Bool({ let t97 = py::getitem(&v_o.clone(), &Py::str("argument_deg"))?; if py::lt(&t96, &t97)? { { let t98 = Py::Int(180i64); if py::le(&t97, &t98)? { true } else { false } } } else { false } }) }))?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("argument_rad"))?, &v_t.clone())?)?)?, &py::truediv(&py::abs(&py::sub(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("modulus"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("a"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("b"))?, &Py::Int(2i64))?)?)?, &py::minmax(&[Py::Float(1.0), py::pow(&py::getitem(&v_o.clone(), &Py::str("modulus"))?, &Py::Int(2i64))?], true)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("argument_deg"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
