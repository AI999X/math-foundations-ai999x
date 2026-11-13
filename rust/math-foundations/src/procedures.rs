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
    "binomial_series",
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
    "line_conic_intersection",
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
    "rational_function_analysis",
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
    "two_conics_intersection",
    "unit_circle_exact",
    "unit_conversion",
    "unit_rate_compare",
    "variation",
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
        "binomial_series" => {
            let r = f_binomial_series(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("terms") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_binomial_series(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "complex_arithmetic" => {
            let r = f_complex_arithmetic(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("op") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_carith(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "complex_modulus_argument" => {
            let r = f_complex_modulus_argument(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_120(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_272(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "counting_choices" => {
            let r = f_counting_choices(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_counting(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cubic_roots" => {
            let r = f_cubic_roots(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_301(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_427(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_572(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_652(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "half_life_remaining" => {
            let r = f_half_life_remaining(match v.get("n0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("half_life") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_666(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "heron_area" => {
            let r = f_heron_area(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_684(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hyperbola_parameters" => {
            let r = f_hyperbola_parameters(match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("orientation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_hyperbola(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "infinite_geometric_series" => {
            let r = f_infinite_geometric_series(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_716(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inverse_linear_function" => {
            let r = f_inverse_linear_function(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_inv(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "law_of_cosines_sas" => {
            let r = f_law_of_cosines_sas(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_743(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "law_of_cosines_sss" => {
            let r = f_law_of_cosines_sss(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_760(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "law_of_sines_aas" => {
            let r = f_law_of_sines_aas(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_aas(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "law_of_sines_ssa" => {
            let r = f_law_of_sines_ssa(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ssa(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "line_conic_intersection" => {
            let r = f_line_conic_intersection(match v.get("line") { Some(x) => py::from_val(x), None => { return None } }, match v.get("conic") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_line_conic(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "line_intercepts" => {
            let r = f_line_intercepts(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_911(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_981(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "log_laws_combine" => {
            let r = f_log_laws_combine(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_986(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1138(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "percent_of_what" => {
            let r = f_percent_of_what(match v.get("part") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1145(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "piecewise_evaluate" => {
            let r = f_piecewise_evaluate(match v.get("e1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("left_rel") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_pw(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polar_to_rectangular" => {
            let r = f_polar_to_rectangular(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1279(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1425(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rad_to_deg" => {
            let r = f_rad_to_deg(match v.get("rad") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1438(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
        "rational_function_analysis" => {
            let r = f_rational_function_analysis(match v.get("numerator") { Some(x) => py::from_val(x), None => { return None } }, match v.get("denominator") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_rational_fn(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rational_roots" => {
            let r = f_rational_roots(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ratroots(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rectangular_to_polar" => {
            let r = f_rectangular_to_polar(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1866(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "right_triangle_angle_side" => {
            let r = f_right_triangle_angle_side(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("side") { Some(x) => py::from_val(x), None => { return None } }, match v.get("which") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_rt_as(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "right_triangle_leg_hypotenuse" => {
            let r = f_right_triangle_leg_hypotenuse(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1895(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
        "two_conics_intersection" => {
            let r = f_two_conics_intersection(match v.get("first") { Some(x) => py::from_val(x), None => { return None } }, match v.get("second") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_two_conics(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "unit_circle_exact" => {
            let r = f_unit_circle_exact(match v.get("deg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_unit(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "unit_conversion" => {
            let r = f_unit_conversion(match v.get("value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("from_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("to_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_unit_conversion(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "unit_rate_compare" => {
            let r = f_unit_rate_compare(match v.get("n1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("c2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2413(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "variation" => {
            let r = f_variation(match v.get("variation_type") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("z2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_variation(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2513(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
fn c__CG_BAND() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1e-12)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__CG_UNITS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("mm"), Py::list(vec![Py::str("length"), Py::Float(0.001)])), (Py::str("cm"), Py::list(vec![Py::str("length"), Py::Float(0.01)])), (Py::str("dm"), Py::list(vec![Py::str("length"), Py::Float(0.1)])), (Py::str("m"), Py::list(vec![Py::str("length"), Py::Float(1.0)])), (Py::str("dam"), Py::list(vec![Py::str("length"), Py::Float(10.0)])), (Py::str("hm"), Py::list(vec![Py::str("length"), Py::Float(100.0)])), (Py::str("km"), Py::list(vec![Py::str("length"), Py::Float(1000.0)])), (Py::str("in"), Py::list(vec![Py::str("length"), Py::Float(0.0254)])), (Py::str("ft"), Py::list(vec![Py::str("length"), Py::Float(0.3048)])), (Py::str("yd"), Py::list(vec![Py::str("length"), Py::Float(0.9144)])), (Py::str("mi"), Py::list(vec![Py::str("length"), Py::Float(1609.344)])), (Py::str("mg"), Py::list(vec![Py::str("mass"), Py::Float(1e-06)])), (Py::str("cg"), Py::list(vec![Py::str("mass"), Py::Float(1e-05)])), (Py::str("dg"), Py::list(vec![Py::str("mass"), Py::Float(0.0001)])), (Py::str("g"), Py::list(vec![Py::str("mass"), Py::Float(0.001)])), (Py::str("dag"), Py::list(vec![Py::str("mass"), Py::Float(0.01)])), (Py::str("hg"), Py::list(vec![Py::str("mass"), Py::Float(0.1)])), (Py::str("kg"), Py::list(vec![Py::str("mass"), Py::Float(1.0)])), (Py::str("t"), Py::list(vec![Py::str("mass"), Py::Float(1000.0)])), (Py::str("oz"), Py::list(vec![Py::str("mass"), Py::Float(0.028349523125)])), (Py::str("lb"), Py::list(vec![Py::str("mass"), Py::Float(0.45359237)])), (Py::str("ton"), Py::list(vec![Py::str("mass"), Py::Float(907.18474)])), (Py::str("ml"), Py::list(vec![Py::str("volume"), Py::Float(0.001)])), (Py::str("cl"), Py::list(vec![Py::str("volume"), Py::Float(0.01)])), (Py::str("dl"), Py::list(vec![Py::str("volume"), Py::Float(0.1)])), (Py::str("l"), Py::list(vec![Py::str("volume"), Py::Float(1.0)])), (Py::str("dal"), Py::list(vec![Py::str("volume"), Py::Float(10.0)])), (Py::str("hl"), Py::list(vec![Py::str("volume"), Py::Float(100.0)])), (Py::str("kl"), Py::list(vec![Py::str("volume"), Py::Float(1000.0)])), (Py::str("fl oz"), Py::list(vec![Py::str("volume"), Py::Float(0.0295735295625)])), (Py::str("cup"), Py::list(vec![Py::str("volume"), Py::Float(0.2365882365)])), (Py::str("pt"), Py::list(vec![Py::str("volume"), Py::Float(0.473176473)])), (Py::str("qt"), Py::list(vec![Py::str("volume"), Py::Float(0.946352946)])), (Py::str("gal"), Py::list(vec![Py::str("volume"), Py::Float(3.785411784)])), (Py::str("s"), Py::list(vec![Py::str("time"), Py::Float(1.0)])), (Py::str("min"), Py::list(vec![Py::str("time"), Py::Float(60.0)])), (Py::str("h"), Py::list(vec![Py::str("time"), Py::Float(3600.0)])), (Py::str("day"), Py::list(vec![Py::str("time"), Py::Float(86400.0)])), (Py::str("week"), Py::list(vec![Py::str("time"), Py::Float(604800.0)])), (Py::str("celsius"), Py::list(vec![Py::str("temperature"), Py::Float(1.0)])), (Py::str("fahrenheit"), Py::list(vec![Py::str("temperature"), Py::Float(1.0)])), (Py::str("kelvin"), Py::list(vec![Py::str("temperature"), Py::Float(1.0)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__CG_VARY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("direct"), Py::str("inverse"), Py::str("joint"), Py::str("combined"), Py::str("direct square"), Py::str("inverse square")])) })());
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
    return Ok((if ({ let t140 = v_t.clone(); Py::Bool({ let t141 = py::sub(&Py::Float(360.0), &Py::Float(1e-09))?; if py::gt(&t140, &t141)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_t.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__big(mut v_v: Py) -> R {
    if ({ let t1201 = Py::Bool(matches!(v_v.clone(), Py::Frac(_))); if !t1201.truthy() { t1201 } else { { let t1200 = { let t1196 = py::len(&Py::str(&py::to_str(&py::abs(&py::numerator(&v_v.clone())?)?)))?; Py::Bool({ let t1197 = c__MAX_DIGITS()?; if py::gt(&t1196, &t1197)? { true } else { false } }) }; if t1200.truthy() { t1200 } else { { let t1198 = py::len(&Py::str(&py::to_str(&py::denominator(&v_v.clone())?)))?; Py::Bool({ let t1199 = c__MAX_DIGITS()?; if py::gt(&t1198, &t1199)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the numbers become too large"))));
    }
    if ({ let t1202 = Py::Bool(matches!(v_v.clone(), Py::Float(_))); if !t1202.truthy() { t1202 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } }).truthy() {
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
    while ({ let t1793 = { let t1789 = py::len(&v_co.clone())?; Py::Bool({ let t1790 = Py::Int(1i64); if py::gt(&t1789, &t1790)? { true } else { false } }) }; if !t1793.truthy() { t1793 } else { { let t1791 = py::getitem(&v_co.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t1792 = Py::Int(0i64); if py::eq(&t1791, &t1792) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_roots.clone(), "add", &[py::fraction(&[Py::Int(0i64)])?])?;
        let _ = py::method(&v_co.clone(), "pop", &[])?;
    }
    if ({ let t1794 = py::len(&v_co.clone())?; Py::Bool({ let t1795 = Py::Int(1i64); if py::gt(&t1794, &t1795)? { true } else { false } }) }).truthy() {
        for t1799 in py::iter(&(f__divisors(py::int(&py::getitem(&v_co.clone(), &py::neg(&Py::Int(1i64))?)?)?)?))? {
            v_p = t1799;
            for t1800 in py::iter(&(f__divisors(py::int(&py::getitem(&v_co.clone(), &Py::Int(0i64))?)?)?))? {
                v_q = t1800;
                for t1801 in py::iter(&(Py::tuple(vec![py::fraction(&[v_p.clone(), v_q.clone()])?, py::fraction(&[py::neg(&v_p.clone())?, v_q.clone()])?])))? {
                    v_cand = t1801;
                    if ({ let t1807 = { let t1802 = v_cand.clone(); Py::Bool({ let t1803 = v_roots.clone(); if !py::contains(&t1803, &t1802)? { true } else { false } }) }; if !t1807.truthy() { t1807 } else { { let t1804 = f__horner(v_co.clone(), v_cand.clone())?; Py::Bool({ let t1805 = Py::Int(0i64); if py::eq(&t1804, &t1805) { true } else { false } }) } } }).truthy() {
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
    v_real = py::sorted(&{ let mut out = vec![]; for t1418 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t1418;
if ({ let t1419 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t1420 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::le(&t1419, &t1420)? { true } else { false } }) }).truthy() { out.push(py::float(&py::real(&v_r.clone())?)?); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
    v_cplx = py::sorted_key(&{ let mut out = vec![]; for t1421 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t1421;
if ({ let t1422 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t1423 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::gt(&t1422, &t1423)? { true } else { false } }) }).truthy() { out.push(py::complex(&[v_r.clone()])?); }} Py::list(out) }, &{ let v_real = v_real.clone(); let v_roots = v_roots.clone(); py::func(move |a: &[Py]| -> R {
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
fn f__cg_bisect(mut v_co: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_flo: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_mid: Py = Py::None;
    v_flo = f__cg_feval(v_co.clone(), v_lo.clone())?;
    for t1674 in py::range_iter(&[(Py::Int(300i64)).i()?]) {
        v__ = t1674;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t1679 = { let t1675 = v_mid.clone(); Py::Bool({ let t1676 = v_lo.clone(); if py::le(&t1675, &t1676)? { true } else { false } }) }; if t1679.truthy() { t1679 } else { { let t1677 = v_mid.clone(); Py::Bool({ let t1678 = v_hi.clone(); if py::ge(&t1677, &t1678)? { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = f__cg_feval(v_co.clone(), v_mid.clone())?;
        if ({ let t1680 = v_fm.clone(); Py::Bool({ let t1681 = Py::Int(0i64); if py::eq(&t1680, &t1681) { true } else { false } }) }).truthy() {
            return Ok(v_mid.clone());
        }
        if ({ let t1682 = { let t1684 = v_fm.clone(); Py::Bool({ let t1685 = Py::Int(0i64); if py::lt(&t1684, &t1685)? { true } else { false } }) }; Py::Bool({ let t1683 = { let t1686 = v_flo.clone(); Py::Bool({ let t1687 = Py::Int(0i64); if py::lt(&t1686, &t1687)? { true } else { false } }) }; if py::eq(&t1682, &t1683) { true } else { false } }) }).truthy() {
            let t1688 = (Py::tuple(vec![v_mid.clone(), v_fm.clone()])).items()?;
            if t1688.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lo = t1688[0].clone();
            v_flo = t1688[1].clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_conic_res(mut v_co: Py, mut v_x: Py, mut v_y: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_t = Py::list(vec![py::mul(&py::mul(&py::getitem(&v_co.clone(), &Py::Int(0i64))?, &v_x.clone())?, &v_x.clone())?, py::mul(&py::mul(&py::getitem(&v_co.clone(), &Py::Int(1i64))?, &v_x.clone())?, &v_y.clone())?, py::mul(&py::mul(&py::getitem(&v_co.clone(), &Py::Int(2i64))?, &v_y.clone())?, &v_y.clone())?, py::mul(&py::getitem(&v_co.clone(), &Py::Int(3i64))?, &v_x.clone())?, py::mul(&py::getitem(&v_co.clone(), &Py::Int(4i64))?, &v_y.clone())?, py::getitem(&v_co.clone(), &Py::Int(5i64))?]);
    v_v = Py::Float(0.0);
    v_s = Py::Float(0.0);
    for t2257 in py::iter(&(v_t.clone()))? {
        v_u = t2257;
        v_v = py::iadd(&v_v, &v_u.clone())?;
        v_s = py::iadd(&v_s, &py::abs(&v_u.clone())?)?;
    }
    return Ok(py::truediv(&py::abs(&v_v.clone())?, &py::minmax(&[Py::Float(1.0), v_s.clone()], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_count_or_none(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let t896: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            return Ok(Some(py::len(&f__cg_quadratic_real(v_a.clone(), v_b.clone(), v_c.clone())?)?));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t896 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::None);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_deriv(mut v_co: Py) -> R {
    let mut v_deg: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_deg = py::sub(&py::len(&v_co.clone())?, &Py::Int(1i64))?;
    v_out = Py::list(vec![]);
    for t1717 in py::range_iter(&[(v_deg.clone()).i()?]) {
        v_k = t1717;
        let _ = py::method(&v_out.clone(), "append", &[py::mul(&py::getitem(&v_co.clone(), &v_k.clone())?, &py::sub(&v_deg.clone(), &v_k.clone())?)?])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_divide_root(mut v_co: Py, mut v_p: Py, mut v_q: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_prev: Py = Py::None;
    v_out = Py::list(vec![]);
    v_prev = Py::Int(0i64);
    for t1605 in py::range_iter(&[(py::sub(&py::len(&v_co.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t1605;
        v_num = py::add(&py::getitem(&v_co.clone(), &v_k.clone())?, &py::mul(&v_p.clone(), &v_prev.clone())?)?;
        if ({ let t1606 = py::modulo(&v_num.clone(), &v_q.clone())?; Py::Bool({ let t1607 = Py::Int(0i64); if py::ne(&t1606, &t1607) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("internal: p/q is not a zero"))));
        }
        v_prev = py::floordiv(&v_num.clone(), &v_q.clone())?;
        let _ = py::method(&v_out.clone(), "append", &[v_prev.clone()])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_divisors(mut v_n: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_n = py::abs(&v_n.clone())?;
    v_out = Py::list(vec![]);
    v_d = Py::Int(1i64);
    while ({ let t1596 = py::mul(&v_d.clone(), &v_d.clone())?; Py::Bool({ let t1597 = v_n.clone(); if py::le(&t1596, &t1597)? { true } else { false } }) }).truthy() {
        if ({ let t1592 = py::modulo(&v_n.clone(), &v_d.clone())?; Py::Bool({ let t1593 = Py::Int(0i64); if py::eq(&t1592, &t1593) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[v_d.clone()])?;
            if ({ let t1594 = py::mul(&v_d.clone(), &v_d.clone())?; Py::Bool({ let t1595 = v_n.clone(); if py::ne(&t1594, &t1595) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_out.clone(), "append", &[py::floordiv(&v_n.clone(), &v_d.clone())?])?;
            }
        }
        v_d = py::iadd(&v_d, &Py::Int(1i64))?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_feval(mut v_co: Py, mut v_x: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = Py::Float(0.0);
    for t1671 in py::iter(&(v_co.clone()))? {
        v_c = t1671;
        v_v = py::add(&py::mul(&v_v.clone(), &v_x.clone())?, &v_c.clone())?;
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_finite(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t2427 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t2427.truthy() { t2427 } else { { let t2426 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t2426.truthy() { t2426 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&v_name.clone(), &Py::str(" must be a finite number"))?)));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_gcd(mut v_a: Py, mut v_b: Py) -> R {
    let t1603 = (Py::tuple(vec![py::abs(&v_a.clone())?, py::abs(&v_b.clone())?])).items()?;
    if t1603.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1603[0].clone();
    v_b = t1603[1].clone();
    while (v_b.clone()).truthy() {
        let t1604 = (Py::tuple(vec![v_b.clone(), py::modulo(&v_a.clone(), &v_b.clone())?])).items()?;
        if t1604.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1604[0].clone();
        v_b = t1604[1].clone();
    }
    return Ok(v_a.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_heval(mut v_co: Py, mut v_p: Py, mut v_q: Py) -> R {
    let mut v_deg: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_total: Py = Py::None;
    v_deg = py::sub(&py::len(&v_co.clone())?, &Py::Int(1i64))?;
    v_total = Py::Int(0i64);
    for t1612 in py::range_iter(&[(py::len(&v_co.clone())?).i()?]) {
        v_i = t1612;
        v_total = py::iadd(&v_total, &py::mul(&py::mul(&py::getitem(&v_co.clone(), &v_i.clone())?, &py::pow(&v_p.clone(), &py::sub(&v_deg.clone(), &v_i.clone())?)?)?, &py::pow(&v_q.clone(), &v_i.clone())?)?)?;
    }
    return Ok(v_total.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_irrational_real_roots(mut v_rest: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bound: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cuts: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_deg: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_qq: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sq: Py = Py::None;
    v_deg = py::sub(&py::len(&v_rest.clone())?, &Py::Int(1i64))?;
    if ({ let t1648 = v_deg.clone(); Py::Bool({ let t1649 = Py::Int(1i64); if py::le(&t1648, &t1649)? { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![]));
    }
    if ({ let t1650 = v_deg.clone(); Py::Bool({ let t1651 = Py::Int(2i64); if py::eq(&t1650, &t1651) { true } else { false } }) }).truthy() {
        let t1652 = (Py::tuple(vec![py::getitem(&v_rest.clone(), &Py::Int(0i64))?, py::getitem(&v_rest.clone(), &Py::Int(1i64))?, py::getitem(&v_rest.clone(), &Py::Int(2i64))?])).items()?;
        if t1652.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1652[0].clone();
        v_b = t1652[1].clone();
        v_c = t1652[2].clone();
        v_disc = py::sub(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_a.clone())?, &v_c.clone())?)?;
        if ({ let t1653 = v_disc.clone(); Py::Bool({ let t1654 = Py::Int(0i64); if py::le(&t1653, &t1654)? { true } else { false } }) }).truthy() {
            return Ok(Py::list(vec![]));
        }
        v_sq = py::math1("sqrt", &v_disc.clone())?;
        if ({ let t1655 = v_b.clone(); Py::Bool({ let t1656 = Py::Int(0i64); if py::ge(&t1655, &t1656)? { true } else { false } }) }).truthy() {
            v_qq = py::mul(&py::neg(&Py::Float(0.5))?, &py::add(&v_b.clone(), &v_sq.clone())?)?;
        } else {
            v_qq = py::mul(&py::neg(&Py::Float(0.5))?, &py::sub(&v_b.clone(), &v_sq.clone())?)?;
        }
        return Ok(f__cg_sorted(Py::list(vec![py::truediv(&v_qq.clone(), &v_a.clone())?, py::truediv(&v_c.clone(), &v_qq.clone())?]))?);
    }
    let t1662 = (Py::tuple(vec![py::getitem(&v_rest.clone(), &Py::Int(0i64))?, py::getitem(&v_rest.clone(), &Py::Int(1i64))?, py::getitem(&v_rest.clone(), &Py::Int(2i64))?, py::getitem(&v_rest.clone(), &Py::Int(3i64))?])).items()?;
    if t1662.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1662[0].clone();
    v_b = t1662[1].clone();
    v_c = t1662[2].clone();
    v_d = t1662[3].clone();
    v_bound = py::add(&Py::Float(1.0), &py::truediv(&py::minmax(&[py::abs(&v_b.clone())?, py::abs(&v_c.clone())?, py::abs(&v_d.clone())?], true)?, &py::abs(&v_a.clone())?)?)?;
    v_cuts = Py::list(vec![py::neg(&v_bound.clone())?]);
    v_dd = py::sub(&py::mul(&py::mul(&Py::Int(4i64), &v_b.clone())?, &v_b.clone())?, &py::mul(&py::mul(&Py::Int(12i64), &v_a.clone())?, &v_c.clone())?)?;
    if ({ let t1663 = v_dd.clone(); Py::Bool({ let t1664 = Py::Int(0i64); if py::gt(&t1663, &t1664)? { true } else { false } }) }).truthy() {
        v_s = py::math1("sqrt", &v_dd.clone())?;
        v_cuts = py::add(&v_cuts.clone(), &f__cg_sorted(Py::list(vec![py::truediv(&py::sub(&py::mul(&py::neg(&Py::Int(2i64))?, &v_b.clone())?, &v_s.clone())?, &py::mul(&Py::Int(6i64), &v_a.clone())?)?, py::truediv(&py::add(&py::mul(&py::neg(&Py::Int(2i64))?, &v_b.clone())?, &v_s.clone())?, &py::mul(&Py::Int(6i64), &v_a.clone())?)?]))?)?;
    }
    let _ = py::method(&v_cuts.clone(), "append", &[v_bound.clone()])?;
    v_out = Py::list(vec![]);
    for t1665 in py::range_iter(&[(py::sub(&py::len(&v_cuts.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t1665;
        let t1666 = (Py::tuple(vec![py::getitem(&v_cuts.clone(), &v_k.clone())?, py::getitem(&v_cuts.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?])).items()?;
        if t1666.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t1666[0].clone();
        v_hi = t1666[1].clone();
        if ({ let t1667 = { let t1669 = f__cg_feval(v_rest.clone(), v_lo.clone())?; Py::Bool({ let t1670 = Py::Int(0i64); if py::lt(&t1669, &t1670)? { true } else { false } }) }; Py::Bool({ let t1668 = { let t1672 = f__cg_feval(v_rest.clone(), v_hi.clone())?; Py::Bool({ let t1673 = Py::Int(0i64); if py::lt(&t1672, &t1673)? { true } else { false } }) }; if py::ne(&t1667, &t1668) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[f__cg_bisect(v_rest.clone(), v_lo.clone(), v_hi.clone())?])?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_mirrored(mut v_pts: Py, mut v_sx: Py, mut v_sy: Py) -> R {
    let mut v_found: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    for t2266 in py::iter(&(v_pts.clone()))? {
        v_p = t2266;
        v_found = Py::Bool(false);
        for t2267 in py::iter(&(v_pts.clone()))? {
            v_q = t2267;
            if ({ let t2272 = { let t2268 = py::abs(&py::sub(&py::getitem(&v_q.clone(), &Py::Int(0i64))?, &py::mul(&v_sx.clone(), &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?)?)?; Py::Bool({ let t2269 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_p.clone(), &Py::Int(0i64))?)?], true)?)?; if py::le(&t2268, &t2269)? { true } else { false } }) }; if !t2272.truthy() { t2272 } else { { let t2270 = py::abs(&py::sub(&py::getitem(&v_q.clone(), &Py::Int(1i64))?, &py::mul(&v_sy.clone(), &py::getitem(&v_p.clone(), &Py::Int(1i64))?)?)?)?; Py::Bool({ let t2271 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_p.clone(), &Py::Int(1i64))?)?], true)?)?; if py::le(&t2270, &t2271)? { true } else { false } }) } } }).truthy() {
                v_found = Py::Bool(true);
            }
        }
        if (py::not(&v_found.clone())).truthy() {
            return Ok(Py::Bool(false));
        }
    }
    return Ok(Py::Bool(true));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_mult(mut v_roots: Py, mut v_p: Py, mut v_q: Py) -> R {
    let mut v_r: Py = Py::None;
    for t1633 in py::iter(&(v_roots.clone()))? {
        v_r = t1633;
        if ({ let t1638 = { let t1634 = py::getitem(&v_r.clone(), &Py::Int(0i64))?; Py::Bool({ let t1635 = v_p.clone(); if py::eq(&t1634, &t1635) { true } else { false } }) }; if !t1638.truthy() { t1638 } else { { let t1636 = py::getitem(&v_r.clone(), &Py::Int(1i64))?; Py::Bool({ let t1637 = v_q.clone(); if py::eq(&t1636, &t1637) { true } else { false } }) } } }).truthy() {
            return Ok(py::getitem(&v_r.clone(), &Py::Int(2i64))?);
        }
    }
    return Ok(Py::Int(0i64));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_order_at(mut v_co: Py, mut v_x: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_P = v_co.clone();
    v_k = Py::Int(0i64);
    while ({ let t1718 = py::len(&v_P.clone())?; Py::Bool({ let t1719 = Py::Int(0i64); if py::gt(&t1718, &t1719)? { true } else { false } }) }).truthy() {
        v_v = Py::Float(0.0);
        v_s = Py::Float(0.0);
        for t1714 in py::iter(&(v_P.clone()))? {
            v_c = t1714;
            v_v = py::add(&py::mul(&v_v.clone(), &v_x.clone())?, &v_c.clone())?;
            v_s = py::add(&py::mul(&v_s.clone(), &py::abs(&v_x.clone())?)?, &py::abs(&v_c.clone())?)?;
        }
        if ({ let t1715 = py::abs(&v_v.clone())?; Py::Bool({ let t1716 = py::mul(&Py::Float(1e-09), &v_s.clone())?; if py::gt(&t1715, &t1716)? { true } else { false } }) }).truthy() {
            return Ok(v_k.clone());
        }
        v_P = f__cg_deriv(v_P.clone())?;
        v_k = py::iadd(&v_k, &Py::Int(1i64))?;
    }
    return Ok(v_k.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_points_in_order(mut v_pts: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    for t2245 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_pts.clone())?).i()?]) {
        v_k = t2245;
        let t2246 = (Py::tuple(vec![py::getitem(&v_pts.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, py::getitem(&v_pts.clone(), &v_k.clone())?])).items()?;
        if t2246.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t2246[0].clone();
        v_q = t2246[1].clone();
        if (py::not(&{ let t2254 = { let t2247 = py::getitem(&v_p.clone(), &Py::Int(0i64))?; Py::Bool({ let t2248 = py::getitem(&v_q.clone(), &Py::Int(0i64))?; if py::lt(&t2247, &t2248)? { true } else { false } }) }; if t2254.truthy() { t2254 } else { { let t2253 = { let t2249 = py::getitem(&v_p.clone(), &Py::Int(0i64))?; Py::Bool({ let t2250 = py::getitem(&v_q.clone(), &Py::Int(0i64))?; if py::eq(&t2249, &t2250) { true } else { false } }) }; if !t2253.truthy() { t2253 } else { { let t2251 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t2252 = py::getitem(&v_q.clone(), &Py::Int(1i64))?; if py::lt(&t2251, &t2252)? { true } else { false } }) } } } } })).truthy() {
            return Ok(Py::Bool(false));
        }
    }
    return Ok(Py::Bool(true));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_quadratic_real(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_disc: Py = Py::None;
    let mut v_qq: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_sq: Py = Py::None;
    v_scale = py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?, py::abs(&v_c.clone())?], true)?;
    if ({ let t2177 = v_a.clone(); Py::Bool({ let t2178 = Py::Float(0.0); if py::eq(&t2177, &t2178) { true } else { false } }) }).truthy() {
        if ({ let t2179 = v_b.clone(); Py::Bool({ let t2180 = Py::Float(0.0); if py::eq(&t2179, &t2180) { true } else { false } }) }).truthy() {
            if ({ let t2181 = v_c.clone(); Py::Bool({ let t2182 = Py::Float(0.0); if py::eq(&t2181, &t2182) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("infinitely many solutions: the curves share a whole line or curve"))));
            }
            return Ok(Py::list(vec![]));
        }
        return Ok(Py::list(vec![py::add(&py::truediv(&py::neg(&v_c.clone())?, &v_b.clone())?, &Py::Float(0.0))?]));
    }
    if ({ let t2183 = py::abs(&v_a.clone())?; Py::Bool({ let t2184 = py::mul(&c__CG_BAND()?, &v_scale.clone())?; if py::le(&t2183, &t2184)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a leading coefficient vanishes within rounding: the number of solutions cannot be decided"))));
    }
    v_disc = py::sub(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&py::mul(&Py::Float(4.0), &v_a.clone())?, &v_c.clone())?)?;
    if ({ let t2185 = v_disc.clone(); Py::Bool({ let t2186 = Py::Float(0.0); if py::eq(&t2185, &t2186) { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![py::add(&py::truediv(&py::neg(&v_b.clone())?, &py::mul(&Py::Float(2.0), &v_a.clone())?)?, &Py::Float(0.0))?]));
    }
    if ({ let t2187 = py::abs(&v_disc.clone())?; Py::Bool({ let t2188 = py::mul(&c__CG_BAND()?, &py::minmax(&[py::mul(&v_b.clone(), &v_b.clone())?, py::abs(&py::mul(&py::mul(&Py::Float(4.0), &v_a.clone())?, &v_c.clone())?)?], true)?)?; if py::le(&t2187, &t2188)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a tangency cannot be decided from rounded values"))));
    }
    if ({ let t2189 = v_disc.clone(); Py::Bool({ let t2190 = Py::Int(0i64); if py::lt(&t2189, &t2190)? { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![]));
    }
    v_sq = py::math1("sqrt", &v_disc.clone())?;
    if ({ let t2191 = v_b.clone(); Py::Bool({ let t2192 = Py::Int(0i64); if py::ge(&t2191, &t2192)? { true } else { false } }) }).truthy() {
        v_qq = py::mul(&py::neg(&Py::Float(0.5))?, &py::add(&v_b.clone(), &v_sq.clone())?)?;
    } else {
        v_qq = py::mul(&py::neg(&Py::Float(0.5))?, &py::sub(&v_b.clone(), &v_sq.clone())?)?;
    }
    v_r1 = py::truediv(&v_qq.clone(), &v_a.clone())?;
    v_r2 = py::truediv(&v_c.clone(), &v_qq.clone())?;
    if ({ let t2193 = v_r1.clone(); Py::Bool({ let t2194 = v_r2.clone(); if py::gt(&t2193, &t2194)? { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![py::add(&v_r2.clone(), &Py::Float(0.0))?, py::add(&v_r1.clone(), &Py::Float(0.0))?]));
    }
    return Ok(Py::list(vec![py::add(&v_r1.clone(), &Py::Float(0.0))?, py::add(&v_r2.clone(), &Py::Float(0.0))?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_rational_roots(mut v_co: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_qs: Py = Py::None;
    let mut v_rest: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_roots = Py::list(vec![]);
    v_rest = v_co.clone();
    v_m = Py::Int(0i64);
    while ({ let t1587 = { let t1583 = py::len(&v_rest.clone())?; Py::Bool({ let t1584 = Py::Int(1i64); if py::gt(&t1583, &t1584)? { true } else { false } }) }; if !t1587.truthy() { t1587 } else { { let t1585 = py::getitem(&v_rest.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t1586 = Py::Int(0i64); if py::eq(&t1585, &t1586) { true } else { false } }) } } }).truthy() {
        v_rest = py::slice(&v_rest.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?;
        v_m = py::iadd(&v_m, &Py::Int(1i64))?;
    }
    if ({ let t1588 = v_m.clone(); Py::Bool({ let t1589 = Py::Int(0i64); if py::gt(&t1588, &t1589)? { true } else { false } }) }).truthy() {
        let _ = py::method(&v_roots.clone(), "append", &[Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), v_m.clone()])])?;
    }
    if ({ let t1590 = py::len(&v_rest.clone())?; Py::Bool({ let t1591 = Py::Int(1i64); if py::gt(&t1590, &t1591)? { true } else { false } }) }).truthy() {
        v_ps = f__cg_divisors(py::getitem(&v_rest.clone(), &py::neg(&Py::Int(1i64))?)?)?;
        v_qs = f__cg_divisors(py::getitem(&v_rest.clone(), &Py::Int(0i64))?)?;
        for t1598 in py::iter(&(v_qs.clone()))? {
            v_q = t1598;
            for t1599 in py::iter(&(v_ps.clone()))? {
                v_p0 = t1599;
                for t1600 in py::iter(&(Py::tuple(vec![v_p0.clone(), py::neg(&v_p0.clone())?])))? {
                    v_p = t1600;
                    if ({ let t1601 = f__cg_gcd(v_p.clone(), v_q.clone())?; Py::Bool({ let t1602 = Py::Int(1i64); if py::ne(&t1601, &t1602) { true } else { false } }) }).truthy() {
                        continue;
                    }
                    v_m = Py::Int(0i64);
                    while ({ let t1613 = { let t1608 = py::len(&v_rest.clone())?; Py::Bool({ let t1609 = Py::Int(1i64); if py::gt(&t1608, &t1609)? { true } else { false } }) }; if !t1613.truthy() { t1613 } else { { let t1610 = f__cg_heval(v_rest.clone(), v_p.clone(), v_q.clone())?; Py::Bool({ let t1611 = Py::Int(0i64); if py::eq(&t1610, &t1611) { true } else { false } }) } } }).truthy() {
                        v_rest = f__cg_divide_root(v_rest.clone(), v_p.clone(), v_q.clone())?;
                        v_m = py::iadd(&v_m, &Py::Int(1i64))?;
                    }
                    if ({ let t1614 = v_m.clone(); Py::Bool({ let t1615 = Py::Int(0i64); if py::gt(&t1614, &t1615)? { true } else { false } }) }).truthy() {
                        let _ = py::method(&v_roots.clone(), "append", &[Py::tuple(vec![v_p.clone(), v_q.clone(), v_m.clone()])])?;
                    }
                }
            }
        }
    }
    v_n = py::len(&v_roots.clone())?;
    for t1616 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t1616;
        for t1617 in py::range_iter(&[(py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_i.clone())?).i()?]) {
            v_j = t1617;
            if ({ let t1618 = py::mul(&py::getitem(&py::getitem(&v_roots.clone(), &v_j.clone())?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_roots.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?, &Py::Int(1i64))?)?; Py::Bool({ let t1619 = py::mul(&py::getitem(&py::getitem(&v_roots.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_roots.clone(), &v_j.clone())?, &Py::Int(1i64))?)?; if py::gt(&t1618, &t1619)? { true } else { false } }) }).truthy() {
                v_t = py::getitem(&v_roots.clone(), &v_j.clone())?;
                py::setitem(&v_roots.clone(), &v_j.clone(), py::getitem(&v_roots.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?;
                py::setitem(&v_roots.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?, v_t.clone())?;
            }
        }
    }
    return Ok(Py::tuple(vec![v_roots.clone(), v_rest.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_sorted(mut v_values: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_out = Py::list(vec![]);
    for t1657 in py::iter(&(v_values.clone()))? {
        v_v = t1657;
        let _ = py::method(&v_out.clone(), "append", &[v_v.clone()])?;
    }
    v_n = py::len(&v_out.clone())?;
    for t1658 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t1658;
        for t1659 in py::range_iter(&[(py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_i.clone())?).i()?]) {
            v_j = t1659;
            if ({ let t1660 = py::getitem(&v_out.clone(), &v_j.clone())?; Py::Bool({ let t1661 = py::getitem(&v_out.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?; if py::gt(&t1660, &t1661)? { true } else { false } }) }).truthy() {
                v_t = py::getitem(&v_out.clone(), &v_j.clone())?;
                py::setitem(&v_out.clone(), &v_j.clone(), py::getitem(&v_out.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?;
                py::setitem(&v_out.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?, v_t.clone())?;
            }
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_sorted_points(mut v_pts: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_out = Py::list(vec![]);
    for t2198 in py::iter(&(v_pts.clone()))? {
        v_p = t2198;
        let _ = py::method(&v_out.clone(), "append", &[v_p.clone()])?;
    }
    v_n = py::len(&v_out.clone())?;
    for t2199 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2199;
        for t2200 in py::range_iter(&[(py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_i.clone())?).i()?]) {
            v_j = t2200;
            if ({ let t2208 = { let t2201 = py::getitem(&py::getitem(&v_out.clone(), &v_j.clone())?, &Py::Int(0i64))?; Py::Bool({ let t2202 = py::getitem(&py::getitem(&v_out.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?, &Py::Int(0i64))?; if py::gt(&t2201, &t2202)? { true } else { false } }) }; if t2208.truthy() { t2208 } else { { let t2207 = { let t2203 = py::getitem(&py::getitem(&v_out.clone(), &v_j.clone())?, &Py::Int(0i64))?; Py::Bool({ let t2204 = py::getitem(&py::getitem(&v_out.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?, &Py::Int(0i64))?; if py::eq(&t2203, &t2204) { true } else { false } }) }; if !t2207.truthy() { t2207 } else { { let t2205 = py::getitem(&py::getitem(&v_out.clone(), &v_j.clone())?, &Py::Int(1i64))?; Py::Bool({ let t2206 = py::getitem(&py::getitem(&v_out.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?, &Py::Int(1i64))?; if py::gt(&t2205, &t2206)? { true } else { false } }) } } } } }).truthy() {
                v_t = py::getitem(&v_out.clone(), &v_j.clone())?;
                py::setitem(&v_out.clone(), &v_j.clone(), py::getitem(&v_out.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?;
                py::setitem(&v_out.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?, v_t.clone())?;
            }
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_temperature(mut v_v: Py, mut v_frm: Py, mut v_to: Py) -> R {
    if ({ let t2351 = v_frm.clone(); Py::Bool({ let t2352 = v_to.clone(); if py::eq(&t2351, &t2352) { true } else { false } }) }).truthy() {
        return Ok(v_v.clone());
    }
    if ({ let t2357 = { let t2353 = v_frm.clone(); Py::Bool({ let t2354 = Py::str("celsius"); if py::eq(&t2353, &t2354) { true } else { false } }) }; if !t2357.truthy() { t2357 } else { { let t2355 = v_to.clone(); Py::Bool({ let t2356 = Py::str("fahrenheit"); if py::eq(&t2355, &t2356) { true } else { false } }) } } }).truthy() {
        return Ok(py::add(&py::truediv(&py::mul(&v_v.clone(), &Py::Float(9.0))?, &Py::Float(5.0))?, &Py::Float(32.0))?);
    }
    if ({ let t2362 = { let t2358 = v_frm.clone(); Py::Bool({ let t2359 = Py::str("fahrenheit"); if py::eq(&t2358, &t2359) { true } else { false } }) }; if !t2362.truthy() { t2362 } else { { let t2360 = v_to.clone(); Py::Bool({ let t2361 = Py::str("celsius"); if py::eq(&t2360, &t2361) { true } else { false } }) } } }).truthy() {
        return Ok(py::truediv(&py::mul(&py::sub(&v_v.clone(), &Py::Float(32.0))?, &Py::Float(5.0))?, &Py::Float(9.0))?);
    }
    if ({ let t2367 = { let t2363 = v_frm.clone(); Py::Bool({ let t2364 = Py::str("celsius"); if py::eq(&t2363, &t2364) { true } else { false } }) }; if !t2367.truthy() { t2367 } else { { let t2365 = v_to.clone(); Py::Bool({ let t2366 = Py::str("kelvin"); if py::eq(&t2365, &t2366) { true } else { false } }) } } }).truthy() {
        return Ok(py::add(&v_v.clone(), &Py::Float(273.15))?);
    }
    if ({ let t2372 = { let t2368 = v_frm.clone(); Py::Bool({ let t2369 = Py::str("kelvin"); if py::eq(&t2368, &t2369) { true } else { false } }) }; if !t2372.truthy() { t2372 } else { { let t2370 = v_to.clone(); Py::Bool({ let t2371 = Py::str("celsius"); if py::eq(&t2370, &t2371) { true } else { false } }) } } }).truthy() {
        return Ok(py::sub(&v_v.clone(), &Py::Float(273.15))?);
    }
    if ({ let t2377 = { let t2373 = v_frm.clone(); Py::Bool({ let t2374 = Py::str("fahrenheit"); if py::eq(&t2373, &t2374) { true } else { false } }) }; if !t2377.truthy() { t2377 } else { { let t2375 = v_to.clone(); Py::Bool({ let t2376 = Py::str("kelvin"); if py::eq(&t2375, &t2376) { true } else { false } }) } } }).truthy() {
        return Ok(py::add(&py::truediv(&py::mul(&py::sub(&v_v.clone(), &Py::Float(32.0))?, &Py::Float(5.0))?, &Py::Float(9.0))?, &Py::Float(273.15))?);
    }
    return Ok(py::add(&py::truediv(&py::mul(&py::sub(&v_v.clone(), &Py::Float(273.15))?, &Py::Float(9.0))?, &Py::Float(5.0))?, &Py::Float(32.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_vary_term(mut v_kind: Py, mut v_x: Py, mut v_z: Py) -> R {
    if ({ let t2461 = v_kind.clone(); Py::Bool({ let t2462 = Py::str("direct"); if py::eq(&t2461, &t2462) { true } else { false } }) }).truthy() {
        return Ok(v_x.clone());
    }
    if ({ let t2463 = v_kind.clone(); Py::Bool({ let t2464 = Py::str("inverse"); if py::eq(&t2463, &t2464) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&Py::Float(1.0), &v_x.clone())?);
    }
    if ({ let t2465 = v_kind.clone(); Py::Bool({ let t2466 = Py::str("direct square"); if py::eq(&t2465, &t2466) { true } else { false } }) }).truthy() {
        return Ok(py::mul(&v_x.clone(), &v_x.clone())?);
    }
    if ({ let t2467 = v_kind.clone(); Py::Bool({ let t2468 = Py::str("inverse square"); if py::eq(&t2467, &t2468) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&Py::Float(1.0), &py::mul(&v_x.clone(), &v_x.clone())?)?);
    }
    if ({ let t2469 = v_kind.clone(); Py::Bool({ let t2470 = Py::str("joint"); if py::eq(&t2469, &t2470) { true } else { false } }) }).truthy() {
        return Ok(py::mul(&v_x.clone(), &v_z.clone())?);
    }
    return Ok(py::truediv(&v_x.clone(), &v_z.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cg_whole_coeffs(mut v_co: Py, mut v_name: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_out = Py::list(vec![]);
    for t1560 in py::iter(&(v_co.clone()))? {
        v_v = t1560;
        if ({ let t1565 = { let t1561 = v_v.clone(); Py::Bool({ let t1562 = py::math1("floor", &v_v.clone())?; if py::ne(&t1561, &t1562) { true } else { false } }) }; if t1565.truthy() { t1565 } else { { let t1563 = py::abs(&v_v.clone())?; Py::Bool({ let t1564 = Py::Int(1000i64); if py::gt(&t1563, &t1564)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&py::add(&v_name.clone(), &Py::str(": whole-number coefficients from -1000 to 1000 are covered"))?)));
        }
        let _ = py::method(&v_out.clone(), "append", &[py::int(&v_v.clone())?])?;
    }
    while ({ let t1570 = { let t1566 = py::len(&v_out.clone())?; Py::Bool({ let t1567 = Py::Int(1i64); if py::gt(&t1566, &t1567)? { true } else { false } }) }; if !t1570.truthy() { t1570 } else { { let t1568 = py::getitem(&v_out.clone(), &Py::Int(0i64))?; Py::Bool({ let t1569 = Py::Int(0i64); if py::eq(&t1568, &t1569) { true } else { false } }) } } }).truthy() {
        v_out = py::slice(&v_out.clone(), Some((Py::Int(1i64)).i()?), None, None)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_name: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t1394 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("a"), v_a.clone()]), Py::tuple(vec![Py::str("b"), v_b.clone()]), Py::tuple(vec![Py::str("c"), v_c.clone()])])))? {
        let t1396 = (t1394).items()?;
        if t1396.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_name = t1396[0].clone();
        v_v = t1396[1].clone();
        if (py::not(&{ let t1395 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1395.truthy() { t1395 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite real number".to_string()].concat()))));
        }
    }
    if ({ let t1397 = v_a.clone(); Py::Bool({ let t1398 = Py::Int(0i64); if py::eq(&t1397, &t1398) { true } else { false } }) }).truthy() {
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
    if ({ let t322 = v_n.clone(); Py::Bool({ let t323 = Py::Int(0i64); if py::lt(&t322, &t323)? { true } else { false } }) }).truthy() {
        v_den = py::add(&py::mul(&v_A.clone(), &v_A.clone())?, &py::mul(&v_B.clone(), &v_B.clone())?)?;
        let t324 = (Py::tuple(vec![py::truediv(&v_A.clone(), &v_den.clone())?, py::truediv(&py::neg(&v_B.clone())?, &v_den.clone())?, py::neg(&v_n.clone())?])).items()?;
        if t324.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_A = t324[0].clone();
        v_B = t324[1].clone();
        v_n = t324[2].clone();
    }
    let t325 = (Py::tuple(vec![py::fraction(&[Py::Int(1i64)])?, py::fraction(&[Py::Int(0i64)])?])).items()?;
    if t325.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rr = t325[0].clone();
    v_ri = t325[1].clone();
    while (v_n.clone()).truthy() {
        if (py::bitand(&v_n.clone(), &Py::Int(1i64))?).truthy() {
            let t326 = (Py::tuple(vec![py::sub(&py::mul(&v_rr.clone(), &v_A.clone())?, &py::mul(&v_ri.clone(), &v_B.clone())?)?, py::add(&py::mul(&v_rr.clone(), &v_B.clone())?, &py::mul(&v_ri.clone(), &v_A.clone())?)?])).items()?;
            if t326.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_rr = t326[0].clone();
            v_ri = t326[1].clone();
        }
        let t327 = (Py::tuple(vec![py::sub(&py::mul(&v_A.clone(), &v_A.clone())?, &py::mul(&v_B.clone(), &v_B.clone())?)?, py::mul(&py::mul(&Py::Int(2i64), &v_A.clone())?, &v_B.clone())?])).items()?;
        if t327.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_A = t327[0].clone();
        v_B = t327[1].clone();
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
    if ({ let t2484 = py::len(&v_u.clone())?; Py::Bool({ let t2485 = Py::Int(2i64); if py::eq(&t2484, &t2485) { true } else { false } }) }).truthy() {
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
    return Ok(py::modulo(&(if ({ let t2502 = v_y.clone(); Py::Bool({ let t2503 = Py::Int(0i64); if py::lt(&t2502, &t2503)? { true } else { false } }) }).truthy() { py::sub(&Py::Float(360.0), &v_t.clone())? } else { v_t.clone() }), &Py::Float(360.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__divisors(mut v_n: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_n = py::abs(&v_n.clone())?;
    v_out = py::set(&Py::list(vec![]))?;
    for t1796 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::isqrt(&v_n.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t1796;
        if ({ let t1797 = py::modulo(&v_n.clone(), &v_k.clone())?; Py::Bool({ let t1798 = Py::Int(0i64); if py::eq(&t1797, &t1798) { true } else { false } }) }).truthy() {
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
    v_up = { let t384 = v_a.clone(); Py::Bool({ let t385 = Py::Int(0i64); if py::gt(&t384, &t385)? { true } else { false } }) };
    v_side = (if (v_up.clone()).truthy() { (if (v_strict.clone()).truthy() { Py::str(">") } else { Py::str(">=") }) } else { (if (v_strict.clone()).truthy() { Py::str("<") } else { Py::str("<=") }) });
    v_E = Py::None;
    if ({ let t386 = v_c.clone(); Py::Bool({ let t387 = Py::Int(0i64); if py::ne(&t386, &t387) { true } else { false } }) }).truthy() {
        v_e = py::truediv(&py::neg(&f__fr(v_d.clone())?)?, &f__fr(v_c.clone())?)?;
        v_inside = { let t395 = (if (v_up.clone()).truthy() { { let t388 = v_e.clone(); Py::Bool({ let t389 = v_B.clone(); if py::gt(&t388, &t389)? { true } else { false } }) } } else { { let t390 = v_e.clone(); Py::Bool({ let t391 = v_B.clone(); if py::lt(&t390, &t391)? { true } else { false } }) } }); if t395.truthy() { t395 } else { { let t394 = { let t392 = v_e.clone(); Py::Bool({ let t393 = v_B.clone(); if py::eq(&t392, &t393) { true } else { false } }) }; if !t394.truthy() { t394 } else { py::not(&v_strict.clone()) } } } };
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
    let t396 = (Py::tuple(vec![(if (v_strict.clone()).truthy() { Py::str("(") } else { Py::str("[") }), (if (v_strict.clone()).truthy() { Py::str("(") } else { Py::str("]") })])).items()?;
    if t396.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo_br = t396[0].clone();
    v_hi_br = t396[1].clone();
    if (v_up.clone()).truthy() {
        if ({ let t397 = v_E.clone(); Py::Bool({ let t398 = Py::None; if py::is(&t397, &t398) { true } else { false } }) }).truthy() {
            v_text = Py::str(&[py::format(&v_lo_br.clone(), "")?, py::format(&py::call(&v_f.clone(), &[v_B.clone()])?, "")?, ", inf)".to_string()].concat());
        } else {
            if ({ let t399 = v_E.clone(); Py::Bool({ let t400 = v_B.clone(); if py::eq(&t399, &t400) { true } else { false } }) }).truthy() {
                v_text = Py::str(&["(".to_string(), py::format(&py::call(&v_f.clone(), &[v_B.clone()])?, "")?, ", inf)".to_string()].concat());
            } else {
                v_text = Py::str(&[py::format(&v_lo_br.clone(), "")?, py::format(&py::call(&v_f.clone(), &[v_B.clone()])?, "")?, ", ".to_string(), py::format(&py::call(&v_f.clone(), &[v_E.clone()])?, "")?, ") U (".to_string(), py::format(&py::call(&v_f.clone(), &[v_E.clone()])?, "")?, ", inf)".to_string()].concat());
            }
        }
    } else {
        v_hi_br = (if (v_strict.clone()).truthy() { Py::str(")") } else { Py::str("]") });
        if ({ let t401 = v_E.clone(); Py::Bool({ let t402 = Py::None; if py::is(&t401, &t402) { true } else { false } }) }).truthy() {
            v_text = Py::str(&["(-inf, ".to_string(), py::format(&py::call(&v_f.clone(), &[v_B.clone()])?, "")?, py::format(&v_hi_br.clone(), "")?].concat());
        } else {
            if ({ let t403 = v_E.clone(); Py::Bool({ let t404 = v_B.clone(); if py::eq(&t403, &t404) { true } else { false } }) }).truthy() {
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
    v_env = { let t1159 = v_env.clone(); if t1159.truthy() { t1159 } else { Py::dict(vec![]) } };
    v_toks = f__tokens(f__prep_expr(v_text.clone(), v_var.clone())?)?;
    v_pos = Py::list(vec![Py::Int(0i64)]);
    v_lbp = (if (py::not(&v_flat.clone())).truthy() { Py::dict(vec![(Py::str("+"), Py::Int(10i64)), (Py::str("-"), Py::Int(10i64)), (Py::str("*"), Py::Int(20i64)), (Py::str("/"), Py::Int(20i64)), (Py::str("**"), Py::Int(30i64))]) } else { Py::dict(vec![(Py::str("+"), Py::Int(10i64)), (Py::str("-"), Py::Int(10i64)), (Py::str("*"), Py::Int(10i64)), (Py::str("/"), Py::Int(10i64)), (Py::str("**"), Py::Int(10i64))]) });
    v_peek = { let v_env = v_env.clone(); let v_flat = v_flat.clone(); let v_lbp = v_lbp.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let v_var = v_var.clone(); let c_nud = c_nud.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok((if ({ let t1171 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t1172 = py::len(&v_toks.clone())?; if py::lt(&t1171, &t1172)? { true } else { false } }) }).truthy() { py::getitem(&v_toks.clone(), &py::getitem(&v_pos.clone(), &Py::Int(0i64))?)? } else { Py::tuple(vec![Py::str("end"), Py::None]) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_take = { let v_env = v_env.clone(); let v_flat = v_flat.clone(); let v_lbp = v_lbp.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let v_var = v_var.clone(); let c_nud = c_nud.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = Py::None;
            v_t = py::call(&v_peek.clone(), &[])?;
            let t1173 = v_pos.clone(); let t1174 = Py::Int(0i64);
            py::setitem(&t1173, &t1174, py::iadd(&py::getitem(&t1173, &t1174)?, &Py::Int(1i64))?)?;
            return Ok(v_t.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let rec1175 = py::rec_cell();
    v_parse = { let v_env = v_env.clone(); let v_flat = v_flat.clone(); let v_lbp = v_lbp.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let v_var = v_var.clone(); let c_nud = c_nud.clone(); let rec1175 = rec1175.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_rbp: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_parse: Py = py::rec_get(&rec1175)?;
            let mut v_left: Py = Py::None;
            let mut v_p: Py = Py::None;
            let mut v_right: Py = Py::None;
            let mut v_t: Py = Py::None;
            v_t = py::call(&v_take.clone(), &[])?;
            v_left = py::call(&py::cell_get(&c_nud)?, &[v_t.clone()])?;
            while (Py::Bool(true)).truthy() {
                v_p = py::call(&v_peek.clone(), &[])?;
                if ({ let t1183 = { let t1176 = py::getitem(&v_p.clone(), &Py::Int(0i64))?; Py::Bool({ let t1177 = Py::str("op"); if py::ne(&t1176, &t1177) { true } else { false } }) }; if t1183.truthy() { t1183 } else { { let t1182 = { let t1178 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t1179 = v_lbp.clone(); if !py::contains(&t1179, &t1178)? { true } else { false } }) }; if t1182.truthy() { t1182 } else { { let t1180 = py::getitem(&v_lbp.clone(), &py::getitem(&v_p.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1181 = v_rbp.clone(); if py::le(&t1180, &t1181)? { true } else { false } }) } } } } }).truthy() {
                    break;
                }
                let _ = py::call(&v_take.clone(), &[])?;
                if ({ let t1184 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t1185 = Py::str("**"); if py::eq(&t1184, &t1185) { true } else { false } }) }).truthy() {
                    v_right = py::call(&v_parse.clone(), &[(if (py::not(&v_flat.clone())).truthy() { Py::Int(29i64) } else { Py::Int(10i64) })])?;
                    v_left = f__pow(v_left.clone(), (if (Py::Bool(matches!(v_right.clone(), Py::Frac(_)))).truthy() { v_right.clone() } else { py::fraction(&[v_right.clone()])? }))?;
                    continue;
                }
                v_right = py::call(&v_parse.clone(), &[py::getitem(&v_lbp.clone(), &py::getitem(&v_p.clone(), &Py::Int(1i64))?)?])?;
                if ({ let t1219 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t1220 = Py::str("+"); if py::eq(&t1219, &t1220) { true } else { false } }) }).truthy() {
                    v_left = f__big(py::add(&v_left.clone(), &v_right.clone())?)?;
                } else {
                    if ({ let t1221 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t1222 = Py::str("-"); if py::eq(&t1221, &t1222) { true } else { false } }) }).truthy() {
                        v_left = f__big(py::sub(&v_left.clone(), &v_right.clone())?)?;
                    } else {
                        if ({ let t1223 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t1224 = Py::str("*"); if py::eq(&t1223, &t1224) { true } else { false } }) }).truthy() {
                            v_left = f__big(py::mul(&v_left.clone(), &v_right.clone())?)?;
                        } else {
                            if ({ let t1225 = v_right.clone(); Py::Bool({ let t1226 = Py::Int(0i64); if py::eq(&t1225, &t1226) { true } else { false } }) }).truthy() {
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
    py::rec_set(&rec1175, &v_parse);
    py::cell_set(&c_nud, { let v_env = v_env.clone(); let v_flat = v_flat.clone(); let v_lbp = v_lbp.clone(); let v_parse = v_parse.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let v_var = v_var.clone(); let c_nud = c_nud.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_inner: Py = Py::None;
            let mut v_kind: Py = Py::None;
            let mut v_v: Py = Py::None;
            let mut v_x: Py = Py::None;
            let t1227 = (v_t.clone()).items()?;
            if t1227.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_kind = t1227[0].clone();
            v_v = t1227[1].clone();
            if ({ let t1228 = v_kind.clone(); Py::Bool({ let t1229 = Py::str("num"); if py::eq(&t1228, &t1229) { true } else { false } }) }).truthy() {
                return Ok(py::fraction(&[v_v.clone()])?);
            }
            if ({ let t1230 = v_kind.clone(); Py::Bool({ let t1231 = Py::str("name"); if py::eq(&t1230, &t1231) { true } else { false } }) }).truthy() {
                if ({ let t1232 = v_v.clone(); Py::Bool({ let t1233 = v_env.clone(); if py::contains(&t1233, &t1232)? { true } else { false } }) }).truthy() {
                    v_x = py::getitem(&v_env.clone(), &v_v.clone())?;
                    return Ok((if (Py::Bool(matches!(v_x.clone(), Py::Float(_)))).truthy() { py::fraction(&[Py::str(&py::repr(&py::float(&v_x.clone())?))])? } else { py::fraction(&[v_x.clone()])? }));
                }
                if ({ let t1234 = v_v.clone(); Py::Bool({ let t1235 = c__FUNC_NAMES()?; if py::contains(&t1235, &t1234)? { true } else { false } }) }).truthy() {
                    if ({ let t1236 = py::call(&v_take.clone(), &[])?; Py::Bool({ let t1237 = Py::tuple(vec![Py::str("op"), Py::str("(")]); if py::ne(&t1236, &t1237) { true } else { false } }) }).truthy() {
                        return Err(py::value_error(py::to_str(&Py::str("a function needs parentheses"))));
                    }
                    v_inner = py::call(&v_parse.clone(), &[Py::Int(0i64)])?;
                    if ({ let t1238 = py::call(&v_take.clone(), &[])?; Py::Bool({ let t1239 = Py::tuple(vec![Py::str("op"), Py::str(")")]); if py::ne(&t1238, &t1239) { true } else { false } }) }).truthy() {
                        return Err(py::value_error(py::to_str(&Py::str("unbalanced parentheses"))));
                    }
                    return Ok(f__fn(v_v.clone(), v_inner.clone())?);
                }
                return Err(py::value_error(py::to_str(&Py::str(&["unknown name ".to_string(), py::format(&Py::str(&py::repr(&v_v.clone())), "")?].concat()))));
            }
            if ({ let t1248 = v_t.clone(); Py::Bool({ let t1249 = Py::tuple(vec![Py::str("op"), Py::str("(")]); if py::eq(&t1248, &t1249) { true } else { false } }) }).truthy() {
                v_inner = py::call(&v_parse.clone(), &[Py::Int(0i64)])?;
                if ({ let t1250 = py::call(&v_take.clone(), &[])?; Py::Bool({ let t1251 = Py::tuple(vec![Py::str("op"), Py::str(")")]); if py::ne(&t1250, &t1251) { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("unbalanced parentheses"))));
                }
                return Ok(v_inner.clone());
            }
            if ({ let t1252 = v_t.clone(); Py::Bool({ let t1253 = Py::tuple(vec![Py::str("op"), Py::str("-")]); if py::eq(&t1252, &t1253) { true } else { false } }) }).truthy() {
                return Ok(py::neg(&py::call(&v_parse.clone(), &[(if (py::not(&v_flat.clone())).truthy() { Py::Int(25i64) } else { Py::Int(30i64) })])?)?);
            }
            if ({ let t1254 = v_t.clone(); Py::Bool({ let t1255 = Py::tuple(vec![Py::str("op"), Py::str("+")]); if py::eq(&t1254, &t1255) { true } else { false } }) }).truthy() {
                return Ok(py::call(&v_parse.clone(), &[(if (py::not(&v_flat.clone())).truthy() { Py::Int(25i64) } else { Py::Int(30i64) })])?);
            }
            return Err(py::value_error(py::to_str(&Py::str("cannot read the expression"))));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) });
    v_value = py::call(&v_parse.clone(), &[Py::Int(0i64)])?;
    if ({ let t1256 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t1257 = py::len(&v_toks.clone())?; if py::ne(&t1256, &t1257) { true } else { false } }) }).truthy() {
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
    if ({ let t2324 = v_t.clone(); Py::Bool({ let t2325 = Py::str("undefined"); if py::eq(&t2324, &t2325) { true } else { false } }) }).truthy() {
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
    for t2507 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t2510 = (t2507).items()?;
        if t2510.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2510[0].clone();
        v_v = t2510[1].clone();
        if ({ let t2509 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t2509.truthy() { t2509 } else { { let t2508 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t2508.truthy() { t2508 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must be a finite number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fn(mut v_name: Py, mut v_v: Py) -> R {
    if ({ let t1240 = v_name.clone(); Py::Bool({ let t1241 = Py::str("sqrt"); if py::eq(&t1240, &t1241) { true } else { false } }) }).truthy() {
        return Ok(f__sqrt(v_v.clone())?);
    }
    if ({ let t1242 = v_name.clone(); Py::Bool({ let t1243 = Py::str("abs"); if py::eq(&t1242, &t1243) { true } else { false } }) }).truthy() {
        return Ok(py::abs(&v_v.clone())?);
    }
    if ({ let t1244 = v_name.clone(); Py::Bool({ let t1245 = Py::str("ln"); if py::eq(&t1244, &t1245) { true } else { false } }) }).truthy() {
        if ({ let t1246 = v_v.clone(); Py::Bool({ let t1247 = Py::Int(0i64); if py::le(&t1246, &t1247)? { true } else { false } }) }).truthy() {
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
    if ({ let t1158 = py::not(&Py::Bool(matches!(v_expression.clone(), Py::Str(_)))); if t1158.truthy() { t1158 } else { py::not(&py::method(&c__FEXPR()?, "match", &[py::method(&f__clean(v_expression.clone())?, "lower", &[])?])?) } }).truthy() {
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
    v_m = { let mut out = vec![]; for t918 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t918;
out.push({ let mut out = vec![]; for t919 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t919;
out.push(f__fr(v_v.clone())?); } Py::list(out) }); } Py::list(out) };
    for t920 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_col = t920;
        v_piv = (match ('g921: { let mut out = vec![]; for t922 in py::range_iter(&[(v_col.clone()).i()?, (v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t922;
if ({ let t923 = py::getitem(&py::getitem(&v_m.clone(), &v_r.clone())?, &v_col.clone())?; Py::Bool({ let t924 = Py::Int(0i64); if py::ne(&t923, &t924) { true } else { false } }) }).truthy() { out.push(v_r.clone()); break 'g921 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
        if ({ let t925 = v_piv.clone(); Py::Bool({ let t926 = Py::None; if py::is(&t925, &t926) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the system has no unique solution (its determinant is 0)"))));
        }
        let t927 = (Py::tuple(vec![py::getitem(&v_m.clone(), &v_piv.clone())?, py::getitem(&v_m.clone(), &v_col.clone())?])).items()?;
        if t927.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_m.clone(), &v_col.clone(), t927[0].clone())?;
        py::setitem(&v_m.clone(), &v_piv.clone(), t927[1].clone())?;
        for t928 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_r = t928;
            if ({ let t933 = { let t929 = v_r.clone(); Py::Bool({ let t930 = v_col.clone(); if py::ne(&t929, &t930) { true } else { false } }) }; if !t933.truthy() { t933 } else { { let t931 = py::getitem(&py::getitem(&v_m.clone(), &v_r.clone())?, &v_col.clone())?; Py::Bool({ let t932 = Py::Int(0i64); if py::ne(&t931, &t932) { true } else { false } }) } } }).truthy() {
                v_f = py::truediv(&py::getitem(&py::getitem(&v_m.clone(), &v_r.clone())?, &v_col.clone())?, &py::getitem(&py::getitem(&v_m.clone(), &v_col.clone())?, &v_col.clone())?)?;
                py::setitem(&v_m.clone(), &v_r.clone(), { let mut out = vec![]; for t934 in py::iter(&(py::zip(&[py::getitem(&v_m.clone(), &v_r.clone())?, py::getitem(&v_m.clone(), &v_col.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t935 = (t934).items()?;
if t935.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t935[0].clone();
v_b = t935[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_f.clone(), &v_b.clone())?)?); } Py::list(out) })?;
            }
        }
    }
    return Ok({ let mut out = vec![]; for t936 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t936;
out.push(py::truediv(&py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_n.clone())?, &py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_i.clone())?)?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__horner(mut v_co: Py, mut v_x: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = py::fraction(&[Py::Int(0i64)])?;
    for t1806 in py::iter(&(v_co.clone()))? {
        v_c = t1806;
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
    let t706 = (Py::tuple(vec![py::math1("sqrt", &py::getitem(&v_i.clone(), &Py::str("A"))?)?, py::math1("sqrt", &py::getitem(&v_i.clone(), &Py::str("B"))?)?])).items()?;
    if t706.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t706[0].clone();
    v_b = t706[1].clone();
    if ({ let t707 = py::getitem(&v_i.clone(), &Py::str("orientation"))?; Py::Bool({ let t708 = Py::str("horizontal"); if py::eq(&t707, &t708) { true } else { false } }) }).truthy() {
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
        v_r = (if ({ let t1375 = v_rel.clone(); Py::Bool({ let t1376 = Py::tuple(vec![Py::str("<"), Py::str(">")]); if py::contains(&t1376, &t1375)? { true } else { false } }) }).truthy() { Py::str("<") } else { Py::str("<=") });
        return Ok(Py::str(&[py::format(&py::call(&v_f.clone(), &[v_lo.clone()])?, "")?, " ".to_string(), py::format(&v_r.clone(), "")?, " x ".to_string(), py::format(&v_r.clone(), "")?, " ".to_string(), py::format(&py::call(&v_f.clone(), &[v_hi.clone()])?, "")?].concat()));
    }
    let t1379 = ((if ({ let t1377 = v_rel.clone(); Py::Bool({ let t1378 = Py::tuple(vec![Py::str("<"), Py::str(">")]); if py::contains(&t1378, &t1377)? { true } else { false } }) }).truthy() { Py::tuple(vec![Py::str("<"), Py::str(">")]) } else { Py::tuple(vec![Py::str("<="), Py::str(">=")]) })).items()?;
    if t1379.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lt = t1379[0].clone();
    v_gt = t1379[1].clone();
    return Ok(Py::str(&["x ".to_string(), py::format(&v_lt.clone(), "")?, " ".to_string(), py::format(&py::call(&v_f.clone(), &[v_lo.clone()])?, "")?, " or x ".to_string(), py::format(&v_gt.clone(), "")?, " ".to_string(), py::format(&py::call(&v_f.clone(), &[v_hi.clone()])?, "")?].concat()));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__poly_roots(mut v_coeffs: Py) -> R {
    let mut v_name: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t1412 in py::iter(&(py::zip(&[c__LETTERS()?, v_coeffs.clone()])?))? {
        let t1414 = (t1412).items()?;
        if t1414.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_name = t1414[0].clone();
        v_v = t1414[1].clone();
        if (py::not(&{ let t1413 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1413.truthy() { t1413 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite real number".to_string()].concat()))));
        }
    }
    if ({ let t1415 = py::getitem(&v_coeffs.clone(), &Py::Int(0i64))?; Py::Bool({ let t1416 = Py::Int(0i64); if py::eq(&t1415, &t1416) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["a must not be 0: with a = 0 the equation has a lower degree".to_string()].concat()))));
    }
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_roots = ax_engine::pymp::call("polyroots", &[{ let mut out = vec![]; for t1417 in py::iter(&(v_coeffs.clone()))? { let mut v_c: Py; 
v_c = t1417;
out.push(ax_engine::pymp::call("mpf", &[Py::str(&py::to_str(&v_c.clone()))], &[])?); } Py::list(out) }], &[(String::from("maxsteps"), Py::Int(200i64)), (String::from("extraprec"), Py::Int(60i64))])?;
    }
    return Ok(f__canonical_roots({ let mut out = vec![]; for t1424 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t1424;
out.push(py::complex(&[v_r.clone()])?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pow(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_r: Py = Py::None;
    if ({ let t1188 = Py::Bool(matches!(v_b.clone(), Py::Frac(_))); if !t1188.truthy() { t1188 } else { { let t1186 = py::denominator(&v_b.clone())?; Py::Bool({ let t1187 = Py::Int(1i64); if py::eq(&t1186, &t1187) { true } else { false } }) } } }).truthy() {
        if ({ let t1189 = py::abs(&v_b.clone())?; Py::Bool({ let t1190 = Py::Int(64i64); if py::gt(&t1189, &t1190)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("exponents are limited to 64 in size"))));
        }
        if ({ let t1195 = { let t1191 = v_a.clone(); Py::Bool({ let t1192 = Py::Int(0i64); if py::eq(&t1191, &t1192) { true } else { false } }) }; if !t1195.truthy() { t1195 } else { { let t1193 = v_b.clone(); Py::Bool({ let t1194 = Py::Int(0i64); if py::lt(&t1193, &t1194)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("division by zero (zero to a negative power)"))));
        }
        return Ok((if (Py::Bool(matches!(v_a.clone(), Py::Frac(_)))).truthy() { f__big(py::pow(&v_a.clone(), &py::int(&v_b.clone())?)?)? } else { f__big(py::pow(&py::float(&v_a.clone())?, &py::int(&v_b.clone())?)?)? }));
    }
    if ({ let t1208 = Py::Bool(matches!(v_b.clone(), Py::Frac(_))); if !t1208.truthy() { t1208 } else { { let t1207 = { let t1203 = py::denominator(&v_b.clone())?; Py::Bool({ let t1204 = Py::Int(2i64); if py::eq(&t1203, &t1204) { true } else { false } }) }; if !t1207.truthy() { t1207 } else { { let t1205 = v_a.clone(); Py::Bool({ let t1206 = Py::Int(0i64); if py::ge(&t1205, &t1206)? { true } else { false } }) } } } } }).truthy() {
        v_r = f__sqrt((if (Py::Bool(matches!(v_a.clone(), Py::Frac(_)))).truthy() { v_a.clone() } else { py::fraction(&[v_a.clone()])? }))?;
        return Ok(f__pow(v_r.clone(), py::fraction(&[py::numerator(&v_b.clone())?])?)?);
    }
    if ({ let t1217 = v_a.clone(); Py::Bool({ let t1218 = Py::Int(0i64); if py::le(&t1217, &t1218)? { true } else { false } }) }).truthy() {
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
    let t2492 = (Py::tuple(vec![py::complex(&[v_x.clone()])?, py::complex(&[v_y.clone()])?])).items()?;
    if t2492.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2492[0].clone();
    v_y = t2492[1].clone();
    return Ok(py::truediv(&py::abs(&py::sub(&v_x.clone(), &v_y.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_x.clone())?, py::abs(&v_y.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__roots_out(mut v_roots: Py) -> R {
    return Ok({ let mut out = vec![]; for t1410 in py::iter(&(py::enumerate(&v_roots.clone(), 0)?))? { let mut v_i: Py; let mut v_r: Py; 
let t1411 = (t1410).items()?;
if t1411.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1411[0].clone();
v_r = t1411[1].clone();
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
    return Ok(py::truediv(&(if ({ let t1937 = v_q.clone(); Py::Bool({ let t1938 = Py::Int(0i64); if py::ge(&t1937, &t1938)? { true } else { false } }) }).truthy() { v_k.clone() } else { py::neg(&v_k.clone())? }), &v_scale.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sig_exponent(mut v_q: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    v_a = py::abs(&v_q.clone())?;
    v_e = py::sub(&py::len(&Py::str(&py::to_str(&py::numerator(&v_a.clone())?)))?, &py::len(&Py::str(&py::to_str(&py::denominator(&v_a.clone())?)))?)?;
    while ({ let t1960 = py::pow(&py::fraction(&[Py::Int(10i64)])?, &v_e.clone())?; Py::Bool({ let t1961 = v_a.clone(); if py::gt(&t1960, &t1961)? { true } else { false } }) }).truthy() {
        v_e = py::sub(&v_e, &Py::Int(1i64))?;
    }
    while ({ let t1962 = py::pow(&py::fraction(&[Py::Int(10i64)])?, &py::add(&v_e.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1963 = v_a.clone(); if py::le(&t1962, &t1963)? { true } else { false } }) }).truthy() {
        v_e = py::iadd(&v_e, &Py::Int(1i64))?;
    }
    return Ok(v_e.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sign_root_expr(mut v_p: Py, mut v_q: Py, mut v_D: Py) -> R {
    let mut v_big: Py = Py::None;
    if ({ let t1473 = { let t1469 = v_q.clone(); Py::Bool({ let t1470 = Py::Int(0i64); if py::eq(&t1469, &t1470) { true } else { false } }) }; if t1473.truthy() { t1473 } else { { let t1471 = v_D.clone(); Py::Bool({ let t1472 = Py::Int(0i64); if py::eq(&t1471, &t1472) { true } else { false } }) } } }).truthy() {
        return Ok(py::sub(&{ let t1474 = v_p.clone(); Py::Bool({ let t1475 = Py::Int(0i64); if py::gt(&t1474, &t1475)? { true } else { false } }) }, &{ let t1476 = v_p.clone(); Py::Bool({ let t1477 = Py::Int(0i64); if py::lt(&t1476, &t1477)? { true } else { false } }) })?);
    }
    if ({ let t1482 = { let t1478 = v_p.clone(); Py::Bool({ let t1479 = Py::Int(0i64); if py::ge(&t1478, &t1479)? { true } else { false } }) }; if !t1482.truthy() { t1482 } else { { let t1480 = v_q.clone(); Py::Bool({ let t1481 = Py::Int(0i64); if py::gt(&t1480, &t1481)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Int(1i64));
    }
    if ({ let t1487 = { let t1483 = v_p.clone(); Py::Bool({ let t1484 = Py::Int(0i64); if py::le(&t1483, &t1484)? { true } else { false } }) }; if !t1487.truthy() { t1487 } else { { let t1485 = v_q.clone(); Py::Bool({ let t1486 = Py::Int(0i64); if py::lt(&t1485, &t1486)? { true } else { false } }) } } }).truthy() {
        return Ok(py::neg(&Py::Int(1i64))?);
    }
    v_big = py::sub(&py::mul(&v_p.clone(), &v_p.clone())?, &py::mul(&py::mul(&v_q.clone(), &v_q.clone())?, &v_D.clone())?)?;
    if ({ let t1488 = v_big.clone(); Py::Bool({ let t1489 = Py::Int(0i64); if py::eq(&t1488, &t1489) { true } else { false } }) }).truthy() {
        return Ok(Py::Int(0i64));
    }
    return Ok((if ({ let t1490 = v_big.clone(); Py::Bool({ let t1491 = Py::Int(0i64); if py::gt(&t1490, &t1491)? { true } else { false } }) }).truthy() { py::sub(&{ let t1492 = v_p.clone(); Py::Bool({ let t1493 = Py::Int(0i64); if py::gt(&t1492, &t1493)? { true } else { false } }) }, &{ let t1494 = v_p.clone(); Py::Bool({ let t1495 = Py::Int(0i64); if py::lt(&t1494, &t1495)? { true } else { false } }) })? } else { py::sub(&{ let t1496 = v_q.clone(); Py::Bool({ let t1497 = Py::Int(0i64); if py::gt(&t1496, &t1497)? { true } else { false } }) }, &{ let t1498 = v_q.clone(); Py::Bool({ let t1499 = Py::Int(0i64); if py::lt(&t1498, &t1499)? { true } else { false } }) })? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__signed(mut v_text_val: Py, mut v_sign: Py) -> R {
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2304 = (v_text_val.clone()).items()?;
    if t2304.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t2304[0].clone();
    v_v = t2304[1].clone();
    if ({ let t2305 = v_v.clone(); Py::Bool({ let t2306 = Py::None; if py::is(&t2305, &t2306) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![v_t.clone(), Py::None]));
    }
    if ({ let t2311 = { let t2307 = v_sign.clone(); Py::Bool({ let t2308 = Py::Int(0i64); if py::lt(&t2307, &t2308)? { true } else { false } }) }; if !t2311.truthy() { t2311 } else { { let t2309 = v_v.clone(); Py::Bool({ let t2310 = Py::Int(0i64); if py::ne(&t2309, &t2310) { true } else { false } }) } } }).truthy() {
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
    if ({ let t1209 = v_v.clone(); Py::Bool({ let t1210 = Py::Int(0i64); if py::lt(&t1209, &t1210)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("square root of a negative number"))));
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Frac(_)))).truthy() {
        let t1211 = (Py::tuple(vec![py::isqrt(&py::numerator(&v_v.clone())?)?, py::isqrt(&py::denominator(&v_v.clone())?)?])).items()?;
        if t1211.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t1211[0].clone();
        v_d = t1211[1].clone();
        if ({ let t1216 = { let t1212 = py::mul(&v_n.clone(), &v_n.clone())?; Py::Bool({ let t1213 = py::numerator(&v_v.clone())?; if py::eq(&t1212, &t1213) { true } else { false } }) }; if !t1216.truthy() { t1216 } else { { let t1214 = py::mul(&v_d.clone(), &v_d.clone())?; Py::Bool({ let t1215 = py::denominator(&v_v.clone())?; if py::eq(&t1214, &t1215) { true } else { false } }) } } }).truthy() {
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
    while ({ let t1975 = { let t1971 = py::len(&v_co.clone())?; Py::Bool({ let t1972 = Py::Int(1i64); if py::gt(&t1971, &t1972)? { true } else { false } }) }; if !t1975.truthy() { t1975 } else { { let t1973 = py::getitem(&v_co.clone(), &Py::Int(0i64))?; Py::Bool({ let t1974 = Py::Int(0i64); if py::eq(&t1973, &t1974) { true } else { false } }) } } }).truthy() {
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
    let t1160 = (Py::tuple(vec![Py::list(vec![]), Py::Int(0i64)])).items()?;
    if t1160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_out = t1160[0].clone();
    v_pos = t1160[1].clone();
    v_s = py::method(&v_s.clone(), "strip", &[])?;
    while ({ let t1169 = v_pos.clone(); Py::Bool({ let t1170 = py::len(&v_s.clone())?; if py::lt(&t1169, &t1170)? { true } else { false } }) }).truthy() {
        v_m = py::method(&c__TOK()?, "match", &[v_s.clone(), v_pos.clone()])?;
        if ({ let t1163 = py::not(&v_m.clone()); if t1163.truthy() { t1163 } else { { let t1161 = py::method(&v_m.clone(), "end", &[])?; Py::Bool({ let t1162 = v_pos.clone(); if py::eq(&t1161, &t1162) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the expression at ".to_string(), py::format(&Py::str(&py::repr(&py::slice(&v_s.clone(), Some((v_pos.clone()).i()?), Some((py::add(&v_pos.clone(), &Py::Int(10i64))?).i()?), None)?)), "")?].concat()))));
        }
        let _ = py::method(&v_out.clone(), "append", &[(if (py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?).truthy() { Py::tuple(vec![Py::str("num"), py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?]) } else { (if (py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?).truthy() { Py::tuple(vec![Py::str("name"), py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?]) } else { Py::tuple(vec![Py::str("op"), py::method(&v_m.clone(), "group", &[Py::Int(3i64)])?]) }) })])?;
        v_pos = py::method(&v_m.clone(), "end", &[])?;
        while ({ let t1168 = { let t1164 = v_pos.clone(); Py::Bool({ let t1165 = py::len(&v_s.clone())?; if py::lt(&t1164, &t1165)? { true } else { false } }) }; if !t1168.truthy() { t1168 } else { { let t1166 = py::getitem(&v_s.clone(), &v_pos.clone())?; Py::Bool({ let t1167 = Py::str(" "); if py::eq(&t1166, &t1167) { true } else { false } }) } } }).truthy() {
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
    if ({ let t758 = { let t748 = py::minmax(&[v_a.clone(), v_b.clone(), v_c.clone()], false)?; Py::Bool({ let t749 = Py::Int(0i64); if py::le(&t748, &t749)? { true } else { false } }) }; if t758.truthy() { t758 } else { { let t757 = { let t750 = py::add(&v_a.clone(), &v_b.clone())?; Py::Bool({ let t751 = v_c.clone(); if py::le(&t750, &t751)? { true } else { false } }) }; if t757.truthy() { t757 } else { { let t756 = { let t752 = py::add(&v_a.clone(), &v_c.clone())?; Py::Bool({ let t753 = v_b.clone(); if py::le(&t752, &t753)? { true } else { false } }) }; if t756.truthy() { t756 } else { { let t754 = py::add(&v_b.clone(), &v_c.clone())?; Py::Bool({ let t755 = v_a.clone(); if py::le(&t754, &t755)? { true } else { false } }) } } } } } } }).truthy() {
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
    let t775 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_i.clone(), &Py::str("B"))?, py::getitem(&v_i.clone(), &Py::str("a"))?])).items()?;
    if t775.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t775[0].clone();
    v_B = t775[1].clone();
    v_a = t775[2].clone();
    let t776 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("b"))?, py::getitem(&v_o.clone(), &Py::str("c"))?, py::getitem(&v_o.clone(), &Py::str("C"))?])).items()?;
    if t776.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b = t776[0].clone();
    v_c = t776[1].clone();
    v_C = t776[2].clone();
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
fn f__verify_binomial_series(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_ex: Py = Py::None;
    let mut v_fact: Py = Py::None;
    let mut v_got: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_pc: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t71 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("x"))?, py::getitem(&v_i.clone(), &Py::str("n"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("terms"))?)?])).items()?;
    if t71.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t71[0].clone();
    v_n = t71[1].clone();
    v_m = t71[2].clone();
    v_vals = py::getitem(&v_o.clone(), &Py::str("term_values"))?;
    if ({ let t72 = py::len(&v_vals.clone())?; Py::Bool({ let t73 = v_m.clone(); if py::ne(&t72, &t73) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    v_total = Py::Float(0.0);
    for t74 in py::range_iter(&[(py::add(&v_m.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t74;
        v_num = Py::Float(1.0);
        v_fact = Py::Float(1.0);
        for t75 in py::range_iter(&[(v_k.clone()).i()?]) {
            v_j = t75;
            v_num = py::mul(&v_num.clone(), &py::sub(&v_n.clone(), &v_j.clone())?)?;
            v_fact = py::mul(&v_fact.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?;
        }
        v_want = py::mul(&py::truediv(&v_num.clone(), &v_fact.clone())?, &py::pow(&v_x.clone(), &v_k.clone())?)?;
        v_got = (if ({ let t76 = v_k.clone(); Py::Bool({ let t77 = v_m.clone(); if py::lt(&t76, &t77)? { true } else { false } }) }).truthy() { py::getitem(&v_vals.clone(), &v_k.clone())? } else { py::getitem(&v_o.clone(), &Py::str("next_term"))? });
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&v_got.clone(), &v_want.clone())?)?, &py::minmax(&[py::abs(&v_want.clone())?, py::abs(&v_got.clone())?, Py::Float(1e-300)], true)?)?], true)?;
        if ({ let t78 = v_k.clone(); Py::Bool({ let t79 = v_m.clone(); if py::lt(&t78, &t79)? { true } else { false } }) }).truthy() {
            v_total = py::iadd(&v_total, &v_want.clone())?;
        }
    }
    v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("approx"))?, &v_total.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_total.clone())?], true)?)?], true)?;
    v_ex = py::pow(&py::add(&Py::Float(1.0), &v_x.clone())?, &v_n.clone())?;
    v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("exact"))?, &v_ex.clone())?)?, &py::minmax(&[py::abs(&v_ex.clone())?, Py::Float(1e-300)], true)?)?], true)?;
    v_pc = py::mul(&py::mul(&Py::Float(100.0), &v_n.clone())?, &v_x.clone())?;
    return Ok(py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("percent_change_linear"))?, &v_pc.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_pc.clone())?], true)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_carith(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_op: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t102 = (Py::tuple(vec![py::complex(&[py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?])?, py::complex(&[py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("d"))?])?, py::complex(&[py::getitem(&v_o.clone(), &Py::str("re"))?, py::getitem(&v_o.clone(), &Py::str("im"))?])?])).items()?;
    if t102.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_z = t102[0].clone();
    v_w = t102[1].clone();
    v_r = t102[2].clone();
    v_op = py::getitem(&v_i.clone(), &Py::str("op"))?;
    if ({ let t103 = v_op.clone(); Py::Bool({ let t104 = Py::str("+"); if py::eq(&t103, &t104) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::abs(&py::sub(&v_r.clone(), &py::add(&v_z.clone(), &v_w.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_z.clone())?, py::abs(&v_w.clone())?], true)?)?);
    }
    if ({ let t105 = v_op.clone(); Py::Bool({ let t106 = Py::str("-"); if py::eq(&t105, &t106) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::abs(&py::sub(&v_r.clone(), &py::sub(&v_z.clone(), &v_w.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_z.clone())?, py::abs(&v_w.clone())?], true)?)?);
    }
    if ({ let t107 = v_op.clone(); Py::Bool({ let t108 = Py::str("*"); if py::eq(&t107, &t108) { true } else { false } }) }).truthy() {
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
    let t264 = ((if ({ let t262 = py::getitem(&v_i.clone(), &Py::str("order"))?; Py::Bool({ let t263 = Py::str("f(g(x))"); if py::eq(&t262, &t263) { true } else { false } }) }).truthy() { Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("g"))?, py::getitem(&v_i.clone(), &Py::str("f"))?]) } else { Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("g"))?]) })).items()?;
    if t264.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_first = t264[0].clone();
    v_second = t264[1].clone();
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
        v_v = (if ({ let t358 = py::getitem(&v_o.clone(), &Py::str("exact"))?; Py::Bool({ let t359 = Py::str("0"); if py::eq(&t358, &t359) { true } else { false } }) }).truthy() { py::fraction(&[Py::Int(0i64)])? } else { py::fraction(&[py::mul(&(if (py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?).truthy() { py::neg(&Py::Int(1i64))? } else { Py::Int(1i64) }), &py::int(&{ let t360 = py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?; if t360.truthy() { t360 } else { Py::Int(1i64) } })?)?, py::int(&{ let t361 = py::method(&v_m.clone(), "group", &[Py::Int(3i64)])?; if t361.truthy() { t361 } else { Py::Int(1i64) } })?])? });
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
    let t335 = (Py::tuple(vec![f__fr(py::getitem(&v_i.clone(), &Py::str("a"))?)?, f__fr(py::getitem(&v_i.clone(), &Py::str("b"))?)?])).items()?;
    if t335.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t335[0].clone();
    v_B = t335[1].clone();
    if ({ let t336 = v_n.clone(); Py::Bool({ let t337 = Py::Int(0i64); if py::lt(&t336, &t337)? { true } else { false } }) }).truthy() {
        v_den = py::add(&py::mul(&v_A.clone(), &v_A.clone())?, &py::mul(&v_B.clone(), &v_B.clone())?)?;
        let t338 = (Py::tuple(vec![py::truediv(&v_A.clone(), &v_den.clone())?, py::truediv(&py::neg(&v_B.clone())?, &v_den.clone())?])).items()?;
        if t338.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_A = t338[0].clone();
        v_B = t338[1].clone();
    }
    let t339 = (Py::tuple(vec![py::fraction(&[Py::Int(1i64)])?, py::fraction(&[Py::Int(0i64)])?])).items()?;
    if t339.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rr = t339[0].clone();
    v_ri = t339[1].clone();
    for t340 in py::range_iter(&[(py::abs(&v_n.clone())?).i()?]) {
        v__ = t340;
        let t341 = (Py::tuple(vec![py::sub(&py::mul(&v_rr.clone(), &v_A.clone())?, &py::mul(&v_ri.clone(), &v_B.clone())?)?, py::add(&py::mul(&v_rr.clone(), &v_B.clone())?, &py::mul(&v_ri.clone(), &v_A.clone())?)?])).items()?;
        if t341.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_rr = t341[0].clone();
        v_ri = t341[1].clone();
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
    let t371 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("x0"))?, py::getitem(&v_i.clone(), &Py::str("y0"))?])).items()?;
    if t371.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_a = t371[0].clone();
    v_b = t371[1].clone();
    v_c = t371[2].clone();
    v_x0 = t371[3].clone();
    v_y0 = t371[4].clone();
    v_n = py::hypot(&[v_a.clone(), v_b.clone()])?;
    v_t = py::truediv(&py::neg(&py::add(&py::add(&py::mul(&v_a.clone(), &v_x0.clone())?, &py::mul(&v_b.clone(), &v_y0.clone())?)?, &v_c.clone())?)?, &py::mul(&v_n.clone(), &v_n.clone())?)?;
    let t372 = (Py::tuple(vec![py::add(&v_x0.clone(), &py::mul(&v_a.clone(), &v_t.clone())?)?, py::add(&v_y0.clone(), &py::mul(&v_b.clone(), &v_t.clone())?)?])).items()?;
    if t372.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t372[0].clone();
    v_fy = t372[1].clone();
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
    let t408 = (f__domain_parts(py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("d"))?, Py::Bool((py::method(&v_i.clone(), "get", &[Py::str("root_in_denominator")])?).truthy()))?).items()?;
    if t408.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_B = t408[0].clone();
    v_side = t408[1].clone();
    v_E = t408[2].clone();
    v_text = t408[3].clone();
    v_r = py::truediv(&py::abs(&py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_o.clone(), &Py::str("boundary"))?)?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("b"))?)?, py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_o.clone(), &Py::str("boundary"))?)?)?], true)?)?;
    v_r = py::iadd(&v_r, &py::add(&{ let t409 = py::getitem(&v_o.clone(), &Py::str("side"))?; Py::Bool({ let t410 = v_side.clone(); if py::ne(&t409, &t410) { true } else { false } }) }, &{ let t411 = py::getitem(&v_o.clone(), &Py::str("domain"))?; Py::Bool({ let t412 = v_text.clone(); if py::ne(&t411, &t412) { true } else { false } }) })?)?;
    if ({ let t413 = { let t415 = v_E.clone(); Py::Bool({ let t416 = Py::None; if py::is(&t415, &t416) { true } else { false } }) }; Py::Bool({ let t414 = { let t417 = py::getitem(&v_o.clone(), &Py::str("excluded"))?; Py::Bool({ let t418 = Py::None; if py::is(&t417, &t418) { true } else { false } }) }; if py::ne(&t413, &t414) { true } else { false } }) }).truthy() {
        return Ok(py::add(&v_r.clone(), &Py::Float(1.0))?);
    }
    if ({ let t419 = v_E.clone(); Py::Bool({ let t420 = Py::None; if !py::is(&t419, &t420) { true } else { false } }) }).truthy() {
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
    let t445 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("h"))?, py::getitem(&v_i.clone(), &Py::str("k"))?, py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_i.clone(), &Py::str("B"))?])).items()?;
    if t445.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_h = t445[0].clone();
    v_k = t445[1].clone();
    v_A = t445[2].clone();
    v_B = t445[3].clone();
    v_r = py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("c"))?, &Py::Int(2i64))?, &py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("a"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("b"))?, &Py::Int(2i64))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::pow(&py::getitem(&v_o.clone(), &Py::str("a"))?, &Py::Int(2i64))?], true)?)?, &f__rel(py::pow(&py::getitem(&v_o.clone(), &Py::str("a"))?, &Py::Int(2i64))?, py::minmax(&[v_A.clone(), v_B.clone()], true)?)?)?, &f__rel(py::pow(&py::getitem(&v_o.clone(), &Py::str("b"))?, &Py::Int(2i64))?, py::minmax(&[v_A.clone(), v_B.clone()], false)?)?)?;
    v_r = py::iadd(&v_r, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("eccentricity"))?, &py::getitem(&v_o.clone(), &Py::str("a"))?)?, py::getitem(&v_o.clone(), &Py::str("c"))?)?)?;
    for t447 in py::iter(&(Py::tuple(vec![Py::Float(0.3), Py::Float(1.7), Py::Float(4.0)])))? {
        v_t = t447;
        let t448 = (Py::tuple(vec![py::add(&v_h.clone(), &py::mul(&py::math1("sqrt", &v_A.clone())?, &py::math1("cos", &v_t.clone())?)?)?, py::add(&v_k.clone(), &py::mul(&py::math1("sqrt", &v_B.clone())?, &py::math1("sin", &v_t.clone())?)?)?])).items()?;
        if t448.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_px = t448[0].clone();
        v_py = t448[1].clone();
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
    let t598 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("g"))?, py::getitem(&v_o.clone(), &Py::str("p"))?, py::getitem(&v_o.clone(), &Py::str("q"))?, py::getitem(&v_o.clone(), &Py::str("r"))?, py::getitem(&v_o.clone(), &Py::str("s"))?])).items()?;
    if t598.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_g = t598[0].clone();
    v_p = t598[1].clone();
    v_q = t598[2].clone();
    v_r = t598[3].clone();
    v_s = t598[4].clone();
    if ({ let t607 = Py::Bool(({ let mut out = vec![]; for t599 in py::iter(&(Py::tuple(vec![v_g.clone(), v_p.clone(), v_q.clone(), v_r.clone(), v_s.clone()])))? { let mut v_v: Py; 
v_v = t599;
out.push({ let t600 = v_v.clone(); Py::Bool({ let t601 = py::int(&v_v.clone())?; if py::ne(&t600, &t601) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t607.truthy() { t607 } else { { let t606 = { let t602 = v_p.clone(); Py::Bool({ let t603 = Py::Int(0i64); if py::le(&t602, &t603)? { true } else { false } }) }; if t606.truthy() { t606 } else { { let t604 = v_r.clone(); Py::Bool({ let t605 = Py::Int(0i64); if py::le(&t604, &t605)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t615 = { let t608 = py::gcd(&py::int(&v_p.clone())?, &py::int(&v_q.clone())?)?; Py::Bool({ let t609 = Py::Int(1i64); if py::ne(&t608, &t609) { true } else { false } }) }; if t615.truthy() { t615 } else { { let t614 = { let t610 = py::gcd(&py::int(&v_r.clone())?, &py::int(&v_s.clone())?)?; Py::Bool({ let t611 = Py::Int(1i64); if py::ne(&t610, &t611) { true } else { false } }) }; if t614.truthy() { t614 } else { { let t612 = py::fraction(&[py::int(&py::neg(&v_q.clone())?)?, py::int(&v_p.clone())?])?; Py::Bool({ let t613 = py::fraction(&[py::int(&py::neg(&v_s.clone())?)?, py::int(&v_r.clone())?])?; if py::gt(&t612, &t613)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_want = Py::tuple(vec![py::mul(&py::mul(&v_g.clone(), &v_p.clone())?, &v_r.clone())?, py::mul(&v_g.clone(), &py::add(&py::mul(&v_p.clone(), &v_s.clone())?, &py::mul(&v_q.clone(), &v_r.clone())?)?)?, py::mul(&py::mul(&v_g.clone(), &v_q.clone())?, &v_s.clone())?]);
    return Ok(py::minmax(&[{ let mut out = vec![]; for t616 in py::iter(&(py::zip(&[v_want.clone(), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])])?))? { let mut v_u: Py; let mut v_v: Py; 
let t617 = (t616).items()?;
if t617.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t617[0].clone();
v_v = t617[1].clone();
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
    if ({ let t555 = Py::Bool(matches!(v_v.clone(), Py::Frac(_))); if !t555.truthy() { t555 } else { py::method(&v_o.clone(), "get", &[Py::str("denominator")])? } }).truthy() {
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
    v_r = py::iadd(&v_r, &f__rel(py::getitem(&v_o.clone(), &Py::str("asymptote_slope"))?, (if ({ let t703 = py::getitem(&v_i.clone(), &Py::str("orientation"))?; Py::Bool({ let t704 = Py::str("horizontal"); if py::eq(&t703, &t704) { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&v_o.clone(), &Py::str("b"))?, &py::getitem(&v_o.clone(), &Py::str("a"))?)? } else { py::truediv(&py::getitem(&v_o.clone(), &Py::str("a"))?, &py::getitem(&v_o.clone(), &Py::str("b"))?)? }))?)?;
    for t705 in py::iter(&(Py::tuple(vec![Py::Float(0.4), Py::Float(1.1), py::neg(&Py::Float(2.0))?])))? {
        v_t = t705;
        let t709 = (f__hyp_point(v_i.clone(), v_t.clone())?).items()?;
        if t709.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_px = t709[0].clone();
        v_py = t709[1].clone();
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
    let t725 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?])).items()?;
    if t725.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t725[0].clone();
    v_b = t725[1].clone();
    v_r = py::add(&py::abs(&py::sub(&py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("inv_slope"))?)?, &Py::Int(1i64))?)?, &py::truediv(&py::abs(&py::add(&py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("inv_intercept"))?)?, &v_b.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_b.clone())?], true)?)?)?;
    if ({ let t726 = py::method(&v_i.clone(), "get", &[Py::str("y")])?; Py::Bool({ let t727 = Py::None; if !py::is(&t726, &t727) { true } else { false } }) }).truthy() {
        v_r = py::iadd(&v_r, &(if ({ let t728 = Py::str("value"); Py::Bool({ let t729 = v_o.clone(); if py::contains(&t729, &t728)? { true } else { false } }) }).truthy() { py::truediv(&py::abs(&py::sub(&py::add(&py::mul(&v_a.clone(), &py::method(&v_o.clone(), "get", &[Py::str("value"), py::float(&Py::str("inf"))?])?)?, &v_b.clone())?, &py::getitem(&v_i.clone(), &Py::str("y"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("y"))?)?, py::abs(&v_b.clone())?], true)?)? } else { Py::Float(1.0) }))?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_line_conic(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_co: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t876 = (Py::tuple(vec![py::getitem(&py::getitem(&v_i.clone(), &Py::str("line"))?, &Py::Int(0i64))?, py::getitem(&py::getitem(&v_i.clone(), &Py::str("line"))?, &Py::Int(1i64))?, py::getitem(&py::getitem(&v_i.clone(), &Py::str("line"))?, &Py::Int(2i64))?])).items()?;
    if t876.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t876[0].clone();
    v_b = t876[1].clone();
    v_c = t876[2].clone();
    v_co = py::getitem(&v_i.clone(), &Py::str("conic"))?;
    v_pts = py::getitem(&v_o.clone(), &Py::str("points"))?;
    if ({ let t889 = { let t877 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t878 = py::len(&v_pts.clone())?; if py::ne(&t877, &t878) { true } else { false } }) }; if t889.truthy() { t889 } else { py::not(&f__cg_points_in_order(v_pts.clone())?) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    for t890 in py::iter(&(v_pts.clone()))? {
        v_p = t890;
        let t891 = (Py::tuple(vec![py::getitem(&v_p.clone(), &Py::Int(0i64))?, py::getitem(&v_p.clone(), &Py::Int(1i64))?])).items()?;
        if t891.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t891[0].clone();
        v_y = t891[1].clone();
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::add(&py::mul(&v_a.clone(), &v_x.clone())?, &py::mul(&v_b.clone(), &v_y.clone())?)?, &v_c.clone())?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::add(&py::abs(&py::mul(&v_a.clone(), &v_x.clone())?)?, &py::abs(&py::mul(&v_b.clone(), &v_y.clone())?)?)?, &py::abs(&v_c.clone())?)?], true)?)?, f__cg_conic_res(v_co.clone(), v_x.clone(), v_y.clone())?], true)?;
    }
    if ({ let t893 = v_a.clone(); Py::Bool({ let t894 = Py::Int(0i64); if py::ne(&t893, &t894) { true } else { false } }) }).truthy() {
        let t895 = (Py::tuple(vec![py::getitem(&v_co.clone(), &Py::Int(0i64))?, py::getitem(&v_co.clone(), &Py::Int(1i64))?, py::getitem(&v_co.clone(), &Py::Int(2i64))?, py::getitem(&v_co.clone(), &Py::Int(3i64))?, py::getitem(&v_co.clone(), &Py::Int(4i64))?, py::getitem(&v_co.clone(), &Py::Int(5i64))?])).items()?;
        if t895.len() != 6 { return Err(PyErr::Value("unpack".into())); }
        v_A = t895[0].clone();
        v_B = t895[1].clone();
        v_C = t895[2].clone();
        v_D = t895[3].clone();
        v_E = t895[4].clone();
        v_F = t895[5].clone();
        v_n = f__cg_count_or_none(py::add(&py::sub(&py::mul(&py::mul(&v_A.clone(), &v_b.clone())?, &v_b.clone())?, &py::mul(&py::mul(&v_B.clone(), &v_a.clone())?, &v_b.clone())?)?, &py::mul(&py::mul(&v_C.clone(), &v_a.clone())?, &v_a.clone())?)?, py::add(&py::sub(&py::add(&py::mul(&py::mul(&py::mul(&py::neg(&Py::Float(2.0))?, &v_A.clone())?, &v_b.clone())?, &v_c.clone())?, &py::mul(&py::mul(&v_B.clone(), &v_a.clone())?, &v_c.clone())?)?, &py::mul(&py::mul(&v_D.clone(), &v_a.clone())?, &v_b.clone())?)?, &py::mul(&py::mul(&v_E.clone(), &v_a.clone())?, &v_a.clone())?)?, py::add(&py::add(&py::mul(&py::mul(&v_A.clone(), &v_c.clone())?, &v_c.clone())?, &py::mul(&py::mul(&v_D.clone(), &v_a.clone())?, &v_c.clone())?)?, &py::mul(&py::mul(&v_F.clone(), &v_a.clone())?, &v_a.clone())?)?)?;
        if ({ let t901 = { let t897 = v_n.clone(); Py::Bool({ let t898 = Py::None; if !py::is(&t897, &t898) { true } else { false } }) }; if !t901.truthy() { t901 } else { { let t899 = v_n.clone(); Py::Bool({ let t900 = py::len(&v_pts.clone())?; if py::ne(&t899, &t900) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_logeq(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_arg: Py = Py::None;
    v_arg = py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("c"))?, &py::getitem(&v_o.clone(), &Py::str("x"))?)?, &py::getitem(&v_i.clone(), &Py::str("d"))?)?;
    if ({ let t953 = v_arg.clone(); Py::Bool({ let t954 = Py::Int(0i64); if py::le(&t953, &t954)? { true } else { false } }) }).truthy() {
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
    let t1307 = (Py::tuple(vec![f__strip({ let mut out = vec![]; for t1305 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("dividend"))?))? { let mut v_c: Py; 
v_c = t1305;
out.push(py::float(&v_c.clone())?); } Py::list(out) })?, f__strip({ let mut out = vec![]; for t1306 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("divisor"))?))? { let mut v_c: Py; 
v_c = t1306;
out.push(py::float(&v_c.clone())?); } Py::list(out) })?])).items()?;
    if t1307.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1307[0].clone();
    v_d = t1307[1].clone();
    let t1308 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("quotient"))?, py::getitem(&v_o.clone(), &Py::str("remainder"))?])).items()?;
    if t1308.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t1308[0].clone();
    v_r = t1308[1].clone();
    if ({ let t1311 = { let t1309 = py::len(&v_r.clone())?; Py::Bool({ let t1310 = py::len(&v_d.clone())?; if py::ge(&t1309, &t1310)? { true } else { false } }) }; if !t1311.truthy() { t1311 } else { Py::Bool((py::slice(&v_r.clone(), None, Some((py::add(&py::sub(&py::len(&v_r.clone())?, &py::len(&v_d.clone())?)?, &Py::Int(1i64))?).i()?), None)?).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_prod = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&py::add(&py::len(&v_q.clone())?, &py::len(&v_d.clone())?)?, &Py::Int(1i64))?)?;
    for t1312 in py::iter(&(py::enumerate(&v_q.clone(), 0)?))? {
        let t1317 = (t1312).items()?;
        if t1317.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a_i = t1317[0].clone();
        v_a = t1317[1].clone();
        for t1313 in py::iter(&(py::enumerate(&v_d.clone(), 0)?))? {
            let t1316 = (t1313).items()?;
            if t1316.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_b_i = t1316[0].clone();
            v_b = t1316[1].clone();
            let t1314 = v_prod.clone(); let t1315 = py::add(&v_a_i.clone(), &v_b_i.clone())?;
            py::setitem(&t1314, &t1315, py::iadd(&py::getitem(&t1314, &t1315)?, &py::mul(&v_a.clone(), &v_b.clone())?)?)?;
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
    v_back = { let mut out = vec![]; for t1318 in py::iter(&(py::zip(&[py::call(&v_pad.clone(), &[v_prod.clone()])?, py::call(&v_pad.clone(), &[v_r.clone()])?])?))? { let mut v_u: Py; let mut v_v: Py; 
let t1319 = (t1318).items()?;
if t1319.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t1319[0].clone();
v_v = t1319[1].clone();
out.push(py::add(&v_u.clone(), &v_v.clone())?); } Py::list(out) };
    v_scale = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t1320 in py::iter(&(v_p.clone()))? { let mut v_c: Py; 
v_c = t1320;
out.push(py::abs(&v_c.clone())?); } Py::list(out) }], true)?, py::minmax(&[{ let mut out = vec![]; for t1321 in py::iter(&(v_prod.clone()))? { let mut v_v: Py; 
v_v = t1321;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?], true)?;
    return Ok(py::truediv(&py::minmax(&[{ let mut out = vec![]; for t1322 in py::iter(&(py::zip(&[v_back.clone(), py::call(&v_pad.clone(), &[v_p.clone()])?])?))? { let mut v_u: Py; let mut v_v: Py; 
let t1323 = (t1322).items()?;
if t1323.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t1323[0].clone();
v_v = t1323[1].clone();
out.push(py::abs(&py::sub(&v_u.clone(), &v_v.clone())?)?); } Py::list(out) }], true)?, &v_scale.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_magdir(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_t = py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("direction_deg"))?)?;
    if (py::not(&{ let t2504 = Py::Int(0i64); Py::Bool({ let t2505 = py::getitem(&v_o.clone(), &Py::str("direction_deg"))?; if py::le(&t2504, &t2505)? { { let t2506 = Py::Int(360i64); if py::lt(&t2505, &t2506)? { true } else { false } } } else { false } }) })).truthy() {
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
    let t142 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::complex(&[py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?])?])).items()?;
    if t142.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t142[0].clone();
    v_z = t142[1].clone();
    v_rs = py::getitem(&v_o.clone(), &Py::str("roots"))?;
    if ({ let t147 = { let t143 = py::len(&v_rs.clone())?; Py::Bool({ let t144 = v_n.clone(); if py::ne(&t143, &t144) { true } else { false } }) }; if t147.truthy() { t147 } else { { let t145 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t146 = v_n.clone(); if py::ne(&t145, &t146) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_args = { let mut out = vec![]; for t148 in py::iter(&(v_rs.clone()))? { let mut v_w: Py; 
v_w = t148;
out.push(f__arg360(v_w.clone())?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t149 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t149;
out.push({ let t150 = py::getitem(&v_args.clone(), &v_j.clone())?; Py::Bool({ let t151 = py::getitem(&v_args.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?; if py::ge(&t150, &t151)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = f__rel(py::pow(&py::getitem(&v_o.clone(), &Py::str("modulus"))?, &v_n.clone())?, py::abs(&v_z.clone())?)?;
    for t153 in py::iter(&(v_rs.clone()))? {
        v_w = t153;
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
        if ({ let t1092 = { let t1088 = py::method(&v_o.clone(), "get", &[Py::str("numerator")])?; Py::Bool({ let t1089 = Py::None; if py::is(&t1088, &t1089) { true } else { false } }) }; if t1092.truthy() { t1092 } else { { let t1090 = py::getitem(&v_o.clone(), &Py::str("denominator"))?; Py::Bool({ let t1091 = Py::Int(0i64); if py::le(&t1090, &t1091)? { true } else { false } }) } } }).truthy() {
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
    let t1103 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("m"))?, py::method(&v_i.clone(), "get", &[Py::str("h"), Py::Float(0.0)])?, py::method(&v_i.clone(), "get", &[Py::str("k"), Py::Float(0.0)])?])).items()?;
    if t1103.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1103[0].clone();
    v_h = t1103[1].clone();
    v_k = t1103[2].clone();
    v_worst = py::truediv(&py::abs(&py::sub(&py::mul(&Py::Int(4i64), &py::getitem(&v_o.clone(), &Py::str("p"))?)?, &v_m.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_m.clone())?], true)?)?;
    for t1104 in py::iter(&(Py::tuple(vec![Py::Float(0.5), Py::Float(1.0), Py::Float(2.0), py::neg(&Py::Float(3.0))?])))? {
        v_t = t1104;
        if ({ let t1105 = py::getitem(&v_i.clone(), &Py::str("orientation"))?; Py::Bool({ let t1106 = Py::str("vertical"); if py::eq(&t1105, &t1106) { true } else { false } }) }).truthy() {
            let t1107 = (Py::tuple(vec![py::add(&v_h.clone(), &v_t.clone())?, py::add(&v_k.clone(), &py::truediv(&py::mul(&v_t.clone(), &v_t.clone())?, &v_m.clone())?)?])).items()?;
            if t1107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_px = t1107[0].clone();
            v_py = t1107[1].clone();
            v_d_line = py::abs(&py::sub(&v_py.clone(), &py::getitem(&v_o.clone(), &Py::str("directrix"))?)?)?;
        } else {
            let t1108 = (Py::tuple(vec![py::add(&v_h.clone(), &py::truediv(&py::mul(&v_t.clone(), &v_t.clone())?, &v_m.clone())?)?, py::add(&v_k.clone(), &v_t.clone())?])).items()?;
            if t1108.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_px = t1108[0].clone();
            v_py = t1108[1].clone();
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
    v_want_dir = (if ({ let t1127 = py::getitem(&v_i.clone(), &Py::str("new"))?; Py::Bool({ let t1128 = py::getitem(&v_i.clone(), &Py::str("old"))?; if py::gt(&t1127, &t1128)? { true } else { false } }) }).truthy() { Py::str("increase") } else { (if ({ let t1129 = py::getitem(&v_i.clone(), &Py::str("new"))?; Py::Bool({ let t1130 = py::getitem(&v_i.clone(), &Py::str("old"))?; if py::lt(&t1129, &t1130)? { true } else { false } }) }).truthy() { Py::str("decrease") } else { Py::str("no change") }) });
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("percent_change"))?, &py::getitem(&v_i.clone(), &Py::str("old"))?)?, &py::mul(&Py::Int(100i64), &py::sub(&py::getitem(&v_i.clone(), &Py::str("new"))?, &py::getitem(&v_i.clone(), &Py::str("old"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&Py::Int(100i64), &py::sub(&py::getitem(&v_i.clone(), &Py::str("new"))?, &py::getitem(&v_i.clone(), &Py::str("old"))?)?)?)?, py::abs(&py::mul(&py::getitem(&v_o.clone(), &Py::str("percent_change"))?, &py::getitem(&v_i.clone(), &Py::str("old"))?)?)?], true)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("difference"))?, py::sub(&py::getitem(&v_i.clone(), &Py::str("new"))?, &py::getitem(&v_i.clone(), &Py::str("old"))?)?)?)?, &{ let t1132 = py::getitem(&v_o.clone(), &Py::str("direction"))?; Py::Bool({ let t1133 = v_want_dir.clone(); if py::ne(&t1132, &t1133) { true } else { false } }) })?);
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
    let t1115 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("B"))?])).items()?;
    if t1115.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1115[0].clone();
    v_B = t1115[1].clone();
    let t1116 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("p"))?, py::getitem(&v_i.clone(), &Py::str("q"))?, py::getitem(&v_i.clone(), &Py::str("r1"))?, py::getitem(&v_i.clone(), &Py::str("r2"))?])).items()?;
    if t1116.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1116[0].clone();
    v_q = t1116[1].clone();
    v_r1 = t1116[2].clone();
    v_r2 = t1116[3].clone();
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
    v_co = { let mut out = vec![]; for t1426 in py::iter(&(py::slice(&c__LETTERS()?, None, Some((py::add(&v_n.clone(), &Py::Int(1i64))?).i()?), None)?))? { let mut v_k: Py; 
v_k = t1426;
out.push(py::getitem(&v_inputs.clone(), &v_k.clone())?); } Py::list(out) };
    v_worst = Py::Float(0.0);
    for t1427 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t1427;
        v_x = py::complex(&[py::getitem(&v_outputs.clone(), &Py::str(&["x".to_string(), py::format(&py::add(&v_i.clone(), &Py::Int(1i64))?, "")?].concat()))?])?;
        v_terms = { let mut out = vec![]; for t1428 in py::iter(&(py::enumerate(&v_co.clone(), 0)?))? { let mut v_c: Py; let mut v_j: Py; 
let t1429 = (t1428).items()?;
if t1429.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t1429[0].clone();
v_c = t1429[1].clone();
out.push(py::mul(&v_c.clone(), &py::pow(&v_x.clone(), &py::sub(&v_n.clone(), &v_j.clone())?)?)?); } Py::list(out) };
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sum(&v_terms.clone(), None)?)?, &{ let t1431 = py::sum(&{ let mut out = vec![]; for t1430 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t1430;
out.push(py::abs(&v_t.clone())?); } Py::list(out) }, None)?; if t1431.truthy() { t1431 } else { Py::Float(1.0) } })?], true)?;
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
    let t1349 = (v_t.clone()).items()?;
    if t1349.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t1349[0].clone();
    v_d1 = t1349[1].clone();
    v_n2 = t1349[2].clone();
    v_d2 = t1349[3].clone();
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&v_n1.clone(), &v_d2.clone())?, &py::mul(&v_n2.clone(), &v_d1.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&v_n1.clone(), &v_d2.clone())?)?, py::abs(&py::mul(&v_n2.clone(), &v_d1.clone())?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pw(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_first: Py = Py::None;
    v_first = (if ({ let t1259 = py::getitem(&v_i.clone(), &Py::str("left_rel"))?; Py::Bool({ let t1260 = Py::str("<"); if py::eq(&t1259, &t1260) { true } else { false } }) }).truthy() { { let t1261 = py::getitem(&v_i.clone(), &Py::str("x"))?; Py::Bool({ let t1262 = py::getitem(&v_i.clone(), &Py::str("c"))?; if py::lt(&t1261, &t1262)? { true } else { false } }) } } else { { let t1263 = py::getitem(&v_i.clone(), &Py::str("x"))?; Py::Bool({ let t1264 = py::getitem(&v_i.clone(), &Py::str("c"))?; if py::le(&t1263, &t1264)? { true } else { false } }) } });
    if ({ let t1265 = py::getitem(&v_o.clone(), &Py::str("piece"))?; Py::Bool({ let t1266 = (if (v_first.clone()).truthy() { Py::Float(1.0) } else { Py::Float(2.0) }); if py::ne(&t1265, &t1266) { true } else { false } }) }).truthy() {
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
    let t1380 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])).items()?;
    if t1380.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1380[0].clone();
    v_b = t1380[1].clone();
    v_c = t1380[2].clone();
    v_worst = Py::Float(0.0);
    for t1381 in py::iter(&(Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("r1"))?, py::getitem(&v_o.clone(), &Py::str("r2"))?])))? {
        v_x = t1381;
        v_t = Py::list(vec![py::mul(&py::mul(&v_a.clone(), &v_x.clone())?, &v_x.clone())?, py::mul(&v_b.clone(), &v_x.clone())?, v_c.clone()]);
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sum(&v_t.clone(), None)?)?, &{ let t1383 = py::sum(&{ let mut out = vec![]; for t1382 in py::iter(&(v_t.clone()))? { let mut v_v: Py; 
v_v = t1382;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }, None)?; if t1383.truthy() { t1383 } else { Py::Float(1.0) } })?], true)?;
    }
    v_worst = py::iadd(&v_worst, &py::truediv(&py::abs(&py::add(&py::add(&py::getitem(&v_o.clone(), &Py::str("r1"))?, &py::getitem(&v_o.clone(), &Py::str("r2"))?)?, &py::truediv(&v_b.clone(), &v_a.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::truediv(&v_b.clone(), &v_a.clone())?)?], true)?)?)?;
    v_worst = py::iadd(&v_worst, &py::add(&{ let t1384 = py::getitem(&v_o.clone(), &Py::str("r1"))?; Py::Bool({ let t1385 = py::getitem(&v_o.clone(), &Py::str("r2"))?; if py::ge(&t1384, &t1385)? { true } else { false } }) }, &{ let t1386 = py::getitem(&v_o.clone(), &Py::str("region"))?; Py::Bool({ let t1387 = (if ({ let t1388 = { let t1390 = py::getitem(&v_i.clone(), &Py::str("rel"))?; Py::Bool({ let t1391 = Py::tuple(vec![Py::str("<"), Py::str("<=")]); if py::contains(&t1391, &t1390)? { true } else { false } }) }; Py::Bool({ let t1389 = { let t1392 = v_a.clone(); Py::Bool({ let t1393 = Py::Int(0i64); if py::gt(&t1392, &t1393)? { true } else { false } }) }; if py::eq(&t1388, &t1389) { true } else { false } }) }).truthy() { Py::str("between") } else { Py::str("outside") }); if py::ne(&t1386, &t1387) { true } else { false } }) })?)?;
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
    let t1504 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("d"))?])).items()?;
    if t1504.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1504[0].clone();
    v_b = t1504[1].clone();
    v_c = t1504[2].clone();
    v_d = t1504[3].clone();
    if ({ let t1505 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t1506 = py::len(&py::getitem(&v_o.clone(), &Py::str("solutions"))?)?; if py::ne(&t1505, &t1506) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    for t1507 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("solutions"))?))? {
        v_x = t1507;
        v_lhs2 = py::add(&py::mul(&v_a.clone(), &v_x.clone())?, &v_b.clone())?;
        if ({ let t1508 = v_lhs2.clone(); Py::Bool({ let t1509 = py::mul(&py::neg(&Py::Float(1e-12))?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&v_a.clone(), &v_x.clone())?)?, py::abs(&v_b.clone())?], true)?)?; if py::lt(&t1508, &t1509)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
        v_lhs = py::math1("sqrt", &py::minmax(&[Py::Float(0.0), v_lhs2.clone()], true)?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&v_lhs.clone(), &py::add(&py::mul(&v_c.clone(), &v_x.clone())?, &v_d.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_lhs.clone())?, py::abs(&py::mul(&v_c.clone(), &v_x.clone())?)?, py::abs(&v_d.clone())?], true)?)?], true)?;
    }
    let t1510 = (Py::tuple(vec![py::mul(&v_c.clone(), &v_c.clone())?, py::sub(&py::mul(&py::mul(&Py::Int(2i64), &v_c.clone())?, &v_d.clone())?, &v_a.clone())?, py::sub(&py::mul(&v_d.clone(), &v_d.clone())?, &v_b.clone())?])).items()?;
    if t1510.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_qa = t1510[0].clone();
    v_qb = t1510[1].clone();
    v_qc = t1510[2].clone();
    v_n_roots = (if ({ let t1511 = v_qa.clone(); Py::Bool({ let t1512 = Py::Int(0i64); if py::eq(&t1511, &t1512) { true } else { false } }) }).truthy() { (if ({ let t1513 = v_qb.clone(); Py::Bool({ let t1514 = Py::Int(0i64); if py::ne(&t1513, &t1514) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }) } else { py::call(&{ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_i = v_i.clone(); let v_lhs = v_lhs.clone(); let v_lhs2 = v_lhs2.clone(); let v_o = v_o.clone(); let v_qa = v_qa.clone(); let v_qb = v_qb.clone(); let v_qc = v_qc.clone(); let v_worst = v_worst.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_D: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t1515 = v_D.clone(); Py::Bool({ let t1516 = Py::Int(0i64); if py::lt(&t1515, &t1516)? { true } else { false } }) }).truthy() { Py::Int(0i64) } else { (if ({ let t1517 = v_D.clone(); Py::Bool({ let t1518 = Py::Int(0i64); if py::eq(&t1517, &t1518) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(2i64) }) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::sub(&py::pow(&f__fr(v_qb.clone())?, &Py::Int(2i64))?, &py::mul(&py::mul(&Py::Int(4i64), &f__fr(v_qa.clone())?)?, &f__fr(v_qc.clone())?)?)?])? });
    if ({ let t1519 = py::add(&py::len(&py::getitem(&v_o.clone(), &Py::str("solutions"))?)?, &py::len(&py::method(&v_o.clone(), "get", &[Py::str("extraneous"), Py::list(vec![])])?)?)?; Py::Bool({ let t1520 = v_n_roots.clone(); if py::ne(&t1519, &t1520) { true } else { false } }) }).truthy() {
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
    let t1554 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("d"))?])).items()?;
    if t1554.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1554[0].clone();
    v_b = t1554[1].clone();
    v_c = t1554[2].clone();
    v_d = t1554[3].clone();
    if ({ let t1559 = { let t1555 = py::add(&v_x.clone(), &v_b.clone())?; Py::Bool({ let t1556 = Py::Int(0i64); if py::eq(&t1555, &t1556) { true } else { false } }) }; if t1559.truthy() { t1559 } else { { let t1557 = py::add(&v_x.clone(), &v_d.clone())?; Py::Bool({ let t1558 = Py::Int(0i64); if py::eq(&t1557, &t1558) { true } else { false } }) } } }).truthy() {
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
    v_parts = py::add(&Py::list(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?]), &(if ({ let t1533 = py::method(&v_i.clone(), "get", &[Py::str("c")])?; Py::Bool({ let t1534 = Py::None; if !py::is(&t1533, &t1534) { true } else { false } }) }).truthy() { Py::list(vec![py::getitem(&v_i.clone(), &Py::str("c"))?]) } else { Py::list(vec![]) }))?;
    v_got = { let mut out = vec![]; for t1535 in py::range_iter(&[(py::len(&v_parts.clone())?).i()?]) { let mut v_k: Py; 
v_k = t1535;
out.push(py::method(&v_o.clone(), "get", &[Py::str(&["part".to_string(), py::format(&py::add(&v_k.clone(), &Py::Int(1i64))?, "")?].concat())])?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t1536 in py::iter(&(v_got.clone()))? { let mut v_g: Py; 
v_g = t1536;
out.push({ let t1537 = v_g.clone(); Py::Bool({ let t1538 = Py::None; if py::is(&t1537, &t1538) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = f__rel(py::sum(&v_got.clone(), None)?, py::getitem(&v_i.clone(), &Py::str("total"))?)?;
    for t1540 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_parts.clone())?).i()?]) {
        v_k = t1540;
        v_r = py::iadd(&v_r, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_got.clone(), &v_k.clone())?, &py::getitem(&v_parts.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_got.clone(), &Py::Int(0i64))?, &py::getitem(&v_parts.clone(), &v_k.clone())?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&py::getitem(&v_got.clone(), &Py::Int(0i64))?, &py::getitem(&v_parts.clone(), &v_k.clone())?)?)?, py::abs(&py::mul(&py::getitem(&v_got.clone(), &v_k.clone())?, &py::getitem(&v_parts.clone(), &Py::Int(0i64))?)?)?], true)?)?)?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_rational_fn(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_Pd: Py = Py::None;
    let mut v_Pn: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_dn: Py = Py::None;
    let mut v_exc: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_ha: Py = Py::None;
    let mut v_holes: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_md: Py = Py::None;
    let mut v_merged: Py = Py::None;
    let mut v_mn: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vas: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_y0: Py = Py::None;
    v_N = f__cg_whole_coeffs(py::getitem(&v_i.clone(), &Py::str("numerator"))?, Py::str("numerator"))?;
    v_D = f__cg_whole_coeffs(py::getitem(&v_i.clone(), &Py::str("denominator"))?, Py::str("denominator"))?;
    let t1699 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("excluded"))?, py::getitem(&v_o.clone(), &Py::str("holes"))?, py::getitem(&v_o.clone(), &Py::str("vertical_asymptotes"))?, py::getitem(&v_o.clone(), &Py::str("x_intercepts"))?])).items()?;
    if t1699.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_exc = t1699[0].clone();
    v_holes = t1699[1].clone();
    v_vas = t1699[2].clone();
    v_xs = t1699[3].clone();
    if ({ let t1700 = py::getitem(&v_o.clone(), &Py::str("vertical_count"))?; Py::Bool({ let t1701 = py::len(&v_vas.clone())?; if py::ne(&t1700, &t1701) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_merged = Py::list(vec![]);
    for t1702 in py::iter(&(v_holes.clone()))? {
        v_h = t1702;
        let _ = py::method(&v_merged.clone(), "append", &[py::getitem(&v_h.clone(), &Py::Int(0i64))?])?;
    }
    for t1703 in py::iter(&(v_vas.clone()))? {
        v_v = t1703;
        let _ = py::method(&v_merged.clone(), "append", &[v_v.clone()])?;
    }
    v_merged = f__cg_sorted(v_merged.clone())?;
    if ({ let t1704 = py::len(&v_merged.clone())?; Py::Bool({ let t1705 = py::len(&v_exc.clone())?; if py::ne(&t1704, &t1705) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    for t1706 in py::range_iter(&[(py::len(&v_exc.clone())?).i()?]) {
        v_k = t1706;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::getitem(&v_merged.clone(), &v_k.clone())?, &py::getitem(&v_exc.clone(), &v_k.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_exc.clone(), &v_k.clone())?)?], true)?)?], true)?;
        if ({ let t1720 = { let t1711 = { let t1707 = v_k.clone(); Py::Bool({ let t1708 = Py::Int(0i64); if py::gt(&t1707, &t1708)? { true } else { false } }) }; if !t1711.truthy() { t1711 } else { { let t1709 = py::getitem(&v_exc.clone(), &v_k.clone())?; Py::Bool({ let t1710 = py::getitem(&v_exc.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::le(&t1709, &t1710)? { true } else { false } }) } } }; if t1720.truthy() { t1720 } else { { let t1712 = f__cg_order_at(v_D.clone(), py::getitem(&v_exc.clone(), &v_k.clone())?)?; Py::Bool({ let t1713 = Py::Int(0i64); if py::eq(&t1712, &t1713) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
    }
    for t1721 in py::iter(&(v_vas.clone()))? {
        v_v = t1721;
        if ({ let t1722 = f__cg_order_at(v_N.clone(), v_v.clone())?; Py::Bool({ let t1723 = f__cg_order_at(v_D.clone(), v_v.clone())?; if py::ge(&t1722, &t1723)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
    }
    for t1724 in py::iter(&(v_holes.clone()))? {
        v_h = t1724;
        let t1725 = (Py::tuple(vec![f__cg_order_at(v_D.clone(), py::getitem(&v_h.clone(), &Py::Int(0i64))?)?, f__cg_order_at(v_N.clone(), py::getitem(&v_h.clone(), &Py::Int(0i64))?)?])).items()?;
        if t1725.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_md = t1725[0].clone();
        v_mn = t1725[1].clone();
        if ({ let t1726 = v_mn.clone(); Py::Bool({ let t1727 = v_md.clone(); if py::lt(&t1726, &t1727)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
        v_want = Py::Float(0.0);
        if ({ let t1728 = v_mn.clone(); Py::Bool({ let t1729 = v_md.clone(); if py::eq(&t1728, &t1729) { true } else { false } }) }).truthy() {
            let t1730 = (Py::tuple(vec![v_N.clone(), v_D.clone()])).items()?;
            if t1730.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_Pn = t1730[0].clone();
            v_Pd = t1730[1].clone();
            for t1731 in py::range_iter(&[(v_md.clone()).i()?]) {
                v__ = t1731;
                let t1732 = (Py::tuple(vec![f__cg_deriv(v_Pn.clone())?, f__cg_deriv(v_Pd.clone())?])).items()?;
                if t1732.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_Pn = t1732[0].clone();
                v_Pd = t1732[1].clone();
            }
            v_want = py::truediv(&f__cg_feval(v_Pn.clone(), py::getitem(&v_h.clone(), &Py::Int(0i64))?)?, &f__cg_feval(v_Pd.clone(), py::getitem(&v_h.clone(), &Py::Int(0i64))?)?)?;
        }
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::getitem(&v_h.clone(), &Py::Int(1i64))?, &v_want.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_want.clone())?], true)?)?], true)?;
    }
    for t1733 in py::range_iter(&[(py::len(&v_xs.clone())?).i()?]) {
        v_k = t1733;
        if ({ let t1744 = { let t1734 = f__cg_order_at(v_N.clone(), py::getitem(&v_xs.clone(), &v_k.clone())?)?; Py::Bool({ let t1735 = Py::Int(0i64); if py::eq(&t1734, &t1735) { true } else { false } }) }; if t1744.truthy() { t1744 } else { { let t1743 = { let t1736 = f__cg_order_at(v_D.clone(), py::getitem(&v_xs.clone(), &v_k.clone())?)?; Py::Bool({ let t1737 = Py::Int(0i64); if py::gt(&t1736, &t1737)? { true } else { false } }) }; if t1743.truthy() { t1743 } else { { let t1742 = { let t1738 = v_k.clone(); Py::Bool({ let t1739 = Py::Int(0i64); if py::gt(&t1738, &t1739)? { true } else { false } }) }; if !t1742.truthy() { t1742 } else { { let t1740 = py::getitem(&v_xs.clone(), &v_k.clone())?; Py::Bool({ let t1741 = py::getitem(&v_xs.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::le(&t1740, &t1741)? { true } else { false } }) } } } } } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
    }
    v_y0 = py::getitem(&v_o.clone(), &Py::str("y_intercept"))?;
    if ({ let t1745 = py::getitem(&v_D.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t1746 = Py::Int(0i64); if py::eq(&t1745, &t1746) { true } else { false } }) }).truthy() {
        if ({ let t1747 = v_y0.clone(); Py::Bool({ let t1748 = Py::None; if !py::is(&t1747, &t1748) { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
    } else {
        if ({ let t1749 = v_y0.clone(); Py::Bool({ let t1750 = Py::None; if py::is(&t1749, &t1750) { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::mul(&v_y0.clone(), &py::getitem(&v_D.clone(), &py::neg(&Py::Int(1i64))?)?)?, &py::getitem(&v_N.clone(), &py::neg(&Py::Int(1i64))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_N.clone(), &py::neg(&Py::Int(1i64))?)?)?], true)?)?], true)?;
    }
    let t1751 = (Py::tuple(vec![py::sub(&py::len(&v_N.clone())?, &Py::Int(1i64))?, py::sub(&py::len(&v_D.clone())?, &Py::Int(1i64))?])).items()?;
    if t1751.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dn = t1751[0].clone();
    v_dd = t1751[1].clone();
    let t1752 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("horizontal_asymptote"))?, py::getitem(&v_o.clone(), &Py::str("slant_asymptote"))?])).items()?;
    if t1752.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ha = t1752[0].clone();
    v_sl = t1752[1].clone();
    if ({ let t1753 = v_dn.clone(); Py::Bool({ let t1754 = v_dd.clone(); if py::le(&t1753, &t1754)? { true } else { false } }) }).truthy() {
        if ({ let t1759 = { let t1755 = v_ha.clone(); Py::Bool({ let t1756 = Py::None; if py::is(&t1755, &t1756) { true } else { false } }) }; if t1759.truthy() { t1759 } else { { let t1757 = v_sl.clone(); Py::Bool({ let t1758 = Py::None; if !py::is(&t1757, &t1758) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::mul(&v_ha.clone(), &py::getitem(&v_D.clone(), &Py::Int(0i64))?)?, &(if ({ let t1760 = v_dn.clone(); Py::Bool({ let t1761 = v_dd.clone(); if py::eq(&t1760, &t1761) { true } else { false } }) }).truthy() { py::getitem(&v_N.clone(), &Py::Int(0i64))? } else { Py::Int(0i64) }))?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_N.clone(), &Py::Int(0i64))?)?], true)?)?], true)?;
    } else {
        if ({ let t1762 = v_dn.clone(); Py::Bool({ let t1763 = py::add(&v_dd.clone(), &Py::Int(1i64))?; if py::eq(&t1762, &t1763) { true } else { false } }) }).truthy() {
            if ({ let t1768 = { let t1764 = v_ha.clone(); Py::Bool({ let t1765 = Py::None; if !py::is(&t1764, &t1765) { true } else { false } }) }; if t1768.truthy() { t1768 } else { { let t1766 = v_sl.clone(); Py::Bool({ let t1767 = Py::None; if py::is(&t1766, &t1767) { true } else { false } }) } } }).truthy() {
                return Ok(Py::Float(1.0));
            }
            v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_sl.clone(), &Py::Int(0i64))?, &py::getitem(&v_D.clone(), &Py::Int(0i64))?)?, &py::getitem(&v_N.clone(), &Py::Int(0i64))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_N.clone(), &Py::Int(0i64))?)?], true)?)?, py::truediv(&py::abs(&py::sub(&py::add(&py::mul(&py::getitem(&v_sl.clone(), &Py::Int(0i64))?, &py::getitem(&v_D.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_sl.clone(), &Py::Int(1i64))?, &py::getitem(&v_D.clone(), &Py::Int(0i64))?)?)?, &py::getitem(&v_N.clone(), &Py::Int(1i64))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_N.clone(), &Py::Int(1i64))?)?, py::abs(&py::mul(&py::getitem(&v_sl.clone(), &Py::Int(0i64))?, &py::getitem(&v_D.clone(), &Py::Int(1i64))?)?)?], true)?)?], true)?;
        } else {
            if ({ let t1773 = { let t1769 = v_ha.clone(); Py::Bool({ let t1770 = Py::None; if !py::is(&t1769, &t1770) { true } else { false } }) }; if t1773.truthy() { t1773 } else { { let t1771 = v_sl.clone(); Py::Bool({ let t1772 = Py::None; if !py::is(&t1771, &t1772) { true } else { false } }) } } }).truthy() {
                return Ok(Py::Float(1.0));
            }
        }
    }
    return Ok(v_worst.clone());
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
    v_co = { let mut out = vec![]; for t1811 in py::iter(&(f__strip({ let mut out = vec![]; for t1810 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("coefficients"))?))? { let mut v_c: Py; 
v_c = t1810;
out.push(py::float(&v_c.clone())?); } Py::list(out) })?))? { let mut v_c: Py; 
v_c = t1811;
out.push(py::int(&v_c.clone())?); } Py::list(out) };
    v_ex = { let mut out = vec![]; for t1812 in py::iter(&(py::method(&v_o.clone(), "get", &[Py::str("roots_exact"), Py::list(vec![])])?))? { let mut v_s: Py; 
v_s = t1812;
out.push(py::fraction(&[v_s.clone()])?); } Py::list(out) };
    if ({ let t1820 = { let t1813 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t1814 = py::len(&v_ex.clone())?; if py::ne(&t1813, &t1814) { true } else { false } }) }; if t1820.truthy() { t1820 } else { { let t1819 = { let t1815 = py::len(&py::getitem(&v_o.clone(), &Py::str("roots"))?)?; Py::Bool({ let t1816 = py::len(&v_ex.clone())?; if py::ne(&t1815, &t1816) { true } else { false } }) }; if t1819.truthy() { t1819 } else { { let t1817 = py::len(&py::set(&v_ex.clone())?)?; Py::Bool({ let t1818 = py::len(&v_ex.clone())?; if py::ne(&t1817, &t1818) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t1829 = Py::Bool(({ let mut out = vec![]; for t1821 in py::iter(&(v_ex.clone()))? { let mut v_r: Py; 
v_r = t1821;
out.push({ let t1822 = f__horner(v_co.clone(), v_r.clone())?; Py::Bool({ let t1823 = Py::Int(0i64); if py::ne(&t1822, &t1823) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t1829.truthy() { t1829 } else { Py::Bool(({ let mut out = vec![]; for t1824 in py::iter(&(py::zip(&[v_ex.clone(), py::getitem(&v_o.clone(), &Py::str("roots"))?])?))? { let mut v_r: Py; let mut v_v: Py; 
let t1825 = (t1824).items()?;
if t1825.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t1825[0].clone();
v_v = t1825[1].clone();
out.push({ let t1826 = f__rel(py::float(&v_r.clone())?, v_v.clone())?; Py::Bool({ let t1827 = Py::Float(1e-15); if py::gt(&t1826, &t1827)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_rest = { let mut out = vec![]; for t1830 in py::iter(&(v_co.clone()))? { let mut v_c: Py; 
v_c = t1830;
out.push(py::fraction(&[v_c.clone()])?); } Py::list(out) };
    for t1831 in py::iter(&(v_ex.clone()))? {
        v_r = t1831;
        while ({ let t1837 = { let t1833 = py::len(&v_rest.clone())?; Py::Bool({ let t1834 = Py::Int(1i64); if py::gt(&t1833, &t1834)? { true } else { false } }) }; if !t1837.truthy() { t1837 } else { { let t1835 = f__horner(v_rest.clone(), v_r.clone())?; Py::Bool({ let t1836 = Py::Int(0i64); if py::eq(&t1835, &t1836) { true } else { false } }) } } }).truthy() {
            v_out = Py::list(vec![py::getitem(&v_rest.clone(), &Py::Int(0i64))?]);
            for t1832 in py::iter(&(py::slice(&v_rest.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
                v_c = t1832;
                let _ = py::method(&v_out.clone(), "append", &[py::add(&v_c.clone(), &py::mul(&py::getitem(&v_out.clone(), &py::neg(&Py::Int(1i64))?)?, &v_r.clone())?)?])?;
            }
            v_rest = py::slice(&v_out.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?;
        }
    }
    if ({ let t1838 = py::len(&v_rest.clone())?; Py::Bool({ let t1839 = Py::Int(1i64); if py::gt(&t1838, &t1839)? { true } else { false } }) }).truthy() {
        v_den = py::gcd_lcm("lcm", &({ let mut v: Vec<Py> = vec![]; v.extend(({ let mut out = vec![]; for t1840 in py::iter(&(v_rest.clone()))? { let mut v_f: Py; 
v_f = t1840;
out.push(py::denominator(&v_f.clone())?); } Py::list(out) }).items()?); v })[..])?;
        v_ints = { let mut out = vec![]; for t1841 in py::iter(&(v_rest.clone()))? { let mut v_f: Py; 
v_f = t1841;
out.push(py::int(&py::mul(&v_f.clone(), &v_den.clone())?)?); } Py::list(out) };
        v_g = py::gcd_lcm("gcd", &({ let mut v: Vec<Py> = vec![]; v.extend((v_ints.clone()).items()?); v })[..])?;
        v_ints = { let mut out = vec![]; for t1842 in py::iter(&(v_ints.clone()))? { let mut v_v: Py; 
v_v = t1842;
out.push(py::floordiv(&v_v.clone(), &v_g.clone())?); } Py::list(out) };
        while ({ let t1847 = { let t1843 = py::len(&v_ints.clone())?; Py::Bool({ let t1844 = Py::Int(1i64); if py::gt(&t1843, &t1844)? { true } else { false } }) }; if !t1847.truthy() { t1847 } else { { let t1845 = py::getitem(&v_ints.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t1846 = Py::Int(0i64); if py::eq(&t1845, &t1846) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        for t1848 in py::iter(&(f__divisors(py::getitem(&v_ints.clone(), &py::neg(&Py::Int(1i64))?)?)?))? {
            v_p = t1848;
            for t1849 in py::iter(&(f__divisors(py::getitem(&v_ints.clone(), &Py::Int(0i64))?)?))? {
                v_q = t1849;
                if ({ let t1854 = { let t1850 = f__horner(v_ints.clone(), py::fraction(&[v_p.clone(), v_q.clone()])?)?; Py::Bool({ let t1851 = Py::Int(0i64); if py::eq(&t1850, &t1851) { true } else { false } }) }; if t1854.truthy() { t1854 } else { { let t1852 = f__horner(v_ints.clone(), py::fraction(&[py::neg(&v_p.clone())?, v_q.clone()])?)?; Py::Bool({ let t1853 = Py::Int(0i64); if py::eq(&t1852, &t1853) { true } else { false } }) } } }).truthy() {
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
    let t1915 = (Py::tuple(vec![f__dec(py::getitem(&v_i.clone(), &Py::str("x"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?])).items()?;
    if t1915.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t1915[0].clone();
    v_n = t1915[1].clone();
    v_r = f__dec(py::getitem(&v_o.clone(), &Py::str("rounded"))?)?;
    v_unit = (if ({ let t1916 = v_n.clone(); Py::Bool({ let t1917 = Py::Int(0i64); if py::ge(&t1916, &t1917)? { true } else { false } }) }).truthy() { py::fraction(&[Py::Int(1i64), py::pow(&Py::Int(10i64), &v_n.clone())?])? } else { py::fraction(&[py::pow(&Py::Int(10i64), &py::neg(&v_n.clone())?)?])? });
    if ({ let t1918 = py::denominator(&py::truediv(&v_r.clone(), &v_unit.clone())?)?; Py::Bool({ let t1919 = Py::Int(1i64); if py::ne(&t1918, &t1919) { true } else { false } }) }).truthy() {
        return Ok(py::add(&Py::Float(1.0), &py::float(&py::abs(&py::sub(&v_r.clone(), &v_q.clone())?)?)?)?);
    }
    v_gap = py::abs(&py::sub(&v_r.clone(), &v_q.clone())?)?;
    if ({ let t1927 = { let t1920 = v_gap.clone(); Py::Bool({ let t1921 = py::truediv(&v_unit.clone(), &Py::Int(2i64))?; if py::gt(&t1920, &t1921)? { true } else { false } }) }; if t1927.truthy() { t1927 } else { { let t1926 = { let t1922 = v_gap.clone(); Py::Bool({ let t1923 = py::truediv(&v_unit.clone(), &Py::Int(2i64))?; if py::eq(&t1922, &t1923) { true } else { false } }) }; if !t1926.truthy() { t1926 } else { { let t1924 = py::abs(&v_r.clone())?; Py::Bool({ let t1925 = py::abs(&v_q.clone())?; if py::lt(&t1924, &t1925)? { true } else { false } }) } } } } }).truthy() {
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
    let t1943 = (Py::tuple(vec![f__dec(py::getitem(&v_i.clone(), &Py::str("x"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?])).items()?;
    if t1943.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t1943[0].clone();
    v_n = t1943[1].clone();
    v_r = f__dec(py::getitem(&v_o.clone(), &Py::str("rounded"))?)?;
    v_places = py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &f__sig_exponent(v_q.clone())?)?;
    v_unit = py::pow(&py::fraction(&[Py::Int(10i64)])?, &py::neg(&v_places.clone())?)?;
    if ({ let t1944 = py::denominator(&py::truediv(&v_r.clone(), &v_unit.clone())?)?; Py::Bool({ let t1945 = Py::Int(1i64); if py::ne(&t1944, &t1945) { true } else { false } }) }).truthy() {
        return Ok(py::add(&Py::Float(1.0), &py::float(&py::abs(&py::sub(&v_r.clone(), &v_q.clone())?)?)?)?);
    }
    v_gap = py::abs(&py::sub(&v_r.clone(), &v_q.clone())?)?;
    if ({ let t1953 = { let t1946 = v_gap.clone(); Py::Bool({ let t1947 = py::truediv(&v_unit.clone(), &Py::Int(2i64))?; if py::gt(&t1946, &t1947)? { true } else { false } }) }; if t1953.truthy() { t1953 } else { { let t1952 = { let t1948 = v_gap.clone(); Py::Bool({ let t1949 = py::truediv(&v_unit.clone(), &Py::Int(2i64))?; if py::eq(&t1948, &t1949) { true } else { false } }) }; if !t1952.truthy() { t1952 } else { { let t1950 = py::abs(&v_r.clone())?; Py::Bool({ let t1951 = py::abs(&v_q.clone())?; if py::lt(&t1950, &t1951)? { true } else { false } }) } } } } }).truthy() {
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
    let t1886 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("opposite"))?, py::getitem(&v_o.clone(), &Py::str("adjacent"))?, py::getitem(&v_o.clone(), &Py::str("hypotenuse"))?])).items()?;
    if t1886.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1886[0].clone();
    v_b = t1886[1].clone();
    v_c = t1886[2].clone();
    v_given = py::getitem(&Py::dict(vec![(Py::str("hypotenuse"), v_c.clone()), (Py::str("opposite"), v_a.clone()), (Py::str("adjacent"), v_b.clone())]), &py::getitem(&v_i.clone(), &Py::str("which"))?)?;
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&v_c.clone(), &v_c.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&v_c.clone(), &v_c.clone())?], true)?)?, &py::truediv(&py::abs(&py::sub(&v_a.clone(), &py::mul(&v_c.clone(), &py::math1("sin", &py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("A"))?)?)?)?)?)?, &py::minmax(&[Py::Float(1.0), v_c.clone()], true)?)?)?, &f__rel(v_given.clone(), py::getitem(&v_i.clone(), &Py::str("side"))?)?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("B"))?, &py::getitem(&v_i.clone(), &Py::str("A"))?)?, &Py::Int(90i64))?)?, &Py::Int(90i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_rt_legs(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let t1905 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?])).items()?;
    if t1905.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1905[0].clone();
    v_b = t1905[1].clone();
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("c"))?, &Py::Int(2i64))?, &py::mul(&v_a.clone(), &v_a.clone())?)?, &py::mul(&v_b.clone(), &v_b.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::math1("tan", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("A"))?)?)?, &v_b.clone())?, &v_a.clone())?)?, &py::minmax(&[Py::Float(1.0), v_a.clone()], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("A"))?, &py::getitem(&v_o.clone(), &Py::str("B"))?)?, &Py::Int(90i64))?)?, &Py::Int(90i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_sci(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t1964 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("mantissa"))?, py::getitem(&v_o.clone(), &Py::str("exponent"))?])).items()?;
    if t1964.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1964[0].clone();
    v_e = t1964[1].clone();
    if ({ let t1970 = { let t1965 = v_e.clone(); Py::Bool({ let t1966 = py::int(&v_e.clone())?; if py::ne(&t1965, &t1966) { true } else { false } }) }; if t1970.truthy() { t1970 } else { py::not(&{ let t1967 = Py::Int(1i64); Py::Bool({ let t1968 = py::abs(&v_m.clone())?; if py::le(&t1967, &t1968)? { { let t1969 = Py::Int(10i64); if py::lt(&t1968, &t1969)? { true } else { false } } } else { false } }) }) } }).truthy() {
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
    let t802 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("A"))?])).items()?;
    if t802.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t802[0].clone();
    v_b = t802[1].clone();
    v_A = t802[2].clone();
    v_n = py::int(&py::getitem(&v_o.clone(), &Py::str("count"))?)?;
    if ({ let t814 = { let t803 = v_n.clone(); Py::Bool({ let t804 = py::getitem(&v_o.clone(), &Py::str("count"))?; if py::ne(&t803, &t804) { true } else { false } }) }; if t814.truthy() { t814 } else { { let t813 = { let t805 = v_n.clone(); Py::Bool({ let t806 = Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)]); if !py::contains(&t806, &t805)? { true } else { false } }) }; if t813.truthy() { t813 } else { { let t812 = Py::Bool(({ let mut out = vec![]; for t807 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t807;
out.push({ let t808 = Py::str(&["B".to_string(), py::format(&v_k.clone(), "")?].concat()); Py::Bool({ let t809 = v_o.clone(); if !py::contains(&t809, &t808)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t812.truthy() { t812 } else { { let t810 = Py::str(&["B".to_string(), py::format(&py::add(&v_n.clone(), &Py::Int(1i64))?, "")?].concat()); Py::Bool({ let t811 = v_o.clone(); if py::contains(&t811, &t810)? { true } else { false } }) } } } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    for t815 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t815;
        let t816 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str(&["B".to_string(), py::format(&v_k.clone(), "")?].concat()))?, py::getitem(&v_o.clone(), &Py::str(&["C".to_string(), py::format(&v_k.clone(), "")?].concat()))?, py::getitem(&v_o.clone(), &Py::str(&["c".to_string(), py::format(&v_k.clone(), "")?].concat()))?])).items()?;
        if t816.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_B = t816[0].clone();
        v_C = t816[1].clone();
        v_c = t816[2].clone();
        v_worst = py::iadd(&v_worst, &py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::add(&v_A.clone(), &v_B.clone())?, &v_C.clone())?, &Py::Int(180i64))?)?, &Py::Int(180i64))?, &py::truediv(&py::abs(&py::sub(&py::mul(&v_a.clone(), &v_a.clone())?, &py::sub(&py::add(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_b.clone())?, &v_c.clone())?, &py::math1("cos", &py::math1("radians", &v_A.clone())?)?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&v_a.clone(), &v_a.clone())?, py::mul(&v_b.clone(), &v_b.clone())?], true)?)?)?)?;
        v_worst = py::iadd(&v_worst, &py::truediv(&py::abs(&py::sub(&py::mul(&v_b.clone(), &f__deg_sin(v_A.clone())?)?, &py::mul(&v_a.clone(), &f__deg_sin(v_B.clone())?)?)?)?, &py::minmax(&[Py::Float(1.0), v_a.clone(), v_b.clone()], true)?)?)?;
    }
    let t817 = (Py::tuple(vec![py::mul(&py::mul(&py::neg(&Py::Int(2i64))?, &v_b.clone())?, &py::math1("cos", &py::math1("radians", &v_A.clone())?)?)?, py::sub(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&v_a.clone(), &v_a.clone())?)?])).items()?;
    if t817.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t817[0].clone();
    v_q = t817[1].clone();
    v_disc = py::sub(&py::mul(&v_p.clone(), &v_p.clone())?, &py::mul(&Py::Int(4i64), &v_q.clone())?)?;
    if ({ let t818 = py::abs(&v_disc.clone())?; Py::Bool({ let t819 = py::mul(&Py::Float(1e-11), &py::minmax(&[Py::Float(1.0), py::mul(&v_p.clone(), &v_p.clone())?, py::abs(&py::mul(&Py::Int(4i64), &v_q.clone())?)?], true)?)?; if py::le(&t818, &t819)? { true } else { false } }) }).truthy() {
        v_disc = Py::Float(0.0);
    }
    v_roots = (if ({ let t820 = v_disc.clone(); Py::Bool({ let t821 = Py::Int(0i64); if py::lt(&t820, &t821)? { true } else { false } }) }).truthy() { Py::list(vec![]) } else { Py::list(vec![py::truediv(&py::sub(&py::neg(&v_p.clone())?, &py::math1("sqrt", &v_disc.clone())?)?, &Py::Int(2i64))?, py::truediv(&py::add(&py::neg(&v_p.clone())?, &py::math1("sqrt", &v_disc.clone())?)?, &Py::Int(2i64))?]) });
    v_want = py::len(&py::set(&{ let mut out = vec![]; for t822 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t822;
if ({ let t823 = v_r.clone(); Py::Bool({ let t824 = Py::Float(1e-09); if py::gt(&t823, &t824)? { true } else { false } }) }).truthy() { out.push(py::round(&v_r.clone(), Some((Py::Int(9i64)).i()?))?); }} Py::list(out) })?)?;
    return Ok(py::add(&v_worst.clone(), &{ let t825 = v_want.clone(); Py::Bool({ let t826 = v_n.clone(); if py::ne(&t825, &t826) { true } else { false } }) })?);
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
    v_co = f__strip({ let mut out = vec![]; for t1985 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("coefficients"))?))? { let mut v_c: Py; 
v_c = t1985;
out.push(py::float(&v_c.clone())?); } Py::list(out) })?;
    let t1986 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("quotient"))?, py::getitem(&v_o.clone(), &Py::str("remainder"))?, py::getitem(&v_i.clone(), &Py::str("r"))?])).items()?;
    if t1986.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t1986[0].clone();
    v_rem = t1986[1].clone();
    v_r = t1986[2].clone();
    if ({ let t1987 = py::len(&v_q.clone())?; Py::Bool({ let t1988 = py::sub(&py::len(&v_co.clone())?, &Py::Int(1i64))?; if py::ne(&t1987, &t1988) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_back = py::add(&py::add(&Py::list(vec![py::getitem(&v_q.clone(), &Py::Int(0i64))?]), &{ let mut out = vec![]; for t1989 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_q.clone())?).i()?]) { let mut v_k: Py; 
v_k = t1989;
out.push(py::sub(&py::getitem(&v_q.clone(), &v_k.clone())?, &py::mul(&v_r.clone(), &py::getitem(&v_q.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?)?); } Py::list(out) })?, &Py::list(vec![py::sub(&v_rem.clone(), &py::mul(&v_r.clone(), &py::getitem(&v_q.clone(), &py::neg(&Py::Int(1i64))?)?)?)?]))?;
    v_scale = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t1990 in py::iter(&(v_co.clone()))? { let mut v_c: Py; 
v_c = t1990;
out.push(py::abs(&v_c.clone())?); } Py::list(out) }], true)?, py::mul(&py::minmax(&[{ let mut out = vec![]; for t1991 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t1991;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?], true)?;
    return Ok(py::truediv(&py::minmax(&[{ let mut out = vec![]; for t1992 in py::iter(&(py::zip(&[v_back.clone(), v_co.clone()])?))? { let mut v_u: Py; let mut v_v: Py; 
let t1993 = (t1992).items()?;
if t1993.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t1993[0].clone();
v_v = t1993[1].clone();
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
    for t938 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("augmented"))?))? {
        v_r = t938;
        v_terms = Py::list(vec![py::mul(&py::getitem(&v_r.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::str("x"))?)?, py::mul(&py::getitem(&v_r.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::str("y"))?)?, py::mul(&py::getitem(&v_r.clone(), &Py::Int(2i64))?, &py::getitem(&v_o.clone(), &Py::str("z"))?)?]);
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::sum(&v_terms.clone(), None)?, &py::getitem(&v_r.clone(), &Py::Int(3i64))?)?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::sum(&{ let mut out = vec![]; for t939 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t939;
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
    let t2031 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("func"))?, py::getitem(&v_i.clone(), &Py::str("k"))?, py::int(&py::method(&v_i.clone(), "get", &[Py::str("n"), Py::Int(1i64)])?)?])).items()?;
    if t2031.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_func = t2031[0].clone();
    v_k = t2031[1].clone();
    v_n = t2031[2].clone();
    v_unit = py::method(&v_i.clone(), "get", &[Py::str("unit"), Py::str("degrees")])?;
    v_per = (if ({ let t2032 = v_func.clone(); Py::Bool({ let t2033 = Py::str("tan"); if py::eq(&t2032, &t2033) { true } else { false } }) }).truthy() { Py::Int(2i64) } else { (if ({ let t2034 = py::abs(&v_k.clone())?; Py::Bool({ let t2035 = Py::Int(1i64); if py::lt(&t2034, &t2035)? { true } else { false } }) }).truthy() { Py::Int(2i64) } else { (if ({ let t2036 = py::abs(&v_k.clone())?; Py::Bool({ let t2037 = Py::Int(1i64); if py::eq(&t2036, &t2037) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }) }) });
    v_sols = py::getitem(&v_o.clone(), &Py::str("solutions"))?;
    if ({ let t2042 = { let t2038 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t2039 = py::len(&v_sols.clone())?; if py::ne(&t2038, &t2039) { true } else { false } }) }; if t2042.truthy() { t2042 } else { { let t2040 = py::len(&v_sols.clone())?; Py::Bool({ let t2041 = py::mul(&v_per.clone(), &v_n.clone())?; if py::ne(&t2040, &t2041) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_P = (if ({ let t2043 = v_unit.clone(); Py::Bool({ let t2044 = Py::str("degrees"); if py::eq(&t2043, &t2044) { true } else { false } }) }).truthy() { Py::Float(360.0) } else { py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))? });
    v_worst = Py::Float(0.0);
    for t2045 in py::iter(&(py::enumerate(&v_sols.clone(), 0)?))? {
        let t2055 = (t2045).items()?;
        if t2055.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t2055[0].clone();
        v_x = t2055[1].clone();
        if ({ let t2052 = py::not(&{ let t2046 = Py::Int(0i64); Py::Bool({ let t2047 = v_x.clone(); if py::le(&t2046, &t2047)? { { let t2048 = v_P.clone(); if py::lt(&t2047, &t2048)? { true } else { false } } } else { false } }) }); if t2052.truthy() { t2052 } else { { let t2051 = v_j.clone(); if !t2051.truthy() { t2051 } else { { let t2049 = v_x.clone(); Py::Bool({ let t2050 = py::add(&py::getitem(&v_sols.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?, &py::mul(&Py::Float(1e-12), &v_P.clone())?)?; if py::le(&t2049, &t2050)? { true } else { false } }) } } } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        v_rad = (if ({ let t2053 = v_unit.clone(); Py::Bool({ let t2054 = Py::str("degrees"); if py::eq(&t2053, &t2054) { true } else { false } }) }).truthy() { py::math1("radians", &v_x.clone())? } else { v_x.clone() });
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
    let t2111 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("sin"))?, py::getitem(&v_o.clone(), &Py::str("cos"))?])).items()?;
    if t2111.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t2111[0].clone();
    v_c = t2111[1].clone();
    if ({ let t2124 = { let t2112 = { let t2114 = v_s.clone(); Py::Bool({ let t2115 = Py::Int(0i64); if py::gt(&t2114, &t2115)? { true } else { false } }) }; Py::Bool({ let t2113 = { let t2116 = v_q.clone(); Py::Bool({ let t2117 = Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)]); if py::contains(&t2117, &t2116)? { true } else { false } }) }; if py::ne(&t2112, &t2113) { true } else { false } }) }; if t2124.truthy() { t2124 } else { { let t2118 = { let t2120 = v_c.clone(); Py::Bool({ let t2121 = Py::Int(0i64); if py::gt(&t2120, &t2121)? { true } else { false } }) }; Py::Bool({ let t2119 = { let t2122 = v_q.clone(); Py::Bool({ let t2123 = Py::tuple(vec![Py::Int(1i64), Py::Int(4i64)]); if py::contains(&t2123, &t2122)? { true } else { false } }) }; if py::ne(&t2118, &t2119) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_given = py::getitem(&Py::dict(vec![(Py::str("sin"), v_s.clone()), (Py::str("cos"), v_c.clone()), (Py::str("tan"), py::getitem(&v_o.clone(), &Py::str("tan"))?)]), &py::getitem(&v_i.clone(), &Py::str("func"))?)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::abs(&py::sub(&py::add(&py::mul(&v_s.clone(), &v_s.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?, &Py::Int(1i64))?)?, &f__rel(v_given.clone(), py::getitem(&v_i.clone(), &Py::str("value"))?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("tan"))?, &v_c.clone())?, v_s.clone())?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("sin2"))?, &py::mul(&py::mul(&Py::Int(2i64), &v_s.clone())?, &v_c.clone())?)?)?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("cos2"))?, &py::sub(&py::mul(&v_c.clone(), &v_c.clone())?, &py::mul(&v_s.clone(), &v_s.clone())?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_two_conics(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2242 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("first"))?, py::getitem(&v_i.clone(), &Py::str("second"))?])).items()?;
    if t2242.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t2242[0].clone();
    v_g = t2242[1].clone();
    v_pts = py::getitem(&v_o.clone(), &Py::str("points"))?;
    if ({ let t2255 = { let t2243 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t2244 = py::len(&v_pts.clone())?; if py::ne(&t2243, &t2244) { true } else { false } }) }; if t2255.truthy() { t2255 } else { py::not(&f__cg_points_in_order(v_pts.clone())?) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    for t2256 in py::iter(&(v_pts.clone()))? {
        v_p = t2256;
        v_worst = py::minmax(&[v_worst.clone(), f__cg_conic_res(v_f.clone(), py::getitem(&v_p.clone(), &Py::Int(0i64))?, py::getitem(&v_p.clone(), &Py::Int(1i64))?)?, f__cg_conic_res(v_g.clone(), py::getitem(&v_p.clone(), &Py::Int(0i64))?, py::getitem(&v_p.clone(), &Py::Int(1i64))?)?], true)?;
    }
    if ({ let t2276 = { let t2258 = py::getitem(&v_f.clone(), &Py::Int(1i64))?; Py::Bool({ let t2259 = Py::Int(0i64); if py::eq(&t2258, &t2259) { true } else { false } }) }; if !t2276.truthy() { t2276 } else { { let t2275 = { let t2260 = py::getitem(&v_g.clone(), &Py::Int(1i64))?; Py::Bool({ let t2261 = Py::Int(0i64); if py::eq(&t2260, &t2261) { true } else { false } }) }; if !t2275.truthy() { t2275 } else { { let t2274 = { let t2262 = py::getitem(&v_f.clone(), &Py::Int(3i64))?; Py::Bool({ let t2263 = Py::Int(0i64); if py::eq(&t2262, &t2263) { true } else { false } }) }; if !t2274.truthy() { t2274 } else { { let t2273 = { let t2264 = py::getitem(&v_g.clone(), &Py::Int(3i64))?; Py::Bool({ let t2265 = Py::Int(0i64); if py::eq(&t2264, &t2265) { true } else { false } }) }; if !t2273.truthy() { t2273 } else { py::not(&f__cg_mirrored(v_pts.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?) } } } } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t2288 = { let t2277 = py::getitem(&v_f.clone(), &Py::Int(1i64))?; Py::Bool({ let t2278 = Py::Int(0i64); if py::eq(&t2277, &t2278) { true } else { false } }) }; if !t2288.truthy() { t2288 } else { { let t2287 = { let t2279 = py::getitem(&v_g.clone(), &Py::Int(1i64))?; Py::Bool({ let t2280 = Py::Int(0i64); if py::eq(&t2279, &t2280) { true } else { false } }) }; if !t2287.truthy() { t2287 } else { { let t2286 = { let t2281 = py::getitem(&v_f.clone(), &Py::Int(4i64))?; Py::Bool({ let t2282 = Py::Int(0i64); if py::eq(&t2281, &t2282) { true } else { false } }) }; if !t2286.truthy() { t2286 } else { { let t2285 = { let t2283 = py::getitem(&v_g.clone(), &Py::Int(4i64))?; Py::Bool({ let t2284 = Py::Int(0i64); if py::eq(&t2283, &t2284) { true } else { false } }) }; if !t2285.truthy() { t2285 } else { py::not(&f__cg_mirrored(v_pts.clone(), Py::Float(1.0), py::neg(&Py::Float(1.0))?)?) } } } } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(v_worst.clone());
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
    if ({ let t2315 = { let t2317 = py::getitem(&v_o.clone(), &Py::str("tan"))?; Py::Bool({ let t2318 = Py::None; if py::is(&t2317, &t2318) { true } else { false } }) }; Py::Bool({ let t2316 = { let t2319 = py::abs(&py::math1("cos", &v_t.clone())?)?; Py::Bool({ let t2320 = Py::Float(1e-12); if py::lt(&t2319, &t2320)? { true } else { false } }) }; if py::ne(&t2315, &t2316) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t2321 = py::getitem(&v_o.clone(), &Py::str("tan"))?; Py::Bool({ let t2322 = Py::None; if !py::is(&t2321, &t2322) { true } else { false } }) }).truthy() {
        v_r = py::iadd(&v_r, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("tan"))?, &py::math1("tan", &v_t.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::math1("tan", &v_t.clone())?)?], true)?)?)?;
    }
    for t2323 in py::iter(&(Py::tuple(vec![Py::str("sin"), Py::str("cos"), Py::str("tan")])))? {
        v_k = t2323;
        v_tv = f__exact_text_value(py::getitem(&v_o.clone(), &py::add(&v_k.clone(), &Py::str("_exact"))?)?)?;
        if ({ let t2337 = { let t2326 = { let t2328 = v_tv.clone(); Py::Bool({ let t2329 = Py::None; if py::is(&t2328, &t2329) { true } else { false } }) }; Py::Bool({ let t2327 = { let t2330 = py::getitem(&v_o.clone(), &v_k.clone())?; Py::Bool({ let t2331 = Py::None; if py::is(&t2330, &t2331) { true } else { false } }) }; if py::ne(&t2326, &t2327) { true } else { false } }) }; if t2337.truthy() { t2337 } else { { let t2336 = { let t2332 = v_tv.clone(); Py::Bool({ let t2333 = Py::None; if !py::is(&t2332, &t2333) { true } else { false } }) }; if !t2336.truthy() { t2336 } else { { let t2334 = py::abs(&py::sub(&v_tv.clone(), &py::getitem(&v_o.clone(), &v_k.clone())?)?)?; Py::Bool({ let t2335 = Py::Float(1e-15); if py::gt(&t2334, &t2335)? { true } else { false } }) } } } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_unit_conversion(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_frm: Py = Py::None;
    let mut v_k1: Py = Py::None;
    let mut v_k2: Py = Py::None;
    let mut v_to: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2380 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("from_unit"))?, py::getitem(&v_i.clone(), &Py::str("to_unit"))?])).items()?;
    if t2380.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_frm = t2380[0].clone();
    v_to = t2380[1].clone();
    let t2381 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("value"))?, py::getitem(&v_o.clone(), &Py::str("converted"))?])).items()?;
    if t2381.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t2381[0].clone();
    v_c = t2381[1].clone();
    if ({ let t2382 = py::getitem(&py::getitem(&c__CG_UNITS()?, &v_frm.clone())?, &Py::Int(0i64))?; Py::Bool({ let t2383 = Py::str("temperature"); if py::eq(&t2382, &t2383) { true } else { false } }) }).truthy() {
        v_k1 = f__cg_temperature(v_v.clone(), v_frm.clone(), Py::str("kelvin"))?;
        v_k2 = f__cg_temperature(v_c.clone(), v_to.clone(), Py::str("kelvin"))?;
        return Ok(py::truediv(&py::abs(&py::sub(&v_k1.clone(), &v_k2.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_k1.clone())?], true)?)?);
    }
    v_a = py::mul(&v_v.clone(), &py::getitem(&py::getitem(&c__CG_UNITS()?, &v_frm.clone())?, &Py::Int(1i64))?)?;
    v_b = py::mul(&v_c.clone(), &py::getitem(&py::getitem(&c__CG_UNITS()?, &v_to.clone())?, &Py::Int(1i64))?)?;
    return Ok(py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?, Py::Float(1e-300)], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_variation(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_kind: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let mut v_y1: Py = Py::None;
    v_kind = py::getitem(&v_i.clone(), &Py::str("variation_type"))?;
    v_t1 = f__cg_vary_term(v_kind.clone(), py::getitem(&v_i.clone(), &Py::str("x1"))?, py::method(&v_i.clone(), "get", &[Py::str("z1")])?)?;
    v_t2 = f__cg_vary_term(v_kind.clone(), py::getitem(&v_i.clone(), &Py::str("x2"))?, py::method(&v_i.clone(), "get", &[Py::str("z2")])?)?;
    v_y1 = py::getitem(&v_i.clone(), &Py::str("y1"))?;
    v_r1 = py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("k"))?, &v_t1.clone())?, &v_y1.clone())?)?, &py::minmax(&[py::abs(&v_y1.clone())?, py::abs(&py::mul(&py::getitem(&v_o.clone(), &Py::str("k"))?, &v_t1.clone())?)?, Py::Float(1e-300)], true)?)?;
    v_r2 = py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("y2"))?, &v_t1.clone())?, &py::mul(&v_y1.clone(), &v_t2.clone())?)?)?, &py::minmax(&[py::abs(&py::mul(&py::getitem(&v_o.clone(), &Py::str("y2"))?, &v_t1.clone())?)?, py::abs(&py::mul(&v_y1.clone(), &v_t2.clone())?)?, Py::Float(1e-300)], true)?)?;
    return Ok(py::add(&v_r1.clone(), &v_r2.clone())?);
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
    let t2486 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("u"))?, py::getitem(&v_i.clone(), &Py::str("v"))?])).items()?;
    if t2486.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t2486[0].clone();
    v_v = t2486[1].clone();
    let t2489 = (Py::tuple(vec![py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t2487 in py::iter(&(v_u.clone()))? { let mut v_a: Py; 
v_a = t2487;
out.push(py::mul(&v_a.clone(), &v_a.clone())?); } Py::list(out) }, None)?)?, py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t2488 in py::iter(&(v_v.clone()))? { let mut v_b: Py; 
v_b = t2488;
out.push(py::mul(&v_b.clone(), &v_b.clone())?); } Py::list(out) }, None)?)?])).items()?;
    if t2489.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mu = t2489[0].clone();
    v_mv = t2489[1].clone();
    v_dot = py::sum(&{ let mut out = vec![]; for t2490 in py::iter(&(py::zip(&[v_u.clone(), v_v.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2491 = (t2490).items()?;
if t2491.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2491[0].clone();
v_b = t2491[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?;
    v_t = py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("angle_deg"))?)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("dot"))?, v_dot.clone())?, &f__rel(py::getitem(&v_o.clone(), &Py::str("mag_u"))?, v_mu.clone())?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("mag_v"))?, v_mv.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&py::math1("cos", &v_t.clone())?, &v_mu.clone())?, &v_mv.clone())?, &v_dot.clone())?)?, &py::minmax(&[Py::Float(1.0), py::mul(&v_mu.clone(), &v_mv.clone())?], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&py::math1("sin", &v_t.clone())?, &v_mu.clone())?, &v_mv.clone())?, &f__cross_mag(v_u.clone(), v_v.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&v_mu.clone(), &v_mv.clone())?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__whole(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t278 = { let t274 = py::float(&v_v.clone())?; Py::Bool({ let t275 = py::int(&py::float(&v_v.clone())?)?; if py::ne(&t274, &t275) { true } else { false } }) }; if t278.truthy() { t278 } else { { let t276 = py::float(&v_v.clone())?; Py::Bool({ let t277 = Py::Int(0i64); if py::lt(&t276, &t277)? { true } else { false } }) } } }).truthy() {
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
fn f_binomial_series(mut v_x: Py, mut v_n: Py, mut v_terms: Py) -> R {
    let mut v_exact: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let mut v_whole: Py = Py::None;
    let _ = f__cg_finite(v_x.clone(), Py::str("x"))?;
    let _ = f__cg_finite(v_n.clone(), Py::str("n"))?;
    v_m = py::int(&v_terms.clone())?;
    if ({ let t61 = { let t57 = v_m.clone(); Py::Bool({ let t58 = Py::Int(1i64); if py::lt(&t57, &t58)? { true } else { false } }) }; if t61.truthy() { t61 } else { { let t59 = v_m.clone(); Py::Bool({ let t60 = Py::Int(30i64); if py::gt(&t59, &t60)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of terms is from 1 to 30"))));
    }
    v_whole = { let t66 = { let t62 = v_n.clone(); Py::Bool({ let t63 = Py::Int(0i64); if py::ge(&t62, &t63)? { true } else { false } }) }; if !t66.truthy() { t66 } else { { let t64 = v_n.clone(); Py::Bool({ let t65 = py::math1("floor", &v_n.clone())?; if py::eq(&t64, &t65) { true } else { false } }) } } };
    if ({ let t69 = py::not(&v_whole.clone()); if !t69.truthy() { t69 } else { py::not(&{ let t67 = py::abs(&v_x.clone())?; Py::Bool({ let t68 = Py::Int(1i64); if py::lt(&t67, &t68)? { true } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("for a power that is not a whole number >= 0 the series converges only for |x| < 1"))));
    }
    v_vals = Py::list(vec![]);
    v_total = Py::Float(0.0);
    v_t = Py::Float(1.0);
    for t70 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_k = t70;
        let _ = py::method(&v_vals.clone(), "append", &[py::add(&v_t.clone(), &Py::Float(0.0))?])?;
        v_total = py::iadd(&v_total, &v_t.clone())?;
        v_t = py::mul(&py::truediv(&py::mul(&v_t.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &v_x.clone())?;
    }
    v_exact = py::pow(&py::add(&Py::Float(1.0), &v_x.clone())?, &v_n.clone())?;
    return Ok(Py::dict(vec![(Py::str("approx"), py::add(&v_total.clone(), &Py::Float(0.0))?), (Py::str("exact"), py::add(&py::float(&Py::str(&[py::format(&v_exact.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?), (Py::str("next_term"), py::add(&v_t.clone(), &Py::Float(0.0))?), (Py::str("percent_change_linear"), py::add(&py::mul(&py::mul(&Py::Float(100.0), &v_n.clone())?, &v_x.clone())?, &Py::Float(0.0))?), (Py::str("term_values"), v_vals.clone())]));
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
    let t84 = (Py::tuple(vec![py::complex(&[v_a.clone(), v_b.clone()])?, py::complex(&[v_c.clone(), v_d.clone()])?])).items()?;
    if t84.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_z = t84[0].clone();
    v_w = t84[1].clone();
    if ({ let t89 = { let t85 = v_op.clone(); Py::Bool({ let t86 = Py::str("/"); if py::eq(&t85, &t86) { true } else { false } }) }; if !t89.truthy() { t89 } else { { let t87 = v_w.clone(); Py::Bool({ let t88 = Py::Int(0i64); if py::eq(&t87, &t88) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("division by 0"))));
    }
    let t91 = ({ let mut out = vec![]; for t90 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()])))? { let mut v_v: Py; 
v_v = t90;
out.push(f__fr(v_v.clone())?); } Py::list(out) }).items()?;
    if t91.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t91[0].clone();
    v_B = t91[1].clone();
    v_C = t91[2].clone();
    v_D = t91[3].clone();
    if ({ let t92 = v_op.clone(); Py::Bool({ let t93 = Py::str("+"); if py::eq(&t92, &t93) { true } else { false } }) }).truthy() {
        let t94 = (Py::tuple(vec![py::add(&v_A.clone(), &v_C.clone())?, py::add(&v_B.clone(), &v_D.clone())?])).items()?;
        if t94.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_re_ = t94[0].clone();
        v_im = t94[1].clone();
    } else {
        if ({ let t95 = v_op.clone(); Py::Bool({ let t96 = Py::str("-"); if py::eq(&t95, &t96) { true } else { false } }) }).truthy() {
            let t97 = (Py::tuple(vec![py::sub(&v_A.clone(), &v_C.clone())?, py::sub(&v_B.clone(), &v_D.clone())?])).items()?;
            if t97.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_re_ = t97[0].clone();
            v_im = t97[1].clone();
        } else {
            if ({ let t98 = v_op.clone(); Py::Bool({ let t99 = Py::str("*"); if py::eq(&t98, &t99) { true } else { false } }) }).truthy() {
                let t100 = (Py::tuple(vec![py::sub(&py::mul(&v_A.clone(), &v_C.clone())?, &py::mul(&v_B.clone(), &v_D.clone())?)?, py::add(&py::mul(&v_A.clone(), &v_D.clone())?, &py::mul(&v_B.clone(), &v_C.clone())?)?])).items()?;
                if t100.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_re_ = t100[0].clone();
                v_im = t100[1].clone();
            } else {
                v_den = py::add(&py::mul(&v_C.clone(), &v_C.clone())?, &py::mul(&v_D.clone(), &v_D.clone())?)?;
                let t101 = (Py::tuple(vec![py::truediv(&py::add(&py::mul(&v_A.clone(), &v_C.clone())?, &py::mul(&v_B.clone(), &v_D.clone())?)?, &v_den.clone())?, py::truediv(&py::sub(&py::mul(&v_B.clone(), &v_C.clone())?, &py::mul(&v_A.clone(), &v_D.clone())?)?, &v_den.clone())?])).items()?;
                if t101.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_re_ = t101[0].clone();
                v_im = t101[1].clone();
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
    if ({ let t117 = { let t113 = v_a.clone(); Py::Bool({ let t114 = Py::Int(0i64); if py::eq(&t113, &t114) { true } else { false } }) }; if !t117.truthy() { t117 } else { { let t115 = v_b.clone(); Py::Bool({ let t116 = Py::Int(0i64); if py::eq(&t115, &t116) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("0 has modulus 0 and no argument"))));
    }
    v_r = py::hypot(&[v_a.clone(), v_b.clone()])?;
    v_t = py::math1("degrees", &py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&v_a.clone(), &v_r.clone())?], false)?], true)?)?)?;
    v_t = (if ({ let t118 = v_b.clone(); Py::Bool({ let t119 = Py::Int(0i64); if py::lt(&t118, &t119)? { true } else { false } }) }).truthy() { py::neg(&v_t.clone())? } else { v_t.clone() });
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
    if (py::not(&{ let t128 = Py::Int(2i64); Py::Bool({ let t129 = v_n.clone(); if py::le(&t128, &t129)? { { let t130 = Py::Int(12i64); if py::le(&t129, &t130)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be a whole number from 2 to 12"))));
    }
    if ({ let t135 = { let t131 = v_a.clone(); Py::Bool({ let t132 = Py::Int(0i64); if py::eq(&t131, &t132) { true } else { false } }) }; if !t135.truthy() { t135 } else { { let t133 = v_b.clone(); Py::Bool({ let t134 = Py::Int(0i64); if py::eq(&t133, &t134) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("0 has only the root 0"))));
    }
    v_r = py::pow(&py::hypot(&[v_a.clone(), v_b.clone()])?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?;
    v_t = py::modulo(&py::atan2(&v_b.clone(), &v_a.clone())?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?;
    v_roots = { let mut out = vec![]; for t136 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t136;
out.push(py::mul(&v_r.clone(), &py::complex(&[py::math1("cos", &py::truediv(&py::add(&v_t.clone(), &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?)?, &v_n.clone())?)?, py::math1("sin", &py::truediv(&py::add(&v_t.clone(), &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?)?, &v_n.clone())?)?])?)?); } Py::list(out) };
    v_snap = { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_n = v_n.clone(); let v_r = v_r.clone(); let v_roots = v_roots.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t137 = py::abs(&v_x.clone())?; Py::Bool({ let t138 = py::mul(&Py::Float(1e-12), &v_r.clone())?; if py::lt(&t137, &t138)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_roots = { let mut out = vec![]; for t139 in py::iter(&(v_roots.clone()))? { let mut v_z: Py; 
v_z = t139;
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
    let t156 = ((if ({ let t154 = v_order.clone(); Py::Bool({ let t155 = Py::str("f(g(x))"); if py::eq(&t154, &t155) { true } else { false } }) }).truthy() { Py::tuple(vec![v_g.clone(), v_f.clone()]) } else { Py::tuple(vec![v_f.clone(), v_g.clone()]) })).items()?;
    if t156.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_first = t156[0].clone();
    v_second = t156[1].clone();
    let t261: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_inner = f__fval(v_first.clone(), v_x.clone())?;
            v_value = f__fval(v_second.clone(), (if (py::not(&Py::Bool(matches!(v_inner.clone(), Py::Frac(_))))).truthy() { py::float(&v_inner.clone())? } else { v_inner.clone() }))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t261 {
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
    if ({ let t270 = py::abs(&py::mul(&v_k.clone(), &v_t.clone())?)?; Py::Bool({ let t271 = Py::Int(700i64); if py::gt(&t270, &t271)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("k*t is too large for a finite result"))));
    }
    return Ok(Py::dict(vec![(Py::str("amount"), py::mul(&v_n0.clone(), &py::math1("exp", &py::mul(&v_k.clone(), &v_t.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_counting_choices(mut v_n: Py, mut v_k: Py) -> R {
    let t279 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"))?, f__whole(v_k.clone(), Py::str("k"))?])).items()?;
    if t279.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t279[0].clone();
    v_k = t279[1].clone();
    if ({ let t280 = v_k.clone(); Py::Bool({ let t281 = v_n.clone(); if py::gt(&t280, &t281)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("k must not exceed n"))));
    }
    if ({ let t282 = v_n.clone(); Py::Bool({ let t283 = c_COUNTING_MAX_N()?; if py::gt(&t282, &t283)? { true } else { false } }) }).truthy() {
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
    if ({ let t312 = py::abs(&v_n.clone())?; Py::Bool({ let t313 = Py::Int(40i64); if py::gt(&t312, &t313)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the power is limited to 40 in size"))));
    }
    if ({ let t321 = { let t314 = v_a.clone(); Py::Bool({ let t315 = Py::Int(0i64); if py::eq(&t314, &t315) { true } else { false } }) }; if !t321.truthy() { t321 } else { { let t320 = { let t316 = v_b.clone(); Py::Bool({ let t317 = Py::Int(0i64); if py::eq(&t316, &t317) { true } else { false } }) }; if !t320.truthy() { t320 } else { { let t318 = v_n.clone(); Py::Bool({ let t319 = Py::Int(0i64); if py::le(&t318, &t319)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("0 to a power that is not positive is undefined"))));
    }
    let t328 = (f__cpow_exact(f__fr(v_a.clone())?, f__fr(v_b.clone())?, v_n.clone())?).items()?;
    if t328.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rr = t328[0].clone();
    v_ri = t328[1].clone();
    let t329 = (Py::tuple(vec![py::float(&v_rr.clone())?, py::float(&v_ri.clone())?])).items()?;
    if t329.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t329[0].clone();
    v_im = t329[1].clone();
    let t334 = (Py::tuple(vec![(if ({ let t330 = py::abs(&v_re_.clone())?; Py::Bool({ let t331 = Py::Float(1e-12); if py::lt(&t330, &t331)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_re_.clone() }), (if ({ let t332 = py::abs(&v_im.clone())?; Py::Bool({ let t333 = Py::Float(1e-12); if py::lt(&t332, &t333)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_im.clone() })])).items()?;
    if t334.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t334[0].clone();
    v_im = t334[1].clone();
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
    if ({ let t347 = py::denominator(&v_f.clone())?; Py::Bool({ let t348 = Py::Int(360i64); if py::le(&t347, &t348)? { true } else { false } }) }).truthy() {
        let t349 = (Py::tuple(vec![py::numerator(&v_f.clone())?, py::denominator(&v_f.clone())?])).items()?;
        if t349.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t349[0].clone();
        v_d = t349[1].clone();
        v_exact = (if ({ let t350 = v_n.clone(); Py::Bool({ let t351 = Py::Int(0i64); if py::eq(&t350, &t351) { true } else { false } }) }).truthy() { Py::str("0") } else { py::add(&py::add(&py::add(&(if ({ let t352 = v_n.clone(); Py::Bool({ let t353 = Py::Int(0i64); if py::lt(&t352, &t353)? { true } else { false } }) }).truthy() { Py::str("-") } else { Py::str("") }), &(if ({ let t354 = py::abs(&v_n.clone())?; Py::Bool({ let t355 = Py::Int(1i64); if py::eq(&t354, &t355) { true } else { false } }) }).truthy() { Py::str("") } else { Py::str(&py::to_str(&py::abs(&v_n.clone())?)) }))?, &Py::str("pi"))?, &(if ({ let t356 = v_d.clone(); Py::Bool({ let t357 = Py::Int(1i64); if py::eq(&t356, &t357) { true } else { false } }) }).truthy() { Py::str("") } else { Py::str(&["/".to_string(), py::format(&v_d.clone(), "")?].concat()) }))? });
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
    v_nature = (if ({ let t1355 = v_d.clone(); Py::Bool({ let t1356 = Py::Int(0i64); if py::gt(&t1355, &t1356)? { true } else { false } }) }).truthy() { Py::str("two distinct real roots") } else { (if ({ let t1357 = v_d.clone(); Py::Bool({ let t1358 = Py::Int(0i64); if py::eq(&t1357, &t1358) { true } else { false } }) }).truthy() { Py::str("one repeated real root") } else { Py::str("two complex conjugate roots") }) });
    return Ok(Py::dict(vec![(Py::str("discriminant"), v_d.clone()), (Py::str("nature"), v_nature.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_distance_point_to_line(mut v_x0: Py, mut v_y0: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("x0"), v_x0.clone()), (Py::str("y0"), v_y0.clone()), (Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone())]))?;
    if ({ let t370 = { let t366 = v_a.clone(); Py::Bool({ let t367 = Py::Int(0i64); if py::eq(&t366, &t367) { true } else { false } }) }; if !t370.truthy() { t370 } else { { let t368 = v_b.clone(); Py::Bool({ let t369 = Py::Int(0i64); if py::eq(&t368, &t369) { true } else { false } }) } } }).truthy() {
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
    if ({ let t377 = v_a.clone(); Py::Bool({ let t378 = Py::Int(0i64); if py::eq(&t377, &t378) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0 (no x under the root)"))));
    }
    if ({ let t383 = { let t379 = v_c.clone(); Py::Bool({ let t380 = Py::Int(0i64); if py::eq(&t379, &t380) { true } else { false } }) }; if !t383.truthy() { t383 } else { { let t381 = v_d.clone(); Py::Bool({ let t382 = Py::Int(0i64); if py::eq(&t381, &t382) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the denominator is 0 for every x"))));
    }
    let t405 = (f__domain_parts(v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone(), Py::Bool((v_root_in_denominator.clone()).truthy()))?).items()?;
    if t405.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_B = t405[0].clone();
    v_side = t405[1].clone();
    v_E = t405[2].clone();
    v_text = t405[3].clone();
    return Ok(Py::dict(vec![(Py::str("boundary"), py::float(&v_B.clone())?), (Py::str("side"), v_side.clone()), (Py::str("excluded"), (if ({ let t406 = v_E.clone(); Py::Bool({ let t407 = Py::None; if !py::is(&t406, &t407) { true } else { false } }) }).truthy() { py::float(&v_E.clone())? } else { Py::None })), (Py::str("domain"), v_text.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_doubling_time_percent(mut v_r: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("r"), v_r.clone())]))?;
    if ({ let t425 = v_r.clone(); Py::Bool({ let t426 = Py::Int(0i64); if py::le(&t425, &t426)? { true } else { false } }) }).truthy() {
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
    if ({ let t437 = { let t433 = v_A.clone(); Py::Bool({ let t434 = Py::Int(0i64); if py::le(&t433, &t434)? { true } else { false } }) }; if t437.truthy() { t437 } else { { let t435 = v_B.clone(); Py::Bool({ let t436 = Py::Int(0i64); if py::le(&t435, &t436)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("both denominators must be positive"))));
    }
    if ({ let t438 = v_A.clone(); Py::Bool({ let t439 = v_B.clone(); if py::eq(&t438, &t439) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("equal denominators give a circle (no distinct foci)"))));
    }
    let t440 = (Py::tuple(vec![py::math1("sqrt", &py::minmax(&[v_A.clone(), v_B.clone()], true)?)?, py::math1("sqrt", &py::minmax(&[v_A.clone(), v_B.clone()], false)?)?])).items()?;
    if t440.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t440[0].clone();
    v_b = t440[1].clone();
    v_c = py::math1("sqrt", &py::abs(&py::sub(&v_A.clone(), &v_B.clone())?)?)?;
    if ({ let t441 = v_A.clone(); Py::Bool({ let t442 = v_B.clone(); if py::gt(&t441, &t442)? { true } else { false } }) }).truthy() {
        v_f = Py::list(vec![Py::tuple(vec![py::sub(&v_h.clone(), &v_c.clone())?, v_k.clone()]), Py::tuple(vec![py::add(&v_h.clone(), &v_c.clone())?, v_k.clone()])]);
    } else {
        v_f = Py::list(vec![Py::tuple(vec![v_h.clone(), py::sub(&v_k.clone(), &v_c.clone())?]), Py::tuple(vec![v_h.clone(), py::add(&v_k.clone(), &v_c.clone())?])]);
    }
    return Ok(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("eccentricity"), py::truediv(&v_c.clone(), &v_a.clone())?), (Py::str("f1x"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?), (Py::str("f1y"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?), (Py::str("f2x"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?), (Py::str("f2y"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?), (Py::str("orientation"), (if ({ let t443 = v_A.clone(); Py::Bool({ let t444 = v_B.clone(); if py::gt(&t443, &t444)? { true } else { false } }) }).truthy() { Py::str("horizontal") } else { Py::str("vertical") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_evaluate_function(mut v_expression: Py, mut v_x: Py) -> R {
    let mut v_exc: Py = Py::None;
    let t553: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            return Ok(Some(f__exact_outputs(f__fval(v_expression.clone(), v_x.clone())?)?));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t553 {
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
    if ({ let t564 = { let t560 = v_a.clone(); Py::Bool({ let t561 = Py::Int(0i64); if py::eq(&t560, &t561) { true } else { false } }) }; if t564.truthy() { t564 } else { { let t562 = v_c.clone(); Py::Bool({ let t563 = Py::Int(0i64); if py::eq(&t562, &t563) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a and c must not be 0"))));
    }
    if ({ let t569 = { let t565 = v_b.clone(); Py::Bool({ let t566 = Py::Int(0i64); if py::le(&t565, &t566)? { true } else { false } }) }; if t569.truthy() { t569 } else { { let t567 = v_b.clone(); Py::Bool({ let t568 = Py::Int(1i64); if py::eq(&t567, &t568) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the base must be positive and not 1"))));
    }
    if ({ let t570 = py::truediv(&v_k.clone(), &v_a.clone())?; Py::Bool({ let t571 = Py::Int(0i64); if py::le(&t570, &t571)? { true } else { false } }) }).truthy() {
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
    let t573 = (Py::tuple(vec![py::int(&v_a.clone())?, py::int(&v_b.clone())?, py::int(&v_c.clone())?])).items()?;
    if t573.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t573[0].clone();
    v_b = t573[1].clone();
    v_c = t573[2].clone();
    if ({ let t574 = v_a.clone(); Py::Bool({ let t575 = Py::Int(0i64); if py::eq(&t574, &t575) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0 (not a quadratic)"))));
    }
    v_g = py::mul(&py::gcd(&py::gcd(&v_a.clone(), &v_b.clone())?, &v_c.clone())?, &(if ({ let t576 = v_a.clone(); Py::Bool({ let t577 = Py::Int(0i64); if py::gt(&t576, &t577)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? }))?;
    let t578 = (Py::tuple(vec![py::floordiv(&v_a.clone(), &v_g.clone())?, py::floordiv(&v_b.clone(), &v_g.clone())?, py::floordiv(&v_c.clone(), &v_g.clone())?])).items()?;
    if t578.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t578[0].clone();
    v_B = t578[1].clone();
    v_C = t578[2].clone();
    v_D = py::sub(&py::mul(&v_B.clone(), &v_B.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_A.clone())?, &v_C.clone())?)?;
    if ({ let t583 = { let t579 = v_D.clone(); Py::Bool({ let t580 = Py::Int(0i64); if py::lt(&t579, &t580)? { true } else { false } }) }; if t583.truthy() { t583 } else { { let t581 = py::pow(&py::isqrt(&v_D.clone())?, &Py::Int(2i64))?; Py::Bool({ let t582 = v_D.clone(); if py::ne(&t581, &t582) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this quadratic does not factor over the integers (its discriminant is not a perfect square)"))));
    }
    v_s = py::isqrt(&v_D.clone())?;
    let t584 = (Py::tuple(vec![py::fraction(&[py::sub(&py::neg(&v_B.clone())?, &v_s.clone())?, py::mul(&Py::Int(2i64), &v_A.clone())?])?, py::fraction(&[py::add(&py::neg(&v_B.clone())?, &v_s.clone())?, py::mul(&Py::Int(2i64), &v_A.clone())?])?])).items()?;
    if t584.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t584[0].clone();
    v_r2 = t584[1].clone();
    let t585 = (Py::tuple(vec![py::denominator(&v_r1.clone())?, py::neg(&py::numerator(&v_r1.clone())?)?, py::denominator(&v_r2.clone())?, py::neg(&py::numerator(&v_r2.clone())?)?])).items()?;
    if t585.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t585[0].clone();
    v_q = t585[1].clone();
    v_r = t585[2].clone();
    v_t = t585[3].clone();
    if ({ let t586 = py::mul(&v_p.clone(), &v_r.clone())?; Py::Bool({ let t587 = v_A.clone(); if py::ne(&t586, &t587) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("internal: the factors do not reproduce the leading coefficient"))));
    }
    v_fac = { let v_A = v_A.clone(); let v_B = v_B.clone(); let v_C = v_C.clone(); let v_D = v_D.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_g = v_g.clone(); let v_p = v_p.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); let v_s = v_s.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_m: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_n: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::add(&(if ({ let t588 = v_m.clone(); Py::Bool({ let t589 = Py::Int(1i64); if py::eq(&t588, &t589) { true } else { false } }) }).truthy() { Py::str("x") } else { Py::str(&[py::format(&v_m.clone(), "")?, "x".to_string()].concat()) }), &(if ({ let t590 = v_n.clone(); Py::Bool({ let t591 = Py::Int(0i64); if py::gt(&t590, &t591)? { true } else { false } }) }).truthy() { Py::str(&[" + ".to_string(), py::format(&v_n.clone(), "")?].concat()) } else { (if ({ let t592 = v_n.clone(); Py::Bool({ let t593 = Py::Int(0i64); if py::lt(&t592, &t593)? { true } else { false } }) }).truthy() { Py::str(&[" - ".to_string(), py::format(&py::neg(&v_n.clone())?, "")?].concat()) } else { Py::str("") }) }))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_text = py::add(&(if ({ let t594 = v_g.clone(); Py::Bool({ let t595 = Py::Int(1i64); if py::eq(&t594, &t595) { true } else { false } }) }).truthy() { Py::str("") } else { (if ({ let t596 = v_g.clone(); Py::Bool({ let t597 = py::neg(&Py::Int(1i64))?; if py::eq(&t596, &t597) { true } else { false } }) }).truthy() { Py::str("-") } else { Py::str(&[py::format(&v_g.clone(), "")?].concat()) }) }), &Py::str(&["(".to_string(), py::format(&py::call(&v_fac.clone(), &[v_p.clone(), v_q.clone()])?, "")?, ")(".to_string(), py::format(&py::call(&v_fac.clone(), &[v_r.clone(), v_t.clone()])?, "")?, ")".to_string()].concat()))?;
    return Ok(Py::dict(vec![(Py::str("g"), py::float(&v_g.clone())?), (Py::str("p"), py::float(&v_p.clone())?), (Py::str("q"), py::float(&v_q.clone())?), (Py::str("r"), py::float(&v_r.clone())?), (Py::str("s"), py::float(&v_t.clone())?), (Py::str("factored"), v_text.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fraction_arithmetic(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_op: Py) -> R {
    let mut v_f1: Py = Py::None;
    let mut v_f2: Py = Py::None;
    let mut v_r: Py = Py::None;
    if ({ let t622 = { let t618 = v_b.clone(); Py::Bool({ let t619 = Py::Int(0i64); if py::eq(&t618, &t619) { true } else { false } }) }; if t622.truthy() { t622 } else { { let t620 = v_d.clone(); Py::Bool({ let t621 = Py::Int(0i64); if py::eq(&t620, &t621) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a denominator must not be 0"))));
    }
    let t623 = (Py::tuple(vec![py::fraction(&[py::int(&v_a.clone())?, py::int(&v_b.clone())?])?, py::fraction(&[py::int(&v_c.clone())?, py::int(&v_d.clone())?])?])).items()?;
    if t623.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f1 = t623[0].clone();
    v_f2 = t623[1].clone();
    if ({ let t628 = { let t624 = v_op.clone(); Py::Bool({ let t625 = Py::str("/"); if py::eq(&t624, &t625) { true } else { false } }) }; if !t628.truthy() { t628 } else { { let t626 = v_f2.clone(); Py::Bool({ let t627 = Py::Int(0i64); if py::eq(&t626, &t627) { true } else { false } }) } } }).truthy() {
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
    if ({ let t651 = { let t647 = v_A.clone(); Py::Bool({ let t648 = Py::Int(0i64); if py::eq(&t647, &t648) { true } else { false } }) }; if t651.truthy() { t651 } else { { let t649 = v_B.clone(); Py::Bool({ let t650 = Py::Int(0i64); if py::eq(&t649, &t650) { true } else { false } }) } } }).truthy() {
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
    if ({ let t665 = { let t658 = v_n0.clone(); Py::Bool({ let t659 = Py::Int(0i64); if py::le(&t658, &t659)? { true } else { false } }) }; if t665.truthy() { t665 } else { { let t664 = { let t660 = v_half_life.clone(); Py::Bool({ let t661 = Py::Int(0i64); if py::le(&t660, &t661)? { true } else { false } }) }; if t664.truthy() { t664 } else { { let t662 = v_t.clone(); Py::Bool({ let t663 = Py::Int(0i64); if py::lt(&t662, &t663)? { true } else { false } }) } } } } }).truthy() {
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
    let t683 = (py::sorted(&Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone()]), (Py::Bool(true)).truthy())?).items()?;
    if t683.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t683[0].clone();
    v_y = t683[1].clone();
    v_z = t683[2].clone();
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
    if ({ let t694 = { let t690 = v_A.clone(); Py::Bool({ let t691 = Py::Int(0i64); if py::le(&t690, &t691)? { true } else { false } }) }; if t694.truthy() { t694 } else { { let t692 = v_B.clone(); Py::Bool({ let t693 = Py::Int(0i64); if py::le(&t692, &t693)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("both denominators must be positive"))));
    }
    if ({ let t695 = v_orientation.clone(); Py::Bool({ let t696 = c__ORIENT()?; if !py::contains(&t696, &t695)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("orientation is horizontal or vertical"))));
    }
    let t697 = (Py::tuple(vec![py::math1("sqrt", &v_A.clone())?, py::math1("sqrt", &v_B.clone())?])).items()?;
    if t697.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t697[0].clone();
    v_b = t697[1].clone();
    v_c = py::math1("sqrt", &py::add(&v_A.clone(), &v_B.clone())?)?;
    v_f = (if ({ let t698 = v_orientation.clone(); Py::Bool({ let t699 = Py::str("horizontal"); if py::eq(&t698, &t699) { true } else { false } }) }).truthy() { Py::list(vec![Py::tuple(vec![py::sub(&v_h.clone(), &v_c.clone())?, v_k.clone()]), Py::tuple(vec![py::add(&v_h.clone(), &v_c.clone())?, v_k.clone()])]) } else { Py::list(vec![Py::tuple(vec![v_h.clone(), py::sub(&v_k.clone(), &v_c.clone())?]), Py::tuple(vec![v_h.clone(), py::add(&v_k.clone(), &v_c.clone())?])]) });
    return Ok(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("eccentricity"), py::truediv(&v_c.clone(), &v_a.clone())?), (Py::str("asymptote_slope"), (if ({ let t700 = v_orientation.clone(); Py::Bool({ let t701 = Py::str("horizontal"); if py::eq(&t700, &t701) { true } else { false } }) }).truthy() { py::truediv(&v_b.clone(), &v_a.clone())? } else { py::truediv(&v_a.clone(), &v_b.clone())? })), (Py::str("f1x"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?), (Py::str("f1y"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?), (Py::str("f2x"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?), (Py::str("f2y"), py::getitem(&py::getitem(&v_f.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_infinite_geometric_series(mut v_a: Py, mut v_r: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("r"), v_r.clone())]))?;
    if (py::not(&{ let t714 = py::abs(&v_r.clone())?; Py::Bool({ let t715 = Py::Int(1i64); if py::lt(&t714, &t715)? { true } else { false } }) })).truthy() {
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
    if ({ let t721 = v_a.clone(); Py::Bool({ let t722 = Py::Int(0i64); if py::eq(&t721, &t722) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0 (a constant function has no inverse)"))));
    }
    v_out = Py::dict(vec![(Py::str("inv_slope"), py::truediv(&Py::Int(1i64), &v_a.clone())?), (Py::str("inv_intercept"), py::truediv(&py::neg(&v_b.clone())?, &v_a.clone())?)]);
    if ({ let t723 = v_y.clone(); Py::Bool({ let t724 = Py::None; if !py::is(&t723, &t724) { true } else { false } }) }).truthy() {
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
    if ({ let t742 = { let t734 = v_b.clone(); Py::Bool({ let t735 = Py::Int(0i64); if py::le(&t734, &t735)? { true } else { false } }) }; if t742.truthy() { t742 } else { { let t741 = { let t736 = v_c.clone(); Py::Bool({ let t737 = Py::Int(0i64); if py::le(&t736, &t737)? { true } else { false } }) }; if t741.truthy() { t741 } else { py::not(&{ let t738 = Py::Int(0i64); Py::Bool({ let t739 = v_A.clone(); if py::lt(&t738, &t739)? { { let t740 = Py::Int(180i64); if py::lt(&t739, &t740)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
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
    let t759 = (Py::tuple(vec![py::call(&v_ang.clone(), &[v_a.clone(), v_b.clone(), v_c.clone()])?, py::call(&v_ang.clone(), &[v_b.clone(), v_a.clone(), v_c.clone()])?])).items()?;
    if t759.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t759[0].clone();
    v_B = t759[1].clone();
    return Ok(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("B"), v_B.clone()), (Py::str("C"), py::sub(&py::sub(&Py::Float(180.0), &v_A.clone())?, &v_B.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_law_of_sines_aas(mut v_A: Py, mut v_B: Py, mut v_a: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("B"), v_B.clone()), (Py::str("a"), v_a.clone())]))?;
    if ({ let t772 = { let t765 = v_A.clone(); Py::Bool({ let t766 = Py::Int(0i64); if py::le(&t765, &t766)? { true } else { false } }) }; if t772.truthy() { t772 } else { { let t771 = { let t767 = v_B.clone(); Py::Bool({ let t768 = Py::Int(0i64); if py::le(&t767, &t768)? { true } else { false } }) }; if t771.truthy() { t771 } else { { let t769 = py::add(&v_A.clone(), &v_B.clone())?; Py::Bool({ let t770 = Py::Int(180i64); if py::ge(&t769, &t770)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two angles must be positive with a sum below 180 degrees"))));
    }
    if ({ let t773 = v_a.clone(); Py::Bool({ let t774 = Py::Int(0i64); if py::le(&t773, &t774)? { true } else { false } }) }).truthy() {
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
    if ({ let t789 = { let t781 = v_a.clone(); Py::Bool({ let t782 = Py::Int(0i64); if py::le(&t781, &t782)? { true } else { false } }) }; if t789.truthy() { t789 } else { { let t788 = { let t783 = v_b.clone(); Py::Bool({ let t784 = Py::Int(0i64); if py::le(&t783, &t784)? { true } else { false } }) }; if t788.truthy() { t788 } else { py::not(&{ let t785 = Py::Int(0i64); Py::Bool({ let t786 = v_A.clone(); if py::lt(&t785, &t786)? { { let t787 = Py::Int(180i64); if py::lt(&t786, &t787)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the sides must be positive and A between 0 and 180 degrees"))));
    }
    v_s = py::truediv(&py::mul(&v_b.clone(), &f__deg_sin(v_A.clone())?)?, &v_a.clone())?;
    if ({ let t790 = v_s.clone(); Py::Bool({ let t791 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::gt(&t790, &t791)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no triangle exists: side a is shorter than the height b*sin(A)"))));
    }
    v_sols = Py::list(vec![]);
    v_B1 = (if ({ let t792 = v_s.clone(); Py::Bool({ let t793 = py::sub(&Py::Int(1i64), &Py::Float(1e-12))?; if py::ge(&t792, &t793)? { true } else { false } }) }).truthy() { Py::Float(90.0) } else { py::math1("degrees", &py::math1("asin", &v_s.clone())?)? });
    for t796 in py::iter(&((if ({ let t794 = v_B1.clone(); Py::Bool({ let t795 = Py::Float(90.0); if py::eq(&t794, &t795) { true } else { false } }) }).truthy() { Py::list(vec![v_B1.clone()]) } else { Py::list(vec![v_B1.clone(), py::sub(&Py::Float(180.0), &v_B1.clone())?]) })))? {
        v_B = t796;
        v_C = py::sub(&py::sub(&Py::Float(180.0), &v_A.clone())?, &v_B.clone())?;
        if ({ let t797 = v_C.clone(); Py::Bool({ let t798 = Py::Float(1e-09); if py::gt(&t797, &t798)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_sols.clone(), "append", &[Py::tuple(vec![v_B.clone(), v_C.clone(), py::truediv(&py::mul(&v_a.clone(), &f__deg_sin(v_C.clone())?)?, &f__deg_sin(v_A.clone())?)?])])?;
        }
    }
    if (py::not(&v_sols.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no triangle exists with these measurements"))));
    }
    v_out = Py::dict(vec![(Py::str("count"), py::float(&py::len(&v_sols.clone())?)?)]);
    for t799 in py::iter(&(py::enumerate(&v_sols.clone(), (Py::Int(1i64)).i()?)?))? {
        let t800 = (t799).items()?;
        if t800.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t800[0].clone();
        let t801 = (t800[1].clone()).items()?;
        if t801.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_B = t801[0].clone();
        v_C = t801[1].clone();
        v_c = t801[2].clone();
        let _ = py::method(&v_out.clone(), "update", &[Py::dict(vec![(Py::str(&["B".to_string(), py::format(&v_k.clone(), "")?].concat()), v_B.clone()), (Py::str(&["C".to_string(), py::format(&v_k.clone(), "")?].concat()), v_C.clone()), (Py::str(&["c".to_string(), py::format(&v_k.clone(), "")?].concat()), v_c.clone())])])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_conic_intersection(mut v_line: Py, mut v_conic: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_q0: Py = Py::None;
    let mut v_q1: Py = Py::None;
    let mut v_q2: Py = Py::None;
    let mut v_snapped: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t2160 = (Py::tuple(vec![py::getitem(&v_line.clone(), &Py::Int(0i64))?, py::getitem(&v_line.clone(), &Py::Int(1i64))?, py::getitem(&v_line.clone(), &Py::Int(2i64))?])).items()?;
    if t2160.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2160[0].clone();
    v_b = t2160[1].clone();
    v_c = t2160[2].clone();
    let t2161 = (Py::tuple(vec![py::getitem(&v_conic.clone(), &Py::Int(0i64))?, py::getitem(&v_conic.clone(), &Py::Int(1i64))?, py::getitem(&v_conic.clone(), &Py::Int(2i64))?, py::getitem(&v_conic.clone(), &Py::Int(3i64))?, py::getitem(&v_conic.clone(), &Py::Int(4i64))?, py::getitem(&v_conic.clone(), &Py::Int(5i64))?])).items()?;
    if t2161.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2161[0].clone();
    v_B = t2161[1].clone();
    v_C = t2161[2].clone();
    v_D = t2161[3].clone();
    v_E = t2161[4].clone();
    v_F = t2161[5].clone();
    if ({ let t2166 = { let t2162 = v_a.clone(); Py::Bool({ let t2163 = Py::Int(0i64); if py::eq(&t2162, &t2163) { true } else { false } }) }; if !t2166.truthy() { t2166 } else { { let t2164 = v_b.clone(); Py::Bool({ let t2165 = Py::Int(0i64); if py::eq(&t2164, &t2165) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the line a x + b y = c needs a or b nonzero"))));
    }
    if ({ let t2174 = { let t2167 = v_A.clone(); Py::Bool({ let t2168 = Py::Int(0i64); if py::eq(&t2167, &t2168) { true } else { false } }) }; if !t2174.truthy() { t2174 } else { { let t2173 = { let t2169 = v_B.clone(); Py::Bool({ let t2170 = Py::Int(0i64); if py::eq(&t2169, &t2170) { true } else { false } }) }; if !t2173.truthy() { t2173 } else { { let t2171 = v_C.clone(); Py::Bool({ let t2172 = Py::Int(0i64); if py::eq(&t2171, &t2172) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the second equation has no second-degree term: it is not a conic"))));
    }
    v_pts = Py::list(vec![]);
    if ({ let t2175 = v_b.clone(); Py::Bool({ let t2176 = Py::Int(0i64); if py::ne(&t2175, &t2176) { true } else { false } }) }).truthy() {
        v_q2 = py::add(&py::sub(&py::mul(&py::mul(&v_A.clone(), &v_b.clone())?, &v_b.clone())?, &py::mul(&py::mul(&v_B.clone(), &v_a.clone())?, &v_b.clone())?)?, &py::mul(&py::mul(&v_C.clone(), &v_a.clone())?, &v_a.clone())?)?;
        v_q1 = py::sub(&py::add(&py::sub(&py::mul(&py::mul(&v_B.clone(), &v_b.clone())?, &v_c.clone())?, &py::mul(&py::mul(&py::mul(&Py::Float(2.0), &v_C.clone())?, &v_a.clone())?, &v_c.clone())?)?, &py::mul(&py::mul(&v_D.clone(), &v_b.clone())?, &v_b.clone())?)?, &py::mul(&py::mul(&v_E.clone(), &v_a.clone())?, &v_b.clone())?)?;
        v_q0 = py::add(&py::add(&py::mul(&py::mul(&v_C.clone(), &v_c.clone())?, &v_c.clone())?, &py::mul(&py::mul(&v_E.clone(), &v_b.clone())?, &v_c.clone())?)?, &py::mul(&py::mul(&v_F.clone(), &v_b.clone())?, &v_b.clone())?)?;
        for t2195 in py::iter(&(f__cg_quadratic_real(v_q2.clone(), v_q1.clone(), v_q0.clone())?))? {
            v_x = t2195;
            let _ = py::method(&v_pts.clone(), "append", &[Py::list(vec![py::add(&v_x.clone(), &Py::Float(0.0))?, py::add(&py::truediv(&py::sub(&v_c.clone(), &py::mul(&v_a.clone(), &v_x.clone())?)?, &v_b.clone())?, &Py::Float(0.0))?])])?;
        }
    } else {
        v_x0 = py::truediv(&v_c.clone(), &v_a.clone())?;
        for t2196 in py::iter(&(f__cg_quadratic_real(v_C.clone(), py::add(&py::mul(&v_B.clone(), &v_x0.clone())?, &v_E.clone())?, py::add(&py::add(&py::mul(&py::mul(&v_A.clone(), &v_x0.clone())?, &v_x0.clone())?, &py::mul(&v_D.clone(), &v_x0.clone())?)?, &v_F.clone())?)?))? {
            v_y = t2196;
            let _ = py::method(&v_pts.clone(), "append", &[Py::list(vec![py::add(&v_x0.clone(), &Py::Float(0.0))?, py::add(&v_y.clone(), &Py::Float(0.0))?])])?;
        }
    }
    v_snapped = Py::list(vec![]);
    for t2197 in py::iter(&(v_pts.clone()))? {
        v_p = t2197;
        let _ = py::method(&v_snapped.clone(), "append", &[Py::list(vec![py::add(&py::float(&Py::str(&[py::format(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, ".12g")?].concat()))?, &Py::Float(0.0))?, py::add(&py::float(&Py::str(&[py::format(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, ".12g")?].concat()))?, &Py::Float(0.0))?])])?;
    }
    v_pts = f__cg_sorted_points(v_snapped.clone())?;
    return Ok(Py::dict(vec![(Py::str("count"), py::float(&py::len(&v_pts.clone())?)?), (Py::str("points"), v_pts.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_intercepts(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone())]))?;
    if ({ let t910 = { let t906 = v_a.clone(); Py::Bool({ let t907 = Py::Int(0i64); if py::eq(&t906, &t907) { true } else { false } }) }; if t910.truthy() { t910 } else { { let t908 = v_b.clone(); Py::Bool({ let t909 = Py::Int(0i64); if py::eq(&t908, &t909) { true } else { false } }) } } }).truthy() {
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
    if ({ let t917 = { let t912 = py::len(&v_augmented.clone())?; Py::Bool({ let t913 = Py::Int(3i64); if py::ne(&t912, &t913) { true } else { false } }) }; if t917.truthy() { t917 } else { Py::Bool(({ let mut out = vec![]; for t914 in py::iter(&(v_augmented.clone()))? { let mut v_r: Py; 
v_r = t914;
out.push({ let t915 = py::len(&v_r.clone())?; Py::Bool({ let t916 = Py::Int(4i64); if py::ne(&t915, &t916) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the augmented matrix must have 3 rows of 4 numbers (a, b, c | d)"))));
    }
    let t937 = (f__gauss(v_augmented.clone())?).items()?;
    if t937.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t937[0].clone();
    v_y = t937[1].clone();
    v_z = t937[2].clone();
    return Ok(Py::dict(vec![(Py::str("x"), py::float(&v_x.clone())?), (Py::str("y"), py::float(&v_y.clone())?), (Py::str("z"), py::float(&v_z.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_log_equation_linear_argument(mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_k: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("d"), v_d.clone()), (Py::str("k"), v_k.clone())]))?;
    if ({ let t948 = { let t944 = v_b.clone(); Py::Bool({ let t945 = Py::Int(0i64); if py::le(&t944, &t945)? { true } else { false } }) }; if t948.truthy() { t948 } else { { let t946 = v_b.clone(); Py::Bool({ let t947 = Py::Int(1i64); if py::eq(&t946, &t947) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the base must be positive and not 1"))));
    }
    if ({ let t949 = v_c.clone(); Py::Bool({ let t950 = Py::Int(0i64); if py::eq(&t949, &t950) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("c must not be 0 (no x inside the logarithm)"))));
    }
    if ({ let t951 = py::abs(&py::mul(&v_k.clone(), &py::math1("log", &v_b.clone())?)?)?; Py::Bool({ let t952 = Py::Int(700i64); if py::gt(&t951, &t952)? { true } else { false } }) }).truthy() {
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
    if ({ let t963 = { let t959 = v_b.clone(); Py::Bool({ let t960 = Py::Int(0i64); if py::le(&t959, &t960)? { true } else { false } }) }; if t963.truthy() { t963 } else { { let t961 = v_b.clone(); Py::Bool({ let t962 = Py::Int(1i64); if py::eq(&t961, &t962) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the base must be positive and not 1"))));
    }
    if ({ let t964 = v_x.clone(); Py::Bool({ let t965 = Py::Int(0i64); if py::le(&t964, &t965)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the logarithm is defined only for positive numbers"))));
    }
    v_v = (if ({ let t966 = v_b.clone(); Py::Bool({ let t967 = Py::Int(2i64); if py::eq(&t966, &t967) { true } else { false } }) }).truthy() { py::truediv(&py::math1("log2", &v_x.clone())?, &py::math1("log2", &v_b.clone())?)? } else { (if ({ let t968 = v_b.clone(); Py::Bool({ let t969 = Py::Int(10i64); if py::eq(&t968, &t969) { true } else { false } }) }).truthy() { py::truediv(&py::math1("log10", &v_x.clone())?, &py::math1("log10", &v_b.clone())?)? } else { py::truediv(&py::math1("log", &v_x.clone())?, &py::math1("log", &v_b.clone())?)? }) });
    v_n = py::round(&v_v.clone(), None)?;
    if ({ let t980 = { let t970 = py::abs(&py::sub(&v_v.clone(), &v_n.clone())?)?; Py::Bool({ let t971 = Py::Float(1e-12); if py::lt(&t970, &t971)? { true } else { false } }) }; if !t980.truthy() { t980 } else { { let t979 = { let t972 = v_b.clone(); Py::Bool({ let t973 = py::int(&v_b.clone())?; if py::eq(&t972, &t973) { true } else { false } }) }; if !t979.truthy() { t979 } else { { let t978 = { let t974 = py::abs(&v_n.clone())?; Py::Bool({ let t975 = Py::Int(60i64); if py::le(&t974, &t975)? { true } else { false } }) }; if !t978.truthy() { t978 } else { { let t976 = py::pow(&py::fraction(&[py::int(&v_b.clone())?])?, &v_n.clone())?; Py::Bool({ let t977 = f__fr(v_x.clone())?; if py::eq(&t976, &t977) { true } else { false } }) } } } } } } }).truthy() {
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
    if ({ let t987 = py::not(&Py::Bool(matches!(v_expression.clone(), Py::Str(_)))); if t987.truthy() { t987 } else { py::not(&py::method(&c__ARITH()?, "match", &[py::method(&f__clean(v_expression.clone())?, "replace", &[Py::str(" divided by "), Py::str("/")])?])?) } }).truthy() {
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
    if ({ let t1097 = v_m.clone(); Py::Bool({ let t1098 = Py::Int(0i64); if py::eq(&t1097, &t1098) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("m must not be 0 (no parabola)"))));
    }
    if ({ let t1099 = v_orientation.clone(); Py::Bool({ let t1100 = c__ORIENT()?; if !py::contains(&t1100, &t1099)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("orientation is vertical or horizontal"))));
    }
    v_p = py::truediv(&v_m.clone(), &Py::Int(4i64))?;
    if ({ let t1101 = v_orientation.clone(); Py::Bool({ let t1102 = Py::str("vertical"); if py::eq(&t1101, &t1102) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("p"), v_p.clone()), (Py::str("focus_x"), v_h.clone()), (Py::str("focus_y"), py::add(&v_k.clone(), &v_p.clone())?), (Py::str("directrix"), py::sub(&v_k.clone(), &v_p.clone())?), (Py::str("vertex_x"), v_h.clone()), (Py::str("vertex_y"), v_k.clone())]));
    }
    return Ok(Py::dict(vec![(Py::str("p"), v_p.clone()), (Py::str("focus_x"), py::add(&v_h.clone(), &v_p.clone())?), (Py::str("focus_y"), v_k.clone()), (Py::str("directrix"), py::sub(&v_h.clone(), &v_p.clone())?), (Py::str("vertex_x"), v_h.clone()), (Py::str("vertex_y"), v_k.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_partial_fractions_linear(mut v_p: Py, mut v_q: Py, mut v_r1: Py, mut v_r2: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("p"), v_p.clone()), (Py::str("q"), v_q.clone()), (Py::str("r1"), v_r1.clone()), (Py::str("r2"), v_r2.clone())]))?;
    if ({ let t1113 = v_r1.clone(); Py::Bool({ let t1114 = v_r2.clone(); if py::eq(&t1113, &t1114) { true } else { false } }) }).truthy() {
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
    if ({ let t1121 = v_old.clone(); Py::Bool({ let t1122 = Py::Int(0i64); if py::le(&t1121, &t1122)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the original value must be positive (a percent change is measured from it)"))));
    }
    v_pc = py::truediv(&py::mul(&Py::Int(100i64), &py::sub(&v_new.clone(), &v_old.clone())?)?, &v_old.clone())?;
    return Ok(Py::dict(vec![(Py::str("percent_change"), v_pc.clone()), (Py::str("difference"), py::sub(&v_new.clone(), &v_old.clone())?), (Py::str("direction"), (if ({ let t1123 = v_new.clone(); Py::Bool({ let t1124 = v_old.clone(); if py::gt(&t1123, &t1124)? { true } else { false } }) }).truthy() { Py::str("increase") } else { (if ({ let t1125 = v_new.clone(); Py::Bool({ let t1126 = v_old.clone(); if py::lt(&t1125, &t1126)? { true } else { false } }) }).truthy() { Py::str("decrease") } else { Py::str("no change") }) }))]));
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
    if ({ let t1143 = v_p.clone(); Py::Bool({ let t1144 = Py::Int(0i64); if py::eq(&t1143, &t1144) { true } else { false } }) }).truthy() {
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
    if ({ let t1150 = v_left_rel.clone(); Py::Bool({ let t1151 = Py::tuple(vec![Py::str("<"), Py::str("<=")]); if !py::contains(&t1151, &t1150)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the first piece must be for x < c or x <= c"))));
    }
    v_first = (if ({ let t1152 = v_left_rel.clone(); Py::Bool({ let t1153 = Py::str("<"); if py::eq(&t1152, &t1153) { true } else { false } }) }).truthy() { { let t1154 = v_x.clone(); Py::Bool({ let t1155 = v_c.clone(); if py::lt(&t1154, &t1155)? { true } else { false } }) } } else { { let t1156 = v_x.clone(); Py::Bool({ let t1157 = v_c.clone(); if py::le(&t1156, &t1157)? { true } else { false } }) } });
    let t1258: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_v = f__fval((if (v_first.clone()).truthy() { v_e1.clone() } else { v_e2.clone() }), v_x.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1258 {
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
    if ({ let t1272 = v_unit.clone(); Py::Bool({ let t1273 = c__UNITS()?; if !py::contains(&t1273, &t1272)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angle unit is degrees or radians"))));
    }
    v_t = (if ({ let t1274 = v_unit.clone(); Py::Bool({ let t1275 = Py::str("degrees"); if py::eq(&t1274, &t1275) { true } else { false } }) }).truthy() { py::math1("radians", &v_theta.clone())? } else { v_theta.clone() });
    let t1276 = (Py::tuple(vec![py::mul(&v_r.clone(), &py::math1("cos", &v_t.clone())?)?, py::mul(&v_r.clone(), &py::math1("sin", &v_t.clone())?)?])).items()?;
    if t1276.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1276[0].clone();
    v_y = t1276[1].clone();
    v_snap = { let v_r = v_r.clone(); let v_t = v_t.clone(); let v_theta = v_theta.clone(); let v_unit = v_unit.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t1277 = py::abs(&v_v.clone())?; Py::Bool({ let t1278 = py::mul(&Py::Float(1e-15), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::lt(&t1277, &t1278)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_v.clone() }));
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
    let t1289 = (Py::tuple(vec![f__strip({ let mut out = vec![]; for t1287 in py::iter(&(v_dividend.clone()))? { let mut v_c: Py; 
v_c = t1287;
out.push(f__fr(v_c.clone())?); } Py::list(out) })?, f__strip({ let mut out = vec![]; for t1288 in py::iter(&(v_divisor.clone()))? { let mut v_c: Py; 
v_c = t1288;
out.push(f__fr(v_c.clone())?); } Py::list(out) })?])).items()?;
    if t1289.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1289[0].clone();
    v_d = t1289[1].clone();
    if ({ let t1290 = py::getitem(&v_d.clone(), &Py::Int(0i64))?; Py::Bool({ let t1291 = Py::Int(0i64); if py::eq(&t1290, &t1291) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the divisor must not be the zero polynomial"))));
    }
    if ({ let t1292 = py::len(&v_d.clone())?; Py::Bool({ let t1293 = Py::Int(2i64); if py::lt(&t1292, &t1293)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the divisor must have degree at least 1"))));
    }
    if ({ let t1294 = py::len(&v_p.clone())?; Py::Bool({ let t1295 = py::len(&v_d.clone())?; if py::lt(&t1294, &t1295)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("quotient"), Py::list(vec![Py::Float(0.0)])), (Py::str("remainder"), { let mut out = vec![]; for t1296 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t1296;
out.push(py::float(&v_v.clone())?); } Py::list(out) })]));
    }
    let t1297 = (Py::tuple(vec![Py::list((v_p.clone()).items()?), Py::list(vec![])])).items()?;
    if t1297.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rem = t1297[0].clone();
    v_q = t1297[1].clone();
    for t1298 in py::range_iter(&[(py::add(&py::sub(&py::len(&v_p.clone())?, &py::len(&v_d.clone())?)?, &Py::Int(1i64))?).i()?]) {
        v_k = t1298;
        v_f = py::truediv(&py::getitem(&v_rem.clone(), &v_k.clone())?, &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?;
        let _ = py::method(&v_q.clone(), "append", &[v_f.clone()])?;
        for t1299 in py::range_iter(&[(py::len(&v_d.clone())?).i()?]) {
            v_j = t1299;
            let t1300 = v_rem.clone(); let t1301 = py::add(&v_k.clone(), &v_j.clone())?;
            py::setitem(&t1300, &t1301, py::sub(&py::getitem(&t1300, &t1301)?, &py::mul(&v_f.clone(), &py::getitem(&v_d.clone(), &v_j.clone())?)?)?)?;
        }
    }
    v_r = { let t1302 = f__strip(py::slice(&v_rem.clone(), Some((py::add(&py::sub(&py::len(&v_p.clone())?, &py::len(&v_d.clone())?)?, &Py::Int(1i64))?).i()?), None, None)?)?; if t1302.truthy() { t1302 } else { Py::list(vec![py::fraction(&[Py::Int(0i64)])?]) } };
    return Ok(Py::dict(vec![(Py::str("quotient"), { let mut out = vec![]; for t1303 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t1303;
out.push(py::float(&v_v.clone())?); } Py::list(out) }), (Py::str("remainder"), { let mut out = vec![]; for t1304 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t1304;
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
    let t1328 = (f__prop_terms(v_p.clone(), v_q.clone(), v_r.clone(), v_position.clone())?).items()?;
    if t1328.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t1328[0].clone();
    v_d1 = t1328[1].clone();
    v_n2 = t1328[2].clone();
    v_d2 = t1328[3].clone();
    if ({ let t1333 = { let t1329 = v_d1.clone(); Py::Bool({ let t1330 = Py::Int(0i64); if py::eq(&t1329, &t1330) { true } else { false } }) }; if t1333.truthy() { t1333 } else { { let t1331 = v_d2.clone(); Py::Bool({ let t1332 = Py::Int(0i64); if py::eq(&t1331, &t1332) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a denominator must not be 0"))));
    }
    v_k = py::method(&c__POSITIONS()?, "index", &[v_position.clone()])?;
    if ({ let t1334 = v_k.clone(); Py::Bool({ let t1335 = Py::Int(0i64); if py::eq(&t1334, &t1335) { true } else { false } }) }).truthy() {
        v_x = py::truediv(&py::mul(&v_n2.clone(), &v_d1.clone())?, &v_d2.clone())?;
    } else {
        if ({ let t1336 = v_k.clone(); Py::Bool({ let t1337 = Py::Int(1i64); if py::eq(&t1336, &t1337) { true } else { false } }) }).truthy() {
            if ({ let t1338 = v_n2.clone(); Py::Bool({ let t1339 = Py::Int(0i64); if py::eq(&t1338, &t1339) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("no solution: the unknown denominator would have to make a nonzero ratio equal to 0"))));
            }
            v_x = py::truediv(&py::mul(&v_n1.clone(), &v_d2.clone())?, &v_n2.clone())?;
        } else {
            if ({ let t1340 = v_k.clone(); Py::Bool({ let t1341 = Py::Int(2i64); if py::eq(&t1340, &t1341) { true } else { false } }) }).truthy() {
                v_x = py::truediv(&py::mul(&v_n1.clone(), &v_d2.clone())?, &v_d1.clone())?;
            } else {
                if ({ let t1342 = v_n1.clone(); Py::Bool({ let t1343 = Py::Int(0i64); if py::eq(&t1342, &t1343) { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("no solution: the unknown denominator would have to make a nonzero ratio equal to 0"))));
                }
                v_x = py::truediv(&py::mul(&v_n2.clone(), &v_d1.clone())?, &v_n1.clone())?;
            }
        }
    }
    if ({ let t1348 = { let t1344 = v_k.clone(); Py::Bool({ let t1345 = Py::tuple(vec![Py::Int(1i64), Py::Int(3i64)]); if py::contains(&t1345, &t1344)? { true } else { false } }) }; if !t1348.truthy() { t1348 } else { { let t1346 = v_x.clone(); Py::Bool({ let t1347 = Py::Int(0i64); if py::eq(&t1346, &t1347) { true } else { false } }) } } }).truthy() {
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
    if ({ let t1364 = v_a.clone(); Py::Bool({ let t1365 = Py::Int(0i64); if py::eq(&t1364, &t1365) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0 (not a quadratic)"))));
    }
    v_disc = py::sub(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_a.clone())?, &v_c.clone())?)?;
    if ({ let t1366 = v_disc.clone(); Py::Bool({ let t1367 = Py::Int(0i64); if py::le(&t1366, &t1367)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("refused: the quadratic has no two distinct real roots (this capability answers that case only)"))));
    }
    v_q = py::truediv(&py::neg(&py::add(&v_b.clone(), &py::copysign(&py::math1("sqrt", &v_disc.clone())?, &v_b.clone())?)?)?, &Py::Int(2i64))?;
    let t1368 = (py::sorted(&Py::list(vec![py::truediv(&v_q.clone(), &v_a.clone())?, py::truediv(&v_c.clone(), &v_q.clone())?]), (Py::Bool(false)).truthy())?).items()?;
    if t1368.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t1368[0].clone();
    v_r2 = t1368[1].clone();
    v_between = { let t1369 = { let t1371 = v_rel.clone(); Py::Bool({ let t1372 = Py::tuple(vec![Py::str("<"), Py::str("<=")]); if py::contains(&t1372, &t1371)? { true } else { false } }) }; Py::Bool({ let t1370 = { let t1373 = v_a.clone(); Py::Bool({ let t1374 = Py::Int(0i64); if py::gt(&t1373, &t1374)? { true } else { false } }) }; if py::eq(&t1369, &t1370) { true } else { false } }) };
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
    if ({ let t1399 = v_disc.clone(); Py::Bool({ let t1400 = Py::Int(0i64); if py::gt(&t1399, &t1400)? { true } else { false } }) }).truthy() {
        v_q = py::truediv(&py::neg(&py::add(&v_b.clone(), &py::copysign(&py::math1("sqrt", &v_disc.clone())?, &v_b.clone())?)?)?, &Py::Int(2i64))?;
        v_r = (if ({ let t1401 = v_q.clone(); Py::Bool({ let t1402 = Py::Int(0i64); if py::ne(&t1401, &t1402) { true } else { false } }) }).truthy() { py::sorted(&Py::list(vec![py::truediv(&v_q.clone(), &v_a.clone())?, py::truediv(&v_c.clone(), &v_q.clone())?]), (Py::Bool(false)).truthy())? } else { Py::list(vec![Py::Float(0.0), Py::Float(0.0)]) });
        return Ok(Py::dict(vec![(Py::str("x1"), py::getitem(&v_r.clone(), &Py::Int(0i64))?), (Py::str("x2"), py::getitem(&v_r.clone(), &Py::Int(1i64))?), (Py::str("discriminant"), v_disc.clone())]));
    }
    if ({ let t1403 = v_disc.clone(); Py::Bool({ let t1404 = Py::Int(0i64); if py::eq(&t1403, &t1404) { true } else { false } }) }).truthy() {
        v_x = py::truediv(&py::neg(&v_b.clone())?, &py::mul(&Py::Int(2i64), &v_a.clone())?)?;
        return Ok(Py::dict(vec![(Py::str("x1"), v_x.clone()), (Py::str("x2"), v_x.clone()), (Py::str("discriminant"), v_disc.clone())]));
    }
    let t1405 = (Py::tuple(vec![py::add(&py::truediv(&py::neg(&v_b.clone())?, &py::mul(&Py::Int(2i64), &v_a.clone())?)?, &Py::Float(0.0))?, py::truediv(&py::math1("sqrt", &py::neg(&v_disc.clone())?)?, &py::mul(&Py::Int(2i64), &py::abs(&v_a.clone())?)?)?])).items()?;
    if t1405.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t1405[0].clone();
    v_im = t1405[1].clone();
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
    if ({ let t1436 = py::abs(&py::sub(&v_d.clone(), &v_snapped.clone())?)?; Py::Bool({ let t1437 = py::mul(&Py::Float(1e-11), &py::minmax(&[Py::Float(1.0), py::abs(&v_d.clone())?], true)?)?; if py::le(&t1436, &t1437)? { true } else { false } }) }).truthy() {
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
    let t1444 = ({ let mut out = vec![]; for t1443 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()])))? { let mut v_v: Py; 
v_v = t1443;
out.push(f__fr(v_v.clone())?); } Py::list(out) }).items()?;
    if t1444.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A0 = t1444[0].clone();
    v_B0 = t1444[1].clone();
    v_C0 = t1444[2].clone();
    v_D0 = t1444[3].clone();
    if ({ let t1445 = v_A0.clone(); Py::Bool({ let t1446 = Py::Int(0i64); if py::eq(&t1445, &t1446) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0 (no x under the root)"))));
    }
    let t1447 = (Py::tuple(vec![py::mul(&v_C0.clone(), &v_C0.clone())?, py::sub(&py::mul(&py::mul(&Py::Int(2i64), &v_C0.clone())?, &v_D0.clone())?, &v_A0.clone())?, py::sub(&py::mul(&v_D0.clone(), &v_D0.clone())?, &v_B0.clone())?])).items()?;
    if t1447.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_qa = t1447[0].clone();
    v_qb = t1447[1].clone();
    v_qc = t1447[2].clone();
    v_cands = Py::list(vec![]);
    if ({ let t1448 = v_qa.clone(); Py::Bool({ let t1449 = Py::Int(0i64); if py::eq(&t1448, &t1449) { true } else { false } }) }).truthy() {
        v_x = py::truediv(&py::neg(&v_qc.clone())?, &v_qb.clone())?;
        let _ = py::method(&v_cands.clone(), "append", &[v_x.clone()])?;
        v_valid = (if ({ let t1450 = py::add(&py::mul(&v_C0.clone(), &v_x.clone())?, &v_D0.clone())?; Py::Bool({ let t1451 = Py::Int(0i64); if py::ge(&t1450, &t1451)? { true } else { false } }) }).truthy() { Py::list(vec![v_x.clone()]) } else { Py::list(vec![]) });
        v_bad = (if (v_valid.clone()).truthy() { Py::list(vec![]) } else { Py::list(vec![v_x.clone()]) });
        let t1454 = (Py::tuple(vec![{ let mut out = vec![]; for t1452 in py::iter(&(v_valid.clone()))? { let mut v_v: Py; 
v_v = t1452;
out.push(py::float(&v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t1453 in py::iter(&(v_bad.clone()))? { let mut v_v: Py; 
v_v = t1453;
out.push(py::float(&v_v.clone())?); } Py::list(out) }])).items()?;
        if t1454.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sols = t1454[0].clone();
        v_ext = t1454[1].clone();
    } else {
        v_disc = py::sub(&py::mul(&v_qb.clone(), &v_qb.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_qa.clone())?, &v_qc.clone())?)?;
        let t1455 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
        if t1455.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sols = t1455[0].clone();
        v_ext = t1455[1].clone();
        if ({ let t1456 = v_disc.clone(); Py::Bool({ let t1457 = Py::Int(0i64); if py::ge(&t1456, &t1457)? { true } else { false } }) }).truthy() {
            let t1458 = (Py::tuple(vec![py::isqrt(&py::numerator(&v_disc.clone())?)?, py::isqrt(&py::denominator(&v_disc.clone())?)?])).items()?;
            if t1458.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_n = t1458[0].clone();
            v_m = t1458[1].clone();
            for t1459 in py::iter(&(Py::tuple(vec![py::neg(&Py::Int(1i64))?, Py::Int(1i64)])))? {
                v_s = t1459;
                if ({ let t1464 = { let t1460 = py::mul(&v_n.clone(), &v_n.clone())?; Py::Bool({ let t1461 = py::numerator(&v_disc.clone())?; if py::eq(&t1460, &t1461) { true } else { false } }) }; if !t1464.truthy() { t1464 } else { { let t1462 = py::mul(&v_m.clone(), &v_m.clone())?; Py::Bool({ let t1463 = py::denominator(&v_disc.clone())?; if py::eq(&t1462, &t1463) { true } else { false } }) } } }).truthy() {
                    v_x = py::truediv(&py::add(&py::neg(&v_qb.clone())?, &py::mul(&v_s.clone(), &py::fraction(&[v_n.clone(), v_m.clone()])?)?)?, &py::mul(&Py::Int(2i64), &v_qa.clone())?)?;
                    v_ok = { let t1465 = py::add(&py::mul(&v_C0.clone(), &v_x.clone())?, &v_D0.clone())?; Py::Bool({ let t1466 = Py::Int(0i64); if py::ge(&t1465, &t1466)? { true } else { false } }) };
                    v_xv = py::float(&v_x.clone())?;
                } else {
                    v_ok = { let t1467 = f__sign_root_expr(py::add(&py::mul(&py::neg(&v_C0.clone())?, &v_qb.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_qa.clone())?, &v_D0.clone())?)?, py::mul(&v_s.clone(), &v_C0.clone())?, v_disc.clone())?; Py::Bool({ let t1468 = Py::Int(0i64); if py::ge(&t1467, &t1468)? { true } else { false } }) };
                    v_xv = py::truediv(&py::add(&py::float(&py::neg(&v_qb.clone())?)?, &py::mul(&v_s.clone(), &py::math1("sqrt", &py::float(&v_disc.clone())?)?)?)?, &py::float(&py::mul(&Py::Int(2i64), &v_qa.clone())?)?)?;
                }
                let _ = py::method(&(if (v_ok.clone()).truthy() { v_sols.clone() } else { v_ext.clone() }), "append", &[v_xv.clone()])?;
            }
            if ({ let t1500 = v_disc.clone(); Py::Bool({ let t1501 = Py::Int(0i64); if py::eq(&t1500, &t1501) { true } else { false } }) }).truthy() {
                let t1502 = (Py::tuple(vec![py::sorted(&py::set(&v_sols.clone())?, (Py::Bool(false)).truthy())?, py::sorted(&py::set(&v_ext.clone())?, (Py::Bool(false)).truthy())?])).items()?;
                if t1502.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_sols = t1502[0].clone();
                v_ext = t1502[1].clone();
            }
        }
    }
    let t1503 = (Py::tuple(vec![py::sorted(&v_sols.clone(), (Py::Bool(false)).truthy())?, py::sorted(&v_ext.clone(), (Py::Bool(false)).truthy())?])).items()?;
    if t1503.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sols = t1503[0].clone();
    v_ext = t1503[1].clone();
    return Ok(Py::dict(vec![(Py::str("count"), py::float(&py::len(&v_sols.clone())?)?), (Py::str("solutions"), v_sols.clone()), (Py::str("extraneous"), v_ext.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ratio_split(mut v_total: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_parts: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("total"), v_total.clone()), (Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone())]))?;
    v_parts = py::add(&Py::list(vec![v_a.clone(), v_b.clone()]), &(if ({ let t1525 = v_c.clone(); Py::Bool({ let t1526 = Py::None; if !py::is(&t1525, &t1526) { true } else { false } }) }).truthy() { Py::list(vec![v_c.clone()]) } else { Py::list(vec![]) }))?;
    if (Py::Bool(({ let mut out = vec![]; for t1527 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t1527;
out.push({ let t1530 = py::not(&py::math1("isfinite", &v_p.clone())?); if t1530.truthy() { t1530 } else { { let t1528 = v_p.clone(); Py::Bool({ let t1529 = Py::Int(0i64); if py::le(&t1528, &t1529)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every term of the ratio must be positive"))));
    }
    v_s = py::sum(&v_parts.clone(), None)?;
    return Ok({ let mut out = vec![]; for t1531 in py::iter(&(py::enumerate(&v_parts.clone(), 0)?))? { let mut v_i: Py; let mut v_p: Py; 
let t1532 = (t1531).items()?;
if t1532.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1532[0].clone();
v_p = t1532[1].clone();
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
    let t1546 = ({ let mut out = vec![]; for t1545 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()])))? { let mut v_v: Py; 
v_v = t1545;
out.push(f__fr(v_v.clone())?); } Py::list(out) }).items()?;
    if t1546.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1546[0].clone();
    v_B = t1546[1].clone();
    v_C = t1546[2].clone();
    v_D = t1546[3].clone();
    if ({ let t1547 = v_A.clone(); Py::Bool({ let t1548 = v_C.clone(); if py::eq(&t1547, &t1548) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no unique solution (equal numerators: no solution or the same expression on both sides)"))));
    }
    v_x = py::truediv(&py::sub(&py::mul(&v_C.clone(), &v_B.clone())?, &py::mul(&v_A.clone(), &v_D.clone())?)?, &py::sub(&v_A.clone(), &v_C.clone())?)?;
    if ({ let t1553 = { let t1549 = v_x.clone(); Py::Bool({ let t1550 = py::neg(&v_B.clone())?; if py::eq(&t1549, &t1550) { true } else { false } }) }; if t1553.truthy() { t1553 } else { { let t1551 = v_x.clone(); Py::Bool({ let t1552 = py::neg(&v_D.clone())?; if py::eq(&t1551, &t1552) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["no solution: the only candidate x = ".to_string(), py::format(&py::float(&v_x.clone())?, "g")?, " makes a denominator 0 (extraneous)".to_string()].concat()))));
    }
    return Ok(Py::dict(vec![(Py::str("x"), py::float(&v_x.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rational_function_analysis(mut v_numerator: Py, mut v_denominator: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_Dr: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_Nr: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_dn: Py = Py::None;
    let mut v_drest: Py = Py::None;
    let mut v_droots: Py = Py::None;
    let mut v_excluded: Py = Py::None;
    let mut v_ha: Py = Py::None;
    let mut v_holes: Py = Py::None;
    let mut v_md: Py = Py::None;
    let mut v_mn: Py = Py::None;
    let mut v_nrest: Py = Py::None;
    let mut v_nroots: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_slant: Py = Py::None;
    let mut v_vas: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xints: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_y0: Py = Py::None;
    v_N = f__cg_whole_coeffs(v_numerator.clone(), Py::str("numerator"))?;
    v_D = f__cg_whole_coeffs(v_denominator.clone(), Py::str("denominator"))?;
    if ({ let t1575 = { let t1571 = py::len(&v_N.clone())?; Py::Bool({ let t1572 = Py::Int(4i64); if py::gt(&t1571, &t1572)? { true } else { false } }) }; if t1575.truthy() { t1575 } else { { let t1573 = py::len(&v_D.clone())?; Py::Bool({ let t1574 = Py::Int(4i64); if py::gt(&t1573, &t1574)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("numerator and denominator of degree up to 3 are covered"))));
    }
    if ({ let t1580 = { let t1576 = py::len(&v_N.clone())?; Py::Bool({ let t1577 = Py::Int(1i64); if py::eq(&t1576, &t1577) { true } else { false } }) }; if !t1580.truthy() { t1580 } else { { let t1578 = py::getitem(&v_N.clone(), &Py::Int(0i64))?; Py::Bool({ let t1579 = Py::Int(0i64); if py::eq(&t1578, &t1579) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the numerator is the zero polynomial"))));
    }
    if ({ let t1581 = py::len(&v_D.clone())?; Py::Bool({ let t1582 = Py::Int(2i64); if py::lt(&t1581, &t1582)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the denominator must contain x (degree 1 to 3)"))));
    }
    let t1620 = (f__cg_rational_roots(v_D.clone())?).items()?;
    if t1620.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_droots = t1620[0].clone();
    v_drest = t1620[1].clone();
    if ({ let t1628 = { let t1621 = py::len(&v_drest.clone())?; Py::Bool({ let t1622 = Py::Int(4i64); if py::eq(&t1621, &t1622) { true } else { false } }) }; if t1628.truthy() { t1628 } else { { let t1627 = { let t1623 = py::len(&v_drest.clone())?; Py::Bool({ let t1624 = Py::Int(3i64); if py::eq(&t1623, &t1624) { true } else { false } }) }; if !t1627.truthy() { t1627 } else { { let t1625 = py::sub(&py::mul(&py::getitem(&v_drest.clone(), &Py::Int(1i64))?, &py::getitem(&v_drest.clone(), &Py::Int(1i64))?)?, &py::mul(&py::mul(&Py::Int(4i64), &py::getitem(&v_drest.clone(), &Py::Int(0i64))?)?, &py::getitem(&v_drest.clone(), &Py::Int(2i64))?)?)?; Py::Bool({ let t1626 = Py::Int(0i64); if py::ge(&t1625, &t1626)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the denominator has irrational real zeros; covered: denominators whose real zeros are all rational"))));
    }
    let t1629 = (f__cg_rational_roots(v_N.clone())?).items()?;
    if t1629.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nroots = t1629[0].clone();
    v_nrest = t1629[1].clone();
    let t1630 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t1630.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_excluded = t1630[0].clone();
    v_holes = t1630[1].clone();
    v_vas = t1630[2].clone();
    v_xints = t1630[3].clone();
    for t1631 in py::iter(&(v_droots.clone()))? {
        v_r = t1631;
        let t1632 = (Py::tuple(vec![py::getitem(&v_r.clone(), &Py::Int(0i64))?, py::getitem(&v_r.clone(), &Py::Int(1i64))?, py::getitem(&v_r.clone(), &Py::Int(2i64))?])).items()?;
        if t1632.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_p = t1632[0].clone();
        v_q = t1632[1].clone();
        v_md = t1632[2].clone();
        v_x = py::add(&py::truediv(&v_p.clone(), &v_q.clone())?, &Py::Float(0.0))?;
        let _ = py::method(&v_excluded.clone(), "append", &[v_x.clone()])?;
        v_mn = f__cg_mult(v_nroots.clone(), v_p.clone(), v_q.clone())?;
        if ({ let t1639 = v_mn.clone(); Py::Bool({ let t1640 = v_md.clone(); if py::ge(&t1639, &t1640)? { true } else { false } }) }).truthy() {
            if ({ let t1641 = v_mn.clone(); Py::Bool({ let t1642 = v_md.clone(); if py::gt(&t1641, &t1642)? { true } else { false } }) }).truthy() {
                v_y = Py::Float(0.0);
            } else {
                let t1643 = (Py::tuple(vec![v_N.clone(), v_D.clone()])).items()?;
                if t1643.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_Nr = t1643[0].clone();
                v_Dr = t1643[1].clone();
                for t1644 in py::range_iter(&[(v_md.clone()).i()?]) {
                    v__ = t1644;
                    v_Nr = f__cg_divide_root(v_Nr.clone(), v_p.clone(), v_q.clone())?;
                    v_Dr = f__cg_divide_root(v_Dr.clone(), v_p.clone(), v_q.clone())?;
                }
                v_y = py::truediv(&py::mul(&f__cg_heval(v_Nr.clone(), v_p.clone(), v_q.clone())?, &py::pow(&v_q.clone(), &py::sub(&py::len(&v_Dr.clone())?, &Py::Int(1i64))?)?)?, &py::mul(&f__cg_heval(v_Dr.clone(), v_p.clone(), v_q.clone())?, &py::pow(&v_q.clone(), &py::sub(&py::len(&v_Nr.clone())?, &Py::Int(1i64))?)?)?)?;
            }
            let _ = py::method(&v_holes.clone(), "append", &[Py::list(vec![v_x.clone(), py::add(&v_y.clone(), &Py::Float(0.0))?])])?;
        } else {
            let _ = py::method(&v_vas.clone(), "append", &[v_x.clone()])?;
        }
    }
    for t1645 in py::iter(&(v_nroots.clone()))? {
        v_r = t1645;
        if ({ let t1646 = f__cg_mult(v_droots.clone(), py::getitem(&v_r.clone(), &Py::Int(0i64))?, py::getitem(&v_r.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1647 = Py::Int(0i64); if py::eq(&t1646, &t1647) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_xints.clone(), "append", &[py::add(&py::truediv(&py::getitem(&v_r.clone(), &Py::Int(0i64))?, &py::getitem(&v_r.clone(), &Py::Int(1i64))?)?, &Py::Float(0.0))?])?;
        }
    }
    for t1689 in py::iter(&(f__cg_irrational_real_roots(v_nrest.clone())?))? {
        v_x = t1689;
        let _ = py::method(&v_xints.clone(), "append", &[py::add(&v_x.clone(), &Py::Float(0.0))?])?;
    }
    v_xints = f__cg_sorted(v_xints.clone())?;
    v_y0 = Py::None;
    if ({ let t1690 = py::getitem(&v_D.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t1691 = Py::Int(0i64); if py::ne(&t1690, &t1691) { true } else { false } }) }).truthy() {
        v_y0 = py::add(&py::truediv(&py::getitem(&v_N.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_D.clone(), &py::neg(&Py::Int(1i64))?)?)?, &Py::Float(0.0))?;
    }
    let t1692 = (Py::tuple(vec![py::sub(&py::len(&v_N.clone())?, &Py::Int(1i64))?, py::sub(&py::len(&v_D.clone())?, &Py::Int(1i64))?])).items()?;
    if t1692.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dn = t1692[0].clone();
    v_dd = t1692[1].clone();
    v_ha = Py::None;
    v_slant = Py::None;
    if ({ let t1693 = v_dn.clone(); Py::Bool({ let t1694 = v_dd.clone(); if py::lt(&t1693, &t1694)? { true } else { false } }) }).truthy() {
        v_ha = Py::Float(0.0);
    } else {
        if ({ let t1695 = v_dn.clone(); Py::Bool({ let t1696 = v_dd.clone(); if py::eq(&t1695, &t1696) { true } else { false } }) }).truthy() {
            v_ha = py::add(&py::truediv(&py::getitem(&v_N.clone(), &Py::Int(0i64))?, &py::getitem(&v_D.clone(), &Py::Int(0i64))?)?, &Py::Float(0.0))?;
        } else {
            if ({ let t1697 = v_dn.clone(); Py::Bool({ let t1698 = py::add(&v_dd.clone(), &Py::Int(1i64))?; if py::eq(&t1697, &t1698) { true } else { false } }) }).truthy() {
                v_slant = Py::list(vec![py::add(&py::truediv(&py::getitem(&v_N.clone(), &Py::Int(0i64))?, &py::getitem(&v_D.clone(), &Py::Int(0i64))?)?, &Py::Float(0.0))?, py::add(&py::truediv(&py::sub(&py::mul(&py::getitem(&v_N.clone(), &Py::Int(1i64))?, &py::getitem(&v_D.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_N.clone(), &Py::Int(0i64))?, &py::getitem(&v_D.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_D.clone(), &Py::Int(0i64))?, &py::getitem(&v_D.clone(), &Py::Int(0i64))?)?)?, &Py::Float(0.0))?]);
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("vertical_count"), py::float(&py::len(&v_vas.clone())?)?), (Py::str("excluded"), v_excluded.clone()), (Py::str("holes"), v_holes.clone()), (Py::str("vertical_asymptotes"), v_vas.clone()), (Py::str("x_intercepts"), v_xints.clone()), (Py::str("y_intercept"), v_y0.clone()), (Py::str("horizontal_asymptote"), v_ha.clone()), (Py::str("slant_asymptote"), v_slant.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rational_roots(mut v_coefficients: Py) -> R {
    let mut v_co: Py = Py::None;
    let mut v_roots: Py = Py::None;
    v_co = f__strip({ let mut out = vec![]; for t1779 in py::iter(&(v_coefficients.clone()))? { let mut v_c: Py; 
v_c = t1779;
out.push(py::float(&v_c.clone())?); } Py::list(out) })?;
    if (Py::Bool(({ let mut out = vec![]; for t1780 in py::iter(&(v_co.clone()))? { let mut v_c: Py; 
v_c = t1780;
out.push({ let t1781 = v_c.clone(); Py::Bool({ let t1782 = py::int(&v_c.clone())?; if py::ne(&t1781, &t1782) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the coefficients must be whole numbers"))));
    }
    v_co = { let mut out = vec![]; for t1783 in py::iter(&(v_co.clone()))? { let mut v_c: Py; 
v_c = t1783;
out.push(py::int(&v_c.clone())?); } Py::list(out) };
    if ({ let t1784 = py::len(&v_co.clone())?; Py::Bool({ let t1785 = Py::Int(2i64); if py::lt(&t1784, &t1785)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the polynomial must have degree at least 1"))));
    }
    if ({ let t1786 = py::minmax(&[{ let mut out = vec![]; for t1788 in py::iter(&(v_co.clone()))? { let mut v_c: Py; 
v_c = t1788;
out.push(py::abs(&v_c.clone())?); } Py::list(out) }], true)?; Py::Bool({ let t1787 = py::pow(&Py::Int(10i64), &Py::Int(7i64))?; if py::gt(&t1786, &t1787)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("coefficients are limited to 10^7 in size"))));
    }
    v_roots = f__candidates_roots(v_co.clone())?;
    return Ok(Py::dict(vec![(Py::str("count"), py::float(&py::len(&v_roots.clone())?)?), (Py::str("roots"), { let mut out = vec![]; for t1808 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t1808;
out.push(py::float(&v_r.clone())?); } Py::list(out) }), (Py::str("roots_exact"), { let mut out = vec![]; for t1809 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t1809;
out.push(Py::str(&py::to_str(&v_r.clone()))); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rectangular_to_polar(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("x"), v_x.clone()), (Py::str("y"), v_y.clone())]))?;
    if ({ let t1863 = { let t1859 = v_x.clone(); Py::Bool({ let t1860 = Py::Int(0i64); if py::eq(&t1859, &t1860) { true } else { false } }) }; if !t1863.truthy() { t1863 } else { { let t1861 = v_y.clone(); Py::Bool({ let t1862 = Py::Int(0i64); if py::eq(&t1861, &t1862) { true } else { false } }) } } }).truthy() {
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
    if (py::not(&{ let t1874 = Py::Int(0i64); Py::Bool({ let t1875 = v_A.clone(); if py::lt(&t1874, &t1875)? { { let t1876 = Py::Int(90i64); if py::lt(&t1875, &t1876)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the acute angle must be between 0 and 90 degrees"))));
    }
    if ({ let t1877 = v_side.clone(); Py::Bool({ let t1878 = Py::Int(0i64); if py::le(&t1877, &t1878)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the side must be positive"))));
    }
    v_t = py::math1("radians", &v_A.clone())?;
    if ({ let t1879 = v_which.clone(); Py::Bool({ let t1880 = Py::str("hypotenuse"); if py::eq(&t1879, &t1880) { true } else { false } }) }).truthy() {
        v_c = v_side.clone();
        let t1881 = (Py::tuple(vec![py::mul(&v_c.clone(), &py::math1("sin", &v_t.clone())?)?, py::mul(&v_c.clone(), &py::math1("cos", &v_t.clone())?)?])).items()?;
        if t1881.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1881[0].clone();
        v_b = t1881[1].clone();
    } else {
        if ({ let t1882 = v_which.clone(); Py::Bool({ let t1883 = Py::str("opposite"); if py::eq(&t1882, &t1883) { true } else { false } }) }).truthy() {
            v_a = v_side.clone();
            let t1884 = (Py::tuple(vec![py::truediv(&v_a.clone(), &py::math1("sin", &v_t.clone())?)?, py::truediv(&v_a.clone(), &py::math1("tan", &v_t.clone())?)?])).items()?;
            if t1884.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t1884[0].clone();
            v_b = t1884[1].clone();
        } else {
            v_b = v_side.clone();
            let t1885 = (Py::tuple(vec![py::truediv(&v_b.clone(), &py::math1("cos", &v_t.clone())?)?, py::mul(&v_b.clone(), &py::math1("tan", &v_t.clone())?)?])).items()?;
            if t1885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t1885[0].clone();
            v_a = t1885[1].clone();
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
    if (py::not(&{ let t1892 = Py::Int(0i64); Py::Bool({ let t1893 = v_a.clone(); if py::lt(&t1892, &t1893)? { { let t1894 = v_c.clone(); if py::lt(&t1893, &t1894)? { true } else { false } } } else { false } }) })).truthy() {
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
    if ({ let t1904 = { let t1900 = v_a.clone(); Py::Bool({ let t1901 = Py::Int(0i64); if py::le(&t1900, &t1901)? { true } else { false } }) }; if t1904.truthy() { t1904 } else { { let t1902 = v_b.clone(); Py::Bool({ let t1903 = Py::Int(0i64); if py::le(&t1902, &t1903)? { true } else { false } }) } } }).truthy() {
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
    if (py::not(&{ let t1910 = py::neg(&Py::Int(9i64))?; Py::Bool({ let t1911 = v_n.clone(); if py::le(&t1910, &t1911)? { { let t1912 = Py::Int(12i64); if py::le(&t1911, &t1912)? { true } else { false } } } else { false } }) })).truthy() {
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
    if ({ let t1932 = v_x.clone(); Py::Bool({ let t1933 = Py::Int(0i64); if py::eq(&t1932, &t1933) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("0 has no significant figures to round to"))));
    }
    if (py::not(&{ let t1934 = Py::Int(1i64); Py::Bool({ let t1935 = v_n.clone(); if py::le(&t1934, &t1935)? { { let t1936 = Py::Int(15i64); if py::le(&t1935, &t1936)? { true } else { false } } } else { false } }) })).truthy() {
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
    if ({ let t1958 = v_x.clone(); Py::Bool({ let t1959 = Py::Int(0i64); if py::eq(&t1958, &t1959) { true } else { false } }) }).truthy() {
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
    v_co = f__strip({ let mut out = vec![]; for t1976 in py::iter(&(v_coefficients.clone()))? { let mut v_c: Py; 
v_c = t1976;
out.push(f__fr(v_c.clone())?); } Py::list(out) })?;
    let _ = f__finite(Py::dict(vec![(Py::str("r"), v_r.clone())]))?;
    if ({ let t1981 = py::len(&v_co.clone())?; Py::Bool({ let t1982 = Py::Int(2i64); if py::lt(&t1981, &t1982)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the dividend must have degree at least 1"))));
    }
    v_R = f__fr(v_r.clone())?;
    v_out = Py::list(vec![py::getitem(&v_co.clone(), &Py::Int(0i64))?]);
    for t1983 in py::iter(&(py::slice(&v_co.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
        v_c = t1983;
        let _ = py::method(&v_out.clone(), "append", &[py::add(&v_c.clone(), &py::mul(&py::getitem(&v_out.clone(), &py::neg(&Py::Int(1i64))?)?, &v_R.clone())?)?])?;
    }
    return Ok(Py::dict(vec![(Py::str("quotient"), { let mut out = vec![]; for t1984 in py::iter(&(py::slice(&v_out.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?))? { let mut v_v: Py; 
v_v = t1984;
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
    if ({ let t2002 = { let t1998 = v_func.clone(); Py::Bool({ let t1999 = c__TRIG()?; if !py::contains(&t1999, &t1998)? { true } else { false } }) }; if t2002.truthy() { t2002 } else { { let t2000 = v_unit.clone(); Py::Bool({ let t2001 = c__UNITS()?; if !py::contains(&t2001, &t2000)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("func is sin, cos or tan; unit is degrees or radians"))));
    }
    if (py::not(&{ let t2003 = Py::Int(1i64); Py::Bool({ let t2004 = v_n.clone(); if py::le(&t2003, &t2004)? { { let t2005 = Py::Int(12i64); if py::le(&t2004, &t2005)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the multiple n of x must be a whole number from 1 to 12"))));
    }
    if ({ let t2006 = v_func.clone(); Py::Bool({ let t2007 = Py::str("tan"); if py::eq(&t2006, &t2007) { true } else { false } }) }).truthy() {
        v_base = Py::list(vec![py::math1("degrees", &py::math1("atan", &v_k.clone())?)?, py::add(&py::math1("degrees", &py::math1("atan", &v_k.clone())?)?, &Py::Int(180i64))?]);
    } else {
        if ({ let t2008 = py::abs(&v_k.clone())?; Py::Bool({ let t2009 = Py::Int(1i64); if py::gt(&t2008, &t2009)? { true } else { false } }) }).truthy() {
            v_base = Py::list(vec![]);
        } else {
            if ({ let t2010 = v_func.clone(); Py::Bool({ let t2011 = Py::str("sin"); if py::eq(&t2010, &t2011) { true } else { false } }) }).truthy() {
                v_a = (if ({ let t2012 = v_k.clone(); Py::Bool({ let t2013 = Py::Int(1i64); if py::eq(&t2012, &t2013) { true } else { false } }) }).truthy() { Py::Float(90.0) } else { (if ({ let t2014 = v_k.clone(); Py::Bool({ let t2015 = py::neg(&Py::Int(1i64))?; if py::eq(&t2014, &t2015) { true } else { false } }) }).truthy() { py::neg(&Py::Float(90.0))? } else { py::math1("degrees", &py::math1("asin", &v_k.clone())?)? }) });
                v_base = Py::list(vec![v_a.clone(), py::sub(&Py::Int(180i64), &v_a.clone())?]);
            } else {
                v_a = (if ({ let t2016 = v_k.clone(); Py::Bool({ let t2017 = Py::Int(1i64); if py::eq(&t2016, &t2017) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { (if ({ let t2018 = v_k.clone(); Py::Bool({ let t2019 = py::neg(&Py::Int(1i64))?; if py::eq(&t2018, &t2019) { true } else { false } }) }).truthy() { Py::Float(180.0) } else { py::math1("degrees", &py::math1("acos", &v_k.clone())?)? }) });
                v_base = Py::list(vec![v_a.clone(), py::neg(&v_a.clone())?]);
            }
        }
    }
    v_us = Py::list(vec![]);
    for t2020 in py::iter(&(v_base.clone()))? {
        v_b = t2020;
        v_v = py::modulo(&v_b.clone(), &Py::Float(360.0))?;
        if ({ let t2021 = v_v.clone(); Py::Bool({ let t2022 = py::sub(&Py::Float(360.0), &Py::Float(1e-09))?; if py::gt(&t2021, &t2022)? { true } else { false } }) }).truthy() {
            v_v = Py::Float(0.0);
        }
        if (py::not(&Py::Bool(({ let mut out = vec![]; for t2023 in py::iter(&(v_us.clone()))? { let mut v_w: Py; 
v_w = t2023;
out.push({ let t2024 = py::abs(&py::sub(&v_v.clone(), &v_w.clone())?)?; Py::Bool({ let t2025 = Py::Float(1e-09); if py::lt(&t2024, &t2025)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())))).truthy() {
            let _ = py::method(&v_us.clone(), "append", &[v_v.clone()])?;
        }
    }
    v_sols = py::sorted(&{ let mut out = vec![]; for t2026 in py::iter(&(v_us.clone()))? { let mut v_u: Py; 
v_u = t2026;
for t2027 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_m: Py; 
v_m = t2027;
out.push(py::truediv(&py::add(&v_u.clone(), &py::mul(&Py::Float(360.0), &v_m.clone())?)?, &v_n.clone())?); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
    if ({ let t2028 = v_unit.clone(); Py::Bool({ let t2029 = Py::str("radians"); if py::eq(&t2028, &t2029) { true } else { false } }) }).truthy() {
        v_sols = { let mut out = vec![]; for t2030 in py::iter(&(v_sols.clone()))? { let mut v_s: Py; 
v_s = t2030;
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
    if ({ let t2064 = { let t2060 = v_func.clone(); Py::Bool({ let t2061 = c__TRIG()?; if !py::contains(&t2061, &t2060)? { true } else { false } }) }; if t2064.truthy() { t2064 } else { { let t2062 = v_q.clone(); Py::Bool({ let t2063 = Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64), Py::Int(4i64)]); if !py::contains(&t2063, &t2062)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("func is sin, cos or tan; the quadrant is 1, 2, 3 or 4"))));
    }
    let t2069 = (Py::tuple(vec![(if ({ let t2065 = v_q.clone(); Py::Bool({ let t2066 = Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)]); if py::contains(&t2066, &t2065)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? }), (if ({ let t2067 = v_q.clone(); Py::Bool({ let t2068 = Py::tuple(vec![Py::Int(1i64), Py::Int(4i64)]); if py::contains(&t2068, &t2067)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? })])).items()?;
    if t2069.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sin_sign = t2069[0].clone();
    v_cos_sign = t2069[1].clone();
    if ({ let t2075 = { let t2070 = v_func.clone(); Py::Bool({ let t2071 = Py::tuple(vec![Py::str("sin"), Py::str("cos")]); if py::contains(&t2071, &t2070)? { true } else { false } }) }; if !t2075.truthy() { t2075 } else { py::not(&{ let t2072 = Py::Int(0i64); Py::Bool({ let t2073 = py::abs(&v_value.clone())?; if py::lt(&t2072, &t2073)? { { let t2074 = Py::Int(1i64); if py::lt(&t2073, &t2074)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a sine or cosine strictly inside a quadrant lies strictly between -1 and 1 and is not 0"))));
    }
    if ({ let t2080 = { let t2076 = v_func.clone(); Py::Bool({ let t2077 = Py::str("tan"); if py::eq(&t2076, &t2077) { true } else { false } }) }; if !t2080.truthy() { t2080 } else { { let t2078 = v_value.clone(); Py::Bool({ let t2079 = Py::Int(0i64); if py::eq(&t2078, &t2079) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("tan = 0 is on an axis, not inside a quadrant"))));
    }
    if ({ let t2081 = v_func.clone(); Py::Bool({ let t2082 = Py::str("sin"); if py::eq(&t2081, &t2082) { true } else { false } }) }).truthy() {
        if ({ let t2083 = { let t2085 = v_value.clone(); Py::Bool({ let t2086 = Py::Int(0i64); if py::gt(&t2085, &t2086)? { true } else { false } }) }; Py::Bool({ let t2084 = { let t2087 = v_sin_sign.clone(); Py::Bool({ let t2088 = Py::Int(0i64); if py::gt(&t2087, &t2088)? { true } else { false } }) }; if py::ne(&t2083, &t2084) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["sine is ".to_string(), py::format(&(if ({ let t2089 = v_sin_sign.clone(); Py::Bool({ let t2090 = Py::Int(0i64); if py::gt(&t2089, &t2090)? { true } else { false } }) }).truthy() { Py::str("positive") } else { Py::str("negative") }), "")?, " in quadrant ".to_string(), py::format(&v_q.clone(), "")?].concat()))));
        }
        let t2091 = (Py::tuple(vec![v_value.clone(), py::mul(&v_cos_sign.clone(), &py::math1("sqrt", &py::mul(&py::sub(&Py::Int(1i64), &v_value.clone())?, &py::add(&Py::Int(1i64), &v_value.clone())?)?)?)?])).items()?;
        if t2091.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s = t2091[0].clone();
        v_c = t2091[1].clone();
    } else {
        if ({ let t2092 = v_func.clone(); Py::Bool({ let t2093 = Py::str("cos"); if py::eq(&t2092, &t2093) { true } else { false } }) }).truthy() {
            if ({ let t2094 = { let t2096 = v_value.clone(); Py::Bool({ let t2097 = Py::Int(0i64); if py::gt(&t2096, &t2097)? { true } else { false } }) }; Py::Bool({ let t2095 = { let t2098 = v_cos_sign.clone(); Py::Bool({ let t2099 = Py::Int(0i64); if py::gt(&t2098, &t2099)? { true } else { false } }) }; if py::ne(&t2094, &t2095) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&["cosine is ".to_string(), py::format(&(if ({ let t2100 = v_cos_sign.clone(); Py::Bool({ let t2101 = Py::Int(0i64); if py::gt(&t2100, &t2101)? { true } else { false } }) }).truthy() { Py::str("positive") } else { Py::str("negative") }), "")?, " in quadrant ".to_string(), py::format(&v_q.clone(), "")?].concat()))));
            }
            let t2102 = (Py::tuple(vec![v_value.clone(), py::mul(&v_sin_sign.clone(), &py::math1("sqrt", &py::mul(&py::sub(&Py::Int(1i64), &v_value.clone())?, &py::add(&Py::Int(1i64), &v_value.clone())?)?)?)?])).items()?;
            if t2102.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t2102[0].clone();
            v_s = t2102[1].clone();
        } else {
            if ({ let t2103 = { let t2105 = v_value.clone(); Py::Bool({ let t2106 = Py::Int(0i64); if py::gt(&t2105, &t2106)? { true } else { false } }) }; Py::Bool({ let t2104 = { let t2107 = py::mul(&v_sin_sign.clone(), &v_cos_sign.clone())?; Py::Bool({ let t2108 = Py::Int(0i64); if py::gt(&t2107, &t2108)? { true } else { false } }) }; if py::ne(&t2103, &t2104) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&["tangent is ".to_string(), py::format(&(if ({ let t2109 = py::mul(&v_sin_sign.clone(), &v_cos_sign.clone())?; Py::Bool({ let t2110 = Py::Int(0i64); if py::gt(&t2109, &t2110)? { true } else { false } }) }).truthy() { Py::str("positive") } else { Py::str("negative") }), "")?, " in quadrant ".to_string(), py::format(&v_q.clone(), "")?].concat()))));
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
fn f_two_conics_intersection(mut v_first: Py, mut v_second: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_A1: Py = Py::None;
    let mut v_A2: Py = Py::None;
    let mut v_B1: Py = Py::None;
    let mut v_B2: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_C1: Py = Py::None;
    let mut v_C2: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_D1: Py = Py::None;
    let mut v_D2: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_E1: Py = Py::None;
    let mut v_E2: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_F1: Py = Py::None;
    let mut v_F2: Py = Py::None;
    let mut v_l1: Py = Py::None;
    let mut v_l2: Py = Py::None;
    let mut v_lf: Py = Py::None;
    let mut v_lx: Py = Py::None;
    let mut v_ly: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_snapped: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t2126 = (Py::tuple(vec![py::getitem(&v_first.clone(), &Py::Int(0i64))?, py::getitem(&v_first.clone(), &Py::Int(1i64))?, py::getitem(&v_first.clone(), &Py::Int(2i64))?, py::getitem(&v_first.clone(), &Py::Int(3i64))?, py::getitem(&v_first.clone(), &Py::Int(4i64))?, py::getitem(&v_first.clone(), &Py::Int(5i64))?])).items()?;
    if t2126.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A1 = t2126[0].clone();
    v_B1 = t2126[1].clone();
    v_C1 = t2126[2].clone();
    v_D1 = t2126[3].clone();
    v_E1 = t2126[4].clone();
    v_F1 = t2126[5].clone();
    let t2127 = (Py::tuple(vec![py::getitem(&v_second.clone(), &Py::Int(0i64))?, py::getitem(&v_second.clone(), &Py::Int(1i64))?, py::getitem(&v_second.clone(), &Py::Int(2i64))?, py::getitem(&v_second.clone(), &Py::Int(3i64))?, py::getitem(&v_second.clone(), &Py::Int(4i64))?, py::getitem(&v_second.clone(), &Py::Int(5i64))?])).items()?;
    if t2127.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A2 = t2127[0].clone();
    v_B2 = t2127[1].clone();
    v_C2 = t2127[2].clone();
    v_D2 = t2127[3].clone();
    v_E2 = t2127[4].clone();
    v_F2 = t2127[5].clone();
    if ({ let t2132 = { let t2128 = v_B1.clone(); Py::Bool({ let t2129 = Py::Int(0i64); if py::ne(&t2128, &t2129) { true } else { false } }) }; if t2132.truthy() { t2132 } else { { let t2130 = v_B2.clone(); Py::Bool({ let t2131 = Py::Int(0i64); if py::ne(&t2130, &t2131) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an xy term is not covered (rotate the axes first)"))));
    }
    if ({ let t2143 = { let t2137 = { let t2133 = v_A1.clone(); Py::Bool({ let t2134 = Py::Int(0i64); if py::eq(&t2133, &t2134) { true } else { false } }) }; if !t2137.truthy() { t2137 } else { { let t2135 = v_C1.clone(); Py::Bool({ let t2136 = Py::Int(0i64); if py::eq(&t2135, &t2136) { true } else { false } }) } } }; if t2143.truthy() { t2143 } else { { let t2142 = { let t2138 = v_A2.clone(); Py::Bool({ let t2139 = Py::Int(0i64); if py::eq(&t2138, &t2139) { true } else { false } }) }; if !t2142.truthy() { t2142 } else { { let t2140 = v_C2.clone(); Py::Bool({ let t2141 = Py::Int(0i64); if py::eq(&t2140, &t2141) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("each equation needs an x^2 or y^2 term (a line and a conic: line_conic_intersection)"))));
    }
    v_pts = Py::list(vec![]);
    if ({ let t2144 = py::sub(&py::mul(&v_A1.clone(), &v_C2.clone())?, &py::mul(&v_A2.clone(), &v_C1.clone())?)?; Py::Bool({ let t2145 = Py::Int(0i64); if py::eq(&t2144, &t2145) { true } else { false } }) }).truthy() {
        if ({ let t2150 = { let t2146 = v_A1.clone(); Py::Bool({ let t2147 = Py::Int(0i64); if py::ne(&t2146, &t2147) { true } else { false } }) }; if t2150.truthy() { t2150 } else { { let t2148 = v_A2.clone(); Py::Bool({ let t2149 = Py::Int(0i64); if py::ne(&t2148, &t2149) { true } else { false } }) } } }).truthy() {
            let t2151 = (Py::tuple(vec![v_A2.clone(), py::neg(&v_A1.clone())?])).items()?;
            if t2151.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_l1 = t2151[0].clone();
            v_l2 = t2151[1].clone();
        } else {
            let t2152 = (Py::tuple(vec![v_C2.clone(), py::neg(&v_C1.clone())?])).items()?;
            if t2152.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_l1 = t2152[0].clone();
            v_l2 = t2152[1].clone();
        }
        v_lx = py::add(&py::mul(&v_l1.clone(), &v_D1.clone())?, &py::mul(&v_l2.clone(), &v_D2.clone())?)?;
        v_ly = py::add(&py::mul(&v_l1.clone(), &v_E1.clone())?, &py::mul(&v_l2.clone(), &v_E2.clone())?)?;
        v_lf = py::add(&py::mul(&v_l1.clone(), &v_F1.clone())?, &py::mul(&v_l2.clone(), &v_F2.clone())?)?;
        if ({ let t2157 = { let t2153 = v_lx.clone(); Py::Bool({ let t2154 = Py::Int(0i64); if py::eq(&t2153, &t2154) { true } else { false } }) }; if !t2157.truthy() { t2157 } else { { let t2155 = v_ly.clone(); Py::Bool({ let t2156 = Py::Int(0i64); if py::eq(&t2155, &t2156) { true } else { false } }) } } }).truthy() {
            if ({ let t2158 = v_lf.clone(); Py::Bool({ let t2159 = Py::Int(0i64); if py::eq(&t2158, &t2159) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the two equations describe the same curve: infinitely many points"))));
            }
            return Ok(Py::dict(vec![(Py::str("count"), Py::Float(0.0)), (Py::str("points"), Py::list(vec![]))]));
        }
        return Ok(f_line_conic_intersection(Py::list(vec![v_lx.clone(), v_ly.clone(), py::neg(&v_lf.clone())?]), v_first.clone())?);
    }
    if ({ let t2213 = { let t2209 = v_D1.clone(); Py::Bool({ let t2210 = Py::Int(0i64); if py::eq(&t2209, &t2210) { true } else { false } }) }; if !t2213.truthy() { t2213 } else { { let t2211 = v_D2.clone(); Py::Bool({ let t2212 = Py::Int(0i64); if py::eq(&t2211, &t2212) { true } else { false } }) } } }).truthy() {
        if ({ let t2214 = py::abs(&v_A1.clone())?; Py::Bool({ let t2215 = py::abs(&v_A2.clone())?; if py::ge(&t2214, &t2215)? { true } else { false } }) }).truthy() {
            let t2216 = (Py::tuple(vec![v_A1.clone(), v_C1.clone(), v_E1.clone(), v_F1.clone()])).items()?;
            if t2216.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_A = t2216[0].clone();
            v_C = t2216[1].clone();
            v_E = t2216[2].clone();
            v_F = t2216[3].clone();
        } else {
            let t2217 = (Py::tuple(vec![v_A2.clone(), v_C2.clone(), v_E2.clone(), v_F2.clone()])).items()?;
            if t2217.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_A = t2217[0].clone();
            v_C = t2217[1].clone();
            v_E = t2217[2].clone();
            v_F = t2217[3].clone();
        }
        for t2218 in py::iter(&(f__cg_quadratic_real(py::sub(&py::mul(&v_A2.clone(), &v_C1.clone())?, &py::mul(&v_A1.clone(), &v_C2.clone())?)?, py::sub(&py::mul(&v_A2.clone(), &v_E1.clone())?, &py::mul(&v_A1.clone(), &v_E2.clone())?)?, py::sub(&py::mul(&v_A2.clone(), &v_F1.clone())?, &py::mul(&v_A1.clone(), &v_F2.clone())?)?)?))? {
            v_y = t2218;
            v_v = py::truediv(&py::neg(&py::add(&py::add(&py::mul(&py::mul(&v_C.clone(), &v_y.clone())?, &v_y.clone())?, &py::mul(&v_E.clone(), &v_y.clone())?)?, &v_F.clone())?)?, &v_A.clone())?;
            if ({ let t2219 = v_v.clone(); Py::Bool({ let t2220 = Py::Int(0i64); if py::eq(&t2219, &t2220) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_pts.clone(), "append", &[Py::list(vec![Py::Float(0.0), py::add(&v_y.clone(), &Py::Float(0.0))?])])?;
            } else {
                if ({ let t2221 = py::abs(&v_v.clone())?; Py::Bool({ let t2222 = py::truediv(&py::mul(&c__CG_BAND()?, &py::add(&py::add(&py::abs(&py::mul(&py::mul(&v_C.clone(), &v_y.clone())?, &v_y.clone())?)?, &py::abs(&py::mul(&v_E.clone(), &v_y.clone())?)?)?, &py::abs(&v_F.clone())?)?)?, &py::abs(&v_A.clone())?)?; if py::le(&t2221, &t2222)? { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("a tangency cannot be decided from rounded values"))));
                } else {
                    if ({ let t2223 = v_v.clone(); Py::Bool({ let t2224 = Py::Int(0i64); if py::gt(&t2223, &t2224)? { true } else { false } }) }).truthy() {
                        v_r = py::math1("sqrt", &v_v.clone())?;
                        let _ = py::method(&v_pts.clone(), "append", &[Py::list(vec![py::neg(&v_r.clone())?, py::add(&v_y.clone(), &Py::Float(0.0))?])])?;
                        let _ = py::method(&v_pts.clone(), "append", &[Py::list(vec![v_r.clone(), py::add(&v_y.clone(), &Py::Float(0.0))?])])?;
                    }
                }
            }
        }
    } else {
        if ({ let t2229 = { let t2225 = v_E1.clone(); Py::Bool({ let t2226 = Py::Int(0i64); if py::eq(&t2225, &t2226) { true } else { false } }) }; if !t2229.truthy() { t2229 } else { { let t2227 = v_E2.clone(); Py::Bool({ let t2228 = Py::Int(0i64); if py::eq(&t2227, &t2228) { true } else { false } }) } } }).truthy() {
            if ({ let t2230 = py::abs(&v_C1.clone())?; Py::Bool({ let t2231 = py::abs(&v_C2.clone())?; if py::ge(&t2230, &t2231)? { true } else { false } }) }).truthy() {
                let t2232 = (Py::tuple(vec![v_A1.clone(), v_C1.clone(), v_D1.clone(), v_F1.clone()])).items()?;
                if t2232.len() != 4 { return Err(PyErr::Value("unpack".into())); }
                v_A = t2232[0].clone();
                v_C = t2232[1].clone();
                v_D = t2232[2].clone();
                v_F = t2232[3].clone();
            } else {
                let t2233 = (Py::tuple(vec![v_A2.clone(), v_C2.clone(), v_D2.clone(), v_F2.clone()])).items()?;
                if t2233.len() != 4 { return Err(PyErr::Value("unpack".into())); }
                v_A = t2233[0].clone();
                v_C = t2233[1].clone();
                v_D = t2233[2].clone();
                v_F = t2233[3].clone();
            }
            for t2234 in py::iter(&(f__cg_quadratic_real(py::sub(&py::mul(&v_C2.clone(), &v_A1.clone())?, &py::mul(&v_C1.clone(), &v_A2.clone())?)?, py::sub(&py::mul(&v_C2.clone(), &v_D1.clone())?, &py::mul(&v_C1.clone(), &v_D2.clone())?)?, py::sub(&py::mul(&v_C2.clone(), &v_F1.clone())?, &py::mul(&v_C1.clone(), &v_F2.clone())?)?)?))? {
                v_x = t2234;
                v_v = py::truediv(&py::neg(&py::add(&py::add(&py::mul(&py::mul(&v_A.clone(), &v_x.clone())?, &v_x.clone())?, &py::mul(&v_D.clone(), &v_x.clone())?)?, &v_F.clone())?)?, &v_C.clone())?;
                if ({ let t2235 = v_v.clone(); Py::Bool({ let t2236 = Py::Int(0i64); if py::eq(&t2235, &t2236) { true } else { false } }) }).truthy() {
                    let _ = py::method(&v_pts.clone(), "append", &[Py::list(vec![py::add(&v_x.clone(), &Py::Float(0.0))?, Py::Float(0.0)])])?;
                } else {
                    if ({ let t2237 = py::abs(&v_v.clone())?; Py::Bool({ let t2238 = py::truediv(&py::mul(&c__CG_BAND()?, &py::add(&py::add(&py::abs(&py::mul(&py::mul(&v_A.clone(), &v_x.clone())?, &v_x.clone())?)?, &py::abs(&py::mul(&v_D.clone(), &v_x.clone())?)?)?, &py::abs(&v_F.clone())?)?)?, &py::abs(&v_C.clone())?)?; if py::le(&t2237, &t2238)? { true } else { false } }) }).truthy() {
                        return Err(py::value_error(py::to_str(&Py::str("a tangency cannot be decided from rounded values"))));
                    } else {
                        if ({ let t2239 = v_v.clone(); Py::Bool({ let t2240 = Py::Int(0i64); if py::gt(&t2239, &t2240)? { true } else { false } }) }).truthy() {
                            v_r = py::math1("sqrt", &v_v.clone())?;
                            let _ = py::method(&v_pts.clone(), "append", &[Py::list(vec![py::add(&v_x.clone(), &Py::Float(0.0))?, py::neg(&v_r.clone())?])])?;
                            let _ = py::method(&v_pts.clone(), "append", &[Py::list(vec![py::add(&v_x.clone(), &Py::Float(0.0))?, v_r.clone()])])?;
                        }
                    }
                }
            }
        } else {
            return Err(py::value_error(py::to_str(&Py::str("covered: proportional second-degree parts (e.g. two circles), or x (or y) appearing only squared in both"))));
        }
    }
    v_snapped = Py::list(vec![]);
    for t2241 in py::iter(&(v_pts.clone()))? {
        v_p = t2241;
        let _ = py::method(&v_snapped.clone(), "append", &[Py::list(vec![py::add(&py::float(&Py::str(&[py::format(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, ".12g")?].concat()))?, &Py::Float(0.0))?, py::add(&py::float(&Py::str(&[py::format(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, ".12g")?].concat()))?, &Py::Float(0.0))?])])?;
    }
    v_pts = f__cg_sorted_points(v_snapped.clone())?;
    return Ok(Py::dict(vec![(Py::str("count"), py::float(&py::len(&v_pts.clone())?)?), (Py::str("points"), v_pts.clone())]));
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
    if ({ let t2296 = { let t2293 = v_deg.clone(); Py::Bool({ let t2294 = py::int(&v_deg.clone())?; if py::ne(&t2293, &t2294) { true } else { false } }) }; if t2296.truthy() { t2296 } else { { let t2295 = py::modulo(&py::int(&v_deg.clone())?, &Py::Int(30i64))?; if !t2295.truthy() { t2295 } else { py::modulo(&py::int(&v_deg.clone())?, &Py::Int(45i64))? } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("exact values are given for multiples of 30 or 45 degrees (pi/6 or pi/4)"))));
    }
    v_t = py::modulo(&py::int(&v_deg.clone())?, &Py::Int(360i64))?;
    let t2303 = ((if ({ let t2297 = v_t.clone(); Py::Bool({ let t2298 = Py::Int(90i64); if py::le(&t2297, &t2298)? { true } else { false } }) }).truthy() { Py::tuple(vec![v_t.clone(), Py::Int(1i64), Py::Int(1i64)]) } else { (if ({ let t2299 = v_t.clone(); Py::Bool({ let t2300 = Py::Int(180i64); if py::le(&t2299, &t2300)? { true } else { false } }) }).truthy() { Py::tuple(vec![py::sub(&Py::Int(180i64), &v_t.clone())?, Py::Int(1i64), py::neg(&Py::Int(1i64))?]) } else { (if ({ let t2301 = v_t.clone(); Py::Bool({ let t2302 = Py::Int(270i64); if py::le(&t2301, &t2302)? { true } else { false } }) }).truthy() { Py::tuple(vec![py::sub(&v_t.clone(), &Py::Int(180i64))?, py::neg(&Py::Int(1i64))?, py::neg(&Py::Int(1i64))?]) } else { Py::tuple(vec![py::sub(&Py::Int(360i64), &v_t.clone())?, py::neg(&Py::Int(1i64))?, Py::Int(1i64)]) }) }) })).items()?;
    if t2303.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ref = t2303[0].clone();
    v_ss = t2303[1].clone();
    v_cs = t2303[2].clone();
    let t2312 = (f__signed(py::getitem(&c__EXACT_SIN()?, &v_ref.clone())?, v_ss.clone())?).items()?;
    if t2312.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_st = t2312[0].clone();
    v_sv = t2312[1].clone();
    let t2313 = (f__signed(py::getitem(&c__EXACT_SIN()?, &py::sub(&Py::Int(90i64), &v_ref.clone())?)?, v_cs.clone())?).items()?;
    if t2313.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ct = t2313[0].clone();
    v_cv = t2313[1].clone();
    let t2314 = (f__signed(py::getitem(&c__EXACT_TAN()?, &v_ref.clone())?, py::mul(&v_ss.clone(), &v_cs.clone())?)?).items()?;
    if t2314.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tt = t2314[0].clone();
    v_tv = t2314[1].clone();
    return Ok(Py::dict(vec![(Py::str("sin"), v_sv.clone()), (Py::str("cos"), v_cv.clone()), (Py::str("tan"), v_tv.clone()), (Py::str("sin_exact"), v_st.clone()), (Py::str("cos_exact"), v_ct.clone()), (Py::str("tan_exact"), v_tt.clone()), (Py::str("reference_angle"), py::float(&v_ref.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_unit_conversion(mut v_value: Py, mut v_from_unit: Py, mut v_to_unit: Py) -> R {
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let _ = f__cg_finite(v_value.clone(), Py::str("value"))?;
    if ({ let t2344 = { let t2340 = v_from_unit.clone(); Py::Bool({ let t2341 = c__CG_UNITS()?; if !py::contains(&t2341, &t2340)? { true } else { false } }) }; if t2344.truthy() { t2344 } else { { let t2342 = v_to_unit.clone(); Py::Bool({ let t2343 = c__CG_UNITS()?; if !py::contains(&t2343, &t2342)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("unknown unit"))));
    }
    v_d1 = py::getitem(&py::getitem(&c__CG_UNITS()?, &v_from_unit.clone())?, &Py::Int(0i64))?;
    v_d2 = py::getitem(&py::getitem(&c__CG_UNITS()?, &v_to_unit.clone())?, &Py::Int(0i64))?;
    if ({ let t2345 = v_d1.clone(); Py::Bool({ let t2346 = v_d2.clone(); if py::ne(&t2345, &t2346) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&Py::str("a "), &v_d1.clone())?, &Py::str(" in "))?, &v_from_unit.clone())?, &Py::str(" cannot be converted to "))?, &v_to_unit.clone())?, &Py::str(", a unit of "))?, &v_d2.clone())?)));
    }
    if ({ let t2347 = v_d1.clone(); Py::Bool({ let t2348 = Py::str("temperature"); if py::eq(&t2347, &t2348) { true } else { false } }) }).truthy() {
        if ({ let t2349 = f__cg_temperature(v_value.clone(), v_from_unit.clone(), Py::str("kelvin"))?; Py::Bool({ let t2350 = py::neg(&Py::Float(1e-09))?; if py::lt(&t2349, &t2350)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the temperature is below absolute zero"))));
        }
        return Ok(Py::dict(vec![(Py::str("converted"), py::add(&f__cg_temperature(v_value.clone(), v_from_unit.clone(), v_to_unit.clone())?, &Py::Float(0.0))?)]));
    }
    if ({ let t2378 = v_value.clone(); Py::Bool({ let t2379 = Py::Int(0i64); if py::lt(&t2378, &t2379)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a length, mass, volume or time to convert is not negative"))));
    }
    return Ok(Py::dict(vec![(Py::str("converted"), py::add(&py::truediv(&py::mul(&v_value.clone(), &py::getitem(&py::getitem(&c__CG_UNITS()?, &v_from_unit.clone())?, &Py::Int(1i64))?)?, &py::getitem(&py::getitem(&c__CG_UNITS()?, &v_to_unit.clone())?, &Py::Int(1i64))?)?, &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_unit_rate_compare(mut v_n1: Py, mut v_c1: Py, mut v_n2: Py, mut v_c2: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("n1"), v_n1.clone()), (Py::str("c1"), v_c1.clone())]))?;
    if ({ let t2392 = { let t2388 = v_n1.clone(); Py::Bool({ let t2389 = Py::Int(0i64); if py::le(&t2388, &t2389)? { true } else { false } }) }; if t2392.truthy() { t2392 } else { { let t2390 = v_c1.clone(); Py::Bool({ let t2391 = Py::Int(0i64); if py::lt(&t2390, &t2391)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("quantities must be positive and amounts not negative"))));
    }
    v_out = Py::dict(vec![(Py::str("unit_rate_1"), py::truediv(&v_c1.clone(), &v_n1.clone())?)]);
    if ({ let t2397 = { let t2393 = v_n2.clone(); Py::Bool({ let t2394 = Py::None; if py::is(&t2393, &t2394) { true } else { false } }) }; if !t2397.truthy() { t2397 } else { { let t2395 = v_c2.clone(); Py::Bool({ let t2396 = Py::None; if py::is(&t2395, &t2396) { true } else { false } }) } } }).truthy() {
        return Ok(v_out.clone());
    }
    if ({ let t2402 = { let t2398 = v_n2.clone(); Py::Bool({ let t2399 = Py::None; if py::is(&t2398, &t2399) { true } else { false } }) }; if t2402.truthy() { t2402 } else { { let t2400 = v_c2.clone(); Py::Bool({ let t2401 = Py::None; if py::is(&t2400, &t2401) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the second offer needs both its quantity and its amount"))));
    }
    let _ = f__finite(Py::dict(vec![(Py::str("n2"), v_n2.clone()), (Py::str("c2"), v_c2.clone())]))?;
    if ({ let t2407 = { let t2403 = v_n2.clone(); Py::Bool({ let t2404 = Py::Int(0i64); if py::le(&t2403, &t2404)? { true } else { false } }) }; if t2407.truthy() { t2407 } else { { let t2405 = v_c2.clone(); Py::Bool({ let t2406 = Py::Int(0i64); if py::lt(&t2405, &t2406)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("quantities must be positive and amounts not negative"))));
    }
    py::setitem(&v_out.clone(), &Py::str("unit_rate_2"), py::truediv(&v_c2.clone(), &v_n2.clone())?)?;
    let t2408 = (Py::tuple(vec![py::truediv(&py::fraction(&[Py::str(&py::repr(&py::float(&v_c1.clone())?))])?, &py::fraction(&[Py::str(&py::repr(&py::float(&v_n1.clone())?))])?)?, py::truediv(&py::fraction(&[Py::str(&py::repr(&py::float(&v_c2.clone())?))])?, &py::fraction(&[Py::str(&py::repr(&py::float(&v_n2.clone())?))])?)?])).items()?;
    if t2408.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2408[0].clone();
    v_b = t2408[1].clone();
    py::setitem(&v_out.clone(), &Py::str("better"), (if ({ let t2409 = v_a.clone(); Py::Bool({ let t2410 = v_b.clone(); if py::lt(&t2409, &t2410)? { true } else { false } }) }).truthy() { Py::str("first") } else { (if ({ let t2411 = v_b.clone(); Py::Bool({ let t2412 = v_a.clone(); if py::lt(&t2411, &t2412)? { true } else { false } }) }).truthy() { Py::str("second") } else { Py::str("equal") }) }))?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_variation(mut v_variation_type: Py, mut v_x1: Py, mut v_y1: Py, mut v_x2: Py, mut v_z1: Py, mut v_z2: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_kind: Py = Py::None;
    let mut v_two: Py = Py::None;
    v_kind = v_variation_type.clone();
    if ({ let t2424 = v_kind.clone(); Py::Bool({ let t2425 = c__CG_VARY()?; if !py::contains(&t2425, &t2424)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the kind of variation is direct, inverse, joint, combined, direct square or inverse square"))));
    }
    let _ = f__cg_finite(v_x1.clone(), Py::str("x1"))?;
    let _ = f__cg_finite(v_y1.clone(), Py::str("y1"))?;
    let _ = f__cg_finite(v_x2.clone(), Py::str("x2"))?;
    v_two = { let t2432 = { let t2428 = v_kind.clone(); Py::Bool({ let t2429 = Py::str("joint"); if py::eq(&t2428, &t2429) { true } else { false } }) }; if t2432.truthy() { t2432 } else { { let t2430 = v_kind.clone(); Py::Bool({ let t2431 = Py::str("combined"); if py::eq(&t2430, &t2431) { true } else { false } }) } } };
    if (v_two.clone()).truthy() {
        if ({ let t2437 = { let t2433 = v_z1.clone(); Py::Bool({ let t2434 = Py::None; if py::is(&t2433, &t2434) { true } else { false } }) }; if t2437.truthy() { t2437 } else { { let t2435 = v_z2.clone(); Py::Bool({ let t2436 = Py::None; if py::is(&t2435, &t2436) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("joint and combined variation need z1 and z2"))));
        }
        let _ = f__cg_finite(v_z1.clone(), Py::str("z1"))?;
        let _ = f__cg_finite(v_z2.clone(), Py::str("z2"))?;
    } else {
        if ({ let t2442 = { let t2438 = v_z1.clone(); Py::Bool({ let t2439 = Py::None; if !py::is(&t2438, &t2439) { true } else { false } }) }; if t2442.truthy() { t2442 } else { { let t2440 = v_z2.clone(); Py::Bool({ let t2441 = Py::None; if !py::is(&t2440, &t2441) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("z1 and z2 belong to joint or combined variation only"))));
        }
    }
    if ({ let t2443 = v_x1.clone(); Py::Bool({ let t2444 = Py::Int(0i64); if py::eq(&t2443, &t2444) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x1 must not be 0: the constant of variation is not determined"))));
    }
    if ({ let t2452 = { let t2449 = { let t2445 = v_kind.clone(); Py::Bool({ let t2446 = Py::str("inverse"); if py::eq(&t2445, &t2446) { true } else { false } }) }; if t2449.truthy() { t2449 } else { { let t2447 = v_kind.clone(); Py::Bool({ let t2448 = Py::str("inverse square"); if py::eq(&t2447, &t2448) { true } else { false } }) } } }; if !t2452.truthy() { t2452 } else { { let t2450 = v_x2.clone(); Py::Bool({ let t2451 = Py::Int(0i64); if py::eq(&t2450, &t2451) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x2 must not be 0 in inverse variation"))));
    }
    if ({ let t2455 = v_two.clone(); if !t2455.truthy() { t2455 } else { { let t2453 = v_z1.clone(); Py::Bool({ let t2454 = Py::Int(0i64); if py::eq(&t2453, &t2454) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("z1 must not be 0: the constant of variation is not determined"))));
    }
    if ({ let t2460 = { let t2456 = v_kind.clone(); Py::Bool({ let t2457 = Py::str("combined"); if py::eq(&t2456, &t2457) { true } else { false } }) }; if !t2460.truthy() { t2460 } else { { let t2458 = v_z2.clone(); Py::Bool({ let t2459 = Py::Int(0i64); if py::eq(&t2458, &t2459) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("z2 must not be 0 in combined variation"))));
    }
    v_k = py::truediv(&v_y1.clone(), &f__cg_vary_term(v_kind.clone(), v_x1.clone(), v_z1.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("k"), py::add(&v_k.clone(), &Py::Float(0.0))?), (Py::str("y2"), py::add(&py::mul(&v_k.clone(), &f__cg_vary_term(v_kind.clone(), v_x2.clone(), v_z2.clone())?)?, &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vector_dot_angle(mut v_u: Py, mut v_v: Py) -> R {
    let mut v_dot: Py = Py::None;
    let mut v_w: Py = Py::None;
    if ({ let t2475 = { let t2471 = py::len(&v_u.clone())?; Py::Bool({ let t2472 = py::len(&v_v.clone())?; if py::ne(&t2471, &t2472) { true } else { false } }) }; if t2475.truthy() { t2475 } else { { let t2473 = py::len(&v_u.clone())?; Py::Bool({ let t2474 = Py::tuple(vec![Py::Int(2i64), Py::Int(3i64)]); if !py::contains(&t2474, &t2473)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("two vectors of the same dimension (2 or 3)"))));
    }
    for t2476 in py::iter(&(Py::tuple(vec![v_u.clone(), v_v.clone()])))? {
        v_w = t2476;
        if (Py::Bool(({ let mut out = vec![]; for t2477 in py::iter(&(v_w.clone()))? { let mut v_c: Py; 
v_c = t2477;
out.push({ let t2478 = v_c.clone(); Py::Bool({ let t2479 = Py::Int(0i64); if py::eq(&t2478, &t2479) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the zero vector has no direction, so the angle is not defined"))));
        }
    }
    v_dot = py::fsum(&{ let mut out = vec![]; for t2480 in py::iter(&(py::zip(&[v_u.clone(), v_v.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2481 = (t2480).items()?;
if t2481.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2481[0].clone();
v_b = t2481[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("dot"), v_dot.clone()), (Py::str("mag_u"), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t2482 in py::iter(&(v_u.clone()))? { let mut v_a: Py; 
v_a = t2482;
out.push(py::mul(&v_a.clone(), &v_a.clone())?); } Py::list(out) })?)?), (Py::str("mag_v"), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t2483 in py::iter(&(v_v.clone()))? { let mut v_b: Py; 
v_b = t2483;
out.push(py::mul(&v_b.clone(), &v_b.clone())?); } Py::list(out) })?)?), (Py::str("angle_deg"), py::math1("degrees", &py::atan2(&f__cross_mag(v_u.clone(), v_v.clone())?, &v_dot.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vector_magnitude_direction(mut v_x: Py, mut v_y: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("x"), v_x.clone()), (Py::str("y"), v_y.clone())]))?;
    if ({ let t2501 = { let t2497 = v_x.clone(); Py::Bool({ let t2498 = Py::Int(0i64); if py::eq(&t2497, &t2498) { true } else { false } }) }; if !t2501.truthy() { t2501 } else { { let t2499 = v_y.clone(); Py::Bool({ let t2500 = Py::Int(0i64); if py::eq(&t2499, &t2500) { true } else { false } }) } } }).truthy() {
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
    let t284 = (Py::tuple(vec![py::int(&py::getitem(&v_inputs.clone(), &Py::str("n"))?)?, py::int(&py::getitem(&v_inputs.clone(), &Py::str("k"))?)?])).items()?;
    if t284.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t284[0].clone();
    v_k = t284[1].clone();
    let t285 = (Py::tuple(vec![py::fraction(&[py::getitem(&v_outputs.clone(), &Py::str("combinations"))?])?, py::fraction(&[py::getitem(&v_outputs.clone(), &Py::str("permutations"))?])?])).items()?;
    if t285.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t285[0].clone();
    v_p = t285[1].clone();
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
    let t1359 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("a"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?, py::getitem(&v_inputs.clone(), &Py::str("c"))?])).items()?;
    if t1359.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1359[0].clone();
    v_b = t1359[1].clone();
    v_c = t1359[2].clone();
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
    let t629 = (f__cross(py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("c"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("d"))?)?, py::getitem(&v_i.clone(), &Py::str("op"))?)?).items()?;
    if t629.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t629[0].clone();
    v_m = t629[1].clone();
    let t630 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("numerator"))?, py::getitem(&v_o.clone(), &Py::str("denominator"))?])).items()?;
    if t630.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t630[0].clone();
    v_den = t630[1].clone();
    if ({ let t641 = { let t631 = v_den.clone(); Py::Bool({ let t632 = Py::Int(0i64); if py::le(&t631, &t632)? { true } else { false } }) }; if t641.truthy() { t641 } else { { let t640 = { let t633 = v_num.clone(); Py::Bool({ let t634 = py::int(&v_num.clone())?; if py::ne(&t633, &t634) { true } else { false } }) }; if t640.truthy() { t640 } else { { let t639 = { let t635 = v_den.clone(); Py::Bool({ let t636 = py::int(&v_den.clone())?; if py::ne(&t635, &t636) { true } else { false } }) }; if t639.truthy() { t639 } else { { let t637 = py::gcd(&py::int(&v_num.clone())?, &py::int(&v_den.clone())?)?; Py::Bool({ let t638 = Py::Int(1i64); if py::ne(&t637, &t638) { true } else { false } }) } } } } } } }).truthy() {
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
    let t1406 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("a"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?, py::getitem(&v_inputs.clone(), &Py::str("c"))?])).items()?;
    if t1406.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1406[0].clone();
    v_b = t1406[1].clone();
    v_c = t1406[2].clone();
    v_worst = Py::Float(0.0);
    for t1407 in py::iter(&(Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("x1"))?, py::getitem(&v_outputs.clone(), &Py::str("x2"))?])))? {
        v_x = t1407;
        v_terms = Py::list(vec![py::mul(&py::mul(&v_a.clone(), &v_x.clone())?, &v_x.clone())?, py::mul(&v_b.clone(), &v_x.clone())?, v_c.clone()]);
        v_scale = { let t1409 = py::sum(&{ let mut out = vec![]; for t1408 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t1408;
out.push(py::abs(&v_t.clone())?); } Py::list(out) }, None)?; if t1409.truthy() { t1409 } else { Py::Float(1.0) } };
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sum(&v_terms.clone(), None)?)?, &v_scale.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_what_percent(mut v_part: Py, mut v_whole: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("part"), v_part.clone()), (Py::str("whole"), v_whole.clone())]))?;
    if ({ let t2511 = v_whole.clone(); Py::Bool({ let t2512 = Py::Int(0i64); if py::eq(&t2511, &t2512) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the whole must not be 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("percent"), py::truediv(&py::mul(&Py::Int(100i64), &v_part.clone())?, &v_whole.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1138(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("result"))?, &Py::Int(100i64))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::getitem(&v_i.clone(), &Py::str("x"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::getitem(&v_i.clone(), &Py::str("x"))?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1145(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("whole"))?, &py::getitem(&v_i.clone(), &Py::str("p"))?)?, &py::mul(&Py::Int(100i64), &py::getitem(&v_i.clone(), &Py::str("part"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&Py::Int(100i64), &py::getitem(&v_i.clone(), &Py::str("part"))?)?)?, py::abs(&py::mul(&py::getitem(&v_o.clone(), &Py::str("whole"))?, &py::getitem(&v_i.clone(), &Py::str("p"))?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_120(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("modulus"))?, &py::math1("cos", &v_t.clone())?)?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_o.clone(), &Py::str("modulus"))?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("modulus"))?, &py::math1("sin", &v_t.clone())?)?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_o.clone(), &Py::str("modulus"))?], true)?)?)?, &py::not(&{ let t121 = py::neg(&Py::Int(180i64))?; Py::Bool({ let t122 = py::getitem(&v_o.clone(), &Py::str("argument_deg"))?; if py::lt(&t121, &t122)? { { let t123 = Py::Int(180i64); if py::le(&t122, &t123)? { true } else { false } } } else { false } }) }))?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("argument_rad"))?, &v_t.clone())?)?)?, &py::truediv(&py::abs(&py::sub(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("modulus"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("a"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("b"))?, &Py::Int(2i64))?)?)?, &py::minmax(&[Py::Float(1.0), py::pow(&py::getitem(&v_o.clone(), &Py::str("modulus"))?, &Py::Int(2i64))?], true)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("argument_deg"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1279(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("x"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("r"))?, &py::math1("cos", &v_t.clone())?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("r"))?)?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("y"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("r"))?, &py::math1("sin", &v_t.clone())?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("r"))?)?], true)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[(if ({ let t1280 = py::getitem(&v_i.clone(), &Py::str("unit"))?; Py::Bool({ let t1281 = Py::str("degrees"); if py::eq(&t1280, &t1281) { true } else { false } }) }).truthy() { py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("theta"))?)? } else { py::getitem(&v_i.clone(), &Py::str("theta"))? })])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1425(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_poly(v_i.clone(), v_o.clone(), Py::Int(4i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1438(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("degrees"))?, &Py::Float(std::f64::consts::PI))?, &Py::Int(180i64))?, &py::getitem(&v_i.clone(), &Py::str("rad"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("rad"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1866(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&f__verify_magdir(v_i.clone(), Py::dict(vec![(Py::str("magnitude"), py::getitem(&v_o.clone(), &Py::str("r"))?), (Py::str("direction_deg"), py::getitem(&v_o.clone(), &Py::str("theta_deg"))?)]))?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("theta_rad"))?, &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("theta_deg"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1895(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::pow(&py::getitem(&v_o.clone(), &Py::str("b"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("a"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Int(2i64))?)?)?, &py::minmax(&[Py::Float(1.0), py::pow(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Int(2i64))?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::math1("sin", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("A"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_i.clone(), &Py::str("a"))?], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("A"))?, &py::getitem(&v_o.clone(), &Py::str("B"))?)?, &Py::Int(90i64))?)?, &Py::Int(90i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2413(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_cmp: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_r = py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("unit_rate_1"))?, &py::getitem(&v_i.clone(), &Py::str("n1"))?)?, &py::getitem(&v_i.clone(), &Py::str("c1"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("c1"))?)?], true)?)?;
    if ({ let t2414 = py::method(&v_i.clone(), "get", &[Py::str("n2")])?; Py::Bool({ let t2415 = Py::None; if !py::is(&t2414, &t2415) { true } else { false } }) }).truthy() {
        if ({ let t2416 = Py::str("unit_rate_2"); Py::Bool({ let t2417 = v_o.clone(); if !py::contains(&t2417, &t2416)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
        v_r = py::iadd(&v_r, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("unit_rate_2"))?, &py::getitem(&v_i.clone(), &Py::str("n2"))?)?, &py::getitem(&v_i.clone(), &Py::str("c2"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("c2"))?)?], true)?)?)?;
        v_cmp = py::sub(&py::mul(&py::fraction(&[Py::str(&py::repr(&py::float(&py::getitem(&v_i.clone(), &Py::str("c1"))?)?))])?, &py::fraction(&[Py::str(&py::repr(&py::float(&py::getitem(&v_i.clone(), &Py::str("n2"))?)?))])?)?, &py::mul(&py::fraction(&[Py::str(&py::repr(&py::float(&py::getitem(&v_i.clone(), &Py::str("c2"))?)?))])?, &py::fraction(&[Py::str(&py::repr(&py::float(&py::getitem(&v_i.clone(), &Py::str("n1"))?)?))])?)?)?;
        v_r = py::iadd(&v_r, &{ let t2418 = py::method(&v_o.clone(), "get", &[Py::str("better")])?; Py::Bool({ let t2419 = (if ({ let t2420 = v_cmp.clone(); Py::Bool({ let t2421 = Py::Int(0i64); if py::lt(&t2420, &t2421)? { true } else { false } }) }).truthy() { Py::str("first") } else { (if ({ let t2422 = v_cmp.clone(); Py::Bool({ let t2423 = Py::Int(0i64); if py::gt(&t2422, &t2423)? { true } else { false } }) }).truthy() { Py::str("second") } else { Py::str("equal") }) }); if py::ne(&t2418, &t2419) { true } else { false } }) })?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2513(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("percent"))?, &py::getitem(&v_i.clone(), &Py::str("whole"))?)?, &py::mul(&Py::Int(100i64), &py::getitem(&v_i.clone(), &Py::str("part"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&Py::Int(100i64), &py::getitem(&v_i.clone(), &Py::str("part"))?)?)?, py::abs(&py::mul(&py::getitem(&v_o.clone(), &Py::str("percent"))?, &py::getitem(&v_i.clone(), &Py::str("whole"))?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_272(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("amount"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("n0"))?, &py::math1("exp", &py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &py::getitem(&v_i.clone(), &Py::str("t"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_301(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_poly(v_i.clone(), v_o.clone(), Py::Int(3i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_427(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::pow(&py::add(&Py::Int(1i64), &py::truediv(&py::getitem(&v_i.clone(), &Py::str("r"))?, &Py::Int(100i64))?)?, &py::getitem(&v_o.clone(), &Py::str("doubling_time"))?)?, &Py::Int(2i64))?)?, &Py::Int(2i64))?, &f__rel(py::getitem(&v_o.clone(), &Py::str("rule_of_70_estimate"))?, py::truediv(&Py::Int(70i64), &py::getitem(&v_i.clone(), &Py::str("r"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_572(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("c"))?, &py::getitem(&v_o.clone(), &Py::str("x"))?)?, &py::getitem(&v_i.clone(), &Py::str("d"))?)?)?)?, &py::getitem(&v_i.clone(), &Py::str("k"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("k"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_652(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::method(&v_i.clone(), "get", &[Py::str("B"), Py::Float(1.0)])?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("x_new"))?, &py::method(&v_i.clone(), "get", &[Py::str("h"), Py::Float(0.0)])?)?)?, &py::getitem(&v_i.clone(), &Py::str("x0"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("x0"))?)?], true)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("y_new"))?, py::add(&py::mul(&py::method(&v_i.clone(), "get", &[Py::str("A"), Py::Float(1.0)])?, &py::getitem(&v_i.clone(), &Py::str("y0"))?)?, &py::method(&v_i.clone(), "get", &[Py::str("k"), Py::Float(0.0)])?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_666(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("remaining"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("n0"))?, &py::pow(&Py::Int(2i64), &py::truediv(&py::neg(&py::getitem(&v_i.clone(), &Py::str("t"))?)?, &py::getitem(&v_i.clone(), &Py::str("half_life"))?)?)?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("fraction"))?, &py::getitem(&v_i.clone(), &Py::str("n0"))?)?, py::getitem(&v_o.clone(), &Py::str("remaining"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_684(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_716(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("sum"))?, &py::sub(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("r"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("a"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_743(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("a"))?, &Py::Int(2i64))?, &py::sub(&py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("b"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::math1("cos", &py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("A"))?)?)?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("b"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Int(2i64))?)?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("a"))?, &f__deg_sin(py::getitem(&v_o.clone(), &Py::str("B"))?)?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("b"))?, &f__deg_sin(py::getitem(&v_i.clone(), &Py::str("A"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_o.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::add(&py::getitem(&v_i.clone(), &Py::str("A"))?, &py::getitem(&v_o.clone(), &Py::str("B"))?)?, &py::getitem(&v_o.clone(), &Py::str("C"))?)?, &Py::Int(180i64))?)?, &Py::Int(180i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_760(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&py::mul(&py::math1("cos", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("A"))?)?)?, &Py::Int(2i64))?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::sub(&py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("b"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("a"))?, &Py::Int(2i64))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&py::mul(&py::math1("cos", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("B"))?)?)?, &Py::Int(2i64))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::sub(&py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("a"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("b"))?, &Py::Int(2i64))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::add(&py::getitem(&v_o.clone(), &Py::str("A"))?, &py::getitem(&v_o.clone(), &Py::str("B"))?)?, &py::getitem(&v_o.clone(), &Py::str("C"))?)?, &Py::Int(180i64))?)?, &Py::Int(180i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_911(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::add(&py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_o.clone(), &Py::str("x_intercept"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?)?, &py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_o.clone(), &Py::str("y_intercept"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("c"))?)?], true)?)?, &py::truediv(&py::abs(&py::add(&py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("a"))?)?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_981(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::math1("log", &py::getitem(&v_i.clone(), &Py::str("x"))?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("value"))?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::math1("log", &py::getitem(&v_i.clone(), &Py::str("x"))?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_986(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("value"))?, &py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::getitem(&v_i.clone(), &Py::str("u"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("q"))?, &py::getitem(&v_i.clone(), &Py::str("v"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::getitem(&v_i.clone(), &Py::str("u"))?)?)?, &py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("q"))?, &py::getitem(&v_i.clone(), &Py::str("v"))?)?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
