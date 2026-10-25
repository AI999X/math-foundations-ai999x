"""math_foundations checks run by tools/clean_install.py inside a fresh environment (written by tools/new_domain.py from the certified procedures' documented questions)."""

SAMPLE = ('Solve |2x + 3| < 7.', -5.0)
LATENCY = ["Solve |2x + 3| < 7.", "Solve |x - 1| > 3.", "Solve |x - 4| < 1."]
QUESTIONS = [['Solve |2x + 3| < 7.', {'lower': -5.0, 'upper': 2.0}], ['Solve |x - 1| > 3.', {'left_limit': -2.0, 'right_limit': 4.0}], ['Solve |x - 4| < 1.', {'lower': 3.0, 'upper': 5.0}], ['Solve |2x - 1| = 5.', {'x1': 3.0, 'x2': -2.0}], ['Add 1/6 and 2/6.', {'sum_den': 6.0, 'sum_num': 3.0}], ['Find the 10th term of the arithmetic sequence with first term 3 and common difference 4.', {'an': 39.0, 'sn': 210.0}], ['If f(0) = 1 and f(5) = 11, what is the average rate of change of f?', {'arc': 2.0}], ['Find the average rate of change between the points (-2, 5) and (3, 0).', {'m': -1.0}], ['Find the coefficient of x^3 in (2x - 1)^5.', {'coefficient': 80.0}], ['Expand (x + 2)(x + 3).', {'p2': 1.0, 'p1': 5.0, 'p0': 6.0}], ['Find the centre and radius of x^2 + y^2 - 4x + 6y - 3 = 0.', {'h': 2.0, 'k': -3.0, 'rad': 4.0}], ['Compute (3 + 2i) / (1 - i).', {'re': 0.5, 'im': 2.5}], ['Find the modulus and argument of -1 + i.', {'modulus': 1.4142135623730951, 'argument_deg': 135.0}], ['Find the cube roots of 8i.', {'modulus': 2.0, 'count': 3.0}], ['Multiply (1 + 2i)(3 + 4i).', {'re': -5.0, 'im': 10.0}], ['If f(x) = 2x + 1 and g(x) = 3x - 4, find f(g(x)).', {'m': 6.0, 'k': -7.0}], ['If f(x) = x^2 + 1 and g(x) = 2x - 3, find f(g(2)).', {'value': 2.0}], ['Find the amount after 2 years if 1000 is invested at rate 0.05 compounded 1 times a year.', {'amount': 1102.5}], ['A population of 500 grows continuously at 3% per year. Find the population after 10 years.', {'amount': 674.9294037880015}], ['A line with slope 4 passes through (3, 10). What is its y-intercept?', {'b': -2.0}], ['How many ways are there to choose 3 books from 10?', {'combinations': 120.0}], ['Solve x^3 - 6x^2 + 11x - 6 = 0.', {'x1': 1.0, 'x2': 2.0, 'x3': 3.0}], ["Use De Moivre's theorem to compute (1 + i)^8.", {'re': 16.0, 'im': 0.0}], ['Convert 135 degrees to radians.', {'radians': 2.356194490192345}], ['Find the determinant of the matrix [[3, 1], [2, 4]].', {'det': 10.0}], ['Find the distance between (1, 1) and (4, 5).', {'dist': 5.0}], ['Find the distance from the point (1, 2) to the line 3x + 4y - 5 = 0.', {'distance': 1.2}], ['Find the domain of f(x) = sqrt(2x - 4).', {'boundary': 2.0}], ['What is the doubling time for a growth constant of 0.1?', {'td': 6.931471806}], ['How long does it take an investment to double at 7% interest compounded annually?', {'doubling_time': 10.244768351058712}], ['Find the foci and eccentricity of the ellipse x^2/25 + y^2/9 = 1.', {'c': 4.0, 'eccentricity': 0.8}], ['Evaluate f(3) for f(x) = 2x^2 - 3x + 1.', {'value': 10.0}], ['Solve 2^x = 32.', {'x': 5.0}], ['Solve 3 * 2^(x + 1) = 48.', {'x': 3.0}], ['Factor x^2 + 5x + 6.', {'p': 3.0, 'q': 2.0}], ['Factor 6x^2 + 5x - 6 over the integers.', {'g': 1.0, 'p': 2.0, 'q': 3.0, 'r': 3.0, 's': -2.0}], ['Compute 3/4 + 5/6 as a fraction in lowest terms.', {'numerator': 19.0, 'denominator': 12.0}], ['Find term 4 and the sum of the first 4 terms of the geometric sequence with first term 2 and ratio 3.', {'gn': 54.0, 'sn': 80.0}], ['The point (2, 5) is on the graph of y = f(x). Find the corresponding point on the graph of y = 3f(2(x - 1)) + 4.', {'x_new': 2.0, 'y_new': 19.0}], ['An 80 g sample has a half-life of 5 years. How much remains after 15 years?', {'remaining': 10.0}]]


def _close(a, b):
    if isinstance(b, list):
        return isinstance(a, (list, tuple)) and len(a) == len(b) and all(_close(x, y) for x, y in zip(a, b))
    try:
        return abs(complex(a) - complex(b)) <= 1e-6 * max(1.0, abs(complex(b)))
    except (TypeError, ValueError):
        return a == b


def checks(P) -> dict:
    out = {"certified_procedures": len(P.pack().procedures) == 100,
           "refuses_nonsense": not P.ask("What is it?").solved}
    for i, (q, want) in enumerate(QUESTIONS):
        a = P.ask(q)
        out[f"question_{i}"] = a.solved and all(_close(a.values.get(k), v) for k, v in want.items())
    return out
