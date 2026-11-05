//! Every Math Foundations capability's own live figures (src/figures.rs): its worked example gives each published figure
//! kind, a changed input changes the drawing, and no drawing holds a non-finite number or text outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use math_foundations_ai999x::pack;

/// The published figure kinds of every capability drawn by the model's own figures
/// (publish/website/models/math-foundations-ai999x/figures, except the generic "curve" and the kinds the generic figures
/// already draw), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("abs_value_inequality_lower_bound", &["field"]),
    ("abs_value_inequality_upper_bound", &["field"]),
    ("complex_modulus_argument", &["field"]),
    ("complex_nth_roots", &["complex", "shape"]),
    ("cubic_roots", &["curve-2"]),
    ("de_moivre_power", &["shape"]),
    ("ellipse_parameters", &["shape"]),
    ("factor_quadratic_integer", &["area"]),
    ("geometric_sequence", &["stems"]),
    ("infinite_geometric_series", &["field"]),
    ("linear_system_3x3", &["matrix"]),
    ("logarithmic_equation", &["field"]),
    ("order_of_operations", &["io"]),
    ("percent_of_number", &["field"]),
    ("percent_of_what", &["field"]),
    ("perimeter_of_rectangle", &["field"]),
    ("piecewise_evaluate", &["field"]),
    ("polar_to_rectangular", &["field"]),
    ("radical_equation", &["io"]),
    ("rational_roots", &["complex"]),
    ("rectangular_to_polar", &["field"]),
    ("right_triangle_leg_hypotenuse", &["field"]),
    ("right_triangle_legs", &["field"]),
    ("round_decimal_places", &["field"]),
    ("round_significant_figures", &["field"]),
    ("unit_circle_exact", &["decision", "polar"]),
    ("vector_magnitude_direction", &["field"]),
    ("what_percent", &["field"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("complex_nth_roots", "n", "5"),
    ("de_moivre_power", "n", "6"),
    ("factor_quadratic_integer", "c", "-4"),
    ("geometric_sequence", "n", "7"),
    ("linear_system_3x3", "augmented", "2 1 1 5; 1 3 2 10; 1 0 0 1"),
    ("order_of_operations", "expression", "(7 - 2) * 3 + 4 / 8"),
    ("rational_roots", "coefficients", "1, -2, -5, 6"),
    ("unit_circle_exact", "deg", "225"),
    ("round_decimal_places", "n", "3"),
    ("round_significant_figures", "n", "2"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn kind_of(f: &Figure) -> &str {
    &f.kind
}

/// Every text inside the canvas, no non-finite number (ax_engine::figkit::check_figure).
fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| kind_of(f) == *kind) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    // ("infinite" is a word of some titles: check_figure looks for inf as a number)
                    assert!(!f.svg.contains("NaN"), "{key} {kind}: a non-finite number");
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        // the model's own figures come first
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12 V" -> "16.44 V"; "3, 6" -> "4.11, 8.22").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        // the other example: a listed replacement, else each number input scaled until the model solves it
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn an_impossible_input_draws_nothing_wrong() {
    // inputs the model refuses (a leg longer than the hypotenuse; a base of 1 for a logarithm; a series that does not
    // converge): no figure at all, never a wrong one
    let p = pack();
    for (key, given) in [("right_triangle_leg_hypotenuse", vec![("a", "13"), ("c", "5")]),
                         ("logarithmic_equation", vec![("b", "1"), ("c", "5")]),
                         ("infinite_geometric_series", vec![("a", "6"), ("r", "2")])] {
        let given: Vec<(String, Input)> = given.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
        let a = ax_engine::solve(p, key, &given, None);
        let figs = figures(p, key, &given, None, None);
        if !a.solved() {
            assert!(figs.is_empty(), "{key}: a refused input drew a figure");
        } else {
            for f in &figs {
                check_texts(f);
            }
        }
    }
}

#[test]
fn own_figures_time() {
    let p = pack();
    for (key, _) in KINDS {
        let given = example(key);
        let t = std::time::Instant::now();
        let _ = figures(p, key, &given, None, None);
        eprintln!("TIME {key}: {} ms", t.elapsed().as_millis());
    }
}
