//! Math Foundations' own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: the answer over a grid of two inputs (one run of the model per cell), roots in the complex plane, the
//! powers of a complex number, an ellipse with its foci, the area model of a factored quadratic, a sequence with its
//! partial sums, the unit circle with exact values, and the inputs and results as a table. A figure the model cannot
//! support for these inputs is left out, never drawn wrong (docs/FIGURE_PARITY.md).

use ax_engine::figkit::{self as k, cx_list, num, num_of, Canvas, DPlot, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        // the answer over a grid of two inputs
        "abs_value_inequality_lower_bound" => vec![field2(s, ("d", "bound d"), ("c", "centre c"), "left_limit",
                                                          "Solution set of |x - c| > d: x below this")],
        "abs_value_inequality_upper_bound" => vec![field2(s, ("c", "centre c"), ("d", "bound d"), "lower",
                                                          "Solution interval of |x - c| < d: its lower end")],
        "complex_modulus_argument" => vec![field2(s, ("b", "imaginary part b"), ("a", "real part a"), "modulus",
                                                  "Modulus |z| of z = a + bi")],
        "infinite_geometric_series" => vec![field2(s, ("r", "common ratio r"), ("a", "first term a"), "sum",
                                                   "Sum of an infinite geometric series")],
        "logarithmic_equation" => vec![field2(s, ("c", "value of the logarithm c"), ("b", "base b"), "x",
                                              "Solution of log_b(x) = c")],
        "percent_of_number" => vec![field2(s, ("p", "percentage p"), ("x", "the whole x"), "result", "p% of x")],
        "percent_of_what" => vec![field2(s, ("part", "the known part"), ("p", "percentage p"), "whole",
                                         "The whole from a part and its percentage")],
        "perimeter_of_rectangle" => vec![field2(s, ("w", "width w"), ("h", "height h"), "p", "Perimeter of a rectangle")],
        "piecewise_evaluate" => vec![field2(s, ("x", "the number x0"), ("c", "break point c"), "value",
                                            "A piecewise function's value f(x0)")],
        "polar_to_rectangular" => vec![field2(s, ("r", "distance r"), ("theta", "angle theta"), "x",
                                              "x = r cos(theta) from polar coordinates")],
        "rectangular_to_polar" => vec![field2(s, ("x", "x-coordinate"), ("y", "y-coordinate"), "r",
                                              "Distance r from rectangular coordinates")],
        "right_triangle_leg_hypotenuse" => vec![field2(s, ("c", "hypotenuse c"), ("a", "known leg a"), "b",
                                                       "The other leg of a right triangle")],
        "right_triangle_legs" => vec![field2(s, ("b", "leg b"), ("a", "leg a"), "c", "Hypotenuse of a right triangle")],
        "round_decimal_places" => vec![field2(s, ("x", "number to round x"), ("n", "decimal places n"), "rounded",
                                              "A number rounded to n decimal places")],
        "round_significant_figures" => vec![field2(s, ("x", "number to round x"), ("n", "significant figures n"),
                                                   "rounded", "A number rounded to n significant figures")],
        "vector_magnitude_direction" => vec![field2(s, ("y", "y-component"), ("x", "x-component"), "magnitude",
                                                    "Magnitude |v| of a plane vector")],
        "what_percent" => vec![field2(s, ("part", "the part"), ("whole", "the whole"), "percent",
                                      "Part as a percentage of the whole")],
        // drawings of their own
        "complex_nth_roots" => vec![roots_complex(s), roots_polygon(s)],
        "rational_roots" => vec![rational_complex(s)],
        "cubic_roots" => vec![cubic_family(s)],
        "de_moivre_power" => vec![power_spiral(s)],
        "ellipse_parameters" => vec![ellipse_shape(s)],
        "factor_quadratic_integer" => vec![area_model(s)],
        "geometric_sequence" => vec![sequence_stems(s)],
        "linear_system_3x3" => vec![system_matrix(s)],
        "order_of_operations" | "radical_equation" => vec![io_table(s)],
        "unit_circle_exact" => vec![unit_decision(s), unit_polar(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ helpers
/// A number input as read (a whole-number input is read as a list of one).
fn get(s: &Solved, sym: &str) -> Option<f64> {
    s.get(sym).or_else(|| s.vec(sym).and_then(|v| (v.len() == 1).then(|| v[0])))
}

/// Whether an input is a whole number in the model's form.
fn is_int(s: &Solved, sym: &str) -> bool {
    s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).map(|i| i.ty == "integer")
        .unwrap_or(false)
}

/// An exact value as the page writes it: "-sqrt(3)/2" as "-√3/2".
fn pretty(t: &str) -> String {
    let mut out = t.replace(' ', "");
    while let Some(i) = out.find("sqrt(") {
        let rest = &out[i + 5..];
        match rest.find(')') {
            Some(j) => {
                let inner = rest[..j].to_string();
                out = format!("{}√{}{}", &out[..i], inner, &rest[j + 1..]);
            }
            None => break,
        }
    }
    out
}

/// "a + bi" to 4 digits.
fn cx_text(z: (f64, f64)) -> String {
    let (re, im) = (k::snap(z.0, z.1.abs().max(1.0)), k::snap(z.1, z.0.abs().max(1.0)));
    if im == 0.0 {
        return num(re);
    }
    if re == 0.0 {
        return format!("{}i", num(im));
    }
    format!("{} {} {}i", num(re), if im < 0.0 { "-" } else { "+" }, num(im.abs()))
}

// ------------------------------------------------------------------------------------------------ the field
/// The answer shaded over a grid of two inputs (13 x 9 runs of the model; a whole-number input steps by 1).
fn field2(s: &Solved, xi: (&str, &str), yi: (&str, &str), out: &str, what: &str) -> Option<Figure> {
    let (xsym, ysym) = (xi.0, yi.0);
    let (x0, y0, v0) = (get(s, xsym)?, get(s, ysym)?, s.out(out)?);
    let axis = |sym: &str, v: f64, n: usize| -> Vec<f64> {
        if is_int(s, sym) {
            let lo = if s.key == "round_significant_figures" { (v - 3.0).max(1.0) } else { v - 3.0 };
            return (0..n).map(|i| lo + i as f64).collect();
        }
        let (lo, hi) = match (s.key, sym) {
            // a ratio of a convergent series stays inside (-1, 1)
            ("infinite_geometric_series", "r") => ((v - 0.4).max(-0.95), (v + 0.4).min(0.95)),
            ("polar_to_rectangular", "theta") => (v - 90.0, v + 90.0),
            _ if v != 0.0 => (v - v.abs() * 0.5, v + v.abs() * 0.5),
            _ => (-1.0, 1.0),
        };
        k::span(lo, hi, n, false)
    };
    let mut xs = axis(xsym, x0, 13);
    let mut ys = axis(ysym, y0, 9);
    // the visitor's own value is one of the cells
    for (v, a) in [(x0, &mut xs), (y0, &mut ys)] {
        if !a.iter().any(|x| (x - v).abs() < 1e-12) {
            a.push(v);
            a.sort_by(|p, q| p.partial_cmp(q).unwrap_or(std::cmp::Ordering::Equal));
        }
    }
    let input = |sym: &str, v: f64| if is_int(s, sym) { Input::Int(v.round() as i64) } else { Input::Float(v) };
    let vals: Vec<Vec<Option<f64>>> = xs.iter().map(|x| ys.iter().map(|y| {
        s.with_input(&[(xsym, input(xsym, *x)), (ysym, input(ysym, *y))]).and_then(|a| num_of(&a, out))
    }).collect()).collect();
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    if all.len() < 20 {
        return None;
    }
    let (lo, hi) = all.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
    let log_shade = lo > 0.0 && hi / lo > 30.0;
    let shown: Vec<Vec<Option<f64>>> = if log_shade {
        vals.iter().map(|c| c.iter().map(|v| v.map(|x| x.log10())).collect()).collect()
    } else {
        vals.clone()
    };
    let head = if log_shade { format!("{out} = {} (shade: log10 of {out})", num(v0)) } else { format!("{out} = {}", num(v0)) };
    let body = k::field(&xs, &ys, &shown, (x0, y0), xi.1, yi.1, &head, (false, false));
    if body.is_empty() {
        return None;
    }
    let refused = vals.iter().flatten().filter(|v| v.is_none()).count();
    Some(s.chart("field", &format!("{what}, over {} and {}", xi.1, yi.1),
                 &format!("{out} over {xsym} from {} to {} and {ysym} from {} to {}: each of the {} × {} cells is one run \
                           of the model, shaded by {} (darker is larger; values from {} to {}){}. Marked: {xsym} = {}, \
                           {ysym} = {}, {out} = {}.", num(xs[0]), num(xs[xs.len() - 1]), num(ys[0]), num(ys[ys.len() - 1]),
                          xs.len(), ys.len(), if log_shade { "the logarithm of its value" } else { "its value" },
                          num(lo), num(hi),
                          if refused > 0 { format!("; {refused} blank cells are inputs the model refuses") } else { String::new() },
                          num(x0), num(y0), num(v0)), &body))
}

// ------------------------------------------------------------------------------------------------ complex roots
fn roots_of(s: &Solved) -> Option<Vec<(f64, f64)>> {
    let r = cx_list(s.a, "roots")?;
    (r.len() <= 24).then_some(r)
}

/// The n-th roots marked in the complex plane, on the circle of their modulus.
fn roots_complex(s: &Solved) -> Option<Figure> {
    let (a, b, n, m) = (get(s, "a")?, get(s, "b")?, get(s, "n")?, s.out("modulus")?);
    let roots = roots_of(s)?;
    let lim = m.max(1e-12);
    let mut p = DPlot::equal(&[-lim * 1.15, lim * 1.15], &[-lim * 1.15, lim * 1.15], "Real part", "Imaginary part",
                            &format!("{} roots, modulus {}", roots.len(), num(m)));
    p.hline(0.0);
    p.vline(0.0);
    p.dashed(&circle((0.0, 0.0), m), 3);
    for (i, z) in roots.iter().enumerate() {
        p.seg((0.0, 0.0), *z, 3, false);
        p.dot(z.0, z.1, 0, 5.0);
        // a root's value beside it when there are few, its number otherwise (the caption lists the values)
        let (ux, uy) = (z.0 / lim, z.1 / lim);
        let t = if roots.len() <= 4 { format!("z{} = {}", i + 1, cx_text(*z)) } else { format!("z{}", i + 1) };
        if roots.len() <= 12 {
            p.label(z.0 + ux * lim * 0.14, z.1 + uy * lim * 0.14 - lim * 0.04,
                    if ux.abs() < 0.3 { "middle" } else if ux > 0.0 { "start" } else { "end" }, "axp-vt", &t);
        }
    }
    let list = roots.iter().map(|z| cx_text(*z)).collect::<Vec<_>>().join(", ");
    Some(s.chart("complex", &format!("The {} roots of z^{} = {} in the complex plane", roots.len(), num(n), cx_text((a, b))),
                 &format!("The {} roots returned for z^{} = {}, drawn in the complex plane on the dashed circle of \
                           radius {} (their common modulus): {list}.", roots.len(), num(n), cx_text((a, b)), num(m)),
                 &p.body()))
}

/// The roots joined in order of their argument: a regular polygon inscribed in the circle of their modulus.
fn roots_polygon(s: &Solved) -> Option<Figure> {
    let (a, b, n, m) = (get(s, "a")?, get(s, "b")?, get(s, "n")?, s.out("modulus")?);
    let mut roots = roots_of(s)?;
    if roots.len() < 2 {
        return None;
    }
    roots.sort_by(|p, q| p.1.atan2(p.0).partial_cmp(&q.1.atan2(q.0)).unwrap_or(std::cmp::Ordering::Equal));
    let lim = m.max(1e-12);
    let name = match roots.len() {
        2 => "a diameter".to_string(),
        3 => "an equilateral triangle".to_string(),
        4 => "a square".to_string(),
        5 => "a regular pentagon".to_string(),
        6 => "a regular hexagon".to_string(),
        k_ => format!("a regular polygon of {k_} sides"),
    };
    let mut p = DPlot::equal(&[-lim * 1.15, lim * 1.15], &[-lim * 1.15, lim * 1.15], "Real part", "Imaginary part",
                            &format!("z^{} = {}: {}", num(n), cx_text((a, b)), name));
    p.dashed(&circle((0.0, 0.0), m), 3);
    if roots.len() > 2 {
        p.fill(&roots, 0);
        p.outline(&roots, 0);
    } else {
        p.seg(roots[0], roots[1], 0, false);
    }
    for z in &roots {
        p.dot(z.0, z.1, 0, 5.0);
    }
    p.dot(0.0, 0.0, 3, 3.0);
    p.label(lim * 0.72, lim * 0.98, "start", "axp-vt", &format!("r = {}", num(m)));
    Some(s.chart("shape", &format!("The {} roots of {} form {}", roots.len(), cx_text((a, b)), name),
                 &format!("The {} complex roots of z^{} = {} returned by the model, joined in order of their argument: \
                           the vertices of {} on the circle of radius {} (their modulus), each turned {}° from the next.",
                          roots.len(), num(n), cx_text((a, b)), name, num(m), num(360.0 / roots.len() as f64)),
                 &p.body()))
}

/// Rational roots marked on the real axis of the complex plane.
fn rational_complex(s: &Solved) -> Option<Figure> {
    let roots = s.out_list("roots")?;
    if roots.len() > 12 {
        return None;
    }
    let exact: Vec<String> = match s.outj("roots_exact") {
        Some(serde_json::Value::Array(v)) => v.iter().map(|x| x.as_str().unwrap_or("").to_string()).collect(),
        _ => roots.iter().map(|r| num(*r)).collect(),
    };
    let span = roots.iter().fold(1.0f64, |a, r| a.max(r.abs()));
    let mut p = DPlot::equal(&[-span * 1.2, span * 1.2], &[-span * 0.5, span * 0.5], "Real part", "Imaginary part",
                            &format!("{} rational roots", roots.len()));
    p.hline(0.0);
    p.vline(0.0);
    for (i, r) in roots.iter().enumerate() {
        p.dot(*r, 0.0, 0, 6.0);
        let up = if i % 2 == 0 { 0.12 } else { -0.2 };
        p.label(*r, span * up, "middle", "axp-vt", &format!("x = {}", exact.get(i).cloned().unwrap_or_default()));
    }
    let coef = s.vec("coefficients").map(|c| c.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ")).unwrap_or_default();
    Some(s.chart("complex", "The rational roots in the complex plane",
                 &format!("The rational roots returned for the polynomial with coefficients {coef} (highest power first), \
                           drawn in the complex plane, all on the real axis: {}.", exact.join(", ")), &p.body()))
}

// ------------------------------------------------------------------------------------------------ cubic family
/// The first root against the leading coefficient, one curve for each of three values of b.
fn cubic_family(s: &Solved) -> Option<Figure> {
    let (a, b, x1) = (get(s, "a")?, get(s, "b")?, s.out("x1")?);
    if a == 0.0 {
        return None;
    }
    let log = a > 0.0;
    let xs = if log { k::span(a / 10.0, a * 10.0, 41, true) } else { k::span(a * 2.0, a / 10.0, 41, false) };
    let bs = if b != 0.0 { vec![b, b * 0.5, b * 2.0] } else { vec![0.0, -1.0, 1.0] };
    let fam = s.family("a", &xs, "x1", "b", &bs, "");
    if fam.is_empty() {
        return None;
    }
    let body = k::lines(&fam, "Coefficient of x^3, a", "First root x1", &format!("x1 = {}", num(x1)), Some((a, x1)),
                        &format!("a = {}", num(a)), log);
    Some(s.chart("curve-2", "Roots of a cubic: the first root against the coefficient of x^3, for three values of b",
                 &format!("x1 (the first root: real roots ascending, then complex) against a from {} to {}{}, one curve \
                           for each value of b ({}), each point one run of the model; marked: a = {}, x1 = {}.",
                          num(xs[0].min(xs[40])), num(xs[0].max(xs[40])), if log { " on a logarithmic axis" } else { "" },
                          bs.iter().map(|v| num(*v)).collect::<Vec<_>>().join(", "), num(a), num(x1)), &body))
}

// ------------------------------------------------------------------------------------------------ de Moivre
/// The powers z^m for m = 0 ... n, each a run of the model, joined into a spiral.
fn power_spiral(s: &Solved) -> Option<Figure> {
    let (a, b, n) = (get(s, "a")?, get(s, "b")?, get(s, "n")?);
    let n = n.round() as i64;
    if n.abs() > 40 {
        return None;
    }
    let step = if n >= 0 { 1 } else { -1 };
    let mut pts: Vec<(i64, (f64, f64))> = vec![];
    let mut m = 0i64;
    loop {
        let r = s.with_input(&[("n", Input::Int(m))])?;
        pts.push((m, (num_of(&r, "re")?, num_of(&r, "im")?)));
        if m == n {
            break;
        }
        m += step;
    }
    let (re, im, md, arg) = (s.out("re")?, s.out("im")?, s.out("modulus")?, s.out("argument_deg")?);
    let xs: Vec<f64> = pts.iter().map(|p| p.1 .0).chain([0.0]).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1 .1).chain([0.0]).collect();
    let mut p = DPlot::equal(&xs, &ys, "Real part", "Imaginary part", &format!("z = {}, n = {}", cx_text((a, b)), n));
    p.hline(0.0);
    p.vline(0.0);
    let path: Vec<(f64, f64)> = pts.iter().map(|q| q.1).collect();
    p.curve(&path, 0);
    // a power is numbered when its point stands clear of the origin and of the numbers already written
    let big = pts.iter().fold(0.0f64, |a, q| a.max(q.1 .0.hypot(q.1 .1)));
    let mut written: Vec<(f64, f64)> = vec![p.px(re, im)];
    for (m, z) in &pts {
        p.dot(z.0, z.1, 0, 3.5);
        let at = p.px(z.0, z.1);
        let clear = written.iter().all(|w| (w.0 - at.0).hypot(w.1 - at.1) > 22.0);
        if pts.len() <= 16 && *m != n && z.0.hypot(z.1) > big * 0.06 && clear {
            p.label(z.0, z.1, "start", "axp-lt", &format!(" {m}"));
            written.push(at);
        }
    }
    p.seg((0.0, 0.0), (re, im), 3, true);
    p.mark(re, im, &format!("z^{n} = {}", cx_text((re, im))));
    Some(s.chart("shape", &format!("Powers of {} (de Moivre)", cx_text((a, b))),
                 &format!("The powers ({})^m for m = 0 to {n}, each a run of the model, joined in order: every step turns \
                           by the argument of z and stretches by its modulus. Marked: z^{n} = {}, modulus {}, argument {}°.",
                          cx_text((a, b)), cx_text((re, im)), num(md), num(arg)), &p.body()))
}

// ------------------------------------------------------------------------------------------------ conics
fn circle(c: (f64, f64), r: f64) -> Vec<(f64, f64)> {
    (0..=96).map(|i| {
        let t = i as f64 / 96.0 * std::f64::consts::TAU;
        (c.0 + r * t.cos(), c.1 + r * t.sin())
    }).collect()
}

/// The ellipse from the returned semi-axes and foci, with the hyperbola of the same A and B (a run of the model's
/// hyperbola capability) and its foci.
fn ellipse_shape(s: &Solved) -> Option<Figure> {
    let (h, kk, aa, bb) = (get(s, "h")?, get(s, "k")?, get(s, "A")?, get(s, "B")?);
    let (a, b, c, e) = (s.out("a")?, s.out("b")?, s.out("c")?, s.out("eccentricity")?);
    let (f1, f2) = ((s.out("f1x")?, s.out("f1y")?), (s.out("f2x")?, s.out("f2y")?));
    let horiz = s.out_text("orientation").map(|o| o != "vertical").unwrap_or(aa >= bb);
    // the ellipse: semi-axis a along its major axis
    let (rx, ry) = if horiz { (a, b) } else { (b, a) };
    let ell: Vec<(f64, f64)> = (0..=120).map(|i| {
        let t = i as f64 / 120.0 * std::f64::consts::TAU;
        (h + rx * t.cos(), kk + ry * t.sin())
    }).collect();
    let hyp = s.solve_other("hyperbola_parameters", &[("h", Input::Float(h)), ("k", Input::Float(kk)),
                                                      ("A", Input::Float(aa)), ("B", Input::Float(bb)),
                                                      ("orientation", Input::Text(if horiz { "horizontal" } else { "vertical" }.into()))]);
    let mut xs = vec![h - rx * 1.6, h + rx * 1.6];
    let mut ys = vec![kk - ry * 1.6, kk + ry * 1.6];
    let hv = hyp.as_ref().and_then(|r| Some((num_of(r, "a")?, num_of(r, "b")?, num_of(r, "c")?, num_of(r, "eccentricity")?)));
    if let Some((_, _, hc, _)) = hv {
        if horiz { xs.extend([h - hc * 1.3, h + hc * 1.3]) } else { ys.extend([kk - hc * 1.3, kk + hc * 1.3]) }
    }
    let mut p = DPlot::equal(&xs, &ys, "x", "y", &format!("ellipse a = {}, b = {}, e = {}", num(a), num(b), num(e)));
    p.hline(kk);
    p.vline(h);
    p.fill(&ell, 0);
    p.curve(&ell, 0);
    p.key("ellipse", 0);
    let mut hyp_note = String::new();
    if let Some((ha, hb, hc, he)) = hv {
        // the two branches x = h ± a cosh(t), y = k + b sinh(t) (swapped for a vertical one), clipped to the plot
        for side in [1.0, -1.0] {
            let br: Vec<(f64, f64)> = (0..=60).map(|i| {
                let t = -2.4 + 4.8 * i as f64 / 60.0;
                let (u, v) = (side * ha * t.cosh(), hb * t.sinh());
                if horiz { (h + u, kk + v) } else { (h + v, kk + u) }
            }).collect();
            p.curve(&br, 2);
        }
        let sl = hb / ha;
        let ((x0, x1), _) = p.ranges();
        for sg in [1.0, -1.0] {
            let pts: Vec<(f64, f64)> = [x0, x1].iter().map(|x| {
                if horiz { (*x, kk + sg * sl * (x - h)) } else { (*x, kk + sg * (x - h) / sl) }
            }).collect();
            p.dashed(&pts, 3);
        }
        let hf = if horiz { [(h - hc, kk), (h + hc, kk)] } else { [(h, kk - hc), (h, kk + hc)] };
        for q in hf {
            p.dot(q.0, q.1, 2, 4.5);
        }
        p.key("hyperbola", 2);
        hyp_note = format!(" The hyperbola of the same A and B (a run of the model): a = {}, b = {}, foci {} apart from \
                            the centre, eccentricity {}, asymptotes of slope ±{}.", num(ha), num(hb), num(hc), num(he),
                           num(if horiz { sl } else { 1.0 / sl }));
    }
    for (i, q) in [f1, f2].iter().enumerate() {
        p.dot(q.0, q.1, 0, 5.0);
        p.label(q.0, q.1 - ry * 0.12, "middle", "axp-vt", &format!("F{}", i + 1));
    }
    p.label(h, kk + ry * 1.12, "middle", "axp-lt", &format!("c = {}", num(c)));
    Some(s.chart("shape", "Ellipse and hyperbola with their foci",
                 &format!("The ellipse with centre ({}, {}), A = {}, B = {}, drawn from the parameters the model returns: \
                           a = {}, b = {}, c = {}, foci ({}, {}) and ({}, {}), eccentricity {}.{hyp_note}",
                          num(h), num(kk), num(aa), num(bb), num(a), num(b), num(c), num(f1.0), num(f1.1), num(f2.0),
                          num(f2.1), num(e)), &p.body()))
}

// ------------------------------------------------------------------------------------------------ area model
/// The area model of g(px + q)(rx + s): sides p x and q, r x and s; each cell the product of its side terms.
fn area_model(s: &Solved) -> Option<Figure> {
    let (a, b, c) = (get(s, "a")?, get(s, "b")?, get(s, "c")?);
    let (g, pp, qq, rr, ss) = (s.out("g")?, s.out("p")?, s.out("q")?, s.out("r")?, s.out("s")?);
    let fact = s.out_text("factored")?;
    if [pp, qq, rr, ss].iter().any(|v| v.abs() > 1e4) {
        return None;
    }
    let term = |co: f64, x: bool| -> String {
        if x { if co == 1.0 { "x".into() } else if co == -1.0 { "-x".into() } else { format!("{}x", num(co)) } } else { num(co) }
    };
    let mut cv = Canvas::new(380.0);
    let (x0, y0, wmax, hmax) = (130.0, 80.0, 330.0, 240.0);
    // side lengths drawn by magnitude at x = 1 (a zero term keeps a thin strip)
    let (w1, w2) = (rr.abs().max(0.3), ss.abs().max(0.3));
    let (h1, h2) = (pp.abs().max(0.3), qq.abs().max(0.3));
    let sx = wmax / (w1 + w2);
    let sy = hmax / (h1 + h2);
    let (cw1, cw2, ch1, ch2) = (w1 * sx, w2 * sx, h1 * sy, h2 * sy);
    let cells = [(0.0, 0.0, cw1, ch1, pp * rr, 2), (cw1, 0.0, cw2, ch1, pp * ss, 1), (0.0, ch1, cw1, ch2, qq * rr, 1),
                 (cw1, ch1, cw2, ch2, qq * ss, 0)];
    for (dx, dy, w, h, v, deg) in cells {
        let class = if v < 0.0 { "axp-s3" } else { "axp-s" };
        cv.raw(&format!("<rect class=\"{class}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{w:.1}\" height=\"{h:.1}\"/>\
                         <rect class=\"axp-k\" x=\"{:.1}\" y=\"{:.1}\" width=\"{w:.1}\" height=\"{h:.1}\"/>",
                        x0 + dx, y0 + dy, x0 + dx, y0 + dy));
        let t = match deg {
            2 => format!("{}x²", num(v)),
            1 => format!("{}x", num(v)),
            _ => num(v),
        };
        cv.value(x0 + dx + w / 2.0, y0 + dy + h / 2.0 + 5.0, "middle", &t);
    }
    // the side terms
    cv.bold(x0 + cw1 / 2.0, y0 - 10.0, "middle", &term(rr, true));
    cv.bold(x0 + cw1 + cw2 / 2.0, y0 - 10.0, "middle", &term(ss, false));
    cv.bold(x0 - 10.0, y0 + ch1 / 2.0 + 5.0, "end", &term(pp, true));
    cv.bold(x0 - 10.0, y0 + ch1 + ch2 / 2.0 + 5.0, "end", &term(qq, false));
    let quad = format!("{}x² + {}x + {}", num(a), num(b), num(c)).replace("+ -", "- ");
    cv.bold(16.0, 24.0, "start", &quad);
    cv.value(16.0, 44.0, "start", &format!("= {fact}"));
    cv.name(504.0, 370.0, "end", if g != 1.0 { "inside the common factor g; sides drawn at x = 1" } else { "sides drawn by size at x = 1" });
    Some(s.figure("area", "Area model of the factored quadratic",
                  &format!("Area model of {quad} with the factors the model returned, {fact}: the sides of the rectangle \
                            are {} and {}; each cell is the product of its side terms (cells of negative sign in the second \
                            colour) and together they make {}the quadratic. Side lengths drawn by size at x = 1.",
                           format!("({} + {})", term(pp, true), term(qq, false)).replace("+ -", "- "),
                           format!("({} + {})", term(rr, true), term(ss, false)).replace("+ -", "- "),
                           if g != 1.0 { format!("{} times ", num(g)) } else { String::new() }),
                  cv.body(), cv.h))
}

// ------------------------------------------------------------------------------------------------ sequences
/// The terms g_m (stems) and partial sums s_m (dots) for m = 1 ... N, each a run of the model.
fn sequence_stems(s: &Solved) -> Option<Figure> {
    let (g1, q, n) = (get(s, "g1")?, get(s, "q")?, get(s, "n")?);
    if n < 1.0 || n.fract() != 0.0 {
        return None;
    }
    let top = (n as usize).clamp(8, 30);
    let mut pts = vec![];
    for m in 1..=top {
        let r = s.with(&[("n", m as f64)])?;
        pts.push((m as f64, num_of(&r, "gn")?, num_of(&r, "sn")?));
    }
    let ys: Vec<f64> = pts.iter().flat_map(|p| [p.1, p.2]).chain([0.0]).collect();
    let (lo, hi) = ys.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
    let mut p = DPlot::new((0.5, top as f64 + 0.5), (lo, hi), "Term number m", "Value",
                          &format!("g1 = {}, q = {}", num(g1), num(q)));
    p.hline(0.0);
    for (m, g, sm) in &pts {
        p.seg((*m, 0.0), (*m, *g), 0, false);
        p.dot(*m, *g, 0, 3.5);
        p.dot(*m, *sm, 1, 4.0);
    }
    p.key("term g_m", 0);
    p.key("partial sum s_m", 1);
    let (gn, sn) = (s.out("gn")?, s.out("sn")?);
    if (n as usize) <= top {
        p.mark(n, sn, &format!("s{} = {}", n, num(sn)));
    }
    let lim = if q.abs() < 1.0 { format!(" With |q| < 1 the partial sums settle on {} (a run of the model's infinite series).",
        s.solve_other("infinite_geometric_series", &[("a", Input::Float(g1)), ("r", Input::Float(q))])
            .and_then(|r| num_of(&r, "sum")).map(num).unwrap_or_else(|| "a limit".into())) } else { String::new() };
    Some(s.chart("stems", "A geometric sequence and its partial sums",
                 &format!("The geometric sequence with g1 = {}, q = {}: its terms g_m (stems) and partial sums s_m (dots) \
                           for m = 1 to {top}, each a run of the model; g{} = {}, s{} = {}.{lim}", num(g1), num(q), n,
                          num(gn), n, num(sn)), &p.body()))
}

// ------------------------------------------------------------------------------------------------ matrix and tables
/// The augmented matrix beside the solution returned.
fn system_matrix(s: &Solved) -> Option<Figure> {
    let m = s.mat("augmented")?;
    if m.len() != 3 || m.iter().any(|r| r.len() != 4) {
        return None;
    }
    let (x, y, z) = (s.out("x")?, s.out("y")?, s.out("z")?);
    let sol = [x, y, z];
    let rows: Vec<Vec<f64>> = m.iter().enumerate().map(|(i, r)| {
        let mut v = r.clone();
        v.push(f64::NAN);
        v.push(sol[i]);
        v
    }).collect();
    let rn: Vec<String> = (1..=3).map(|i| format!("equation {i}")).collect();
    let cn: Vec<String> = ["x", "y", "z", "=", "", "solution"].iter().map(|c| c.to_string()).collect();
    let body = k::heat(&rows, &rn, &cn, &format!("x = {}, y = {}, z = {}", num(x), num(y), num(z)));
    Some(s.chart("matrix", "System of three linear equations: the augmented matrix and the solution",
                 &format!("The augmented matrix as read (one row per equation: the coefficients of x, y, z and the right \
                           side), each cell shaded by its size (negative values in the second colour), and beside it the \
                           solution returned: x = {}, y = {}, z = {}.", num(x), num(y), num(z)), &body))
}

/// The inputs as stated and the results returned, as a table.
fn io_table(s: &Solved) -> Option<Figure> {
    let mut rows: Vec<Vec<String>> = vec![];
    for (name, v) in s.given {
        let t = match v {
            Input::Text(t) => t.clone(),
            Input::Float(x) => num(*x),
            Input::Int(i) => i.to_string(),
            Input::Json(j) => j.to_string(),
        };
        rows.push(vec!["stated".into(), name.clone(), t]);
    }
    let mut shown = vec![];
    for (name, v, _) in s.a.values.iter() {
        let t = match v {
            serde_json::Value::Number(n) => num(n.as_f64()?),
            serde_json::Value::Array(a) => format!("[{}]", a.iter().map(|x| x.as_f64().map(num).unwrap_or_else(|| x.to_string()))
                .collect::<Vec<_>>().join(", ")),
            serde_json::Value::String(t) => t.clone(),
            serde_json::Value::Bool(b) => if *b { "yes".into() } else { "no".into() },
            _ => continue,
        };
        shown.push(format!("{name} = {t}"));
        rows.push(vec!["returned".into(), name.clone(), t]);
    }
    if shown.is_empty() || rows.len() > 14 {
        return None;
    }
    let mark = rows.iter().position(|r| r[0] == "returned");
    let head = ["", "quantity", "value"].iter().map(|c| c.to_string()).collect::<Vec<_>>();
    // a long expression is written whole above the table (a cell shows its start only)
    let top = match s.key {
        "order_of_operations" => s.txt("expression").filter(|e| e.chars().count() <= 48).unwrap_or_default(),
        _ => String::new(),
    };
    let (body, h) = k::table(&head, &rows, mark, &top);
    let title = match s.key {
        "order_of_operations" => "Evaluate an arithmetic expression: inputs and results",
        _ => "Radical equation sqrt(ax + b) = cx + d: inputs and results",
    };
    Some(s.figure("io", title, &format!("The inputs as stated and the results the model returned: {}.", shown.join(", ")),
                  &body, h.max(140.0)))
}

// ------------------------------------------------------------------------------------------------ unit circle
/// The angles the model gives exact values for (multiples of 30° and 45° in one turn).
fn special_angles() -> Vec<f64> {
    let mut v: Vec<f64> = (0..12).map(|i| i as f64 * 30.0).chain((0..8).map(|i| i as f64 * 45.0)).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    v.dedup();
    v
}

/// The exact sine at every special angle of one turn, each a run of the model; bands of one verdict share a colour.
fn unit_decision(s: &Solved) -> Option<Figure> {
    let deg = get(s, "deg")?;
    let mine = s.out_text("sin_exact")?;
    let mut pts: Vec<(f64, f64, String)> = vec![];
    for d in special_angles() {
        let r = s.with(&[("deg", d)])?;
        let t = match r.values.iter().find(|x| x.0 == "sin_exact").map(|x| &x.1) {
            Some(serde_json::Value::String(t)) => t.clone(),
            _ => continue,
        };
        pts.push((d, num_of(&r, "sin")?, t));
    }
    if pts.len() < 4 {
        return None;
    }
    let mut verdicts: Vec<String> = vec![];
    let mut p = DPlot::new((0.0, 360.0), (-1.25, 1.25), "Angle in degrees", "sin", &format!("sin {}° = {}", num(deg), pretty(&mine)));
    p.hline(0.0);
    for (i, (d, v, t)) in pts.iter().enumerate() {
        let kx = match verdicts.iter().position(|x| x == t) {
            Some(j) => j,
            None => {
                verdicts.push(t.clone());
                verdicts.len() - 1
            }
        };
        // the band half way to each neighbour
        let l = if i == 0 { 0.0 } else { (pts[i - 1].0 + d) / 2.0 };
        let r = if i + 1 == pts.len() { 360.0 } else { (pts[i + 1].0 + d) / 2.0 };
        let col = if *v > 1e-9 { 0 } else if *v < -1e-9 { 2 } else { 3 };
        p.fill(&[(l, 0.0), (r, 0.0), (r, *v), (l, *v)], col);
        p.dot(*d, *v, col, 4.0);
        let _ = kx;
        let up = if *v >= 0.0 { 0.16 } else { -0.12 };
        let lab_y = v + up + if i % 2 == 0 { 0.0 } else { 0.0 };
        if d.rem_euclid(30.0) == 0.0 {
            p.label(*d, lab_y, "middle", "axp-vt", &pretty(t));
        }
    }
    let r = s.with(&[("deg", deg.rem_euclid(360.0))]).and_then(|a| num_of(&a, "sin")).or_else(|| s.out("sin"))?;
    p.mark(deg.rem_euclid(360.0), r, &format!("{}°", num(deg)));
    Some(s.chart("decision", "Exact sine values on the unit circle: where the exact value changes",
                 &format!("The exact sine returned at every multiple of 30° and 45° in one turn ({} runs of the model), \
                           each angle's band shaded by sign (accent positive, violet negative, grey zero) and labelled \
                           with its exact value at the multiples of 30°; {} distinct exact values. Marked: deg = {} gives {}.",
                          pts.len(), verdicts.len(), num(deg), pretty(&mine)), &p.body()))
}

/// The unit circle with the exact cosine and sine at the special angles (each a run of the model).
fn unit_polar(s: &Solved) -> Option<Figure> {
    let deg = get(s, "deg")?;
    let (c0, s0) = (s.out("cos")?, s.out("sin")?);
    let (ce, se) = (pretty(&s.out_text("cos_exact")?), pretty(&s.out_text("sin_exact")?));
    let mut p = DPlot::equal(&[-1.25, 1.25], &[-1.25, 1.25], "cos", "sin", &format!("{}°: ({ce}, {se})", num(deg)));
    p.hline(0.0);
    p.vline(0.0);
    p.curve(&circle((0.0, 0.0), 1.0), 3);
    let mut n = 0;
    for d in special_angles() {
        let Some(r) = s.with(&[("deg", d)]) else { continue };
        let (Some(c), Some(sn)) = (num_of(&r, "cos"), num_of(&r, "sin")) else { continue };
        n += 1;
        p.dot(c, sn, 0, 3.5);
        // six labelled: the first quadrant's angles and the axes
        if [30.0, 45.0, 60.0, 90.0, 180.0, 0.0].contains(&d) {
            p.label(c * 1.12, sn * 1.12 + 0.02, if c >= -1e-9 { "start" } else { "end" }, "axp-lt", &format!("{}°", num(d)));
        }
    }
    p.seg((0.0, 0.0), (c0, s0), 0, false);
    p.seg((c0, 0.0), (c0, s0), 3, true);
    p.mark(c0, s0, &format!("({ce}, {se})"));
    Some(s.chart("polar", "The unit circle with exact values",
                 &format!("The unit circle with the cosine and sine the model returns at the multiples of 30° and 45° \
                           ({n} angles, six labelled); at {}°: cos = {ce} = {}, sin = {se} = {} (marked).", num(deg),
                          num(c0), num(s0)), &p.body()))
}
