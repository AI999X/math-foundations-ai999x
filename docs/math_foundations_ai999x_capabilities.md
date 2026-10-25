# math-foundations-ai999x: capability guide

**Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X. Ask the package `Who created you?` or call `math_foundations_ai999x.about()` for the same answer.

Every capability math-foundations-ai999x supports, generated from its certified capability contracts. Every example below was executed when this guide was built. The structured call `solve("<capability id>", ...)` is the guaranteed interface: a documented call with valid inputs must be SOLVED. Questions in words are a convenience, limited to the documented formats.

## Quick start

```python
import math_foundations_ai999x

result = math_foundations_ai999x.solve('quadratic_roots', a=1, b=-5, c=6)
result.show()
# x1 = 2, x2 = 3, discriminant = 1
```

## By topic

| topic | capabilities |
|---|---|
| Algebra / absolute value | [Absolute value inequality \|ax + b\| compared with c](#abs_linear_inequality) |
| Algebra / factoring | [Factor a quadratic over the integers](#factor_quadratic_integer) |
| Algebra / inequalities | [Quadratic inequality](#quadratic_inequality) |
| Algebra / polynomial division | [Synthetic division and the remainder theorem](#synthetic_division), [Polynomial long division](#polynomial_long_division) |
| Algebra / polynomial equations | [Rational roots of a polynomial (rational root theorem)](#rational_roots) |
| Algebra / radical equations | [Radical equation sqrt(ax + b) = cx + d](#radical_equation) |
| Algebra / rational equations | [Rational equation a/(x + b) = c/(x + d)](#rational_equation) |
| Algebra / rational expressions | [Partial fractions (two distinct linear factors)](#partial_fractions_linear) |
| Algebra / sequences and series | [Sum of an infinite geometric series](#infinite_geometric_series) |
| Algebra / systems of linear equations | [System of three linear equations](#linear_system_3x3) |
| Arithmetic / fractions | [Arithmetic with fractions](#fraction_arithmetic) |
| Arithmetic / order of operations | [Evaluate an arithmetic expression (order of operations)](#order_of_operations) |
| Arithmetic / percentages | [Percent of a number](#percent_of_number), [Whole from a part and its percentage](#percent_of_what), [What percent one number is of another](#what_percent), [Percent change (increase or decrease)](#percent_change) |
| Arithmetic / ratios and proportions | [Divide a quantity in a given ratio](#ratio_split), [Solve a proportion](#proportion_solve), [Unit rates and the better buy](#unit_rate_compare) |
| Arithmetic / rounding | [Round to decimal places](#round_decimal_places), [Round to significant figures](#round_significant_figures) |
| Arithmetic / scientific notation | [Scientific notation](#scientific_notation) |
| Complex numbers | [Product and modulus of complex numbers](#complex_product_modulus) |
| Complex numbers / arithmetic | [Arithmetic with complex numbers](#complex_arithmetic) |
| Complex numbers / De Moivre's theorem | [Power of a complex number (De Moivre)](#de_moivre_power) |
| Complex numbers / polar form | [Modulus, argument and polar form of a complex number](#complex_modulus_argument) |
| Complex numbers / roots | [The n-th roots of a complex number](#complex_nth_roots) |
| Conic sections | [Centre and radius of a circle from its general equation](#circle_from_general_form) |
| Coordinate plane | [Distance between two points](#distance_between_points) |
| Counting and probability | [Combinations and permutations](#counting_choices) |
| Exponential functions | [Compound interest](#compound_interest) |
| Exponential models | [Doubling time of exponential growth](#doubling_time) |
| Exponentials and logarithms / exponential equations | [Exponential equation a*b^(cx + d) = k](#exponential_equation_general) |
| Exponentials and logarithms / growth and decay | [Continuous exponential growth or decay](#continuous_growth), [Amount remaining after radioactive decay (half-life)](#half_life_remaining), [Doubling time at a percentage growth rate](#doubling_time_percent) |
| Exponentials and logarithms / laws of logarithms | [Laws of logarithms (product, quotient, power)](#log_laws_combine) |
| Exponentials and logarithms / logarithmic equations | [Logarithmic equation log b(cx + d) = k](#log_equation_linear_argument) |
| Exponentials and logarithms / logarithms | [Evaluate a logarithm](#log_evaluate) |
| Exponents and logarithms | [Solution of an exponential equation](#exponential_equation) |
| Factoring | [Factoring a monic quadratic](#factor_monic_quadratic) |
| Functions | [Average rate of change of a function between two inputs](#average_rate_of_change_definition), [Composition of two linear functions](#compose_linear_functions) |
| Functions / composition | [Value of a composite function](#composite_function_value) |
| Functions / domain | [Domain of a square-root function](#domain_sqrt_over_linear) |
| Functions / evaluation | [Evaluate a function at a number](#evaluate_function) |
| Functions / inverse functions | [Inverse of a linear function](#inverse_linear_function) |
| Functions / lines | [Intercepts and slope of a line](#line_intercepts), [Distance from a point to a line](#distance_point_to_line) |
| Functions / piecewise functions | [Evaluate a piecewise function](#piecewise_evaluate) |
| Functions / transformations | [Image of a point under a graph transformation](#graph_transformation_point) |
| Inequalities | [Solution of a linear inequality](#linear_inequality) |
| Linear and absolute value inequalities | [Solution set of \|x - c\| > d](#abs_value_inequality_lower_bound), [Solution interval of \|x - c\| < d](#abs_value_inequality_upper_bound) |
| Linear equations | [Root of a linear equation](#linear_equation_root) |
| Linear functions | [Average rate of change between two points](#average_rate_of_change_linear), [Y-intercept of a line from its slope and a point](#convert_to_slope_intercept), [Line through two points](#line_through_two_points), [Slope of a perpendicular line](#perpendicular_lines_slope_product) |
| Logarithms | [Solution of a basic logarithmic equation](#logarithmic_equation) |
| Matrices | [Determinant and inverse of a 2 by 2 matrix](#matrix_2x2_inverse) |
| Matrices and linear systems | [Determinant of 2x2 matrix](#determinant_2x2_formula), [Matrix scalar multiple definition](#matrix_scalar_multiple_definition) |
| Other equations | [Solutions of an absolute value equation](#absolute_value_equation) |
| Polynomial equations / cubic and quartic equations | [Roots of a cubic equation](#cubic_roots), [Roots of a quartic equation](#quartic_roots) |
| Polynomial equations / quadratic equations | [Roots of a quadratic equation](#quadratic_roots), [Discriminant of a quadratic](#quadratic_discriminant) |
| Polynomials | [Product of two binomials](#binomial_product) |
| Precalculus / binomial theorem | [Coefficient of a term in a binomial expansion](#binomial_coefficient_term) |
| Precalculus / conic sections | [Focus and directrix of a parabola](#parabola_focus_directrix), [Foci, axes and eccentricity of an ellipse](#ellipse_parameters), [Foci, eccentricity and asymptotes of a hyperbola](#hyperbola_parameters) |
| Precalculus / polar coordinates | [Polar to rectangular coordinates](#polar_to_rectangular), [Rectangular to polar coordinates](#rectangular_to_polar) |
| Precalculus / vectors | [Dot product and angle between two vectors](#vector_dot_angle), [Magnitude and direction of a vector](#vector_magnitude_direction) |
| Proofs and derivations | [Area of rectangle](#area_of_rectangle), [Associative property of addition](#associative_property_of_addition), [Cramer rule x solution](#cramer_rule_x_solution), [Determinant cramer two variables](#determinant_cramer_two_variables), [Difference of cubes factorization](#difference_of_cubes_factorization), [Difference of squares](#difference_of_squares), [Distributive property verification](#distributive_property_verification), [Elimination method identity](#elimination_method_identity), [Elimination solution two variables](#elimination_solution_two_variables), [Linear equation solution formula](#linear_equation_solution_formula), [Partial fraction decomposition linear factors](#partial_fraction_decomposition_linear_factors), [Perfect square trinomial](#perfect_square_trinomial), [Product of sums](#product_of_sums), [Quadratic vertex form conversion](#quadratic_vertex_form_conversion), [Radical equation squaring](#radical_equation_squaring), [Square of difference](#square_of_difference), [Substitution method identity](#substitution_method_identity), [Substitution solution two variables](#substitution_solution_two_variables), [Sum of cubes factorization](#sum_of_cubes_factorization) |
| Quadratic functions | [Vertex form of a quadratic function](#vertex_form) |
| Rational expressions | [Add rational expressions with same denominator](#add_rational_expressions_same_denom), [Multiply rational expressions](#multiply_rational_expressions) |
| Rational functions | [Asymptotes of a linear rational function](#rational_function_asymptotes) |
| Real numbers and algebraic expressions | [Perimeter of a rectangle](#perimeter_of_rectangle) |
| Sequences | [Term and partial sum of an arithmetic sequence](#arithmetic_sequence), [Term and partial sum of a geometric sequence](#geometric_sequence) |
| Systems of equations | [Solution of two linear equations in two unknowns](#linear_system_2x2) |
| The coordinate plane and graphs | [Midpoint of a line segment](#midpoint_of_segment) |
| Trigonometry / angle measure | [Degrees to radians](#deg_to_rad), [Radians to degrees](#rad_to_deg) |
| Trigonometry / identities | [Trigonometric values from one value and the quadrant](#trig_values_from_one) |
| Trigonometry / oblique triangles | [Law of sines (two angles and a side)](#law_of_sines_aas), [Law of sines, the ambiguous case (two sides and a non-included angle)](#law_of_sines_ssa), [Law of cosines (two sides and the included angle)](#law_of_cosines_sas), [Law of cosines (three sides)](#law_of_cosines_sss) |
| Trigonometry / right triangles | [Right triangle from its two legs](#right_triangle_legs), [Right triangle from a leg and the hypotenuse](#right_triangle_leg_hypotenuse), [Right triangle from an acute angle and a side](#right_triangle_angle_side) |
| Trigonometry / triangle area | [Area of a triangle from its three sides (Heron)](#heron_area) |
| Trigonometry / trigonometric equations | [Solve a trigonometric equation over one full turn](#trig_equation_interval) |
| Trigonometry / unit circle | [Exact trigonometric values on the unit circle](#unit_circle_exact) |

## Outcomes

| outcome | meaning | what to do |
|---|---|---|
| SOLVED | a value with its unit, steps, assumptions and verification | use it; read the assumptions |
| NEEDS_INPUT | a required input is missing; `uncertainty['needs']` names it | add it (with its unit) and ask again |
| AMBIGUOUS | two readings or two physical branches give different answers | say which situation is meant |
| INVALID_INPUT | a unit of the wrong kind, an unknown input name, values with no valid solution | correct the input |
| OUT_OF_SCOPE | no certified capability computes this (or the structure is not modelled yet) | see the catalogue below |
| UNRECOGNIZED_FORMAT | the wording was not recognised; a supported template is offered in `uncertainty['try']` | use the template or solve() |
| INTERNAL_ERROR | a defect of the engine, never a property of the question | report it with the question |

## Result fields

| field | meaning |
|---|---|
| value, unit | the result in SI units |
| values | every output of a procedure (x1, x2, discriminant ...) |
| outcome | one of the seven outcomes above |
| steps | each equation used, its inputs (with where each came from) and its result |
| assumptions | conditions and declared defaults the answer relies on |
| verification | residual of every equation, dimensional consistency, independent checks |
| latex() | the derivation as LaTeX |
| show() | renders in a notebook, prints in a terminal |

## Common mistakes

- Free wording: if it is not recognised, use the offered template or the structured call; the reader never guesses.
- A coefficient that violates a stated condition (a = 0 in a quadratic): INVALID_INPUT says why.
- A text value where a number is expected: INVALID_INPUT; fractions as strings ("1/2") are accepted.
- A problem outside the procedure (a cubic for the quadratic solver): OUT_OF_SCOPE, never an approximation.

## Procedures: algorithms with several outputs

Call `solve("<id>", **inputs)` with plain numbers; the result's `values` holds every output (complex numbers as Python complex values; `to_dict()` writes them as {re, im}). Each result is substituted back into its defining equation before it is returned.

| id | operation | inputs | outputs |
|---|---|---|---|
| quadratic_roots | Roots of a quadratic equation | a, b, c | x1, x2, discriminant |
| quadratic_discriminant | Discriminant of a quadratic | a, b, c | discriminant, nature |
| counting_choices | Combinations and permutations | n, k | combinations, permutations |
| cubic_roots | Roots of a cubic equation | a, b, c, d | x1, x2, x3 |
| quartic_roots | Roots of a quartic equation | a, b, c, d, e | x1, x2, x3, x4 |
| fraction_arithmetic | Arithmetic with fractions | a, b, c, d, op | numerator, denominator, value |
| percent_of_number | Percent of a number | p, x | result |
| percent_of_what | Whole from a part and its percentage | part, p | whole |
| what_percent | What percent one number is of another | part, whole | percent |
| percent_change | Percent change (increase or decrease) | old, new | percent_change, difference, direction |
| ratio_split | Divide a quantity in a given ratio | total, a, b, c | part1, part2, part3 |
| proportion_solve | Solve a proportion | p, q, r, position | x |
| unit_rate_compare | Unit rates and the better buy | n1, c1, n2, c2 | unit_rate_1, unit_rate_2, better |
| round_decimal_places | Round to decimal places | x, n | rounded |
| round_significant_figures | Round to significant figures | x, n | rounded |
| scientific_notation | Scientific notation | x | mantissa, exponent |
| order_of_operations | Evaluate an arithmetic expression (order of operations) | expression | value, numerator, denominator |
| linear_system_3x3 | System of three linear equations | augmented | x, y, z |
| abs_linear_inequality | Absolute value inequality \|ax + b\| compared with c | a, b, c, rel | lower, upper, region, solution |
| rational_equation | Rational equation a/(x + b) = c/(x + d) | a, b, c, d | x |
| radical_equation | Radical equation sqrt(ax + b) = cx + d | a, b, c, d | count, solutions, extraneous |
| synthetic_division | Synthetic division and the remainder theorem | coefficients, r | quotient, remainder |
| polynomial_long_division | Polynomial long division | dividend, divisor | quotient, remainder |
| factor_quadratic_integer | Factor a quadratic over the integers | a, b, c | g, p, q, r, s, factored |
| quadratic_inequality | Quadratic inequality | a, b, c, rel | r1, r2, region, solution |
| infinite_geometric_series | Sum of an infinite geometric series | a, r | sum |
| partial_fractions_linear | Partial fractions (two distinct linear factors) | p, q, r1, r2 | A, B |
| rational_roots | Rational roots of a polynomial (rational root theorem) | coefficients | count, roots, roots_exact |
| evaluate_function | Evaluate a function at a number | expression, x | value, numerator, denominator |
| composite_function_value | Value of a composite function | f, g, x, order | inner, value |
| domain_sqrt_over_linear | Domain of a square-root function | a, b, c, d, root_in_denominator | boundary, side, excluded, domain |
| inverse_linear_function | Inverse of a linear function | a, b, y | inv_slope, inv_intercept, value |
| piecewise_evaluate | Evaluate a piecewise function | e1, left_rel, c, e2, x | value, piece |
| graph_transformation_point | Image of a point under a graph transformation | x0, y0, A, B, h, k | x_new, y_new |
| line_intercepts | Intercepts and slope of a line | a, b, c | x_intercept, y_intercept, slope |
| distance_point_to_line | Distance from a point to a line | x0, y0, a, b, c | distance |
| log_evaluate | Evaluate a logarithm | b, x | value |
| log_laws_combine | Laws of logarithms (product, quotient, power) | u, v, p, q | value |
| continuous_growth | Continuous exponential growth or decay | n0, k, t | amount |
| half_life_remaining | Amount remaining after radioactive decay (half-life) | n0, half_life, t | remaining, fraction |
| doubling_time_percent | Doubling time at a percentage growth rate | r | doubling_time, rule_of_70_estimate |
| exponential_equation_general | Exponential equation a*b^(cx + d) = k | a, b, c, d, k | x |
| log_equation_linear_argument | Logarithmic equation log_b(cx + d) = k | b, c, d, k | x |
| deg_to_rad | Degrees to radians | deg | radians, pi_multiple, exact |
| rad_to_deg | Radians to degrees | rad | degrees |
| right_triangle_legs | Right triangle from its two legs | a, b | c, A, B |
| right_triangle_leg_hypotenuse | Right triangle from a leg and the hypotenuse | a, c | b, A, B |
| right_triangle_angle_side | Right triangle from an acute angle and a side | A, side, which | opposite, adjacent, hypotenuse, B |
| law_of_sines_aas | Law of sines (two angles and a side) | A, B, a | C, b, c |
| law_of_sines_ssa | Law of sines, the ambiguous case (two sides and a non-included angle) | a, b, A | count, B1, C1, c1, B2, C2, c2 |
| law_of_cosines_sas | Law of cosines (two sides and the included angle) | b, c, A | a, B, C |
| law_of_cosines_sss | Law of cosines (three sides) | a, b, c | A, B, C |
| heron_area | Area of a triangle from its three sides (Heron) | a, b, c | area, s |
| unit_circle_exact | Exact trigonometric values on the unit circle | deg | sin, cos, tan, sin_exact, cos_exact, tan_exact, reference_angle |
| trig_equation_interval | Solve a trigonometric equation over one full turn | func, k, n, unit | count, solutions |
| trig_values_from_one | Trigonometric values from one value and the quadrant | func, value, quadrant | sin, cos, tan, sin2, cos2 |
| vector_dot_angle | Dot product and angle between two vectors | u, v | dot, mag_u, mag_v, angle_deg |
| vector_magnitude_direction | Magnitude and direction of a vector | x, y | magnitude, direction_deg |
| polar_to_rectangular | Polar to rectangular coordinates | r, theta, unit | x, y |
| rectangular_to_polar | Rectangular to polar coordinates | x, y | r, theta_deg, theta_rad |
| parabola_focus_directrix | Focus and directrix of a parabola | orientation, m, h, k | p, focus_x, focus_y, directrix, vertex_x, vertex_y |
| ellipse_parameters | Foci, axes and eccentricity of an ellipse | h, k, A, B | a, b, c, eccentricity, f1x, f1y, f2x, f2y, orientation |
| hyperbola_parameters | Foci, eccentricity and asymptotes of a hyperbola | h, k, A, B, orientation | a, b, c, eccentricity, asymptote_slope, f1x, f1y, f2x, f2y |
| binomial_coefficient_term | Coefficient of a term in a binomial expansion | a, b, n, k | coefficient, binomial |
| complex_arithmetic | Arithmetic with complex numbers | a, b, c, d, op | re, im |
| complex_modulus_argument | Modulus, argument and polar form of a complex number | a, b | modulus, argument_deg, argument_rad, polar |
| de_moivre_power | Power of a complex number (De Moivre) | a, b, n | re, im, modulus, argument_deg |
| complex_nth_roots | The n-th roots of a complex number | a, b, n | modulus, count, roots |
| abs_value_inequality_lower_bound | Solution set of \|x - c\| > d | c, d | left_limit, right_limit |
| abs_value_inequality_upper_bound | Solution interval of \|x - c\| < d | c, d | lower, upper |
| absolute_value_equation | Solutions of an absolute value equation | a, b, c | x1, x2 |
| add_rational_expressions_same_denom | Add rational expressions with same denominator | num1, num2, den | sum_num, sum_den |
| arithmetic_sequence | Term and partial sum of an arithmetic sequence | a1, d, n | an, sn |
| average_rate_of_change_definition | Average rate of change of a function between two inputs | x_start, f_x_start, x_end, f_x_end | arc |
| average_rate_of_change_linear | Average rate of change between two points | x1, y1, x2, y2 | m |
| binomial_product | Product of two binomials | a, b, c, d | p2, p1, p0 |
| circle_from_general_form | Centre and radius of a circle from its general equation | dd, ee, ff | h, k, rad |
| complex_product_modulus | Product and modulus of complex numbers | a, b, c, d | re, im, modulus |
| compose_linear_functions | Composition of two linear functions | a, b, c, d | m, k |
| compound_interest | Compound interest | principal, r, n, t | amount |
| convert_to_slope_intercept | y-intercept of a line from its slope and a point | m, x1, y1 | b |
| determinant_2x2_formula | Determinant of 2x2 matrix | a, b, c, d | det |
| distance_between_points | Distance between two points | x1, y1, x2, y2 | dist |
| doubling_time | Doubling time of exponential growth | k | td |
| exponential_equation | Solution of an exponential equation | b, y | x |
| factor_monic_quadratic | Factoring a monic quadratic | b, c | p, q |
| geometric_sequence | Term and partial sum of a geometric sequence | g1, q, n | gn, sn |
| line_through_two_points | Line through two points | x1, y1, x2, y2 | m, k |
| linear_equation_root | Root of a linear equation | a, b, c | x |
| linear_inequality | Solution of a linear inequality | a, b, c | x0, s |
| linear_system_2x2 | Solution of two linear equations in two unknowns | a, b, c, d, e, f | x, y |
| logarithmic_equation | Solution of a basic logarithmic equation | b, c | x |
| matrix_2x2_inverse | Determinant and inverse of a 2 by 2 matrix | a, b, c, d | det, i11, i12, i21, i22 |
| matrix_scalar_multiple_definition | Matrix scalar multiple definition | s, a11, a12, a21, a22 | b11, b12, b21, b22 |
| midpoint_of_segment | Midpoint of a line segment | x1, y1, x2, y2 | mx, my |
| multiply_rational_expressions | Multiply rational expressions | num1, den1, num2, den2 | prod_num, prod_den |
| perimeter_of_rectangle | Perimeter of a rectangle | w, h | p |
| perpendicular_lines_slope_product | Slope of a perpendicular line | m | m_perp |
| rational_function_asymptotes | Asymptotes of a linear rational function | a, b, c, d | va, ha |
| vertex_form | Vertex form of a quadratic function | a, b, c | h, k |

<a id="quadratic_roots"></a>

### Roots of a quadratic equation — `quadratic_roots`

Solve $a x^{2} + b x + c = 0$ for x: both roots, real or complex.

$$
x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of $x^{2}$ | $a  \neq  0$ |
| `b` | coefficient of x |  |
| `c` | constant term |  |

| output | meaning |
|---|---|
| `x1` | first root (the smaller real root, or the one with positive imaginary part) |
| `x2` | second root |
| `discriminant` | $b^{2} - 4ac$ |

- **Method:** quadratic formula in a numerically stable form (q = -(b + sign(b)*sqrt(b^2 - 4ac))/2, $x = q/a$ and c/q)
- **Assumes:** $a  \neq  0$ (otherwise the equation is linear); real coefficients
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('quadratic_roots', a=1, b=-5, c=6)
# -> x1 = 2, x2 = 3, discriminant = 1   (SOLVED)
```

```python
math_foundations_ai999x.solve('quadratic_roots', a=2, b=3, c=-2)
# -> x1 = -2, x2 = 0.5, discriminant = 25   (SOLVED)
```

```python
math_foundations_ai999x.solve('quadratic_roots', a=1, b=2, c=1)
# -> x1 = -1, x2 = -1, discriminant = 0   (SOLVED)
```

```python
math_foundations_ai999x.solve('quadratic_roots', a=1, b=0, c=4)
# -> x1 = 0 + 2i, x2 = 0 - 2i, discriminant = -16   (SOLVED)
```

```python
math_foundations_ai999x.solve('quadratic_roots', a=1, b=-100000000.0, c=1)
# -> x1 = 1e-08, x2 = 1e+08, discriminant = 1e+16   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve x^2 - 5x + 6 = 0. | SOLVED | x1 = 2, x2 = 3, discriminant = 1 |
| Find the roots of 2x^2 + 3x - 2 = 0. | SOLVED | x1 = -2, x2 = 0.5, discriminant = 25 |
| Solve the quadratic with a = 1, b = -5, c = 6. | SOLVED | x1 = 2, x2 = 3, discriminant = 1 |
| Solve x^2 + 4 = 0. | SOLVED | x1 = 0 + 2i, x2 = 0 - 2i, discriminant = -16 |
| Solve 3x^2 = 12. | SOLVED | x1 = -2, x2 = 2, discriminant = 144 |
| Solve x^5 - 1 = 0. | OUT_OF_SCOPE | Out of scope: a polynomial of degree 5 or higher (equations up to degree 4 are supported). |
| Solve the quadratic with a = 0, b = 2, c = 1. | INVALID_INPUT | Roots of a quadratic equation: a must not be 0: with a = 0 the equation is linear, not quadratic. |
| Solve the quadratic with a = 1, b = 2. | NEEDS_INPUT | quadratic_roots needs c (constant term). Example: solve('quadratic_roots', a=1, b=-5, c=6). |

<a id="quadratic_discriminant"></a>

### Discriminant of a quadratic — `quadratic_discriminant`

The discriminant $b^{2} - 4ac$ of $a x^{2} + b x + c$ and the nature of the roots it implies.

$$
\Delta = b^2 - 4ac
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of $x^{2}$ | $a  \neq  0$ |
| `b` | coefficient of x |  |
| `c` | constant term |  |

| output | meaning |
|---|---|
| `discriminant` | $b^{2} - 4ac$ |
| `nature` | two distinct real / one repeated real / two complex conjugate roots |

- **Method:** direct evaluation of $b^{2} - 4ac$ and its sign
- **Assumes:** $a  \neq  0$; real coefficients
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('quadratic_discriminant', a=1, b=4, c=4)
# -> discriminant = 0, nature = one repeated real root   (SOLVED)
```

```python
math_foundations_ai999x.solve('quadratic_discriminant', a=2, b=1, c=-3)
# -> discriminant = 25, nature = two distinct real roots   (SOLVED)
```

```python
math_foundations_ai999x.solve('quadratic_discriminant', a=1, b=1, c=1)
# -> discriminant = -3, nature = two complex conjugate roots   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the discriminant of x^2 + 4x + 4? | SOLVED | discriminant = 0, nature = one repeated real root |
| Find the discriminant with a = 2, b = 1, c = -3. | SOLVED | discriminant = 25, nature = two distinct real roots |

<a id="counting_choices"></a>

### Combinations and permutations — `counting_choices`

The number of ways to choose k items from n without order (combinations) and with order (permutations).

$$
\binom{n}{k} = \frac{n!}{k!\,(n-k)!},\quad P(n,k) = \frac{n!}{(n-k)!}
$$

| input | meaning | condition |
|---|---|---|
| `n` | number of items to choose from (a whole number) |  |
| `k` | number of items chosen (a whole number, k <= n) |  |

| output | meaning |
|---|---|
| `combinations` | C(n, k) = n! / (k! (n - k)!) |
| `permutations` | P(n, k) = n! / (n - k)! |

- **Method:** exact integer arithmetic (math.comb, math.perm)
- **Assumes:** n and k whole numbers; $k  \leq  n$; $n  \leq  170$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('counting_choices', n=10, k=3)
# -> combinations = 120, permutations = 720   (SOLVED)
```

```python
math_foundations_ai999x.solve('counting_choices', n=5, k=5)
# -> combinations = 1, permutations = 120   (SOLVED)
```

```python
math_foundations_ai999x.solve('counting_choices', n=52, k=5)
# -> combinations = 2.59896e+06, permutations = 3.11875e+08   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How many ways are there to choose 3 books from 10? | SOLVED | combinations = 120, permutations = 720 |
| Compute C(6, 2). | SOLVED | combinations = 15, permutations = 30 |
| How many ways are there to choose 7 books from 4? | INVALID_INPUT | Combinations and permutations: k must not exceed n. |
| Compute C(1000, 3). | INVALID_INPUT | Combinations and permutations: n must be at most 170 (the result would not fit in a floating-point number). |

<a id="cubic_roots"></a>

### Roots of a cubic equation — `cubic_roots`

Solve $a x^{3} + b x^{2} + c x + d = 0$ for x: all three roots, real or complex.

$$
a x^3 + b x^2 + c x + d = 0
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of $x^{3}$ | $a  \neq  0$ |
| `b` | coefficient of $x^{2}$ |  |
| `c` | coefficient of x |  |
| `d` | constant term |  |

| output | meaning |
|---|---|
| `x1` | first root (real roots ascending, then complex roots) |
| `x2` | second root |
| `x3` | third root |

- **Method:** high-precision polynomial root finding (Durand-Kerner, 30 digits), each root substituted back
- **Assumes:** $a  \neq  0$ (otherwise the equation is not cubic); real coefficients
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('cubic_roots', a=1, b=-6, c=11, d=-6)
# -> x1 = 1, x2 = 2, x3 = 3   (SOLVED)
```

```python
math_foundations_ai999x.solve('cubic_roots', a=2, b=-4, c=-22, d=24)
# -> x1 = -3, x2 = 1, x3 = 4   (SOLVED)
```

```python
math_foundations_ai999x.solve('cubic_roots', a=1, b=0, c=0, d=-1)
# -> x1 = 1, x2 = -0.5 + 0.866025i, x3 = -0.5 - 0.866025i   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve x^3 - 6x^2 + 11x - 6 = 0. | SOLVED | x1 = 1, x2 = 2, x3 = 3 |
| Solve x^3 - 1 = 0. | SOLVED | x1 = 1, x2 = -0.5 + 0.866025i, x3 = -0.5 - 0.866025i |
| Find the roots of 2x^3 - 4x^2 - 22x + 24 = 0. | SOLVED | x1 = -3, x2 = 1, x3 = 4 |
| Solve the cubic with a = 1, b = 0, c = 0, d = -1. | SOLVED | x1 = 1, x2 = -0.5 + 0.866025i, x3 = -0.5 - 0.866025i |
| Solve the cubic with a = 0, b = 1, c = 2, d = 1. | INVALID_INPUT | Roots of a cubic equation: a must not be 0: with a = 0 the equation has a lower degree. |
| Solve the cubic with a = 1, b = 2. | NEEDS_INPUT | cubic_roots needs c, d (coefficient of x; constant term). Example: solve('cubic_roots', a=1, b=-6, c=11, d=-6). |

<a id="quartic_roots"></a>

### Roots of a quartic equation — `quartic_roots`

Solve $a x^{4} + b x^{3} + c x^{2} + d x + e = 0$ for x: all four roots, real or complex.

$$
a x^4 + b x^3 + c x^2 + d x + e = 0
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of $x^{4}$ | $a  \neq  0$ |
| `b` | coefficient of $x^{3}$ |  |
| `c` | coefficient of $x^{2}$ |  |
| `d` | coefficient of x |  |
| `e` | constant term |  |

| output | meaning |
|---|---|
| `x1` | first root (real roots ascending, then complex roots) |
| `x2` | second root |
| `x3` | third root |
| `x4` | fourth root |

- **Method:** high-precision polynomial root finding (Durand-Kerner, 30 digits), each root substituted back
- **Assumes:** $a  \neq  0$ (otherwise the equation is not quartic); real coefficients
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('quartic_roots', a=1, b=0, c=-5, d=0, e=4)
# -> x1 = -2, x2 = -1, x3 = 1, x4 = 2   (SOLVED)
```

```python
math_foundations_ai999x.solve('quartic_roots', a=1, b=0, c=0, d=0, e=-1)
# -> x1 = -1, x2 = 1, x3 = 0 + 1i, x4 = 0 - 1i   (SOLVED)
```

```python
math_foundations_ai999x.solve('quartic_roots', a=1, b=-10, c=35, d=-50, e=24)
# -> x1 = 1, x2 = 2, x3 = 3, x4 = 4   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve x^4 - 5x^2 + 4 = 0. | SOLVED | x1 = -2, x2 = -1, x3 = 1, x4 = 2 |
| Find the roots of x^4 - 1 = 0. | SOLVED | x1 = -1, x2 = 1, x3 = 0 + 1i, x4 = 0 - 1i |
| Solve the quartic with a = 1, b = 0, c = -5, d = 0, e = 4. | SOLVED | x1 = -2, x2 = -1, x3 = 1, x4 = 2 |
| Solve the quartic with a = 1, b = 2. | NEEDS_INPUT | quartic_roots needs c, d, e (coefficient of x^2; coefficient of x; constant term). Example: solve('quartic_roots', a=1,  |

<a id="fraction_arithmetic"></a>

### Arithmetic with fractions — `fraction_arithmetic`

Add, subtract, multiply or divide two fractions a/b and c/d; the result in lowest terms.

$$
\frac{a}{b} \pm \frac{c}{d} = \frac{ad \pm cb}{bd},\quad \frac{a}{b}\cdot\frac{c}{d} = \frac{ac}{bd},\quad \frac{a}{b} \div \frac{c}{d} = \frac{ad}{bc}
$$

| input | meaning | condition |
|---|---|---|
| `a` | numerator of the first fraction |  |
| `b` | denominator of the first fraction (not 0) |  |
| `c` | numerator of the second fraction |  |
| `d` | denominator of the second fraction (not 0) |  |
| `op` | the operation |  |

| output | meaning |
|---|---|
| `numerator` | numerator of the result in lowest terms |
| `denominator` | denominator of the result (positive) |
| `value` | the result as a decimal |

- **Method:** common denominator (a/b ± c/d = (ad ± cb)/bd), product of numerators over product of denominators, division as multiplication by the reciprocal; then divide by the greatest common divisor
- **Assumes:** whole-number numerators and denominators; denominators not 0; no division by 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('fraction_arithmetic', a=3, b=4, c=5, d=6, op='+')
# -> numerator = 19, denominator = 12, value = 1.58333   (SOLVED)
```

```python
math_foundations_ai999x.solve('fraction_arithmetic', a=2, b=3, c=9, d=10, op='*')
# -> numerator = 3, denominator = 5, value = 0.6   (SOLVED)
```

```python
math_foundations_ai999x.solve('fraction_arithmetic', a=1, b=2, c=3, d=4, op='-')
# -> numerator = -1, denominator = 4, value = -0.25   (SOLVED)
```

```python
math_foundations_ai999x.solve('fraction_arithmetic', a=3, b=4, c=9, d=8, op='/')
# -> numerator = 2, denominator = 3, value = 0.666667   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Compute 3/4 + 5/6 as a fraction in lowest terms. | SOLVED | numerator = 19, denominator = 12, value = 1.58333 |
| What is 2/3 times 9/10? | SOLVED | numerator = 3, denominator = 5, value = 0.6 |
| Calculate 7/8 - 1/6. | SOLVED | numerator = 17, denominator = 24, value = 0.708333 |
| Find 5/6 divided by 0/3. | INVALID_INPUT | Arithmetic with fractions: division by the fraction 0. |
| Compute 1/0 + 1/2. | INVALID_INPUT | Arithmetic with fractions: a denominator must not be 0. |

<a id="percent_of_number"></a>

### Percent of a number — `percent_of_number`

p% of x: the part that p per cent of x amounts to.

$$
\frac{p}{100}\,x
$$

| input | meaning | condition |
|---|---|---|
| `p` | the percentage p (per hundred) |  |
| `x` | the whole x |  |

| output | meaning |
|---|---|
| `result` | p% of x |

- **Method:** per cent means per hundred: p% of $x = (p/100)$·x
- **Assumes:** finite numbers
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('percent_of_number', p=15, x=80)
# -> result = 12   (SOLVED)
```

```python
math_foundations_ai999x.solve('percent_of_number', p=12.5, x=240)
# -> result = 30   (SOLVED)
```

```python
math_foundations_ai999x.solve('percent_of_number', p=150, x=40)
# -> result = 60   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is 15% of 80? | SOLVED | result = 12 |
| Find 12.5 percent of 240. | SOLVED | result = 30 |
| Calculate 150% of 40. | SOLVED | result = 60 |

<a id="percent_of_what"></a>

### Whole from a part and its percentage — `percent_of_what`

The whole w of which `part` is p per cent (part is p% of what number?).

$$
w = \frac{100\,\mathrm{part}}{p}
$$

| input | meaning | condition |
|---|---|---|
| `part` | the known part |  |
| `p` | the percentage that the part is of the whole | $p  \neq  0$ |

| output | meaning |
|---|---|
| `whole` | the whole |

- **Method:** part = (p/100)·whole, so whole = 100·part/p
- **Assumes:** $p  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('percent_of_what', part=18, p=15)
# -> whole = 120   (SOLVED)
```

```python
math_foundations_ai999x.solve('percent_of_what', part=26, p=40)
# -> whole = 65   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| 18 is 15% of what number? | SOLVED | whole = 120 |
| 40% of what number is 26? | SOLVED | whole = 65 |
| 5 is 0% of what number? | INVALID_INPUT | Whole from a part and its percentage: p must not be 0 (0% of every number is 0). |

<a id="what_percent"></a>

### What percent one number is of another — `what_percent`

The percentage p such that `part` is p% of `whole`.

$$
p = 100\,\frac{\mathrm{part}}{\mathrm{whole}}
$$

| input | meaning | condition |
|---|---|---|
| `part` | the part |  |
| `whole` | the whole (not 0) | whole != 0 |

| output | meaning |
|---|---|
| `percent` | part as a percentage of whole |

- **Method:** $p = 100$·part/whole
- **Assumes:** whole != 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('what_percent', part=12, whole=80)
# -> percent = 15   (SOLVED)
```

```python
math_foundations_ai999x.solve('what_percent', part=45, whole=60)
# -> percent = 75   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What percent of 80 is 12? | SOLVED | percent = 15 |
| 45 is what percent of 60? | SOLVED | percent = 75 |
| What percent of 0 is 5? | INVALID_INPUT | What percent one number is of another: the whole must not be 0. |

<a id="percent_change"></a>

### Percent change (increase or decrease) — `percent_change`

The percent change from an original value to a new value (negative for a decrease).

$$
\Delta\% = \frac{\mathrm{new}-\mathrm{old}}{\mathrm{old}}\cdot 100
$$

| input | meaning | condition |
|---|---|---|
| `old` | the original value (positive) |  |
| `new` | the new value |  |

| output | meaning |
|---|---|
| `percent_change` | percent change (negative = decrease) |
| `difference` | new - old |
| `direction` | increase, decrease or no change |

- **Method:** percent change = (new - old)/old · 100
- **Assumes:** old > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('percent_change', old=80, new=100)
# -> percent_change = 25, difference = 20, direction = increase   (SOLVED)
```

```python
math_foundations_ai999x.solve('percent_change', old=50, new=40)
# -> percent_change = -20, difference = -10, direction = decrease   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the percent change from 80 to 100. | SOLVED | percent_change = 25, difference = 20, direction = increase |
| What is the percent decrease from 50 to 40? | SOLVED | percent_change = -20, difference = -10, direction = decrease |
| A price rises from 40 to 50. What is the percent increase? | SOLVED | percent_change = 25, difference = 10, direction = increase |
| Find the percent change from 0 to 10. | INVALID_INPUT | Percent change (increase or decrease): the original value must be positive (a percent change is measured from it). |

<a id="ratio_split"></a>

### Divide a quantity in a given ratio — `ratio_split`

Share a total in the ratio a:b (or a:b:c).

$$
\mathrm{part}_i = \mathrm{total}\cdot\frac{r_i}{\sum_j r_j}
$$

| input | meaning | condition |
|---|---|---|
| `total` | the quantity to share |  |
| `a` | first term of the ratio (positive) |  |
| `b` | second term of the ratio (positive) |  |
| `c` | third term of the ratio (optional, positive) |  |

| output | meaning |
|---|---|
| `part1` | first share |
| `part2` | second share |
| `part3` | third share (a three-term ratio only) |

- **Method:** each share = total · term / (sum of the terms)
- **Assumes:** every term of the ratio positive
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('ratio_split', total=120, a=3, b=5)
# -> part1 = 45, part2 = 75   (SOLVED)
```

```python
math_foundations_ai999x.solve('ratio_split', total=900, a=2, b=3, c=4)
# -> part1 = 200, part2 = 300, part3 = 400   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Divide 120 in the ratio 3:5. | SOLVED | part1 = 45, part2 = 75 |
| Share 900 in the ratio 2:3:4. | SOLVED | part1 = 200, part2 = 300, part3 = 400 |
| Split 50 in the ratio 0:5. | INVALID_INPUT | Divide a quantity in a given ratio: every term of the ratio must be positive. |

<a id="proportion_solve"></a>

### Solve a proportion — `proportion_solve`

Solve $n_{1}/d_{1} = n_{2}/d_{2}$ for the one unknown term x (in any of the four positions).

$$
\frac{n_1}{d_1} = \frac{n_2}{d_2} \iff n_1 d_2 = n_2 d_1
$$

| input | meaning | condition |
|---|---|---|
| `p` | the first known term (reading n1, d1, n2, d2 in order) |  |
| `q` | the second known term |  |
| `r` | the third known term |  |
| `position` | where the unknown x stands |  |

| output | meaning |
|---|---|
| `x` | the unknown term |

- **Method:** cross-multiplication: n1·$d_{2} = n_{2}$·d1, then divide
- **Assumes:** denominators not 0; the proportion has a solution
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('proportion_solve', p=3, q=4, r=20, position='second numerator')
# -> x = 15   (SOLVED)
```

```python
math_foundations_ai999x.solve('proportion_solve', p=6, q=10, r=15, position='first numerator')
# -> x = 4   (SOLVED)
```

```python
math_foundations_ai999x.solve('proportion_solve', p=5, q=2, r=7, position='first denominator')
# -> x = 17.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve 3/4 = x/20. | SOLVED | x = 15 |
| Solve the proportion x/6 = 10/15. | SOLVED | x = 4 |
| Solve 5/x = 2/7. | SOLVED | x = 17.5 |
| Solve 3/0 = x/5. | INVALID_INPUT | Solve a proportion: a denominator must not be 0. |

<a id="unit_rate_compare"></a>

### Unit rates and the better buy — `unit_rate_compare`

The unit rate (amount per one unit) of an offer, and of a second offer with which is cheaper per unit.

$$
\text{unit rate} = \frac{\text{amount}}{\text{quantity}}
$$

| input | meaning | condition |
|---|---|---|
| `n1` | quantity of the first offer (positive) |  |
| `c1` | amount (price, distance, ...) of the first offer |  |
| `n2` | quantity of the second offer (optional) |  |
| `c2` | amount of the second offer (optional) |  |

| output | meaning |
|---|---|
| `unit_rate_1` | amount per unit, first offer |
| `unit_rate_2` | amount per unit, second offer |
| `better` | the offer with the lower amount per unit: first, second or equal |

- **Method:** unit rate = amount / quantity; the lower unit price is the better buy
- **Assumes:** quantities positive; amounts not negative
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('unit_rate_compare', n1=12, c1=30, n2=20, c2=45)
# -> unit_rate_1 = 2.5, unit_rate_2 = 2.25, better = second   (SOLVED)
```

```python
math_foundations_ai999x.solve('unit_rate_compare', n1=6, c1=4.5)
# -> unit_rate_1 = 0.75   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Which is the better buy: 12 items for 30 or 20 items for 45? | SOLVED | unit_rate_1 = 2.5, unit_rate_2 = 2.25, better = second |
| Find the unit price of 6 cans for 4.5. | SOLVED | unit_rate_1 = 0.75 |
| Which is the better deal: 3 kg for $7.50 or 5 kg for $12? | SOLVED | unit_rate_1 = 2.5, unit_rate_2 = 2.4, better = second |
| Find the unit price of 0 items for 5. | INVALID_INPUT | Unit rates and the better buy: quantities must be positive and amounts not negative. |

<a id="round_decimal_places"></a>

### Round to decimal places — `round_decimal_places`

Round a number to n decimal places (or to the nearest tenth, hundredth, ten, hundred ...), halves away from zero.

$$
\mathrm{round}(x, n) = \operatorname{sgn}(x)\,\frac{\lfloor |x|\,10^{n} + \tfrac12 \rfloor}{10^{n}}
$$

| input | meaning | condition |
|---|---|---|
| `x` | the number to round |  |
| `n` | decimal places (negative: tens, hundreds) |  |

| output | meaning |
|---|---|
| `rounded` | the rounded number |

- **Method:** exact decimal arithmetic on the number as written; a 5 in the first dropped place rounds away from zero
- **Assumes:** $-9  \leq  n  \leq  12$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('round_decimal_places', x=3.14159, n=2)
# -> rounded = 3.14   (SOLVED)
```

```python
math_foundations_ai999x.solve('round_decimal_places', x=2.675, n=2)
# -> rounded = 2.68   (SOLVED)
```

```python
math_foundations_ai999x.solve('round_decimal_places', x=1234.5, n=-2)
# -> rounded = 1200   (SOLVED)
```

```python
math_foundations_ai999x.solve('round_decimal_places', x=-0.5, n=0)
# -> rounded = -1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Round 3.14159 to 2 decimal places. | SOLVED | rounded = 3.14 |
| Round 2.675 to the nearest hundredth. | SOLVED | rounded = 2.68 |
| Round 1234.5 to the nearest hundred. | SOLVED | rounded = 1200 |
| Round 7.45 to the nearest whole number. | SOLVED | rounded = 7 |

<a id="round_significant_figures"></a>

### Round to significant figures — `round_significant_figures`

Round a nonzero number to n significant figures, halves away from zero.

$$
\mathrm{round}\big(x,\ n - 1 - \lfloor \log_{10}|x| \rfloor\big)
$$

| input | meaning | condition |
|---|---|---|
| `x` | the number to round (not 0) | $x  \neq  0$ |
| `n` | number of significant figures |  |

| output | meaning |
|---|---|
| `rounded` | the rounded number |

- **Method:** find the exponent e of the leading digit (10^e <= |x| < $10^{e+1}$), round to $10^{e-n+1}$ exactly
- **Assumes:** $x  \neq  0$; $1  \leq  n  \leq  15$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('round_significant_figures', x=0.0045678, n=3)
# -> rounded = 0.00457   (SOLVED)
```

```python
math_foundations_ai999x.solve('round_significant_figures', x=45678, n=2)
# -> rounded = 46000   (SOLVED)
```

```python
math_foundations_ai999x.solve('round_significant_figures', x=9.996, n=3)
# -> rounded = 10   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Round 0.0045678 to 3 significant figures. | SOLVED | rounded = 0.00457 |
| Write 45678 to 2 significant figures. | SOLVED | rounded = 46000 |
| Round 0 to 2 significant figures. | INVALID_INPUT | Round to significant figures: 0 has no significant figures to round to. |

<a id="scientific_notation"></a>

### Scientific notation — `scientific_notation`

Write a nonzero number as m × $10^{e}$ with 1 <= |m| < 10.

$$
x = m \times 10^{e},\quad 1 \le |m| < 10
$$

| input | meaning | condition |
|---|---|---|
| `x` | the number (not 0) | $x  \neq  0$ |

| output | meaning |
|---|---|
| `mantissa` | m, with 1 <= \|m\| < 10 |
| `exponent` | the integer power of ten e |

- **Method:** exact decimal: e = position of the leading digit, $m = x / 10^{e}$
- **Assumes:** $x  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('scientific_notation', x=45600)
# -> mantissa = 4.56, exponent = 4   (SOLVED)
```

```python
math_foundations_ai999x.solve('scientific_notation', x=0.00072)
# -> mantissa = 7.2, exponent = -4   (SOLVED)
```

```python
math_foundations_ai999x.solve('scientific_notation', x=-3.0)
# -> mantissa = -3, exponent = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Write 45600 in scientific notation. | SOLVED | mantissa = 4.56, exponent = 4 |
| Express 0.00072 in scientific notation. | SOLVED | mantissa = 7.2, exponent = -4 |
| Write 0 in scientific notation. | INVALID_INPUT | Scientific notation: 0 has no scientific notation with a leading digit from 1 to 9. |

<a id="order_of_operations"></a>

### Evaluate an arithmetic expression (order of operations) — `order_of_operations`

The exact value of an arithmetic expression with + - * / ^ and parentheses, following the order of operations.

$$
\text{(  )} \;\to\; a^{b} \;\to\; \times\,\div \;\to\; +\,-
$$

| input | meaning | condition |
|---|---|---|
| `expression` | the expression, e.g. $3 + 4 (2 - 5)^{2} / 6$ |  |

| output | meaning |
|---|---|
| `value` | the value |
| `numerator` | numerator of the exact value in lowest terms |
| `denominator` | denominator of the exact value |

- **Method:** parentheses first, then powers (right to left), then multiplication and division, then addition and subtraction (left to right); exact rational arithmetic
- **Assumes:** numbers, + - * / ^ and parentheses only; no division by 0; integer exponents up to 64
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('order_of_operations', expression='3 + 4 * (2 - 5)^2 / 6')
# -> value = 9, numerator = 9, denominator = 1   (SOLVED)
```

```python
math_foundations_ai999x.solve('order_of_operations', expression='8 - 2 * 3^2 + 10 / 5')
# -> value = -8, numerator = -8, denominator = 1   (SOLVED)
```

```python
math_foundations_ai999x.solve('order_of_operations', expression='2^3^2')
# -> value = 512, numerator = 512, denominator = 1   (SOLVED)
```

```python
math_foundations_ai999x.solve('order_of_operations', expression='1/3 + 1/6')
# -> value = 0.5, numerator = 1, denominator = 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Evaluate 3 + 4 * (2 - 5)^2 / 6. | SOLVED | value = 9, numerator = 9, denominator = 1 |
| Use the order of operations to evaluate 8 - 2 * 3^2 + 10 / 5. | SOLVED | value = -8, numerator = -8, denominator = 1 |
| Evaluate -2^2 + 3 * (4 - 1). | SOLVED | value = 5, numerator = 5, denominator = 1 |
| Evaluate 6 / (3 - 3) + 1. | INVALID_INPUT | Evaluate an arithmetic expression (order of operations): division by zero. |

<a id="linear_system_3x3"></a>

### System of three linear equations — `linear_system_3x3`

Solve three linear equations in x, y and z (a unique solution, or a refusal when there is none).

$$
A\,\mathbf{x} = \mathbf{d},\quad \mathbf{x} = A^{-1}\mathbf{d}
$$

| input | meaning | condition |
|---|---|---|
| `augmented` | augmented matrix, one row a b c d per equation $a x + b y + c z = d$ |  |

| output | meaning |
|---|---|
| `x` | x |
| `y` | y |
| `z` | z |

- **Method:** Gaussian elimination in exact rational arithmetic
- **Assumes:** determinant of the coefficients != 0 (a unique solution)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('linear_system_3x3', augmented='1 1 1 6; 2 -1 1 3; 1 2 -1 2')
# -> x = 1, y = 2, z = 3   (SOLVED)
```

```python
math_foundations_ai999x.solve('linear_system_3x3', augmented=[[2, 1, -1, 8], [-3, -1, 2, -11], [-2, 1, 2, -3]])
# -> x = 2, y = 3, z = -1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve the system x + y + z = 6, 2x - y + z = 3, x + 2y - z = 2. | SOLVED | x = 1, y = 2, z = 3 |
| Solve the system 2x + y - z = 8; -3x - y + 2z = -11; -2x + y + 2z = -3. | SOLVED | x = 2, y = 3, z = -1 |
| Solve the system x + y + z = 1, 2x + 2y + 2z = 2, x - y = 0. | INVALID_INPUT | System of three linear equations: the system has no unique solution (its determinant is 0). |

<a id="abs_linear_inequality"></a>

### Absolute value inequality |ax + b| compared with c — `abs_linear_inequality`

Solve |$a x + b$| < c, <= c, > c $or  \geq  c$ $(c > 0)$: the two boundary points and whether x lies between or outside them.

$$
|ax+b| < c \iff -c < ax+b < c,\qquad |ax+b| > c \iff ax+b < -c \ \text{or}\ ax+b > c
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of x inside the bars | $a  \neq  0$ |
| `b` | constant inside the bars |  |
| `c` | the bound (positive) |  |
| `rel` | the comparison |  |

| output | meaning |
|---|---|
| `lower` | the smaller boundary point |
| `upper` | the larger boundary point |
| `region` | between (for < and <=) or outside (for > and >=) |
| `solution` | the solution set |

- **Method:** |u| < c means -c < u < c; |u| > c means $u < -c$ or $u > c$; solve each linear piece for x
- **Assumes:** $a  \neq  0$; $c > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('abs_linear_inequality', a=2, b=3, c=7, rel='<')
# -> lower = -5, upper = 2, region = between, solution = -5 < x < 2   (SOLVED)
```

```python
math_foundations_ai999x.solve('abs_linear_inequality', a=3, b=-1, c=5, rel='>=')
# -> lower = -1.33333, upper = 2, region = outside, solution = x <= -1.333333333 or x >= 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve \|2x + 3\| < 7. | SOLVED | lower = -5, upper = 2, region = between, solution = -5 < x < 2 |
| Solve the inequality \|3x - 1\| >= 5. | SOLVED | lower = -1.33333, upper = 2, region = outside, solution = x <= -1.333333333 or x >= 2 |
| Solve \|x + 4\| <= 0. | INVALID_INPUT | Absolute value inequality \|ax + b\| compared with c: c must be positive (with c <= 0 the solution set is empty, a point o |

<a id="rational_equation"></a>

### Rational equation a/(x + b) = c/(x + d) — `rational_equation`

Solve $a/(x + b) = c/(x + d)$ for x, rejecting a candidate that makes a denominator 0.

$$
\frac{a}{x+b} = \frac{c}{x+d} \;\Rightarrow\; x = \frac{cb - ad}{a - c}
$$

| input | meaning | condition |
|---|---|---|
| `a` | numerator on the left |  |
| `b` | constant in the left denominator x + b |  |
| `c` | numerator on the right |  |
| `d` | constant in the right denominator x + d |  |

| output | meaning |
|---|---|
| `x` | the solution |

- **Method:** multiply by both denominators: a(x + d) = c(x + b), solve the linear equation, check the excluded values $x = -b$, $x = -d$
- **Assumes:** $a  \neq  c$; the candidate is not an excluded value
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('rational_equation', a=3, b=1, c=2, d=-4)
# -> x = 14   (SOLVED)
```

```python
math_foundations_ai999x.solve('rational_equation', a=5, b=-2, c=3, d=6)
# -> x = -18   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve 3/(x + 1) = 2/(x - 4). | SOLVED | x = 14 |
| Solve 5/(x - 2) = 3/(x + 6). | SOLVED | x = -18 |
| Solve 2/(x + 1) = 2/(x + 3). | INVALID_INPUT | Rational equation a/(x + b) = c/(x + d): no unique solution (equal numerators: no solution or the same expression on bot |
| Solve 0/(x + 1) = 4/(x - 2). | INVALID_INPUT | Rational equation a/(x + b) = c/(x + d): no solution: the only candidate x = -1 makes a denominator 0 (extraneous). |

<a id="radical_equation"></a>

### Radical equation sqrt(ax + b) = cx + d — `radical_equation`

Solve sqrt(a*x + b) = c*x + d: square, solve, and keep only the roots that satisfy the original equation.

$$
\sqrt{ax+b} = cx+d \;\Rightarrow\; ax+b = (cx+d)^2,\ cx+d \ge 0
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of x under the root | $a  \neq  0$ |
| `b` | constant under the root |  |
| `c` | coefficient of x on the right |  |
| `d` | constant on the right |  |

| output | meaning |
|---|---|
| `count` | number of solutions (0, 1 or 2) |
| `solutions` | the solutions, ascending |
| `extraneous` | roots of the squared equation that fail the original (extraneous) |

- **Method:** square both sides (a x + b = (c x + d)^2), solve the quadratic exactly, keep a root only if c $x + d  \geq  0$
- **Assumes:** $a  \neq  0$; real numbers
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('radical_equation', a=2, b=3, c=1, d=0)
# -> count = 1, solutions = [3.0], extraneous = [-1.0]   (SOLVED)
```

```python
math_foundations_ai999x.solve('radical_equation', a=1, b=7, c=1, d=-5)
# -> count = 1, solutions = [9.0], extraneous = [2.0]   (SOLVED)
```

```python
math_foundations_ai999x.solve('radical_equation', a=1, b=-1, c=1, d=-1)
# -> count = 2, solutions = [1.0, 2.0], extraneous = []   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve sqrt(2x + 3) = x. | SOLVED | count = 1, solutions = [3.0], extraneous = [-1.0] |
| Solve √(x + 7) = x - 5. | SOLVED | count = 1, solutions = [9.0], extraneous = [2.0] |
| Solve sqrt(x - 1) = x - 1. | SOLVED | count = 2, solutions = [1.0, 2.0], extraneous = [] |
| Solve sqrt(x + 4) = -2. | SOLVED | count = 0, solutions = [], extraneous = [0.0] |

<a id="synthetic_division"></a>

### Synthetic division and the remainder theorem — `synthetic_division`

Divide a polynomial by x - r: the quotient's coefficients and the remainder, which equals p(r).

$$
p(x) = (x - r)\,q(x) + p(r)
$$

| input | meaning | condition |
|---|---|---|
| `coefficients` | coefficients of the dividend, highest power first (2x^3 - 5x + 1 -> 2, 0, -5, 1) |  |
| `r` | the number r of the divisor x - r |  |

| output | meaning |
|---|---|
| `quotient` | coefficients of the quotient, highest power first |
| `remainder` | the remainder = p(r) |

- **Method:** synthetic division (Horner's scheme): bring down, multiply by r, add; the last number is the remainder p(r)
- **Assumes:** dividend of degree >= 1
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('synthetic_division', coefficients='1, -6, 11, -6', r=1)
# -> quotient = [1.0, -5.0, 6.0], remainder = 0   (SOLVED)
```

```python
math_foundations_ai999x.solve('synthetic_division', coefficients=[1, 0, -3, 0, 2], r=3)
# -> quotient = [1.0, 3.0, 6.0, 18.0], remainder = 56   (SOLVED)
```

```python
math_foundations_ai999x.solve('synthetic_division', coefficients=[2, 0, 3, -4], r=-2)
# -> quotient = [2.0, -4.0, 11.0], remainder = -26   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Divide x^3 - 6x^2 + 11x - 6 by x - 1. | SOLVED | quotient = [1.0, -5.0, 6.0], remainder = 0 |
| Find the remainder when x^4 - 3x^2 + 2 is divided by x - 3. | SOLVED | quotient = [1.0, 3.0, 6.0, 18.0], remainder = 56 |
| Use synthetic division to divide 2x^3 + 3x - 4 by x + 2. | SOLVED | quotient = [2.0, -4.0, 11.0], remainder = -26 |

<a id="polynomial_long_division"></a>

### Polynomial long division — `polynomial_long_division`

Divide one polynomial by another: quotient and remainder, with $p = q d + r$ and deg r < deg d.

$$
p(x) = q(x)\,d(x) + r(x),\quad \deg r < \deg d
$$

| input | meaning | condition |
|---|---|---|
| `dividend` | coefficients of the dividend, highest power first |  |
| `divisor` | coefficients of the divisor, highest power first (degree >= 1) |  |

| output | meaning |
|---|---|
| `quotient` | coefficients of the quotient, highest power first |
| `remainder` | coefficients of the remainder, highest power first |

- **Method:** long division: divide leading terms, multiply back, subtract, repeat; exact rational arithmetic
- **Assumes:** divisor of degree >= 1
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('polynomial_long_division', dividend='2, 3, -1, 5', divisor='1, 0, 1')
# -> quotient = [2.0, 3.0], remainder = [-3.0, 2.0]   (SOLVED)
```

```python
math_foundations_ai999x.solve('polynomial_long_division', dividend=[6, -5, 0, 4], divisor=[2, -3])
# -> quotient = [3.0, 2.0, 3.0], remainder = [13.0]   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Divide 2x^3 + 3x^2 - x + 5 by x^2 + 1. | SOLVED | quotient = [2.0, 3.0], remainder = [-3.0, 2.0] |
| Use long division to divide 6x^3 - 5x^2 + 4 by 2x - 3. | SOLVED | quotient = [3.0, 2.0, 3.0], remainder = [13.0] |
| Find the quotient and remainder when x^4 - 1 is divided by x^2 + x + 1. | SOLVED | quotient = [1.0, -1.0, 0.0], remainder = [1.0, -1.0] |

<a id="factor_quadratic_integer"></a>

### Factor a quadratic over the integers — `factor_quadratic_integer`

Factor $a x^{2} + b x + c$ with whole-number coefficients as g(px + q)(rx + s), or say that it does not factor over the integers.

$$
ax^2+bx+c = g\,(px+q)(rx+s)
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of $x^{2}$ (not 0) |  |
| `b` | coefficient of x |  |
| `c` | constant term |  |

| output | meaning |
|---|---|
| `g` | common factor (sign of a) |
| `p` | x-coefficient of the first factor |
| `q` | constant of the first factor |
| `r` | x-coefficient of the second factor |
| `s` | constant of the second factor |
| `factored` | the factored form |

- **Method:** take out the greatest common factor; the rational roots -q/p and -s/r of the primitive quadratic give its factors (Gauss's lemma)
- **Assumes:** whole-number coefficients; $a  \neq  0$; the discriminant is a perfect square
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('factor_quadratic_integer', a=6, b=5, c=-6)
# -> g = 1, p = 2, q = 3, r = 3, s = -2, factored = (2x + 3)(3x - 2)   (SOLVED)
```

```python
math_foundations_ai999x.solve('factor_quadratic_integer', a=4, b=0, c=-9)
# -> g = 1, p = 2, q = 3, r = 2, s = -3, factored = (2x + 3)(2x - 3)   (SOLVED)
```

```python
math_foundations_ai999x.solve('factor_quadratic_integer', a=2, b=0, c=-8)
# -> g = 2, p = 1, q = 2, r = 1, s = -2, factored = 2(x + 2)(x - 2)   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Factor 6x^2 + 5x - 6 over the integers. | SOLVED | g = 1, p = 2, q = 3, r = 3, s = -2, factored = (2x + 3)(3x - 2) |
| Factor 4x^2 - 9. | SOLVED | g = 1, p = 2, q = 3, r = 2, s = -3, factored = (2x + 3)(2x - 3) |
| Factor completely 2x^2 - 8. | SOLVED | g = 2, p = 1, q = 2, r = 1, s = -2, factored = 2(x + 2)(x - 2) |
| Factor 2x^2 + 3x + 5 over the integers. | INVALID_INPUT | Factor a quadratic over the integers: this quadratic does not factor over the integers (its discriminant is not a perfec |

<a id="quadratic_inequality"></a>

### Quadratic inequality — `quadratic_inequality`

Solve $a x^{2} + b x + c < 0$ (or <=, >, >=) when the quadratic has two distinct real roots.

$$
a(x - r_1)(x - r_2) < 0 \iff r_1 < x < r_2 \ \ (a > 0)
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of $x^{2}$ | $a  \neq  0$ |
| `b` | coefficient of x |  |
| `c` | constant term |  |
| `rel` | the comparison with 0 |  |

| output | meaning |
|---|---|
| `r1` | the smaller root |
| `r2` | the larger root |
| `region` | between or outside the roots |
| `solution` | the solution set |

- **Method:** the roots split the line into three intervals; the sign of a decides whether the quadratic is negative between or outside them
- **Assumes:** $a  \neq  0$; $b^{2} - 4ac > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('quadratic_inequality', a=1, b=-5, c=6, rel='<')
# -> r1 = 2, r2 = 3, region = between, solution = 2 < x < 3   (SOLVED)
```

```python
math_foundations_ai999x.solve('quadratic_inequality', a=2, b=-1, c=-3, rel='>=')
# -> r1 = -1, r2 = 1.5, region = outside, solution = x <= -1 or x >= 1.5   (SOLVED)
```

```python
math_foundations_ai999x.solve('quadratic_inequality', a=-1, b=0, c=4, rel='>')
# -> r1 = -2, r2 = 2, region = between, solution = -2 < x < 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve x^2 - 5x + 6 < 0. | SOLVED | r1 = 2, r2 = 3, region = between, solution = 2 < x < 3 |
| Solve the quadratic inequality 2x^2 - x - 3 >= 0. | SOLVED | r1 = -1, r2 = 1.5, region = outside, solution = x <= -1 or x >= 1.5 |
| Solve -x^2 + 4 > 0. | SOLVED | r1 = -2, r2 = 2, region = between, solution = -2 < x < 2 |
| Solve x^2 + 1 < 0. | OUT_OF_SCOPE | Out of scope: a quadratic inequality whose quadratic has no two distinct real roots. |

<a id="infinite_geometric_series"></a>

### Sum of an infinite geometric series — `infinite_geometric_series`

$a + a r + a r^{2} + ... = a/(1 - r)$ for |r| < 1 (a refusal when the series diverges).

$$
\sum_{k=0}^{\infty} a r^k = \frac{a}{1-r},\quad |r| < 1
$$

| input | meaning | condition |
|---|---|---|
| `a` | the first term |  |
| `r` | the common ratio, \|r\| < 1 |  |

| output | meaning |
|---|---|
| `sum` | the sum to infinity |

- **Method:** $S_n = a$(1 - r^n)/(1 - r) and $r^{n}$ -> 0 when |r| < 1
- **Assumes:** |r| < 1
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('infinite_geometric_series', a=6, r=0.3333333333333333)
# -> sum = 9   (SOLVED)
```

```python
math_foundations_ai999x.solve('infinite_geometric_series', a=8, r=0.5)
# -> sum = 16   (SOLVED)
```

```python
math_foundations_ai999x.solve('infinite_geometric_series', a=9, r=-0.3333333333333333)
# -> sum = 6.75   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the sum of the infinite geometric series with first term 6 and ratio 1/3. | SOLVED | sum = 9 |
| Find the sum to infinity of 8 + 4 + 2 + 1 + ... | SOLVED | sum = 16 |
| Find the sum to infinity of 9 - 3 + 1 - ... | SOLVED | sum = 6.75 |
| Find the sum of the infinite geometric series with first term 2 and ratio 3/2. | INVALID_INPUT | Sum of an infinite geometric series: the series diverges unless \|r\| < 1. |

<a id="partial_fractions_linear"></a>

### Partial fractions (two distinct linear factors) — `partial_fractions_linear`

Write $(p x + q)$/($(x - r_{1})$$(x - r_{2})$) as $A/(x - r_{1}) + B/(x - r_{2})$.

$$
\frac{px+q}{(x-r_1)(x-r_2)} = \frac{A}{x-r_1} + \frac{B}{x-r_2},\quad A = \frac{pr_1+q}{r_1-r_2}
$$

| input | meaning | condition |
|---|---|---|
| `p` | coefficient of x in the numerator |  |
| `q` | constant of the numerator |  |
| `r1` | root of the first factor $x - r_{1}$ |  |
| `r2` | root of the second factor $x - r_{2}$ $(r_{2}  \neq  r_{1})$ |  |

| output | meaning |
|---|---|
| `A` | numerator over $x - r_{1}$ |
| `B` | numerator over $x - r_{2}$ |

- **Method:** cover-up rule: A = (p r1 + q)/(r1 - r2), B = (p r2 + q)/(r2 - r1)
- **Assumes:** $r_{1}  \neq  r_{2}$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('partial_fractions_linear', p=5, q=1, r1=1, r2=-2)
# -> A = 2, B = 3   (SOLVED)
```

```python
math_foundations_ai999x.solve('partial_fractions_linear', p=3, q=5, r1=-1, r2=2)
# -> A = -0.666667, B = 3.66667   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Decompose (5x + 1)/((x - 1)(x + 2)) into partial fractions. | SOLVED | A = 2, B = 3 |
| Find the partial fraction decomposition of (3x + 5)/(x^2 - x - 2). | SOLVED | A = -0.666667, B = 3.66667 |
| Decompose (2x + 3)/((x - 4)(x - 4)) into partial fractions. | INVALID_INPUT | Partial fractions (two distinct linear factors): the two linear factors must be different (repeated factors need another |

<a id="rational_roots"></a>

### Rational roots of a polynomial (rational root theorem) — `rational_roots`

Every rational root of a polynomial with whole-number coefficients (degree up to 8).

$$
a_n x^n + \dots + a_0 = 0,\ x = \tfrac{p}{q} \Rightarrow p \mid a_0,\ q \mid a_n
$$

| input | meaning | condition |
|---|---|---|
| `coefficients` | whole-number coefficients, highest power first |  |

| output | meaning |
|---|---|
| `count` | number of distinct rational roots |
| `roots` | the rational roots, ascending |
| `roots_exact` | the rational roots as fractions |

- **Method:** rational root theorem: a root p/q in lowest terms has p dividing the constant term and q dividing the leading coefficient; test every candidate exactly
- **Assumes:** whole-number coefficients; degree 1 to 8
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('rational_roots', coefficients='2, -3, -11, 6')
# -> count = 3, roots = [-2.0, 0.5, 3.0], roots_exact = ['-2', '1/2', '3']   (SOLVED)
```

```python
math_foundations_ai999x.solve('rational_roots', coefficients=[1, 0, -7, 6])
# -> count = 3, roots = [-3.0, 1.0, 2.0], roots_exact = ['-3', '1', '2']   (SOLVED)
```

```python
math_foundations_ai999x.solve('rational_roots', coefficients=[1, 0, -2])
# -> count = 0, roots = [], roots_exact = []   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find all rational roots of 2x^3 - 3x^2 - 11x + 6 = 0. | SOLVED | count = 3, roots = [-2.0, 0.5, 3.0], roots_exact = ['-2', '1/2', '3'] |
| List the rational zeros of x^3 - 7x + 6. | SOLVED | count = 3, roots = [-3.0, 1.0, 2.0], roots_exact = ['-3', '1', '2'] |
| Find the rational roots of x^2 - 2. | SOLVED | count = 0, roots = [], roots_exact = [] |
| Find all rational roots of 0.5x^2 - 2. | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: math_foundations_ai999x.templates |

<a id="evaluate_function"></a>

### Evaluate a function at a number — `evaluate_function`

The value f(x0) of a function given by a formula in x (exact when it is rational).

$$
f(x_0)
$$

| input | meaning | condition |
|---|---|---|
| `expression` | the formula of f in x: numbers, x, + - * / ^, parentheses, sqrt, abs, ln |  |
| `x` | the number x0 to put in |  |

| output | meaning |
|---|---|
| `value` | f(x0) |
| `numerator` | numerator of the exact value |
| `denominator` | denominator of the exact value |

- **Method:** substitute x0 for x and evaluate with the order of operations, in exact rational arithmetic
- **Assumes:** x0 in the domain of f (no division by 0, no root of a negative number, no logarithm of a number <= 0)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('evaluate_function', expression='2x^2 - 3x + 1', x=3)
# -> value = 10, numerator = 10, denominator = 1   (SOLVED)
```

```python
math_foundations_ai999x.solve('evaluate_function', expression='x^3 - 2x + 5', x=-2)
# -> value = 1, numerator = 1, denominator = 1   (SOLVED)
```

```python
math_foundations_ai999x.solve('evaluate_function', expression='(x + 1)/(x - 3)', x=2)
# -> value = -3, numerator = -3, denominator = 1   (SOLVED)
```

```python
math_foundations_ai999x.solve('evaluate_function', expression='sqrt(x + 7)', x=2)
# -> value = 3, numerator = 3, denominator = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Evaluate f(3) for f(x) = 2x^2 - 3x + 1. | SOLVED | value = 10, numerator = 10, denominator = 1 |
| If f(x) = x^3 - 2x + 5, find f(-2). | SOLVED | value = 1, numerator = 1, denominator = 1 |
| Find f(1/2) when f(x) = 4x^2 - 1. | SOLVED | value = 0, numerator = 0, denominator = 1 |
| Find f(3) when f(x) = (x + 1)/(x - 3). | INVALID_INPUT | Evaluate a function at a number: f(3) is not defined: division by zero. |

<a id="composite_function_value"></a>

### Value of a composite function — `composite_function_value`

f(g(x0)) or g(f(x0)) for two functions given by formulas.

$$
(f\circ g)(x_0) = f\big(g(x_0)\big)
$$

| input | meaning | condition |
|---|---|---|
| `f` | the formula of f in x |  |
| `g` | the formula of g in x |  |
| `x` | the number x0 |  |
| `order` | which composition |  |

| output | meaning |
|---|---|
| `inner` | value of the inner function at x0 |
| `value` | value of the composite at x0 |

- **Method:** evaluate the inner function at x0, then the outer function at that value
- **Assumes:** each value in the domain of the function applied to it
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('composite_function_value', f='x^2 + 1', g='2x - 3', x=2, order='f(g(x))')
# -> inner = 1, value = 2   (SOLVED)
```

```python
math_foundations_ai999x.solve('composite_function_value', f='3x + 2', g='x^2', x=-1, order='g(f(x))')
# -> inner = -1, value = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| If f(x) = x^2 + 1 and g(x) = 2x - 3, find f(g(2)). | SOLVED | inner = 1, value = 2 |
| If f(x) = x^2 + 1 and g(x) = 2x - 3, find g(f(2)). | SOLVED | inner = 5, value = 7 |
| Let f(x) = 3x + 2 and g(x) = x^2, evaluate g(f(-1)). | SOLVED | inner = -1, value = 1 |
| If f(x) = 1/x and g(x) = x - 2, find f(g(2)). | INVALID_INPUT | Value of a composite function: the composition is not defined at x = 2: division by zero. |

<a id="domain_sqrt_over_linear"></a>

### Domain of a square-root function — `domain_sqrt_over_linear`

The domain of f(x) = sqrt(a*x + b)/(c*x + d) (or $1/sqrt$(a*x + b)): where the radicand is allowed and the denominator is not 0.

$$
ax + b \ge 0,\quad cx + d \ne 0
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of x under the root | $a  \neq  0$ |
| `b` | constant under the root |  |
| `c` | coefficient of x in the denominator (0 for none) |  |
| `d` | constant in the denominator (1 for none) |  |
| `root_in_denominator` | true when the root itself is in the denominator (the radicand must then be positive) |  |

| output | meaning |
|---|---|
| `boundary` | where the radicand is 0 |
| `side` | the inequality x must satisfy with the boundary |
| `excluded` | a point removed because the denominator is 0 there (None if none) |
| `domain` | the domain in interval notation |

- **Method:** radicand $a x + b  \geq  0$ (> 0 when the root is in a denominator); remove the zero of $c x + d$
- **Assumes:** $a  \neq  0$; the denominator is not identically 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('domain_sqrt_over_linear', a=2, b=-4, c=0, d=1)
# -> boundary = 2, side = >=, excluded = none, domain = [2, inf)   (SOLVED)
```

```python
math_foundations_ai999x.solve('domain_sqrt_over_linear', a=1, b=3, c=1, d=-1)
# -> boundary = -3, side = >=, excluded = 1, domain = [-3, 1) U (1, inf)   (SOLVED)
```

```python
math_foundations_ai999x.solve('domain_sqrt_over_linear', a=-1, b=5, c=0, d=1, root_in_denominator=True)
# -> boundary = 5, side = <, excluded = none, domain = (-inf, 5)   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the domain of f(x) = sqrt(2x - 4). | SOLVED | boundary = 2, side = >=, excluded = none, domain = [2, inf) |
| Find the domain of f(x) = sqrt(x + 3)/(x - 1). | SOLVED | boundary = -3, side = >=, excluded = 1, domain = [-3, 1) U (1, inf) |
| Find the domain of f(x) = 1/sqrt(5 - x). | SOLVED | boundary = 5, side = <, excluded = none, domain = (-inf, 5) |
| Find the domain of f(x) = sqrt(x - 4)/(x - 2). | SOLVED | boundary = 4, side = >=, excluded = none, domain = [4, inf) |

<a id="inverse_linear_function"></a>

### Inverse of a linear function — `inverse_linear_function`

For f(x) = a*x + b $(a  \neq  0)$: $f^{-1}$(x) = x/a - b/a, and $f^{-1}$(y) for a given y.

$$
f^{-1}(x) = \frac{x - b}{a}
$$

| input | meaning | condition |
|---|---|---|
| `a` | slope of f | $a  \neq  0$ |
| `b` | intercept of f |  |
| `y` | a value to invert (optional) |  |

| output | meaning |
|---|---|
| `inv_slope` | slope of the inverse, $1/a$ |
| `inv_intercept` | intercept of the inverse, -b/a |
| `value` | $f^{-1}$(y) |

- **Method:** swap x and y in $y = a x + b$ and solve for y
- **Assumes:** $a  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('inverse_linear_function', a=3, b=-6)
# -> inv_slope = 0.333333, inv_intercept = 2   (SOLVED)
```

```python
math_foundations_ai999x.solve('inverse_linear_function', a=3, b=-6, y=9)
# -> inv_slope = 0.333333, inv_intercept = 2, value = 5   (SOLVED)
```

```python
math_foundations_ai999x.solve('inverse_linear_function', a=-2, b=5)
# -> inv_slope = -0.5, inv_intercept = 2.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the inverse of f(x) = 3x - 6. | SOLVED | inv_slope = 0.333333, inv_intercept = 2 |
| Find f^-1(9) if f(x) = 3x - 6. | SOLVED | inv_slope = 0.333333, inv_intercept = 2, value = 5 |
| Find the inverse function of f(x) = -2x + 5. | SOLVED | inv_slope = -0.5, inv_intercept = 2.5 |

<a id="piecewise_evaluate"></a>

### Evaluate a piecewise function — `piecewise_evaluate`

f(x0) for f(x) = e1 when $x < c$ (or x <= c) and e2 otherwise.

$$
f(x) = \begin{cases} e_1(x) & x < c \\ e_2(x) & x \ge c \end{cases}
$$

| input | meaning | condition |
|---|---|---|
| `e1` | formula of the left piece |  |
| `left_rel` | the left piece holds for $x < c$ or $x  \leq  c$ |  |
| `c` | the break point c |  |
| `e2` | formula of the right piece |  |
| `x` | the number x0 |  |

| output | meaning |
|---|---|
| `value` | f(x0) |
| `piece` | which piece applies (1 = left, 2 = right) |

- **Method:** decide which condition x0 satisfies (the break point belongs to the piece whose condition includes it), then evaluate that piece
- **Assumes:** the two conditions split the line at c (one < and one >=, or <= and >); x0 in the domain of its piece
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('piecewise_evaluate', e1='2x + 1', left_rel='<', c=2, e2='x^2 - 1', x=3)
# -> value = 8, piece = 2   (SOLVED)
```

```python
math_foundations_ai999x.solve('piecewise_evaluate', e1='3 - x', left_rel='<=', c=0, e2='x^2 + 4', x=0)
# -> value = 3, piece = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Evaluate f(3) for f(x) = 2x + 1 if x < 2, x^2 - 1 if x >= 2. | SOLVED | value = 8, piece = 2 |
| If f(x) = 3 - x for x <= 0 and x^2 + 4 for x > 0, find f(0). | SOLVED | value = 3, piece = 1 |
| Evaluate f(2) for f(x) = 2x + 1 if x < 2, x^2 - 1 if x >= 2. | SOLVED | value = 3, piece = 2 |
| Evaluate f(1) for f(x) = x if x < 2, x^2 if x > 3. | OUT_OF_SCOPE | Out of scope: a piecewise function whose two pieces do not split the line at one point. |

<a id="graph_transformation_point"></a>

### Image of a point under a graph transformation — `graph_transformation_point`

If (x0, y0) is on $y = f$(x), the corresponding point on $y = A f$($B (x - h)$) + k.

$$
(x_0, y_0) \mapsto \left(\frac{x_0}{B} + h,\; A y_0 + k\right)
$$

| input | meaning | condition |
|---|---|---|
| `x0` | x-coordinate of the point on $y = f$(x) |  |
| `y0` | y-coordinate of the point |  |
| `A` | vertical stretch (negative: reflection in the x-axis) |  |
| `B` | horizontal compression factor (negative: reflection in the y-axis) |  |
| `h` | horizontal shift |  |
| `k` | vertical shift |  |

| output | meaning |
|---|---|
| `x_new` | x-coordinate of the image |
| `y_new` | y-coordinate of the image |

- **Method:** B(x - h) must equal x0, so $x = x_{0}/B + h$; the height becomes $A y_{0} + k$
- **Assumes:** $A  \neq  0$; $B  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('graph_transformation_point', x0=2, y0=5, A=3, B=2, h=1, k=4)
# -> x_new = 2, y_new = 19   (SOLVED)
```

```python
math_foundations_ai999x.solve('graph_transformation_point', x0=-4, y0=6, A=-1, B=1, h=-3, k=0)
# -> x_new = -7, y_new = -6   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| The point (2, 5) is on the graph of y = f(x). Find the corresponding point on the graph of y = 3f(2(x - 1)) + 4. | SOLVED | x_new = 2, y_new = 19 |
| The point (-4, 6) is on the graph of y = f(x). Find the corresponding point on the graph of y = -f(x + 3). | SOLVED | x_new = -7, y_new = -6 |
| The point (3, 1) is on the graph of y = f(x). Find the corresponding point on the graph of y = f(3x) - 2. | SOLVED | x_new = 1, y_new = -1 |

<a id="line_intercepts"></a>

### Intercepts and slope of a line — `line_intercepts`

The x- and y-intercepts and the slope of the line $a x + b y = c.$

$$
ax + by = c:\quad x_0 = \frac{c}{a},\ y_0 = \frac{c}{b},\ m = -\frac{a}{b}
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of x | $a  \neq  0$ |
| `b` | coefficient of y | $b  \neq  0$ |
| `c` | right-hand side |  |

| output | meaning |
|---|---|
| `x_intercept` | where the line meets the x-axis $(y = 0)$ |
| `y_intercept` | where it meets the y-axis $(x = 0)$ |
| `slope` | the slope -a/b |

- **Method:** set $y = 0$ for the x-intercept and $x = 0$ for the y-intercept
- **Assumes:** $a  \neq  0$; $b  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('line_intercepts', a=2, b=3, c=12)
# -> x_intercept = 6, y_intercept = 4, slope = -0.666667   (SOLVED)
```

```python
math_foundations_ai999x.solve('line_intercepts', a=-2, b=1, c=-6)
# -> x_intercept = 3, y_intercept = -6, slope = 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the x- and y-intercepts of 2x + 3y = 12. | SOLVED | x_intercept = 6, y_intercept = 4, slope = -0.666667 |
| Find the intercepts of the line y = 2x - 6. | SOLVED | x_intercept = 3, y_intercept = -6, slope = 2 |
| Find the intercepts of the line y = 5. | INVALID_INPUT | Intercepts and slope of a line: a and b must both be nonzero (otherwise the line is parallel to an axis and misses one o |

<a id="distance_point_to_line"></a>

### Distance from a point to a line — `distance_point_to_line`

The perpendicular distance from (x0, y0) to the line $a x + b y + c = 0.$

$$
d = \frac{|a x_0 + b y_0 + c|}{\sqrt{a^2 + b^2}}
$$

| input | meaning | condition |
|---|---|---|
| `x0` | x-coordinate of the point |  |
| `y0` | y-coordinate of the point |  |
| `a` | coefficient of x |  |
| `b` | coefficient of y |  |
| `c` | constant term |  |

| output | meaning |
|---|---|
| `distance` | the perpendicular distance |

- **Method:** |a $x_{0} + b$ $y_{0} + c$| / sqrt(a^2 + b^2)
- **Assumes:** a and b not both 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('distance_point_to_line', x0=1, y0=2, a=3, b=4, c=-5)
# -> distance = 1.2   (SOLVED)
```

```python
math_foundations_ai999x.solve('distance_point_to_line', x0=0, y0=0, a=-2, b=1, c=-5)
# -> distance = 2.23607   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the distance from the point (1, 2) to the line 3x + 4y - 5 = 0. | SOLVED | distance = 1.2 |
| Find the distance from the point (0, 0) to the line y = 2x + 5. | SOLVED | distance = 2.23607 |

<a id="log_evaluate"></a>

### Evaluate a logarithm — `log_evaluate`

log_b(x) for a positive base $b  \neq  1$ and $x > 0$ (log is base 10, ln base e).

$$
\log_b x = \frac{\ln x}{\ln b}
$$

| input | meaning | condition |
|---|---|---|
| `b` | the base (positive, not 1) |  |
| `x` | the number (positive) |  |

| output | meaning |
|---|---|
| `value` | log_b(x) |

- **Method:** change of base: log_b(x) = ln(x)/ln(b)
- **Assumes:** $b > 0$; $b  \neq  1$; $x > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('log_evaluate', b=2, x=32)
# -> value = 5   (SOLVED)
```

```python
math_foundations_ai999x.solve('log_evaluate', b=3, x=81)
# -> value = 4   (SOLVED)
```

```python
math_foundations_ai999x.solve('log_evaluate', b=2.718281828459045, x=20)
# -> value = 2.99573   (SOLVED)
```

```python
math_foundations_ai999x.solve('log_evaluate', b=10, x=1000)
# -> value = 3   (SOLVED)
```

```python
math_foundations_ai999x.solve('log_evaluate', b=2, x=0.125)
# -> value = -3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Evaluate log base 2 of 32. | SOLVED | value = 5 |
| Find log_3(81). | SOLVED | value = 4 |
| Compute ln(20). | SOLVED | value = 2.99573 |
| Find log(1000). | SOLVED | value = 3 |
| Evaluate log base 2 of -8. | INVALID_INPUT | Evaluate a logarithm: the logarithm is defined only for positive numbers. |
| Evaluate log base 1 of 5. | INVALID_INPUT | Evaluate a logarithm: the base must be positive and not 1. |

<a id="log_laws_combine"></a>

### Laws of logarithms (product, quotient, power) — `log_laws_combine`

Given log_b(M) = u and log_b(N) = v, the value of log_b(M^p * N^q) (quotients and roots included).

$$
\log_b\!\left(M^{p} N^{q}\right) = p\log_b M + q\log_b N
$$

| input | meaning | condition |
|---|---|---|
| `u` | log_b(M) |  |
| `v` | log_b(N) |  |
| `p` | power of M |  |
| `q` | power of N (negative: N in the denominator) |  |

| output | meaning |
|---|---|
| `value` | log_b($M^{p}$ N^q) |

- **Method:** product law log(MN) = log M + log N, quotient law, power law log(M^p) = p log M
- **Assumes:** M, $N > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('log_laws_combine', u=2, v=3, p=2, q=1)
# -> value = 7   (SOLVED)
```

```python
math_foundations_ai999x.solve('log_laws_combine', u=1.5, v=4, p=3, q=-1)
# -> value = 0.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Given log_b(M) = 2 and log_b(N) = 3, find log_b(M^2 N). | SOLVED | value = 7 |
| If log_b(M) = 1.5 and log_b(N) = 4, find log_b(M^3/N). | SOLVED | value = 0.5 |
| Given log_b(M) = 6 and log_b(N) = 2, find log_b(sqrt(M)/N^2). | SOLVED | value = -1 |

<a id="continuous_growth"></a>

### Continuous exponential growth or decay — `continuous_growth`

N($t) = N_{0} e^(k$ t) for continuous growth $(k > 0)$ or decay $(k < 0)$, including continuous compounding.

$$
N(t) = N_0\,e^{kt}
$$

| input | meaning | condition |
|---|---|---|
| `n0` | initial amount N0 |  |
| `k` | continuous rate per unit time, as a decimal (0.03 for 3%; negative for decay) |  |
| `t` | elapsed time, in the rate's unit |  |

| output | meaning |
|---|---|
| `amount` | N(t) |

- **Method:** the solution of $dN/dt = kN$: $N = N_{0}$ $e^{kt}$
- **Assumes:** |k t| <= 700
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('continuous_growth', n0=500, k=0.03, t=10)
# -> amount = 674.929   (SOLVED)
```

```python
math_foundations_ai999x.solve('continuous_growth', n0=1000, k=0.05, t=3)
# -> amount = 1161.83   (SOLVED)
```

```python
math_foundations_ai999x.solve('continuous_growth', n0=200, k=-0.05, t=8)
# -> amount = 134.064   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A population of 500 grows continuously at 3% per year. Find the population after 10 years. | SOLVED | amount = 674.929 |
| Find the amount when 1000 is compounded continuously at 5% for 3 years. | SOLVED | amount = 1161.83 |
| A quantity of 200 decays continuously at a rate of 5% per hour. Find the amount after 8 hours. | SOLVED | amount = 134.064 |
| A population of 500 grows continuously at 3% per year. Find the population after 10 days. | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: math_foundations_ai999x.templates |

<a id="half_life_remaining"></a>

### Amount remaining after radioactive decay (half-life) — `half_life_remaining`

The amount left after time t of a substance with half-life T: $N_{0} (1/2)^{t/T}$.

$$
N(t) = N_0 \left(\tfrac12\right)^{t/T}
$$

| input | meaning | condition |
|---|---|---|
| `n0` | initial amount (positive) |  |
| `half_life` | the half-life T (positive) |  |
| `t` | elapsed time, in the unit of T |  |

| output | meaning |
|---|---|
| `remaining` | amount remaining (in the unit of N0) |
| `fraction` | fraction remaining |

- **Method:** each half-life halves the amount: $N = N_{0}$ $(1/2)^{t/T}$
- **Assumes:** $n_{0} > 0$; $T > 0$; $t  \geq  0$; t and T in the same unit
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('half_life_remaining', n0=80, half_life=5, t=15)
# -> remaining = 10, fraction = 0.125   (SOLVED)
```

```python
math_foundations_ai999x.solve('half_life_remaining', n0=200, half_life=8, t=20)
# -> remaining = 35.3553, fraction = 0.176777   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| An 80 g sample has a half-life of 5 years. How much remains after 15 years? | SOLVED | remaining = 10, fraction = 0.125 |
| A sample of 200 mg of iodine-131 has a half-life of 8 days. How much is left after 20 days? | SOLVED | remaining = 35.3553, fraction = 0.176777 |
| An 80 g sample has a half-life of 5 years. How much remains after 15 days? | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: math_foundations_ai999x.templates |

<a id="doubling_time_percent"></a>

### Doubling time at a percentage growth rate — `doubling_time_percent`

The time for a quantity growing by r% per period (compounded once per period) to double.

$$
T = \frac{\ln 2}{\ln\left(1 + \frac{r}{100}\right)}
$$

| input | meaning | condition |
|---|---|---|
| `r` | growth rate per period, in percent (positive) |  |

| output | meaning |
|---|---|
| `doubling_time` | periods to double (exact) |
| `rule_of_70_estimate` | the rule-of-70 approximation $70/r$ |

- **Method:** solve $(1 + r/100)^{T} = 2$: $T = ln$ $2 / ln$(1 + r/100)
- **Assumes:** $r > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('doubling_time_percent', r=7)
# -> doubling_time = 10.2448, rule_of_70_estimate = 10   (SOLVED)
```

```python
math_foundations_ai999x.solve('doubling_time_percent', r=100)
# -> doubling_time = 1, rule_of_70_estimate = 0.7   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How long does it take an investment to double at 7% interest compounded annually? | SOLVED | doubling_time = 10.2448, rule_of_70_estimate = 10 |
| What is the doubling time at a growth rate of 3% per year? | SOLVED | doubling_time = 23.4498, rule_of_70_estimate = 23.3333 |

<a id="exponential_equation_general"></a>

### Exponential equation a*b^(cx + d) = k — `exponential_equation_general`

Solve $a b^{c x + d} = k$ for x by taking logarithms.

$$
x = \frac{\log_b(k/a) - d}{c}
$$

| input | meaning | condition |
|---|---|---|
| `a` | the coefficient a | $a  \neq  0$ |
| `b` | the base (positive, not 1; e for the natural base) |  |
| `c` | coefficient of x in the exponent | $c  \neq  0$ |
| `d` | constant in the exponent |  |
| `k` | the right-hand side |  |

| output | meaning |
|---|---|
| `x` | the solution |

- **Method:** divide by a, take log base b: c $x + d = log_b$(k/a)
- **Assumes:** $a  \neq  0$; $c  \neq  0$; $b > 0$, $b  \neq  1$; $k/a > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('exponential_equation_general', a=3, b=2, c=1, d=1, k=48)
# -> x = 3   (SOLVED)
```

```python
math_foundations_ai999x.solve('exponential_equation_general', a=5, b=2.718281828459045, c=2, d=0, k=20)
# -> x = 0.693147   (SOLVED)
```

```python
math_foundations_ai999x.solve('exponential_equation_general', a=1, b=2, c=3, d=-1, k=16)
# -> x = 1.66667   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve 3 * 2^(x + 1) = 48. | SOLVED | x = 3 |
| Solve 5e^(2x) = 20. | SOLVED | x = 0.693147 |
| Solve 2^(3x - 1) = 16. | SOLVED | x = 1.66667 |
| Solve 4 * 3^x = 324. | SOLVED | x = 4 |
| Solve 2 * 3^(x - 1) = -6. | INVALID_INPUT | Exponential equation a*b^(cx + d) = k: no real solution: b^(cx + d) is positive, so k/a must be positive. |

<a id="log_equation_linear_argument"></a>

### Logarithmic equation log_b(cx + d) = k — `log_equation_linear_argument`

Solve log_b(c*x + d) = k for x by rewriting it in exponential form.

$$
\log_b(cx+d) = k \iff cx + d = b^{k}
$$

| input | meaning | condition |
|---|---|---|
| `b` | the base (positive, not 1; 10 for log, e for ln) |  |
| `c` | coefficient of x in the argument | $c  \neq  0$ |
| `d` | constant in the argument |  |
| `k` | the right-hand side |  |

| output | meaning |
|---|---|
| `x` | the solution |

- **Method:** log_b(u) = k means $u = b^{k}$; the argument $b^{k}$ is positive, so the solution is always admissible
- **Assumes:** $b > 0$, $b  \neq  1$; $c  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('log_equation_linear_argument', b=2, c=3, d=1, k=4)
# -> x = 5   (SOLVED)
```

```python
math_foundations_ai999x.solve('log_equation_linear_argument', b=2.718281828459045, c=2, d=-1, k=3)
# -> x = 10.5428   (SOLVED)
```

```python
math_foundations_ai999x.solve('log_equation_linear_argument', b=10, c=5, d=0, k=2)
# -> x = 20   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve log base 2 of (3x + 1) = 4. | SOLVED | x = 5 |
| Solve log_3(x - 4) = 2. | SOLVED | x = 13 |
| Solve ln(2x - 1) = 3. | SOLVED | x = 10.5428 |
| Solve log(5x) = 2. | SOLVED | x = 20 |
| Solve for x in the equation log_3(x - 1) = 2. | SOLVED | x = 10 |
| Solve log base 1 of (2x) = 3. | INVALID_INPUT | Logarithmic equation log_b(cx + d) = k: the base must be positive and not 1. |

<a id="deg_to_rad"></a>

### Degrees to radians — `deg_to_rad`

Convert an angle in degrees to radians (also as a multiple of pi).

$$
\theta_{\mathrm{rad}} = \theta_{\deg}\cdot\frac{\pi}{180}
$$

| input | meaning | condition |
|---|---|---|
| `deg` | the angle in degrees |  |

| output | meaning |
|---|---|
| `radians` | the angle in radians |
| `pi_multiple` | the angle as a multiple of pi |
| `exact` | exact form, e.g. $3pi/4$ |

- **Method:** multiply by $pi/180$
- **Assumes:** a finite angle
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('deg_to_rad', deg=135)
# -> radians = 2.35619, pi_multiple = 0.75, exact = 3pi/4   (SOLVED)
```

```python
math_foundations_ai999x.solve('deg_to_rad', deg=210)
# -> radians = 3.66519, pi_multiple = 1.16667, exact = 7pi/6   (SOLVED)
```

```python
math_foundations_ai999x.solve('deg_to_rad', deg=-90)
# -> radians = -1.5708, pi_multiple = -0.5, exact = -pi/2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Convert 135 degrees to radians. | SOLVED | radians = 2.35619, pi_multiple = 0.75, exact = 3pi/4 |
| What is 210° in radians? | SOLVED | radians = 3.66519, pi_multiple = 1.16667, exact = 7pi/6 |
| Express 1 degree in radians. | SOLVED | radians = 0.0174533, pi_multiple = 0.00555556, exact = pi/180 |

<a id="rad_to_deg"></a>

### Radians to degrees — `rad_to_deg`

Convert an angle in radians (a number, or a multiple of pi such as 3pi/4) to degrees.

$$
\theta_{\deg} = \theta_{\mathrm{rad}}\cdot\frac{180}{\pi}
$$

| input | meaning | condition |
|---|---|---|
| `rad` | the angle in radians |  |

| output | meaning |
|---|---|
| `degrees` | the angle in degrees |

- **Method:** multiply by $180/pi$
- **Assumes:** a finite angle
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('rad_to_deg', rad=2.356194490192345)
# -> degrees = 135   (SOLVED)
```

```python
math_foundations_ai999x.solve('rad_to_deg', rad=2.5)
# -> degrees = 143.239   (SOLVED)
```

```python
math_foundations_ai999x.solve('rad_to_deg', rad=3.141592653589793)
# -> degrees = 180   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Convert 3pi/4 radians to degrees. | SOLVED | degrees = 135 |
| Convert 2.5 radians to degrees. | SOLVED | degrees = 143.239 |
| What is 5π/6 in degrees? | SOLVED | degrees = 150 |

<a id="right_triangle_legs"></a>

### Right triangle from its two legs — `right_triangle_legs`

Given the legs a and b of a right triangle: the hypotenuse c and the acute angles A (opposite a) and B.

$$
c = \sqrt{a^2 + b^2},\quad \tan A = \frac{a}{b}
$$

| input | meaning | condition |
|---|---|---|
| `a` | the leg opposite angle A (positive) |  |
| `b` | the leg opposite angle B (positive) |  |

| output | meaning |
|---|---|
| `c` | the hypotenuse |
| `A` | angle opposite a, degrees |
| `B` | angle opposite b, degrees |

- **Method:** Pythagoras $c = \sqrt{a^{2} + b^{2}}$; tan $A = a/b$; $B = 90 - A$
- **Assumes:** $a > 0$; $b > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('right_triangle_legs', a=3, b=4)
# -> c = 5, A = 36.8699, B = 53.1301   (SOLVED)
```

```python
math_foundations_ai999x.solve('right_triangle_legs', a=5, b=12)
# -> c = 13, A = 22.6199, B = 67.3801   (SOLVED)
```

```python
math_foundations_ai999x.solve('right_triangle_legs', a=1, b=1)
# -> c = 1.41421, A = 45, B = 45   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A right triangle has legs 3 and 4. Find the hypotenuse and the acute angles. | SOLVED | c = 5, A = 36.8699, B = 53.1301 |
| Solve the right triangle with legs a = 5 and b = 12. | SOLVED | c = 13, A = 22.6199, B = 67.3801 |
| Solve the right triangle with legs a = 0 and b = 12. | INVALID_INPUT | Right triangle from its two legs: the legs must be positive. |

<a id="right_triangle_leg_hypotenuse"></a>

### Right triangle from a leg and the hypotenuse — `right_triangle_leg_hypotenuse`

Given a leg a and the hypotenuse c of a right triangle: the other leg b and the acute angles A (opposite a) and B.

$$
b = \sqrt{c^2 - a^2},\quad \sin A = \frac{a}{c}
$$

| input | meaning | condition |
|---|---|---|
| `a` | the known leg (positive) |  |
| `c` | the hypotenuse (longer than the leg) |  |

| output | meaning |
|---|---|
| `b` | the other leg |
| `A` | angle opposite a, degrees |
| `B` | angle opposite b, degrees |

- **Method:** $b = \sqrt{c^{2} - a^{2}}$; sin $A = a/c$; $B = 90 - A$
- **Assumes:** $0 < a < c$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('right_triangle_leg_hypotenuse', a=5, c=13)
# -> b = 12, A = 22.6199, B = 67.3801   (SOLVED)
```

```python
math_foundations_ai999x.solve('right_triangle_leg_hypotenuse', a=3, c=6)
# -> b = 5.19615, A = 30, B = 60   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A right triangle has hypotenuse 13 and one leg 5. Find the other leg and the acute angles. | SOLVED | b = 12, A = 22.6199, B = 67.3801 |
| A right triangle has hypotenuse 6 and one leg 3. Find the other leg and the angles. | SOLVED | b = 5.19615, A = 30, B = 60 |
| A right triangle has hypotenuse 5 and one leg 7. Find the other leg. | INVALID_INPUT | Right triangle from a leg and the hypotenuse: the leg must be positive and shorter than the hypotenuse. |

<a id="right_triangle_angle_side"></a>

### Right triangle from an acute angle and a side — `right_triangle_angle_side`

Given an acute angle A and one side (the hypotenuse, the side opposite A or the side adjacent to A): every side and the other angle.

$$
\sin A = \frac{\mathrm{opp}}{\mathrm{hyp}},\ \cos A = \frac{\mathrm{adj}}{\mathrm{hyp}},\ \tan A = \frac{\mathrm{opp}}{\mathrm{adj}}
$$

| input | meaning | condition |
|---|---|---|
| `A` | the acute angle A in degrees |  |
| `side` | the length of the known side |  |
| `which` | which side is known |  |

| output | meaning |
|---|---|
| `opposite` | side opposite A |
| `adjacent` | side adjacent to A |
| `hypotenuse` | the hypotenuse |
| `B` | the other acute angle, degrees |

- **Method:** SOH-CAH-TOA: sin A = opp/hyp, cos A = adj/hyp, tan A = opp/adj
- **Assumes:** $0 < A < 90$; side > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('right_triangle_angle_side', A=30, side=10, which='hypotenuse')
# -> opposite = 5, adjacent = 8.66025, hypotenuse = 10, B = 60   (SOLVED)
```

```python
math_foundations_ai999x.solve('right_triangle_angle_side', A=45, side=7, which='opposite')
# -> opposite = 7, adjacent = 7, hypotenuse = 9.89949, B = 45   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| In a right triangle, angle A is 30 degrees and the hypotenuse is 10. Find the other sides. | SOLVED | opposite = 5, adjacent = 8.66025, hypotenuse = 10, B = 60 |
| In a right triangle, angle A is 45 degrees and the side opposite A is 7. Find the other sides. | SOLVED | opposite = 7, adjacent = 7, hypotenuse = 9.89949, B = 45 |
| In a right triangle, angle A is 95 degrees and the hypotenuse is 10. Find the other sides. | INVALID_INPUT | Right triangle from an acute angle and a side: the acute angle must be between 0 and 90 degrees. |

<a id="law_of_sines_aas"></a>

### Law of sines (two angles and a side) — `law_of_sines_aas`

Given angles A and B and the side a opposite A: the third angle C and the sides b and c.

$$
\frac{a}{\sin A} = \frac{b}{\sin B} = \frac{c}{\sin C}
$$

| input | meaning | condition |
|---|---|---|
| `A` | angle A, degrees |  |
| `B` | angle B, degrees |  |
| `a` | side a, opposite A |  |

| output | meaning |
|---|---|
| `C` | angle C, degrees |
| `b` | side b |
| `c` | side c |

- **Method:** $C = 180 - A - B$; law of sines a/sin $A = b/sin$ $B = c/sin$ C
- **Assumes:** A, $B > 0$; $A + B < 180$; $a > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('law_of_sines_aas', A=40, B=60, a=10)
# -> C = 80, b = 13.473, c = 15.3209   (SOLVED)
```

```python
math_foundations_ai999x.solve('law_of_sines_aas', A=30, B=90, a=5)
# -> C = 60, b = 10, c = 8.66025   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| In triangle ABC, A = 40 degrees, B = 60 degrees and a = 10. Solve the triangle. | SOLVED | C = 80, b = 13.473, c = 15.3209 |
| In triangle ABC, A = 30°, B = 90° and a = 5. Find the remaining sides and angles. | SOLVED | C = 60, b = 10, c = 8.66025 |
| In triangle ABC, A = 100 degrees, B = 90 degrees and a = 5. Solve the triangle. | INVALID_INPUT | Law of sines (two angles and a side): the two angles must be positive with a sum below 180 degrees. |

<a id="law_of_sines_ssa"></a>

### Law of sines, the ambiguous case (two sides and a non-included angle) — `law_of_sines_ssa`

Given sides a and b and the angle A opposite a: how many triangles exist (0, 1 or 2) and every one of them.

$$
\sin B = \frac{b\sin A}{a},\quad B_2 = 180^\circ - B_1
$$

| input | meaning | condition |
|---|---|---|
| `a` | side a, opposite A |  |
| `b` | side b |  |
| `A` | angle A, degrees |  |

| output | meaning |
|---|---|
| `count` | number of triangles (1 or 2; none is refused) |
| `B1` | angle B of the first triangle (the acute one) |
| `C1` | angle C of the first triangle |
| `c1` | side c of the first triangle |
| `B2` | angle B of the second triangle (obtuse), when there are two |
| `C2` | angle C of the second triangle |
| `c2` | side c of the second triangle |

- **Method:** sin $B = b$ sin A / a; B and $180 - B$ both give a triangle when $A + B < 180$ (the ambiguous case)
- **Assumes:** a, $b > 0$; $0 < A < 180$; a triangle exists (else refused)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('law_of_sines_ssa', a=8, b=10, A=40)
# -> count = 2, B1 = 53.4641, C1 = 86.5359, c1 = 12.423, B2 = 126.536, C2 = 13.4641, c2 = 2.89784   (SOLVED)
```

```python
math_foundations_ai999x.solve('law_of_sines_ssa', a=12, b=10, A=40)
# -> count = 1, B1 = 32.3884, C1 = 107.612, c1 = 17.7937   (SOLVED)
```

```python
math_foundations_ai999x.solve('law_of_sines_ssa', a=5, b=10, A=30)
# -> count = 1, B1 = 90, C1 = 60, c1 = 8.66025   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| In triangle ABC, a = 8, b = 10 and A = 40 degrees. Solve the triangle. | SOLVED | count = 2, B1 = 53.4641, C1 = 86.5359, c1 = 12.423, B2 = 126.536, C2 = 13.4641, c2 = 2.89784 |
| In triangle ABC, a = 12, b = 10 and A = 40°. Solve the triangle. | SOLVED | count = 1, B1 = 32.3884, C1 = 107.612, c1 = 17.7937 |
| In triangle ABC, a = 3, b = 10 and A = 40 degrees. Solve the triangle. | INVALID_INPUT | Law of sines, the ambiguous case (two sides and a non-included angle): no triangle exists: side a is shorter than the he |

<a id="law_of_cosines_sas"></a>

### Law of cosines (two sides and the included angle) — `law_of_cosines_sas`

Given sides b and c and the included angle A: the third side a and the angles B and C.

$$
a^2 = b^2 + c^2 - 2bc\cos A
$$

| input | meaning | condition |
|---|---|---|
| `b` | side b |  |
| `c` | side c |  |
| `A` | the included angle A, degrees |  |

| output | meaning |
|---|---|
| `a` | side a |
| `B` | angle B, degrees |
| `C` | angle C, degrees |

- **Method:** $a^{2} = b^{2} + c^{2} - 2bc$ cos A; B from the law of cosines (no ambiguity)
- **Assumes:** b, $c > 0$; $0 < A < 180$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('law_of_cosines_sas', b=5, c=7, A=49)
# -> a = 5.29867, B = 45.4117, C = 85.5883   (SOLVED)
```

```python
math_foundations_ai999x.solve('law_of_cosines_sas', b=3, c=4, A=90)
# -> a = 5, B = 36.8699, C = 53.1301   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| In triangle ABC, b = 5, c = 7 and A = 49 degrees. Solve the triangle. | SOLVED | a = 5.29867, B = 45.4117, C = 85.5883 |
| In triangle ABC, b = 3, c = 4 and A = 90°. Find the remaining sides and angles. | SOLVED | a = 5, B = 36.8699, C = 53.1301 |
| In triangle ABC, b = 3, c = 4 and A = 190 degrees. Solve the triangle. | INVALID_INPUT | Law of cosines (two sides and the included angle): the sides must be positive and A between 0 and 180 degrees. |

<a id="law_of_cosines_sss"></a>

### Law of cosines (three sides) — `law_of_cosines_sss`

Given the three sides a, b, c: the angles A, B, C.

$$
\cos A = \frac{b^2 + c^2 - a^2}{2bc}
$$

| input | meaning | condition |
|---|---|---|
| `a` | side a |  |
| `b` | side b |  |
| `c` | side c |  |

| output | meaning |
|---|---|
| `A` | angle A, degrees |
| `B` | angle B, degrees |
| `C` | angle C, degrees |

- **Method:** cos $A = (b^{2} + c^{2} - a^{2})/(2bc)$, likewise B; $C = 180 - A - B$
- **Assumes:** positive sides; triangle inequality (each side shorter than the sum of the other two)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('law_of_cosines_sss', a=7, b=8, c=9)
# -> A = 48.1897, B = 58.4119, C = 73.3985   (SOLVED)
```

```python
math_foundations_ai999x.solve('law_of_cosines_sss', a=3, b=4, c=5)
# -> A = 36.8699, B = 53.1301, C = 90   (SOLVED)
```

```python
math_foundations_ai999x.solve('law_of_cosines_sss', a=2, b=2, c=2)
# -> A = 60, B = 60, C = 60   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| In triangle ABC, a = 7, b = 8 and c = 9. Find the angles. | SOLVED | A = 48.1897, B = 58.4119, C = 73.3985 |
| In triangle ABC, a = 3, b = 4 and c = 5. Solve the triangle. | SOLVED | A = 36.8699, B = 53.1301, C = 90 |
| In triangle ABC, a = 1, b = 2 and c = 5. Find the angles. | INVALID_INPUT | Law of cosines (three sides): these lengths do not form a triangle (each side must be shorter than the sum of the other  |

<a id="heron_area"></a>

### Area of a triangle from its three sides (Heron) — `heron_area`

The area of a triangle with sides a, b, c, and its semi-perimeter s.

$$
K = \sqrt{s(s-a)(s-b)(s-c)},\quad s = \frac{a+b+c}{2}
$$

| input | meaning | condition |
|---|---|---|
| `a` | side a |  |
| `b` | side b |  |
| `c` | side c |  |

| output | meaning |
|---|---|
| `area` | the area |
| `s` | the semi-perimeter $(a + b + c)/2$ |

- **Method:** Heron's formula, in its numerically stable arrangement
- **Assumes:** positive sides; triangle inequality
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('heron_area', a=13, b=14, c=15)
# -> area = 84, s = 21   (SOLVED)
```

```python
math_foundations_ai999x.solve('heron_area', a=3, b=4, c=5)
# -> area = 6, s = 6   (SOLVED)
```

```python
math_foundations_ai999x.solve('heron_area', a=5, b=6, c=7)
# -> area = 14.6969, s = 9   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the area of a triangle with sides 13, 14 and 15. | SOLVED | area = 84, s = 21 |
| Use Heron's formula to find the area of a triangle with sides 5, 6 and 7. | SOLVED | area = 14.6969, s = 9 |
| Find the area of a triangle with sides 1, 2 and 3. | INVALID_INPUT | Area of a triangle from its three sides (Heron): these lengths do not form a triangle (each side must be shorter than th |

<a id="unit_circle_exact"></a>

### Exact trigonometric values on the unit circle — `unit_circle_exact`

The exact sin, cos and tan of an angle that is a multiple of 30 or 45 degrees (pi/6, pi/4), with the reference angle.

$$
\sin 30^\circ = \tfrac12,\ \sin 45^\circ = \tfrac{\sqrt2}{2},\ \sin 60^\circ = \tfrac{\sqrt3}{2}
$$

| input | meaning | condition |
|---|---|---|
| `deg` | the angle in degrees (a multiple of 30 or 45) |  |

| output | meaning |
|---|---|
| `sin` | sin of the angle |
| `cos` | cos of the angle |
| `tan` | tan of the angle (None where undefined) |
| `sin_exact` | exact sin |
| `cos_exact` | exact cos |
| `tan_exact` | exact tan |
| `reference_angle` | the reference angle, degrees |

- **Method:** the reference angle's exact values from the $30-60-90$ and $45-45-90$ triangles, with the signs of the quadrant (ASTC)
- **Assumes:** a whole multiple of 30 or 45 degrees
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('unit_circle_exact', deg=150)
# -> sin = 0.5, cos = -0.866025, tan = -0.57735, sin_exact = 1/2, cos_exact = -sqrt(3)/2, tan_exact = -sqrt(3)/3, reference_angle = 30   (SOLVED)
```

```python
math_foundations_ai999x.solve('unit_circle_exact', deg=225)
# -> sin = -0.707107, cos = -0.707107, tan = 1, sin_exact = -sqrt(2)/2, cos_exact = -sqrt(2)/2, tan_exact = 1, reference_angle = 45   (SOLVED)
```

```python
math_foundations_ai999x.solve('unit_circle_exact', deg=90)
# -> sin = 1, cos = 0, tan = none, sin_exact = 1, cos_exact = 0, tan_exact = undefined, reference_angle = 90   (SOLVED)
```

```python
math_foundations_ai999x.solve('unit_circle_exact', deg=-60)
# -> sin = -0.866025, cos = 0.5, tan = -1.73205, sin_exact = -sqrt(3)/2, cos_exact = 1/2, tan_exact = -sqrt(3), reference_angle = 60   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the exact values of sin, cos and tan of 150 degrees. | SOLVED | sin = 0.5, cos = -0.866025, tan = -0.57735, sin_exact = 1/2, cos_exact = -sqrt(3)/2, tan_exact = -sqrt(3)/3, reference_angle = 30 |
| Find the exact value of sin(5pi/6). | SOLVED | sin = 0.5, cos = -0.866025, tan = -0.57735, sin_exact = 1/2, cos_exact = -sqrt(3)/2, tan_exact = -sqrt(3)/3, reference_angle = 30 |
| What is cos(225°) exactly? | SOLVED | sin = -0.707107, cos = -0.707107, tan = 1, sin_exact = -sqrt(2)/2, cos_exact = -sqrt(2)/2, tan_exact = 1, reference_angle = 45 |
| Find the exact value of tan(20 degrees). | INVALID_INPUT | Exact trigonometric values on the unit circle: exact values are given for multiples of 30 or 45 degrees (pi/6 or pi/4). |

<a id="trig_equation_interval"></a>

### Solve a trigonometric equation over one full turn — `trig_equation_interval`

Every solution of $a sin$(n x) = k (or cos, tan) with $0  \leq  x < 360$ degrees or $0  \leq  x < 2$ pi.

$$
\sin u = k \Rightarrow u = \arcsin k,\ 180^\circ - \arcsin k \pmod{360^\circ},\quad u = n x
$$

| input | meaning | condition |
|---|---|---|
| `func` | sin, cos or tan |  |
| `k` | the value the function must take (after dividing by its coefficient) |  |
| `n` | the multiple n in f(n x) (default 1) |  |
| `unit` | degrees ([0, 360)) or radians ([0, 2pi)) |  |

| output | meaning |
|---|---|
| `count` | number of solutions in the interval |
| `solutions` | the solutions, ascending |

- **Method:** principal value (asin, acos or atan), the second solution of the period (180 - a, -a, a + 180), then every period: x = (u + 360 m)/n inside the interval
- **Assumes:** n a whole number from 1 to 12
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('trig_equation_interval', func='sin', k=0.5, n=1, unit='degrees')
# -> count = 2, solutions = [30.000000000000004, 150.0]   (SOLVED)
```

```python
math_foundations_ai999x.solve('trig_equation_interval', func='cos', k=0.5, n=1, unit='radians')
# -> count = 2, solutions = [1.0471975511965979, 5.235987755982989]   (SOLVED)
```

```python
math_foundations_ai999x.solve('trig_equation_interval', func='tan', k=1, n=2, unit='degrees')
# -> count = 4, solutions = [22.5, 112.5, 202.5, 292.5]   (SOLVED)
```

```python
math_foundations_ai999x.solve('trig_equation_interval', func='sin', k=1, n=1, unit='degrees')
# -> count = 1, solutions = [90.0]   (SOLVED)
```

```python
math_foundations_ai999x.solve('trig_equation_interval', func='cos', k=2, n=1, unit='degrees')
# -> count = 0, solutions = []   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve sin(x) = 0.5 for 0 <= x < 360 degrees. | SOLVED | count = 2, solutions = [30.000000000000004, 150.0] |
| Solve 2cos(x) = 1 on [0, 2pi). | SOLVED | count = 2, solutions = [1.0471975511965979, 5.235987755982989] |
| Solve tan(2x) = 1 for 0 <= x < 360 degrees. | SOLVED | count = 4, solutions = [22.5, 112.5, 202.5, 292.5] |
| Solve sin(x) = 0 for 0 ≤ x < 2π. | SOLVED | count = 2, solutions = [0.0, 3.141592653589793] |
| Solve 3sin(x) = 4 for 0 <= x < 360 degrees. | SOLVED | count = 0, solutions = [] |

<a id="trig_values_from_one"></a>

### Trigonometric values from one value and the quadrant — `trig_values_from_one`

Given sin, cos or tan of an angle and its quadrant: sin, cos, tan, sin 2t and cos 2t (Pythagorean and double-angle identities).

$$
\cos t = \pm\sqrt{1 - \sin^2 t},\quad \sin 2t = 2\sin t\cos t,\quad \cos 2t = \cos^2 t - \sin^2 t
$$

| input | meaning | condition |
|---|---|---|
| `func` | the function whose value is known |  |
| `value` | its value |  |
| `quadrant` | the quadrant of the angle (1 to 4) |  |

| output | meaning |
|---|---|
| `sin` | sin t |
| `cos` | cos t |
| `tan` | tan t |
| `sin2` | sin 2t |
| `cos2` | cos 2t |

- **Method:** $sin^{2} + cos^{2} = 1$ with the quadrant's signs (ASTC); sin $2t = 2$ sin t cos t; cos $2t = cos^{2}$ $t - sin^{2}$ t
- **Assumes:** the value's sign agrees with the quadrant; 0 < |sin|, |cos| < 1
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('trig_values_from_one', func='sin', value=0.6, quadrant=2)
# -> sin = 0.6, cos = -0.8, tan = -0.75, sin2 = -0.96, cos2 = 0.28   (SOLVED)
```

```python
math_foundations_ai999x.solve('trig_values_from_one', func='cos', value=-0.38461538461538464, quadrant=3)
# -> sin = -0.923077, cos = -0.384615, tan = 2.4, sin2 = 0.710059, cos2 = -0.704142   (SOLVED)
```

```python
math_foundations_ai999x.solve('trig_values_from_one', func='tan', value=-2, quadrant=4)
# -> sin = -0.894427, cos = 0.447214, tan = -2, sin2 = -0.8, cos2 = -0.6   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| If sin(t) = 3/5 and t is in quadrant II, find cos(t), tan(t), sin(2t) and cos(2t). | SOLVED | sin = 0.6, cos = -0.8, tan = -0.75, sin2 = -0.96, cos2 = 0.28 |
| Given cos(x) = -5/13 and x lies in the third quadrant, find sin(x) and tan(x). | SOLVED | sin = -0.923077, cos = -0.384615, tan = 2.4, sin2 = 0.710059, cos2 = -0.704142 |
| If sin(t) = 3/5 and t is in quadrant III, find cos(t). | INVALID_INPUT | Trigonometric values from one value and the quadrant: sine is negative in quadrant 3. |

<a id="vector_dot_angle"></a>

### Dot product and angle between two vectors — `vector_dot_angle`

The dot product, the magnitudes and the angle between two vectors in the plane or in space.

$$
\mathbf u\cdot\mathbf v = |\mathbf u||\mathbf v|\cos\theta
$$

| input | meaning | condition |
|---|---|---|
| `u` | the first vector, e.g. 3, 4 |  |
| `v` | the second vector |  |

| output | meaning |
|---|---|
| `dot` | u . v |
| `mag_u` | \|u\| |
| `mag_v` | \|v\| |
| `angle_deg` | the angle between them, degrees (0 to 180) |

- **Method:** u . v = sum of products of components; the angle from atan2(|u x v|, u . v) (stable for nearly parallel vectors)
- **Assumes:** same dimension (2 or 3); neither vector is zero
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('vector_dot_angle', u='3, 4', v='4, -3')
# -> dot = 0, mag_u = 5, mag_v = 5, angle_deg = 90   (SOLVED)
```

```python
math_foundations_ai999x.solve('vector_dot_angle', u=[1, 2, 3], v=[4, -5, 6])
# -> dot = 12, mag_u = 3.74166, mag_v = 8.77496, angle_deg = 68.5624   (SOLVED)
```

```python
math_foundations_ai999x.solve('vector_dot_angle', u=[1, 0], v=[1, 1])
# -> dot = 1, mag_u = 1, mag_v = 1.41421, angle_deg = 45   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the angle between u = <3, 4> and v = <4, -3>. | SOLVED | dot = 0, mag_u = 5, mag_v = 5, angle_deg = 90 |
| Find the dot product of <1, 2, 3> and <4, -5, 6>. | SOLVED | dot = 12, mag_u = 3.74166, mag_v = 8.77496, angle_deg = 68.5624 |
| Find the dot product and the angle between the vectors (1, 0) and (1, 1). | SOLVED | dot = 1, mag_u = 1, mag_v = 1.41421, angle_deg = 45 |
| Find the angle between u = <0, 0> and v = <1, 2>. | INVALID_INPUT | Dot product and angle between two vectors: the zero vector has no direction, so the angle is not defined. |

<a id="vector_magnitude_direction"></a>

### Magnitude and direction of a vector — `vector_magnitude_direction`

The length of the plane vector <x, y> and its direction angle from the positive x-axis (0 to 360 degrees).

$$
|\mathbf v| = \sqrt{x^2 + y^2},\quad \theta = \operatorname{atan2}(y, x)
$$

| input | meaning | condition |
|---|---|---|
| `x` | the x-component |  |
| `y` | the y-component |  |

| output | meaning |
|---|---|
| `magnitude` | \|v\| |
| `direction_deg` | the direction angle, degrees in [0, 360) |

- **Method:** |v| = sqrt(x^2 + y^2); the angle in the quadrant of (x, y)
- **Assumes:** not the zero vector
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('vector_magnitude_direction', x=-3, y=4)
# -> magnitude = 5, direction_deg = 126.87   (SOLVED)
```

```python
math_foundations_ai999x.solve('vector_magnitude_direction', x=5, y=-12)
# -> magnitude = 13, direction_deg = 292.62   (SOLVED)
```

```python
math_foundations_ai999x.solve('vector_magnitude_direction', x=0, y=-2)
# -> magnitude = 2, direction_deg = 270   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the magnitude and direction of the vector <-3, 4>. | SOLVED | magnitude = 5, direction_deg = 126.87 |
| Find the magnitude and direction angle of v = <0, -2>. | SOLVED | magnitude = 2, direction_deg = 270 |
| Find the magnitude and direction of the vector <0, 0>. | INVALID_INPUT | Magnitude and direction of a vector: the zero vector has no direction. |

<a id="polar_to_rectangular"></a>

### Polar to rectangular coordinates — `polar_to_rectangular`

The point (x, y) with polar coordinates (r, theta).

$$
x = r\cos\theta,\quad y = r\sin\theta
$$

| input | meaning | condition |
|---|---|---|
| `r` | the distance r from the pole |  |
| `theta` | the angle theta |  |
| `unit` | degrees or radians |  |

| output | meaning |
|---|---|
| `x` | $x = r$ cos(theta) |
| `y` | $y = r$ sin(theta) |

- **Method:** $x = r$ cos(theta), $y = r$ sin(theta)
- **Assumes:** the angle's unit stated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('polar_to_rectangular', r=4, theta=60, unit='degrees')
# -> x = 2, y = 3.4641   (SOLVED)
```

```python
math_foundations_ai999x.solve('polar_to_rectangular', r=2, theta=2.356194490192345, unit='radians')
# -> x = -1.41421, y = 1.41421   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Convert the polar point (4, 60 degrees) to rectangular coordinates. | SOLVED | x = 2, y = 3.4641 |
| Convert (2, 3pi/4) from polar to rectangular coordinates. | SOLVED | x = -1.41421, y = 1.41421 |
| Convert the polar point (4, 60) to rectangular coordinates. | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: math_foundations_ai999x.templates |

<a id="rectangular_to_polar"></a>

### Rectangular to polar coordinates — `rectangular_to_polar`

Polar coordinates (r, theta) of the point (x, y), with $r > 0$ and theta in [0, 360) degrees / [0, 2 pi).

$$
r = \sqrt{x^2+y^2},\quad \tan\theta = \frac{y}{x}
$$

| input | meaning | condition |
|---|---|---|
| `x` | x-coordinate |  |
| `y` | y-coordinate |  |

| output | meaning |
|---|---|
| `r` | $r = \sqrt{x^{2} + y^{2}}$ |
| `theta_deg` | theta in degrees, [0, 360) |
| `theta_rad` | theta in radians, [0, 2 pi) |

- **Method:** $r = \sqrt{x^{2} + y^{2}}$; theta in the quadrant of (x, y)
- **Assumes:** not the origin
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('rectangular_to_polar', x=3, y=-3)
# -> r = 4.24264, theta_deg = 315, theta_rad = 5.49779   (SOLVED)
```

```python
math_foundations_ai999x.solve('rectangular_to_polar', x=-1, y=1)
# -> r = 1.41421, theta_deg = 135, theta_rad = 2.35619   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Convert the point (3, -3) to polar coordinates. | SOLVED | r = 4.24264, theta_deg = 315, theta_rad = 5.49779 |
| Find the polar coordinates of (-1, 1). | SOLVED | r = 1.41421, theta_deg = 135, theta_rad = 2.35619 |
| Convert the point (0, 0) to polar coordinates. | INVALID_INPUT | Rectangular to polar coordinates: the origin has r = 0 and no defined angle. |

<a id="parabola_focus_directrix"></a>

### Focus and directrix of a parabola — `parabola_focus_directrix`

Vertex, focal distance p, focus and directrix of $(x - h)^{2} = 4p$(y - k) or $(y - k)^{2} = 4p$(x - h).

$$
(x-h)^2 = 4p(y-k):\ F = (h, k+p),\ y = k - p
$$

| input | meaning | condition |
|---|---|---|
| `orientation` | vertical axis ($(x-h)^{2} = m$(y-k)) or horizontal |  |
| `m` | the coefficient $m = 4p$ | $m  \neq  0$ |
| `h` | x of the vertex |  |
| `k` | y of the vertex |  |

| output | meaning |
|---|---|
| `p` | focal distance $p = m/4$ (signed) |
| `focus_x` | x of the focus |
| `focus_y` | y of the focus |
| `directrix` | the directrix: y = this (vertical axis) or x = this (horizontal axis) |
| `vertex_x` | x of the vertex |
| `vertex_y` | y of the vertex |

- **Method:** compare with $(x - h)^{2} = 4p$(y - k): $p = m/4$, focus at distance p from the vertex along the axis
- **Assumes:** $m  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('parabola_focus_directrix', orientation='vertical', m=12)
# -> p = 3, focus_x = 0, focus_y = 3, directrix = -3, vertex_x = 0, vertex_y = 0   (SOLVED)
```

```python
math_foundations_ai999x.solve('parabola_focus_directrix', orientation='horizontal', m=-8, h=3, k=1)
# -> p = -2, focus_x = 1, focus_y = 1, directrix = 5, vertex_x = 3, vertex_y = 1   (SOLVED)
```

```python
math_foundations_ai999x.solve('parabola_focus_directrix', orientation='vertical', m=0.5)
# -> p = 0.125, focus_x = 0, focus_y = 0.125, directrix = -0.125, vertex_x = 0, vertex_y = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the focus and directrix of the parabola x^2 = 12y. | SOLVED | p = 3, focus_x = 0, focus_y = 3, directrix = -3, vertex_x = 0, vertex_y = 0 |
| Find the focus and directrix of (y - 1)^2 = -8(x - 3). | SOLVED | p = -2, focus_x = 1, focus_y = 1, directrix = 5, vertex_x = 3, vertex_y = 1 |
| Find the focus and directrix of y = 2x^2. | SOLVED | p = 0.125, focus_x = 0, focus_y = 0.125, directrix = -0.125, vertex_x = 0, vertex_y = 0 |
| Find the focus and directrix of the parabola (x + 2)^2 = 4(y - 5). | SOLVED | p = 1, focus_x = -2, focus_y = 6, directrix = 4, vertex_x = -2, vertex_y = 5 |

<a id="ellipse_parameters"></a>

### Foci, axes and eccentricity of an ellipse — `ellipse_parameters`

For $(x - h)^{2}/A + (y - k)^{2}/B = 1$: semi-axes a and b, focal distance c, eccentricity and the two foci.

$$
\frac{(x-h)^2}{A} + \frac{(y-k)^2}{B} = 1,\quad c^2 = a^2 - b^2,\quad e = \frac{c}{a}
$$

| input | meaning | condition |
|---|---|---|
| `h` | x of the centre |  |
| `k` | y of the centre |  |
| `A` | denominator under the x term (positive) |  |
| `B` | denominator under the y term (positive) |  |

| output | meaning |
|---|---|
| `a` | semi-major axis |
| `b` | semi-minor axis |
| `c` | centre-to-focus distance |
| `eccentricity` | $e = c/a$ |
| `f1x` | first focus x |
| `f1y` | first focus y |
| `f2x` | second focus x |
| `f2y` | second focus y |
| `orientation` | the major axis is horizontal or vertical |

- **Method:** a^2 = the larger denominator, b^2 = the smaller, $c^{2} = a^{2} - b^{2}$; foci on the major axis
- **Assumes:** A, $B > 0$; $A  \neq  B$ (otherwise a circle)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('ellipse_parameters', h=0, k=0, A=25, B=9)
# -> a = 5, b = 3, c = 4, eccentricity = 0.8, f1x = -4, f1y = 0, f2x = 4, f2y = 0, orientation = horizontal   (SOLVED)
```

```python
math_foundations_ai999x.solve('ellipse_parameters', h=1, k=-2, A=16, B=25)
# -> a = 5, b = 4, c = 3, eccentricity = 0.6, f1x = 1, f1y = -5, f2x = 1, f2y = 1, orientation = vertical   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the foci and eccentricity of the ellipse x^2/25 + y^2/9 = 1. | SOLVED | a = 5, b = 3, c = 4, eccentricity = 0.8, f1x = -4, f1y = 0, f2x = 4, f2y = 0, orientation = horizontal |
| Find the foci of (x - 1)^2/16 + (y + 2)^2/25 = 1. | SOLVED | a = 5, b = 4, c = 3, eccentricity = 0.6, f1x = 1, f1y = -5, f2x = 1, f2y = 1, orientation = vertical |
| Find the foci of the ellipse x^2/4 + y^2/4 = 1. | INVALID_INPUT | Foci, axes and eccentricity of an ellipse: equal denominators give a circle (no distinct foci). |

<a id="hyperbola_parameters"></a>

### Foci, eccentricity and asymptotes of a hyperbola — `hyperbola_parameters`

For $(x - h)^{2}/A - (y - k)^{2}/B = 1$ (or the vertical form): a, b, c, eccentricity, the asymptotes' slope and the foci.

$$
\frac{(x-h)^2}{a^2} - \frac{(y-k)^2}{b^2} = 1,\quad c^2 = a^2 + b^2
$$

| input | meaning | condition |
|---|---|---|
| `h` | x of the centre |  |
| `k` | y of the centre |  |
| `A` | denominator of the positive term |  |
| `B` | denominator of the negative term |  |
| `orientation` | horizontal (x term positive) or vertical |  |

| output | meaning |
|---|---|
| `a` | $a = \sqrt{A}$ |
| `b` | $b = \sqrt{B}$ |
| `c` | centre-to-focus distance |
| `eccentricity` | $e = c/a$ |
| `asymptote_slope` | slope of the asymptotes (plus or minus this) |
| `f1x` | first focus x |
| `f1y` | first focus y |
| `f2x` | second focus x |
| `f2y` | second focus y |

- **Method:** $c^{2} = a^{2} + b^{2}$; asymptotes y - k = ±(b/a)(x - h) (horizontal) or ±(a/b) (vertical)
- **Assumes:** A, $B > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('hyperbola_parameters', h=0, k=0, A=16, B=9, orientation='horizontal')
# -> a = 4, b = 3, c = 5, eccentricity = 1.25, asymptote_slope = 0.75, f1x = -5, f1y = 0, f2x = 5, f2y = 0   (SOLVED)
```

```python
math_foundations_ai999x.solve('hyperbola_parameters', h=-1, k=2, A=9, B=16, orientation='vertical')
# -> a = 3, b = 4, c = 5, eccentricity = 1.66667, asymptote_slope = 0.75, f1x = -1, f1y = -3, f2x = -1, f2y = 7   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the foci and asymptotes of the hyperbola x^2/16 - y^2/9 = 1. | SOLVED | a = 4, b = 3, c = 5, eccentricity = 1.25, asymptote_slope = 0.75, f1x = -5, f1y = 0, f2x = 5, f2y = 0 |
| Find the foci of (y - 2)^2/9 - (x + 1)^2/16 = 1. | SOLVED | a = 3, b = 4, c = 5, eccentricity = 1.66667, asymptote_slope = 0.75, f1x = -1, f1y = -3, f2x = -1, f2y = 7 |

<a id="binomial_coefficient_term"></a>

### Coefficient of a term in a binomial expansion — `binomial_coefficient_term`

The coefficient of $x^{k}$ in $(a x + b)^{n}$, by the binomial theorem.

$$
(ax + b)^n = \sum_{k=0}^{n} \binom{n}{k} a^k b^{n-k} x^k
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of x in the binomial |  |
| `b` | constant of the binomial |  |
| `n` | the power n |  |
| `k` | the power of x whose coefficient is wanted |  |

| output | meaning |
|---|---|
| `coefficient` | coefficient of $x^{k}$ |
| `binomial` | the binomial coefficient C(n, k) |

- **Method:** binomial theorem: the $x^{k}$ term is C(n, k) $(ax)^{k}$ $b^{n-k}$
- **Assumes:** $0  \leq  n  \leq  60$; $k  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('binomial_coefficient_term', a=2, b=-1, n=5, k=3)
# -> coefficient = 80, binomial = 10   (SOLVED)
```

```python
math_foundations_ai999x.solve('binomial_coefficient_term', a=1, b=3, n=6, k=2)
# -> coefficient = 1215, binomial = 15   (SOLVED)
```

```python
math_foundations_ai999x.solve('binomial_coefficient_term', a=-2, b=3, n=4, k=0)
# -> coefficient = 81, binomial = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the coefficient of x^3 in (2x - 1)^5. | SOLVED | coefficient = 80, binomial = 10 |
| What is the coefficient of x^2 in the expansion of (x + 3)^6? | SOLVED | coefficient = 1215, binomial = 15 |
| Find the constant term in (3 - 2x)^4. | SOLVED | coefficient = 81, binomial = 1 |
| Find the coefficient of x in (x + 2)^3. | SOLVED | coefficient = 12, binomial = 3 |

<a id="complex_arithmetic"></a>

### Arithmetic with complex numbers — `complex_arithmetic`

(a + bi) + - * / (c + di) in the form $re + im i.$

$$
\frac{a+bi}{c+di} = \frac{(ac+bd) + (bc-ad)i}{c^2+d^2}
$$

| input | meaning | condition |
|---|---|---|
| `a` | real part of the first number |  |
| `b` | imaginary part of the first number |  |
| `c` | real part of the second number |  |
| `d` | imaginary part of the second number |  |
| `op` | the operation |  |

| output | meaning |
|---|---|
| `re` | real part of the result |
| `im` | imaginary part of the result |

- **Method:** $i^{2} = -1$; division multiplies numerator and denominator by the conjugate c - di
- **Assumes:** no division by 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('complex_arithmetic', a=3, b=2, c=1, d=-1, op='/')
# -> re = 0.5, im = 2.5   (SOLVED)
```

```python
math_foundations_ai999x.solve('complex_arithmetic', a=2, b=-3, c=4, d=1, op='*')
# -> re = 11, im = -10   (SOLVED)
```

```python
math_foundations_ai999x.solve('complex_arithmetic', a=2, b=-3, c=4, d=1, op='+')
# -> re = 6, im = -2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Compute (3 + 2i) / (1 - i). | SOLVED | re = 0.5, im = 2.5 |
| Simplify (2 - 3i)(4 + i). | SOLVED | re = 11, im = -10 |
| Find (2 - 3i) + (4 + i). | SOLVED | re = 6, im = -2 |
| Compute (5 + i) - (2 - 4i). | SOLVED | re = 3, im = 5 |
| Compute (1 + i) / (0 + 0i). | INVALID_INPUT | Arithmetic with complex numbers: division by 0. |

<a id="complex_modulus_argument"></a>

### Modulus, argument and polar form of a complex number — `complex_modulus_argument`

|z| and the principal argument (-180, 180] degrees of $z = a + bi$, and z in polar form.

$$
z = r(\cos\theta + i\sin\theta),\quad r = \sqrt{a^2+b^2}
$$

| input | meaning | condition |
|---|---|---|
| `a` | real part |  |
| `b` | imaginary part |  |

| output | meaning |
|---|---|
| `modulus` | \|z\| |
| `argument_deg` | principal argument, degrees in (-180, 180] |
| `argument_rad` | principal argument, radians |
| `polar` | $z = r$(cos t + i sin t) |

- **Method:** |z| = sqrt(a^2 + b^2); the argument in the quadrant of (a, b)
- **Assumes:** $z  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('complex_modulus_argument', a=-1, b=1)
# -> modulus = 1.41421, argument_deg = 135, argument_rad = 2.35619, polar = 1.414213562(cos(135 deg) + i sin(135 deg))   (SOLVED)
```

```python
math_foundations_ai999x.solve('complex_modulus_argument', a=1, b=-1)
# -> modulus = 1.41421, argument_deg = -45, argument_rad = -0.785398, polar = 1.414213562(cos(-45 deg) + i sin(-45 deg))   (SOLVED)
```

```python
math_foundations_ai999x.solve('complex_modulus_argument', a=3, b=-4)
# -> modulus = 5, argument_deg = -53.1301, argument_rad = -0.927295, polar = 5(cos(-53.13010235 deg) + i sin(-53.13010235 deg))   (SOLVED)
```

```python
math_foundations_ai999x.solve('complex_modulus_argument', a=-2, b=0)
# -> modulus = 2, argument_deg = 180, argument_rad = 3.14159, polar = 2(cos(180 deg) + i sin(180 deg))   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the modulus and argument of -1 + i. | SOLVED | modulus = 1.41421, argument_deg = 135, argument_rad = 2.35619, polar = 1.414213562(cos(135 deg) + i sin(135 deg)) |
| Write 1 - i in polar form. | SOLVED | modulus = 1.41421, argument_deg = -45, argument_rad = -0.785398, polar = 1.414213562(cos(-45 deg) + i sin(-45 deg)) |
| Find the modulus and argument of z = 3 - 4i. | SOLVED | modulus = 5, argument_deg = -53.1301, argument_rad = -0.927295, polar = 5(cos(-53.13010235 deg) + i sin(-53.13010235 deg)) |
| Find the modulus and argument of 0. | INVALID_INPUT | Modulus, argument and polar form of a complex number: 0 has modulus 0 and no argument. |

<a id="de_moivre_power"></a>

### Power of a complex number (De Moivre) — `de_moivre_power`

$(a + bi)^{n}$ for a whole number n (|n| <= 40): real and imaginary parts, modulus and argument.

$$
\left[r(\cos\theta + i\sin\theta)\right]^n = r^n(\cos n\theta + i\sin n\theta)
$$

| input | meaning | condition |
|---|---|---|
| `a` | real part |  |
| `b` | imaginary part |  |
| `n` | the power n |  |

| output | meaning |
|---|---|
| `re` | real part of $z^{n}$ |
| `im` | imaginary part of $z^{n}$ |
| `modulus` | \|$z^{n}$\| = \|z\|^n |
| `argument_deg` | argument of $z^{n}$, degrees |

- **Method:** De Moivre: $z^{n} = r^{n}$ (cos n t + i sin n t); computed exactly (binary exponentiation in rational arithmetic)
- **Assumes:** |n| <= 40; $z  \neq  0$ when $n  \leq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('de_moivre_power', a=1, b=1, n=8)
# -> re = 16, im = 0, modulus = 16, argument_deg = 0   (SOLVED)
```

```python
math_foundations_ai999x.solve('de_moivre_power', a=0, b=2, n=3)
# -> re = 0, im = -8, modulus = 8, argument_deg = -90   (SOLVED)
```

```python
math_foundations_ai999x.solve('de_moivre_power', a=1, b=-1, n=-2)
# -> re = 0, im = 0.5, modulus = 0.5, argument_deg = 90   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Use De Moivre's theorem to compute (1 + i)^8. | SOLVED | re = 16, im = 0, modulus = 16, argument_deg = 0 |
| Compute (sqrt(3) + i)^6. | SOLVED | re = -64, im = 0, modulus = 64, argument_deg = 180 |
| Find (2i)^3. | SOLVED | re = 0, im = -8, modulus = 8, argument_deg = -90 |
| Compute (0 + 0i)^-2. | INVALID_INPUT | Power of a complex number (De Moivre): 0 to a power that is not positive is undefined. |

<a id="complex_nth_roots"></a>

### The n-th roots of a complex number — `complex_nth_roots`

All n complex n-th roots of $z = a + bi$ (n from 2 to 12), ordered by argument from 0 to 360 degrees.

$$
w_k = \sqrt[n]{r}\left(\cos\frac{\theta + 2\pi k}{n} + i\sin\frac{\theta + 2\pi k}{n}\right)
$$

| input | meaning | condition |
|---|---|---|
| `a` | real part of z |  |
| `b` | imaginary part of z |  |
| `n` | which roots (2 = square roots) |  |

| output | meaning |
|---|---|
| `modulus` | the common modulus \|z\|^$(1/n)$ |
| `count` | the number of roots, n |
| `roots` | the roots, by argument in [0, 360) |

- **Method:** w_k = |z|^$(1/n)$ (cos((t + 2 pi k)/n) + i sin((t + 2 pi k)/n)), $k = 0$ .. $n-1$
- **Assumes:** $z  \neq  0$; $2  \leq  n  \leq  12$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('complex_nth_roots', a=0, b=8, n=3)
# -> modulus = 2, count = 3, roots = [(1.73205080757+1j), (-1.73205080757+1j), -2j]   (SOLVED)
```

```python
math_foundations_ai999x.solve('complex_nth_roots', a=-16, b=0, n=4)
# -> modulus = 2, count = 4, roots = [(1.41421356237+1.41421356237j), (-1.41421356237+1.41421356237j), (-1.41421356237-1.41421356237j), (1.41421356237-1.41421356237j)]   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the cube roots of 8i. | SOLVED | modulus = 2, count = 3, roots = [(1.73205080757+1j), (-1.73205080757+1j), -2j] |
| Find all the fourth roots of -16. | SOLVED | modulus = 2, count = 4, roots = [(1.41421356237+1.41421356237j), (-1.41421356237+1.41421356237j), (-1.41421356237-1.41421356237j), (1.41421356237-1.41421356237j)] |
| Find the square roots of 9. | SOLVED | modulus = 3, count = 2, roots = [(3+0j), (-3+0j)] |
| Find the cube roots of 0. | INVALID_INPUT | The n-th roots of a complex number: 0 has only the root 0. |

<a id="abs_value_inequality_lower_bound"></a>

### Solution set of |x - c| > d — `abs_value_inequality_lower_bound`

For |x - c| > d with $d  \geq  0$, the solutions are x < left_limit or x > right_limit.

$$
\text{right limit} - \text{left limit} = 2 d,\quad \text{right limit} + \text{left limit} = 2 c
$$

| input | meaning | condition |
|---|---|---|
| `c` | centre c |  |
| `d` | bound d | $d  \geq  0$ |

| output | meaning |
|---|---|
| `left_limit` | x must be below this |
| `right_limit` | or above this |

- **Method:** |x - c| > d means $x - c < -d$ or $x - c > d$
- **Assumes:** $d  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('abs_value_inequality_lower_bound', c=1.0, d=3.0)
# -> left_limit = -2, right_limit = 4   (SOLVED)
```

```python
math_foundations_ai999x.solve('abs_value_inequality_lower_bound', c=5.0, d=0.0)
# -> left_limit = 5, right_limit = 5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve \|x - 1\| > 3. | SOLVED | left_limit = -2, right_limit = 4 |
| For which x is \|x - 6\| greater than 2? | SOLVED | left_limit = 4, right_limit = 8 |

<a id="abs_value_inequality_upper_bound"></a>

### Solution interval of |x - c| < d — `abs_value_inequality_upper_bound`

For |x - c| < d with $d > 0$, the solutions are lower < x < upper.

$$
\text{upper} - \text{lower} = 2 d,\quad \text{upper} + \text{lower} = 2 c
$$

| input | meaning | condition |
|---|---|---|
| `c` | centre c |  |
| `d` | bound d | $d > 0$ |

| output | meaning |
|---|---|
| `lower` | lower end of the solution interval (excluded) |
| `upper` | upper end of the solution interval (excluded) |

- **Method:** |x - c| < d means -d < x - c < d
- **Assumes:** $d > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('abs_value_inequality_upper_bound', c=4.0, d=1.0)
# -> lower = 3, upper = 5   (SOLVED)
```

```python
math_foundations_ai999x.solve('abs_value_inequality_upper_bound', c=0.0, d=2.5)
# -> lower = -2.5, upper = 2.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve \|x - 4\| < 1. | SOLVED | lower = 3, upper = 5 |
| Solve \|x - 2\| < 0. | INVALID_INPUT | Solution interval of \|x - c\| < d: the condition d > 0 does not hold for these inputs. |

<a id="absolute_value_equation"></a>

### Solutions of an absolute value equation — `absolute_value_equation`

Solve |$a x + b$| = c for x (two solutions when $c > 0$, one when c = 0).

$$
a x_{1} + b = c,\quad a x_{2} + b = -c
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of x | $a  \neq  0$ |
| `b` | constant inside the absolute value |  |
| `c` | right-hand side | $c  \geq  0$ |

| output | meaning |
|---|---|
| `x1` | solution of $a x + b = c$ |
| `x2` | solution of $a x + b = -c$ |

- **Method:** the expression inside equals c or -c; solve each linear equation
- **Assumes:** $a  \neq  0$; $c  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('absolute_value_equation', a=2.0, b=-1.0, c=5.0)
# -> x1 = 3, x2 = -2   (SOLVED)
```

```python
math_foundations_ai999x.solve('absolute_value_equation', a=1.0, b=3.0, c=0.0)
# -> x1 = -3, x2 = -3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve \|2x - 1\| = 5. | SOLVED | x1 = 3, x2 = -2 |
| Solve \|x + 4\| = 6. | SOLVED | x1 = 2, x2 = -10 |

<a id="add_rational_expressions_same_denom"></a>

### Add rational expressions with same denominator — `add_rational_expressions_same_denom`

Add two rational expressions that share a common denominator.

$$
\text{sum num} = \text{num}_{1} + \text{num}_{2},\quad \text{sum den} = \text{den}
$$

| input | meaning | condition |
|---|---|---|
| `num1` | numerator of first expression |  |
| `num2` | numerator of second expression |  |
| `den` | common denominator |  |

| output | meaning |
|---|---|
| `sum_num` | numerator of the sum |
| `sum_den` | denominator of the sum |

- **Method:** closed form verified against the defining equations
- **Assumes:** den != 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('add_rational_expressions_same_denom', den=5.0, num1=3.0, num2=4.0)
# -> sum_num = 7, sum_den = 5   (SOLVED)
```

```python
math_foundations_ai999x.solve('add_rational_expressions_same_denom', den=4.0, num1=10.0, num2=-2.0)
# -> sum_num = 8, sum_den = 4   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Add 1/6 and 2/6. | SOLVED | sum_num = 3, sum_den = 6 |

<a id="arithmetic_sequence"></a>

### Term and partial sum of an arithmetic sequence — `arithmetic_sequence`

The n-th term and the sum of the first n terms of the arithmetic sequence with first term a1 and common difference d.

$$
a_{n} = a_{1} + \left(n - 1\right) d,\quad 2 s_{n} = n \left(a_{1} + a_{n}\right)
$$

| input | meaning | condition |
|---|---|---|
| `a1` | first term |  |
| `d` | common difference |  |
| `n` | position of the term (a positive whole number) | $n  \geq  1$ |

| output | meaning |
|---|---|
| `an` | the n-th term |
| `sn` | the sum of the first n terms |

- **Method:** the n-th term adds d to the first term $n - 1$ times; the sum pairs first and last terms
- **Assumes:** $n  \geq  1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('arithmetic_sequence', a1=3.0, d=4.0, n=10.0)
# -> an = 39, sn = 210   (SOLVED)
```

```python
math_foundations_ai999x.solve('arithmetic_sequence', a1=100.0, d=-7.0, n=5.0)
# -> an = 72, sn = 430   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the 10th term of the arithmetic sequence with first term 3 and common difference 4. | SOLVED | an = 39, sn = 210 |
| Use the arithmetic sequence with a1 = 1, d = 2, n = 2.5. | INVALID_INPUT | Term and partial sum of an arithmetic sequence: n must be a whole number. |

<a id="average_rate_of_change_definition"></a>

### Average rate of change of a function between two inputs — `average_rate_of_change_definition`

(f(x_end) - f(x_start)) / (x_end - x_start) from two values of a function.

$$
\text{arc} \left(x_{\text{end}} - x_{\text{start}}\right) = f_{x,end} - f_{x,start}
$$

| input | meaning | condition |
|---|---|---|
| `x_start` | first input |  |
| `f_x_start` | value of the function at the first input |  |
| `x_end` | second input |  |
| `f_x_end` | value of the function at the second input |  |

| output | meaning |
|---|---|
| `arc` | average rate of change |

- **Method:** change in output divided by change in input
- **Assumes:** x_end != x_start
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('average_rate_of_change_definition', x_start=0.0, f_x_start=1.0, x_end=5.0, f_x_end=11.0)
# -> arc = 2   (SOLVED)
```

```python
math_foundations_ai999x.solve('average_rate_of_change_definition', x_start=1.0, f_x_start=4.0, x_end=3.0, f_x_end=0.0)
# -> arc = -2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| If f(0) = 1 and f(5) = 11, what is the average rate of change of f? | SOLVED | arc = 2 |
| If f(2) = 7 and f(2) = 9, what is the average rate of change of f? | INVALID_INPUT | Average rate of change of a function between two inputs: the condition x_end != x_start does not hold for these inputs. |

<a id="average_rate_of_change_linear"></a>

### Average rate of change between two points — `average_rate_of_change_linear`

$(y_{2} - y_{1}) / (x_{2} - x_{1})$ between the points (x1, y1) and (x2, y2).

$$
m \left(x_{2} - x_{1}\right) = y_{2} - y_{1}
$$

| input | meaning | condition |
|---|---|---|
| `x1` | x-coordinate of the first point |  |
| `y1` | y-coordinate of the first point |  |
| `x2` | x-coordinate of the second point |  |
| `y2` | y-coordinate of the second point |  |

| output | meaning |
|---|---|
| `m` | average rate of change |

- **Method:** change in y divided by change in x
- **Assumes:** $x_{2}  \neq  x_{1}$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('average_rate_of_change_linear', x1=-2.0, y1=5.0, x2=3.0, y2=0.0)
# -> m = -1   (SOLVED)
```

```python
math_foundations_ai999x.solve('average_rate_of_change_linear', x1=0.0, y1=0.0, x2=4.0, y2=2.0)
# -> m = 0.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the average rate of change between the points (-2, 5) and (3, 0). | SOLVED | m = -1 |
| What is the average rate of change from (1, 2) to (1, 8)? | INVALID_INPUT | Average rate of change between two points: the condition x2 != x1 does not hold for these inputs. |

<a id="binomial_product"></a>

### Product of two binomials — `binomial_product`

Expand $(a x + b) (c x + d)$ into $p_{2} x^{2} + p_{1} x + p_{0}.$

$$
p_{2} t^{2} + p_{1} t + p_{0} = \left(a t + b\right) \left(c t + d\right)
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of x in the first binomial |  |
| `b` | constant of the first binomial |  |
| `c` | coefficient of x in the second binomial |  |
| `d` | constant of the second binomial |  |

| output | meaning |
|---|---|
| `p2` | coefficient of $x^{2}$ |
| `p1` | coefficient of x |
| `p0` | constant term |

- **Method:** distribute each term of the first binomial over the second (FOIL) and collect like terms
- **Assumes:** 
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('binomial_product', a=1.0, b=2.0, c=1.0, d=3.0)
# -> p2 = 1, p1 = 5, p0 = 6   (SOLVED)
```

```python
math_foundations_ai999x.solve('binomial_product', a=2.0, b=-1.0, c=3.0, d=4.0)
# -> p2 = 6, p1 = 5, p0 = -4   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Expand (x + 2)(x + 3). | SOLVED | p2 = 1, p1 = 5, p0 = 6 |
| Multiply (2x - 1) by (3x + 4). | SOLVED | p2 = 6, p1 = 5, p0 = -4 |

<a id="circle_from_general_form"></a>

### Centre and radius of a circle from its general equation — `circle_from_general_form`

For $x^{2} + y^{2} + D x + E y + F = 0$, the centre (h, k) and radius rad.

$$
\left(t - h\right)^{2} + \left(w - k\right)^{2} - \text{rad}^{2} = t^{2} + w^{2} + d_{d} t + e_{e} w + f_{f}
$$

| input | meaning | condition |
|---|---|---|
| `dd` | coefficient D of x |  |
| `ee` | coefficient E of y |  |
| `ff` | constant F |  |

| output | meaning |
|---|---|
| `h` | x-coordinate of the centre |
| `k` | y-coordinate of the centre |
| `rad` | radius |

- **Method:** complete the square in x and in y
- **Assumes:** dd**$2/4 + ee$**$2/4 - ff > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('circle_from_general_form', dd=-4.0, ee=6.0, ff=-3.0)
# -> h = 2, k = -3, rad = 4   (SOLVED)
```

```python
math_foundations_ai999x.solve('circle_from_general_form', dd=0.0, ee=0.0, ff=-25.0)
# -> h = 0, k = 0, rad = 5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the centre and radius of x^2 + y^2 - 4x + 6y - 3 = 0. | SOLVED | h = 2, k = -3, rad = 4 |
| Find the centre and radius of x^2 + y^2 + 2x + 2y + 5 = 0. | INVALID_INPUT | Centre and radius of a circle from its general equation: the condition dd**2/4 + ee**2/4 - ff > 0 does not hold for thes |

<a id="complex_product_modulus"></a>

### Product and modulus of complex numbers — `complex_product_modulus`

The product $(a + b i)$$(c + d i) = re + im i$ and the modulus of the product.

$$
\operatorname{Re} = a c - b d,\quad \operatorname{Im} = a d + b c,\quad \text{modulus}^{2} = \left(a^{2} + b^{2}\right) \left(c^{2} + d^{2}\right)
$$

| input | meaning | condition |
|---|---|---|
| `a` | real part of the first number |  |
| `b` | imaginary part of the first number |  |
| `c` | real part of the second number |  |
| `d` | imaginary part of the second number |  |

| output | meaning |
|---|---|
| `re` | real part of the product |
| `im` | imaginary part of the product |
| `modulus` | modulus of the product |

- **Method:** distribute and use $i^{2} = -1$; the modulus of a product is the product of the moduli
- **Assumes:** 
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('complex_product_modulus', a=1.0, b=2.0, c=3.0, d=4.0)
# -> re = -5, im = 10, modulus = 11.1803   (SOLVED)
```

```python
math_foundations_ai999x.solve('complex_product_modulus', a=2.0, b=-1.0, c=2.0, d=1.0)
# -> re = 5, im = 0, modulus = 5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Multiply (1 + 2i)(3 + 4i). | SOLVED | re = -5, im = 10, modulus = 11.1803 |

<a id="compose_linear_functions"></a>

### Composition of two linear functions — `compose_linear_functions`

For f(x) = a*x + b and g(x) = c*x + d, the composition f(g(x)) = $m x + k.$

$$
m t + k = a \left(c t + d\right) + b
$$

| input | meaning | condition |
|---|---|---|
| `a` | slope of f |  |
| `b` | intercept of f |  |
| `c` | slope of g |  |
| `d` | intercept of g |  |

| output | meaning |
|---|---|
| `m` | slope of f(g(x)) |
| `k` | intercept of f(g(x)) |

- **Method:** substitute g(x) for x in f
- **Assumes:** 
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('compose_linear_functions', a=2.0, b=1.0, c=3.0, d=-4.0)
# -> m = 6, k = -7   (SOLVED)
```

```python
math_foundations_ai999x.solve('compose_linear_functions', a=-1.0, b=5.0, c=2.0, d=3.0)
# -> m = -2, k = 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| If f(x) = 2x + 1 and g(x) = 3x - 4, find f(g(x)). | SOLVED | m = 6, k = -7 |

<a id="compound_interest"></a>

### Compound interest — `compound_interest`

The amount after t years when a principal is compounded n times a year at an annual rate r (a decimal).

$$
\ln\left(\text{amount}\right) = \ln\left(\text{principal}\right) + n t \ln\left(1 + \frac{r}{n}\right)
$$

| input | meaning | condition |
|---|---|---|
| `principal` | starting amount | principal > 0 |
| `r` | annual interest rate as a decimal | $r > -1$ |
| `n` | compounding periods per year | $n  \geq  1$ |
| `t` | time in years | $t  \geq  0$ |

| output | meaning |
|---|---|
| `amount` | amount after t years |

- **Method:** each period multiplies the amount by $(1 + r/n)$, $n t$ times
- **Assumes:** principal > 0; $n  \geq  1$; $t  \geq  0$; $1 + r/n > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('compound_interest', principal=1000.0, r=0.05, n=1.0, t=2.0)
# -> amount = 1102.5   (SOLVED)
```

```python
math_foundations_ai999x.solve('compound_interest', principal=500.0, r=0.12, n=12.0, t=1.0)
# -> amount = 563.413   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the amount after 2 years if 1000 is invested at rate 0.05 compounded 1 times a year. | SOLVED | amount = 1102.5 |

<a id="convert_to_slope_intercept"></a>

### y-intercept of a line from its slope and a point — `convert_to_slope_intercept`

The constant b of $y = m x + b$ for the line with slope m through (x1, y1).

$$
y_{1} = m x_{1} + b
$$

| input | meaning | condition |
|---|---|---|
| `m` | slope |  |
| `x1` | x-coordinate of the point |  |
| `y1` | y-coordinate of the point |  |

| output | meaning |
|---|---|
| `b` | y-intercept |

- **Method:** substitute the point into $y = m x + b$ and solve for b
- **Assumes:** 
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('convert_to_slope_intercept', m=4.0, x1=3.0, y1=10.0)
# -> b = -2   (SOLVED)
```

```python
math_foundations_ai999x.solve('convert_to_slope_intercept', m=0.0, x1=5.0, y1=7.0)
# -> b = 7   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A line with slope 4 passes through (3, 10). What is its y-intercept? | SOLVED | b = -2 |
| Find the y-intercept of the line with slope 0.5 through the point (2, 4). | SOLVED | b = 3 |

<a id="determinant_2x2_formula"></a>

### Determinant of 2x2 matrix — `determinant_2x2_formula`

Computes the determinant of a 2x2 matrix using the standard formula.

$$
\text{det} = a d - b c
$$

| input | meaning | condition |
|---|---|---|
| `a` | entry in row 1 column 1 |  |
| `b` | entry in row 1 column 2 |  |
| `c` | entry in row 2 column 1 |  |
| `d` | entry in row 2 column 2 |  |

| output | meaning |
|---|---|
| `det` | determinant of the matrix |

- **Method:** closed form verified against the defining equations
- **Assumes:** 
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('determinant_2x2_formula', a=2.0, b=3.0, c=1.0, d=4.0)
# -> det = 5   (SOLVED)
```

```python
math_foundations_ai999x.solve('determinant_2x2_formula', a=5.0, b=2.0, c=4.0, d=8.0)
# -> det = 32   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the determinant of the matrix [[3, 1], [2, 4]]. | SOLVED | det = 10 |

<a id="distance_between_points"></a>

### Distance between two points — `distance_between_points`

The distance between (x1, y1) and (x2, y2) in the plane.

$$
\text{dist}^{2} = \left(x_{2} - x_{1}\right)^{2} + \left(y_{2} - y_{1}\right)^{2}
$$

| input | meaning | condition |
|---|---|---|
| `x1` | x-coordinate of the first point |  |
| `y1` | y-coordinate of the first point |  |
| `x2` | x-coordinate of the second point |  |
| `y2` | y-coordinate of the second point |  |

| output | meaning |
|---|---|
| `dist` | distance |

- **Method:** the Pythagorean theorem on the horizontal and vertical differences
- **Assumes:** 
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('distance_between_points', x1=0.0, y1=0.0, x2=3.0, y2=4.0)
# -> dist = 5   (SOLVED)
```

```python
math_foundations_ai999x.solve('distance_between_points', x1=-1.0, y1=2.0, x2=5.0, y2=10.0)
# -> dist = 10   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the distance between (1, 1) and (4, 5). | SOLVED | dist = 5 |

<a id="doubling_time"></a>

### Doubling time of exponential growth — `doubling_time`

The time for a quantity growing as $N = N_{0} e^{k t}$ to double, for a growth constant $k > 0.$

$$
k t_{d} = \ln\left(2\right)
$$

| input | meaning | condition |
|---|---|---|
| `k` | continuous growth constant per unit time | $k > 0$ |

| output | meaning |
|---|---|
| `td` | doubling time |

- **Method:** solve $e^{k t} = 2$ for t
- **Assumes:** $k > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('doubling_time', k=0.5)
# -> td = 1.38629   (SOLVED)
```

```python
math_foundations_ai999x.solve('doubling_time', k=0.1)
# -> td = 6.93147   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the doubling time for a growth constant of 0.1? | SOLVED | td = 6.93147 |

<a id="exponential_equation"></a>

### Solution of an exponential equation — `exponential_equation`

Solve $b^{x} = y$ for x, with $b > 0$, $b  \neq  1$ and $y > 0.$

$$
x \ln\left(b\right) = \ln\left(y\right)
$$

| input | meaning | condition |
|---|---|---|
| `b` | the base | $b > 0$ |
| `y` | the value of the power | $y > 0$ |

| output | meaning |
|---|---|
| `x` | the exponent |

- **Method:** take logarithms of both sides: $x = \frac{\log{\left(y \right)}}{\log{\left(b \right)}}$
- **Assumes:** $b > 0$; $y > 0$; $b  \neq  1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('exponential_equation', b=2.0, y=8.0)
# -> x = 3   (SOLVED)
```

```python
math_foundations_ai999x.solve('exponential_equation', b=10.0, y=0.001)
# -> x = -3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve 2^x = 32. | SOLVED | x = 5 |
| Solve 1^x = 5. | INVALID_INPUT | Solution of an exponential equation: the condition b != 1 does not hold for these inputs. |

<a id="factor_monic_quadratic"></a>

### Factoring a monic quadratic — `factor_monic_quadratic`

Write $x^{2} + b x + c$ as $(x + p) (x + q)$ with real p and q.

$$
\left(t + p\right) \left(t + q\right) = t^{2} + b t + c
$$

| input | meaning | condition |
|---|---|---|
| `b` | coefficient of x |  |
| `c` | constant term |  |

| output | meaning |
|---|---|
| `p` | first constant |
| `q` | second constant |

- **Method:** find p and q with $p + q = b$ and $p q = c$ (the roots of the quadratic are -p and -q)
- **Assumes:** b**$2 - 4 c  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('factor_monic_quadratic', b=5.0, c=6.0)
# -> p = 3, q = 2   (SOLVED)
```

```python
math_foundations_ai999x.solve('factor_monic_quadratic', b=-1.0, c=-12.0)
# -> p = 3, q = -4   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Factor x^2 + 5x + 6. | SOLVED | p = 3, q = 2 |
| Factor x^2 - x - 12. | SOLVED | p = 3, q = -4 |
| Factor x^2 + 2x + 5. | INVALID_INPUT | Factoring a monic quadratic: the condition b**2 - 4*c >= 0 does not hold for these inputs. |

<a id="geometric_sequence"></a>

### Term and partial sum of a geometric sequence — `geometric_sequence`

The n-th term and the sum of the first n terms of the geometric sequence with first term g1 and common ratio q $(q  \neq  1)$.

$$
g_{n} = g_{1} q^{n - 1},\quad s_{n} \left(1 - q\right) = g_{1} \left(1 - q^{n}\right)
$$

| input | meaning | condition |
|---|---|---|
| `g1` | first term |  |
| `q` | common ratio | $q  \neq  1$ |
| `n` | position of the term | $n  \geq  1$ |

| output | meaning |
|---|---|
| `gn` | the n-th term |
| `sn` | the sum of the first n terms |

- **Method:** each term multiplies the previous one by q; the sum telescopes when multiplied by $(1 - q)$
- **Assumes:** $q  \neq  1$; $n  \geq  1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('geometric_sequence', g1=3.0, q=2.0, n=5.0)
# -> gn = 48, sn = 93   (SOLVED)
```

```python
math_foundations_ai999x.solve('geometric_sequence', g1=81.0, q=-0.3333333333333333, n=4.0)
# -> gn = -3, sn = 60   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find term 4 and the sum of the first 4 terms of the geometric sequence with first term 2 and ratio 3. | SOLVED | gn = 54, sn = 80 |

<a id="line_through_two_points"></a>

### Line through two points — `line_through_two_points`

The slope m and y-intercept k of the line through (x1, y1) and (x2, y2), in the form $y = m x + k.$

$$
m x_{1} + k = y_{1},\quad m x_{2} + k = y_{2}
$$

| input | meaning | condition |
|---|---|---|
| `x1` | x-coordinate of the first point |  |
| `y1` | y-coordinate of the first point |  |
| `x2` | x-coordinate of the second point |  |
| `y2` | y-coordinate of the second point |  |

| output | meaning |
|---|---|
| `m` | slope |
| `k` | y-intercept |

- **Method:** slope = rise over run; the intercept follows from one point
- **Assumes:** $x_{2} - x_{1}  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('line_through_two_points', x1=1.0, y1=3.0, x2=3.0, y2=7.0)
# -> m = 2, k = 1   (SOLVED)
```

```python
math_foundations_ai999x.solve('line_through_two_points', x1=-2.0, y1=5.0, x2=2.0, y2=-3.0)
# -> m = -2, k = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the equation of the line through (0, 1) and (2, 5). | SOLVED | m = 2, k = 1 |
| Find the equation of the line through (3, 1) and (3, 4). | INVALID_INPUT | Line through two points: the condition x2 - x1 != 0 does not hold for these inputs. |

<a id="linear_equation_root"></a>

### Root of a linear equation — `linear_equation_root`

Solve $a x + b = c$ for x.

$$
a x + b = c
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of x | $a  \neq  0$ |
| `b` | constant term on the left side |  |
| `c` | right-hand side |  |

| output | meaning |
|---|---|
| `x` | the solution |

- **Method:** subtract b from both sides, then divide by a
- **Assumes:** $a  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('linear_equation_root', a=2.0, b=-4.0, c=6.0)
# -> x = 5   (SOLVED)
```

```python
math_foundations_ai999x.solve('linear_equation_root', a=-3.0, b=1.0, c=1.0)
# -> x = 0   (SOLVED)
```

```python
math_foundations_ai999x.solve('linear_equation_root', a=0.5, b=0.0, c=3.0)
# -> x = 6   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve 3x + 6 = 0. | SOLVED | x = -2 |
| Solve x - 4 = 10. | SOLVED | x = 14 |
| Find x if 5x + 1 = 2. | SOLVED | x = 0.2 |
| Solve the linear equation with a = 0, b = 1, c = 2. | INVALID_INPUT | Root of a linear equation: the condition a != 0 does not hold for these inputs. |
| Solve the linear equation with a = 4, b = 1. | NEEDS_INPUT | linear_equation_root needs c (right-hand side). Example: solve('linear_equation_root', a=2, b=-4, c=6). |

<a id="linear_inequality"></a>

### Solution of a linear inequality — `linear_inequality`

Solve $a x + b < c$ (or >): the boundary x0 and the direction s (s = 1 means the solution of the < form is $x < x_{0}$; $s = -1$ means it reverses to $x > x_{0}$ because a < 0).

$$
a x_{0} + b = c,\quad s \sqrt{a^{2}} = a
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of x | $a  \neq  0$ |
| `b` | constant on the left |  |
| `c` | right-hand side |  |

| output | meaning |
|---|---|
| `x0` | boundary of the solution set |
| `s` | 1 if the inequality keeps its direction, -1 if it reverses |

- **Method:** subtract b, divide by a; dividing by a negative number reverses the inequality
- **Assumes:** $a  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('linear_inequality', a=2.0, b=1.0, c=7.0)
# -> x0 = 3, s = 1   (SOLVED)
```

```python
math_foundations_ai999x.solve('linear_inequality', a=-3.0, b=4.0, c=10.0)
# -> x0 = -2, s = -1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve the inequality 2x + 1 < 7. | SOLVED | x0 = 3, s = 1 |
| Solve the inequality -3x + 4 < 10. | SOLVED | x0 = -2, s = -1 |

<a id="linear_system_2x2"></a>

### Solution of two linear equations in two unknowns — `linear_system_2x2`

Solve $a x + b y = c$ and $d x + e y = f$ for x and y.

$$
a x + b y = c,\quad d x + e y = f
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of x in the first equation |  |
| `b` | coefficient of y in the first equation |  |
| `c` | right-hand side of the first equation |  |
| `d` | coefficient of x in the second equation |  |
| `e` | coefficient of y in the second equation |  |
| `f` | right-hand side of the second equation |  |

| output | meaning |
|---|---|
| `x` | the value of x |
| `y` | the value of y |

- **Method:** elimination (Cramer's rule); the determinant $a e - b d$ must not be zero
- **Assumes:** $a e - b d  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('linear_system_2x2', a=1.0, b=1.0, c=10.0, d=1.0, e=-1.0, f=2.0)
# -> x = 6, y = 4   (SOLVED)
```

```python
math_foundations_ai999x.solve('linear_system_2x2', a=2.0, b=3.0, c=12.0, d=4.0, e=-1.0, f=10.0)
# -> x = 3, y = 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve the system x + y = 10, x - y = 2. | SOLVED | x = 6, y = 4 |
| Solve the system 2x + 3y = 12, 4x - y = 10. | SOLVED | x = 3, y = 2 |
| Solve the system x + 2y = 3, 2x + 4y = 6. | INVALID_INPUT | Solution of two linear equations in two unknowns: the condition a*e - b*d != 0 does not hold for these inputs. |

<a id="logarithmic_equation"></a>

### Solution of a basic logarithmic equation — `logarithmic_equation`

Solve log_b(x) = c for x.

$$
\ln\left(x\right) = c \ln\left(b\right)
$$

| input | meaning | condition |
|---|---|---|
| `b` | base | $b > 0$ |
| `c` | value of the logarithm |  |

| output | meaning |
|---|---|
| `x` | the solution |

- **Method:** rewrite in exponential form: $x = b^{c}$
- **Assumes:** $b > 0$; $b  \neq  1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('logarithmic_equation', b=2.0, c=5.0)
# -> x = 32   (SOLVED)
```

```python
math_foundations_ai999x.solve('logarithmic_equation', b=10.0, c=-2.0)
# -> x = 0.01   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solve log base 3 of x = 4. | SOLVED | x = 81 |
| Solve log base 1 of x = 2. | INVALID_INPUT | Solution of a basic logarithmic equation: the condition b != 1 does not hold for these inputs. |

<a id="matrix_2x2_inverse"></a>

### Determinant and inverse of a 2 by 2 matrix — `matrix_2x2_inverse`

For the matrix [[a, b], [c, d]], its determinant and the entries of its inverse.

$$
\text{det} = a d - b c,\quad a i_{11} + b i_{21} = 1,\quad a i_{12} + b i_{22} = 0,\quad c i_{11} + d i_{21} = 0,\quad c i_{12} + d i_{22} = 1
$$

| input | meaning | condition |
|---|---|---|
| `a` | top-left entry |  |
| `b` | top-right entry |  |
| `c` | bottom-left entry |  |
| `d` | bottom-right entry |  |

| output | meaning |
|---|---|
| `det` | determinant |
| `i11` | inverse, top-left |
| `i12` | inverse, top-right |
| `i21` | inverse, bottom-left |
| `i22` | inverse, bottom-right |

- **Method:** swap the diagonal, negate the off-diagonal, divide by the determinant
- **Assumes:** $a d - b c  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('matrix_2x2_inverse', a=2.0, b=1.0, c=5.0, d=3.0)
# -> det = 1, i11 = 3, i12 = -1, i21 = -5, i22 = 2   (SOLVED)
```

```python
math_foundations_ai999x.solve('matrix_2x2_inverse', a=4.0, b=7.0, c=2.0, d=6.0)
# -> det = 10, i11 = 0.6, i12 = -0.7, i21 = -0.2, i22 = 0.4   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the inverse of the matrix [[2, 1], [5, 3]]. | SOLVED | det = 1, i11 = 3, i12 = -1, i21 = -5, i22 = 2 |
| Find the inverse of the matrix [[1, 2], [2, 4]]. | INVALID_INPUT | Determinant and inverse of a 2 by 2 matrix: the condition a*d - b*c != 0 does not hold for these inputs. |

<a id="matrix_scalar_multiple_definition"></a>

### Matrix scalar multiple definition — `matrix_scalar_multiple_definition`

Computes the entries of a 2x2 matrix scaled by a scalar.

$$
b_{11} = s a_{11},\quad b_{12} = s a_{12},\quad b_{21} = s a_{21},\quad b_{22} = s a_{22}
$$

| input | meaning | condition |
|---|---|---|
| `s` | scalar multiplier |  |
| `a11` | entry in row 1 column 1 |  |
| `a12` | entry in row 1 column 2 |  |
| `a21` | entry in row 2 column 1 |  |
| `a22` | entry in row 2 column 2 |  |

| output | meaning |
|---|---|
| `b11` | entry in row 1 column 1 of result |
| `b12` | entry in row 1 column 2 of result |
| `b21` | entry in row 2 column 1 of result |
| `b22` | entry in row 2 column 2 of result |

- **Method:** closed form verified against the defining equations
- **Assumes:** 
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('matrix_scalar_multiple_definition', a11=1.0, a12=3.0, a21=-4.0, a22=5.0, s=2.0)
# -> b11 = 2, b12 = 6, b21 = -8, b22 = 10   (SOLVED)
```

```python
math_foundations_ai999x.solve('matrix_scalar_multiple_definition', a11=0.0, a12=7.0, a21=2.0, a22=9.0, s=-1.0)
# -> b11 = 0, b12 = -7, b21 = -2, b22 = -9   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Calculate 3 times the matrix [[1, 2], [3, 4]]. | SOLVED | b11 = 3, b12 = 6, b21 = 9, b22 = 12 |

<a id="midpoint_of_segment"></a>

### Midpoint of a line segment — `midpoint_of_segment`

The midpoint (mx, my) of the segment from (x1, y1) to (x2, y2).

$$
2 m_{x} = x_{1} + x_{2},\quad 2 m_{y} = y_{1} + y_{2}
$$

| input | meaning | condition |
|---|---|---|
| `x1` | x-coordinate of the first endpoint |  |
| `y1` | y-coordinate of the first endpoint |  |
| `x2` | x-coordinate of the second endpoint |  |
| `y2` | y-coordinate of the second endpoint |  |

| output | meaning |
|---|---|
| `mx` | x-coordinate of the midpoint |
| `my` | y-coordinate of the midpoint |

- **Method:** average the x-coordinates and the y-coordinates
- **Assumes:** 
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('midpoint_of_segment', x1=-4.0, y1=2.0, x2=0.0, y2=6.0)
# -> mx = -2, my = 4   (SOLVED)
```

```python
math_foundations_ai999x.solve('midpoint_of_segment', x1=1.0, y1=1.0, x2=5.0, y2=3.0)
# -> mx = 3, my = 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the midpoint of the segment from (-4, 2) to (0, 6). | SOLVED | mx = -2, my = 4 |
| What is the midpoint of the line segment joining (1, 1) and (5, 3)? | SOLVED | mx = 3, my = 2 |

<a id="multiply_rational_expressions"></a>

### Multiply rational expressions — `multiply_rational_expressions`

Compute the product of two rational expressions.

$$
\text{prod num} = \text{num}_{1} \text{num}_{2},\quad \text{prod den} = \text{den}_{1} \text{den}_{2}
$$

| input | meaning | condition |
|---|---|---|
| `num1` | numerator of first expression |  |
| `den1` | denominator of first expression |  |
| `num2` | numerator of second expression |  |
| `den2` | denominator of second expression |  |

| output | meaning |
|---|---|
| `prod_num` | numerator of the product |
| `prod_den` | denominator of the product |

- **Method:** closed form verified against the defining equations
- **Assumes:** den1 != 0; den2 != 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('multiply_rational_expressions', den1=3.0, den2=5.0, num1=2.0, num2=4.0)
# -> prod_num = 8, prod_den = 15   (SOLVED)
```

```python
math_foundations_ai999x.solve('multiply_rational_expressions', den1=2.0, den2=4.0, num1=1.0, num2=3.0)
# -> prod_num = 3, prod_den = 8   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Multiply 2/3 by 5/7. | SOLVED | prod_num = 10, prod_den = 21 |

<a id="perimeter_of_rectangle"></a>

### Perimeter of a rectangle — `perimeter_of_rectangle`

The perimeter p of a rectangle with width w and height h.

$$
p - 2 h = 2 w
$$

| input | meaning | condition |
|---|---|---|
| `w` | width | $w  \geq  0$ |
| `h` | height | $h  \geq  0$ |

| output | meaning |
|---|---|
| `p` | perimeter |

- **Method:** add the four sides
- **Assumes:** $w  \geq  0$; $h  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('perimeter_of_rectangle', w=6.0, h=2.0)
# -> p = 16   (SOLVED)
```

```python
math_foundations_ai999x.solve('perimeter_of_rectangle', w=2.5, h=4.0)
# -> p = 13   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the perimeter of a rectangle with width 6 and height 2? | SOLVED | p = 16 |
| Find the perimeter of a rectangle of width 0 and height 3. | SOLVED | p = 6 |

<a id="perpendicular_lines_slope_product"></a>

### Slope of a perpendicular line — `perpendicular_lines_slope_product`

The slope m_perp of a line perpendicular to a line of nonzero slope m.

$$
m m_{\text{perp}} = -1
$$

| input | meaning | condition |
|---|---|---|
| `m` | slope of the given line | $m  \neq  0$ |

| output | meaning |
|---|---|
| `m_perp` | slope of the perpendicular line |

- **Method:** the product of the slopes of perpendicular lines $is -1$
- **Assumes:** $m  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('perpendicular_lines_slope_product', m=5.0)
# -> m_perp = -0.2   (SOLVED)
```

```python
math_foundations_ai999x.solve('perpendicular_lines_slope_product', m=-0.5)
# -> m_perp = 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A line has slope 5. What is the slope of a line perpendicular to it? | SOLVED | m_perp = -0.2 |
| A line has slope 0. What is the slope of a line perpendicular to it? | INVALID_INPUT | Slope of a perpendicular line: the condition m != 0 does not hold for these inputs. |

<a id="rational_function_asymptotes"></a>

### Asymptotes of a linear rational function — `rational_function_asymptotes`

The vertical asymptote $x = va$ and horizontal asymptote $y = ha$ of f(x) = (a*x + b)/(c*x + d).

$$
c v_{a} + d = 0,\quad c h_{a} = a
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of x in the numerator |  |
| `b` | constant of the numerator |  |
| `c` | coefficient of x in the denominator | $c  \neq  0$ |
| `d` | constant of the denominator |  |

| output | meaning |
|---|---|
| `va` | vertical asymptote |
| `ha` | horizontal asymptote |

- **Method:** the denominator vanishes at $x = -d/c$; for large x the ratio tends to a/c (if $a d = b c$ the function is constant and has no vertical asymptote)
- **Assumes:** $c  \neq  0$; $a d - b c  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('rational_function_asymptotes', a=2.0, b=1.0, c=1.0, d=-3.0)
# -> va = 3, ha = 2   (SOLVED)
```

```python
math_foundations_ai999x.solve('rational_function_asymptotes', a=1.0, b=0.0, c=2.0, d=4.0)
# -> va = -2, ha = 0.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the asymptotes of f(x) = (2x + 1)/(x - 3). | SOLVED | va = 3, ha = 2 |
| Find the asymptotes of f(x) = (2x + 4)/(x + 2). | INVALID_INPUT | Asymptotes of a linear rational function: the condition a*d - b*c != 0 does not hold for these inputs. |

<a id="vertex_form"></a>

### Vertex form of a quadratic function — `vertex_form`

Write $a x^{2} + b x + c$ as $a (x - h)^{2} + k$, giving the vertex (h, k).

$$
a \left(t - h\right)^{2} + k = a t^{2} + b t + c
$$

| input | meaning | condition |
|---|---|---|
| `a` | coefficient of $x^{2}$ | $a  \neq  0$ |
| `b` | coefficient of x |  |
| `c` | constant term |  |

| output | meaning |
|---|---|
| `h` | x-coordinate of the vertex |
| `k` | y-coordinate of the vertex |

- **Method:** completing the square
- **Assumes:** $a  \neq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
math_foundations_ai999x.solve('vertex_form', a=1.0, b=-6.0, c=5.0)
# -> h = 3, k = -4   (SOLVED)
```

```python
math_foundations_ai999x.solve('vertex_form', a=2.0, b=8.0, c=1.0)
# -> h = -2, k = -7   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the vertex of y = x^2 - 6x + 5. | SOLVED | h = 3, k = -4 |
| Write 2x^2 + 8x + 1 in vertex form. | SOLVED | h = -2, k = -7 |

## Proofs and derivations

Each derivation below was certified step by step: every step follows mechanically from the ones before and from what it rests on (a certified relation, an identity, a definition or a stated hypothesis), and a wrong variant of the result is refused. Ask in one of the documented forms, or call `math_foundations_ai999x.prove("<id>")`. A statement no certified derivation proves is OUT_OF_SCOPE; a conditional result asked without its hypotheses is NEEDS_INPUT and names them.

| documented question (executed) | id | steps |
|---|---|---|
| Prove that if A = l*w then A = l*w. | `area_of_rectangle` | 1 |
| Prove that (a + b) + c = a + (b + c). | `associative_property_of_addition` | 1 |
| Prove that if a*x + b*y = e and c*x + d*y = f then x = (e*d - b*f) / (a*d - b*c). | `cramer_rule_x_solution` | 8 |
| Prove that if a1*x + b1*y = c1 and a2*x + b2*y = c2 then x*(a1*b2 - a2*b1) = c1*b2 - c2*b1. | `determinant_cramer_two_variables` | 7 |
| Prove that a**3 - b**3 = (a - b)*(a**2 + a*b + b**2). | `difference_of_cubes_factorization` | 2 |
| Prove that (a + b) * (a - b) = a**2 - b**2. | `difference_of_squares` | 6 |
| Prove that a*(b + c) = a*b + a*c. | `distributive_property_verification` | 1 |
| Prove that if a*x + b*y = c and d*x + e*y = f then (a - k*d)*x + (b - k*e)*y = c - k*f. | `elimination_method_identity` | 4 |
| Prove that if a1*x + b1*y = c1 and a2*x + b2*y = c2 then x = (c1*b2 - c2*b1)/(a1*b2 - a2*b1). | `elimination_solution_two_variables` | 6 |
| Prove that if a*x + b = c then x = (c - b)/a. | `linear_equation_solution_formula` | 2 |
| Prove that if ((a*r1 + b)/(c*(r1 - r2)))/(x - r1) + ((a*r2 + b)/(c*(r2 - r1)))/(x - r2) = RHS_combined then (a*x + b)/(c*(x - r1)*(x - r2)) = ((a*r1 + b)/(c*(r1 - r2)))/(x - r1) + ((a*r2 + b)/(c*(r2 - r1)))/(x - r2). | `partial_fraction_decomposition_linear_factors` | 8 |
| Prove that (a + b)**2 = a**2 + 2*a*b + b**2. | `perfect_square_trinomial` | 6 |
| Prove that (a + b)*(c + d) = a*c + a*d + b*c + b*d. | `product_of_sums` | 2 |
| Prove that if h = -b/(2*a) and k = c - b**2/(4*a) then a*x**2 + b*x + c = a*(x - (-b/(2*a)))**2 + (c - b**2/(4*a)). | `quadratic_vertex_form_conversion` | 4 |
| Prove that if sqrt(x) = c then x = c**2. | `radical_equation_squaring` | 2 |
| Prove that (a - b)**2 = a**2 - 2*a*b + b**2. | `square_of_difference` | 4 |
| Prove that if a*x + b*y = c and x = d then a*d + b*y = c. | `substitution_method_identity` | 2 |
| Prove that if a1*x + b1*y = c1 and a2*x + b2*y = c2 then a1*((c1 - b1*((a1*c2 - a2*c1)/(a1*b2 - a2*b1)))/a1) + b1*((a1*c2 - a2*c1)/(a1*b2 - a2*b1)) = c1. | `substitution_solution_two_variables` | 7 |
| Prove that a**3 + b**3 = (a + b)*(a**2 - a*b + b**2). | `sum_of_cubes_factorization` | 5 |
