"""Math Foundations procedures: deterministic algorithms with typed inputs and several outputs (domain code, loaded by
the pack; enabled only once tools/certify_procedures.py has certified them). The first part is the retired algebra
model's hand-written procedures, folded in unchanged except two reader fixes (a decimal such as 1.5 is never cut at
its point; "rational roots" questions go to rational_roots); the second part (from "MATH FOUNDATIONS") was added
2026-10-22.

Conventions (run and reference agree on them exactly): angles in degrees unless radians are stated; a triangle's
angles A, B, C are opposite its sides a, b, c; the ambiguous SSA case reports both triangles (acute B first) and
refuses when none exists; real roots ascending; a complex argument is in (-180, 180] degrees; n-th roots, vector
directions and polar angles are ordered / reported in [0, 360); rounding is half away from zero on the decimal as
written; "log" without a base is base 10 and "ln" base e; an integer factorisation is g(px + q)(rx + s) with p, r > 0
and the factor of the smaller root first; exact rational arithmetic wherever the result is rational.

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question templates it parses,
verified worked examples, an INDEPENDENT reference computation (used only by certification) and a deliberately wrong
variant (certification shows the checks reject it).
"""
from __future__ import annotations

import cmath
import math
import re
from fractions import Fraction

_NUM = r"[-+]?\s*(?:\d+(?:\.\d+)?(?:/\d+)?|\.\d+)"


def _num(text):
    text = text.replace(" ", "")
    if text in ("", "+"):
        return 1.0
    if text == "-":
        return -1.0
    return float(Fraction(text)) if "/" in text else float(text)


def _side(expr: str, var: str):
    """Coefficients (a, b, c) of a polynomial of degree <= 2 written in `var`, or None if it is not one."""
    s = expr.replace("²", "^2").replace("**", "^").replace("·", "*").replace("−", "-").replace(" ", "")
    s = re.sub(r"(?<=\d)\*(?=" + var + ")", "", s)
    if not s:
        return None
    if not s.startswith(("+", "-")):
        s = "+" + s
    terms = re.findall(r"[+-][^+-]+", s)
    if "".join(terms) != s:
        return None
    a = b = c = 0.0
    for t in terms:
        m = re.fullmatch(r"([+-](?:\d+(?:\.\d+)?(?:/\d+)?|\.\d+)?)" + var + r"\^(\d+)", t)
        if m:
            if m.group(2) != "2":
                return None                          # degree other than 2: not this capability
            a += _num(m.group(1))
            continue
        m = re.fullmatch(r"([+-](?:\d+(?:\.\d+)?(?:/\d+)?|\.\d+)?)" + var, t)
        if m:
            b += _num(m.group(1))
            continue
        m = re.fullmatch(r"([+-](?:\d+(?:\.\d+)?(?:/\d+)?|\.\d+))", t)
        if m:
            c += _num(m.group(1))
            continue
        return None
    return a, b, c


def _polynomial(question: str):
    """'... 2x^2 - 3x + 1 = 0 ...' -> (a, b, c), moving the right side over; None when there is no quadratic."""
    m = re.search(r"([-+0-9.\s/]*[a-z](?:\^2|²|\*\*2)[-+0-9a-z.\s/^²*]*?)(?:=\s*([-+0-9a-z.\s/^²*]+?))?(?:\s*(?:[?,;]|\.(?!\d)|$|for|where))",
                  question.lower())
    if not m:
        return None
    pre = question.lower()[:m.start(1)].rstrip()
    term_before = re.search(r"(?:\d[a-z]?|(?<![a-z])[a-z]|\))$", pre)          # '7x', '3', 'x', ')' — not 'solve'
    if re.search(r"[0-9a-z)]\s*[-+*/]$", pre) or (term_before and m.group(1).lstrip()[:1] in "+-"):
        return None                  # the expression continues to the left ('y = 3 + 7x - 22x^2'): never read a part
    left = m.group(1).strip()
    var = re.search(r"([a-z])(?:\^2|²|\*\*2)", left).group(1)
    l = _side(left, var)
    r = _side(m.group(2).strip(), var) if m.group(2) else (0.0, 0.0, 0.0)
    if l is None or r is None:
        return None
    return {"a": l[0] - r[0], "b": l[1] - r[1], "c": l[2] - r[2]}, var


def _coefficients(question: str):
    """'a = 1, b = -5, c = 6' (any order)."""
    found = {k: float(Fraction(v.replace(" ", ""))) for k, v in re.findall(r"\b([abc])\s*=\s*(" + _NUM + r")", question.lower())}
    return found or None


def parse_quadratic(question: str):
    q = question.lower()
    if re.search(r"\brational (?:roots?|zeros?)\b", q):
        return None                  # Math Foundations: rational_roots reads these (the rational root theorem)
    asks_roots = re.search(r"\b(solve|roots?|zeros?|solutions?)\b", q)
    if not re.search(r"\b(solve|roots?|zeros?|solutions?|quadratic)\b", q) or ("discriminant" in q and not asks_roots):
        return None                  # "roots ... and the discriminant" is this capability (it reports the discriminant too)
    # the roots must be what is ASKED: a question about another quantity of a quadratic (its minimum, vertex, the
    # modulus or sum of a root ...) is not this capability, even when it mentions roots or "quadratic"
    if re.search(r"\b(minimum|maximum|vertex|turning point|modulus|absolute value|sum|product|axis of symmetry|"
                 r"intercepts?|value of|evaluate|range|domain|factor(?:ed|ise|ize)?|complete the square)\b", q) \
            or _asks_for_a_coefficient(q):
        return None
    deg = _degree_written(q)
    if deg in (3, 4):
        return None                  # a cubic or quartic: cubic_roots / quartic_roots read it
    if deg >= 5:
        return {"_out_of_scope": "a polynomial of degree 5 or higher (equations up to degree 4 are supported)"}
    poly = _polynomial(question)
    if poly:
        return poly[0]
    return _coefficients(question) if "quadratic" in q else None   # 'a = .., b = ..' belongs to the named procedure


def _asks_for_a_coefficient(q: str) -> bool:
    """'For which value of k ...', 'find k so that ...': an unknown COEFFICIENT is asked; neither the roots nor the
    discriminant procedure computes that."""
    return bool(re.search(r"\b(?:for )?(?:which|what) values? of [a-z]\b|\bfind (?:the value of )?[a-z]\b(?! *=)", q))


def parse_discriminant(question: str):
    q = question.lower()
    if "discriminant" not in q and not re.search(r"\bnature of the roots\b|\bhow many (real )?roots\b", q):
        return None
    if re.search(r"\b(find|report|give|compute|calculate|what are)\b[^.?]*\b(roots|solutions|x1)\b|\bsolve\b", q):
        return None                  # the roots are asked as well: quadratic_roots answers both
    if _asks_for_a_coefficient(q):
        return None
    poly = _polynomial(question)
    return poly[0] if poly else _coefficients(question)


def _check(a, b, c):
    for name, v in (("a", a), ("b", b), ("c", c)):
        if not (isinstance(v, (int, float)) and math.isfinite(v)):
            raise ValueError(f"{name} must be a finite real number")
    if a == 0:
        raise ValueError("a must not be 0: with a = 0 the equation is linear, not quadratic")


def quadratic_roots(a, b, c) -> dict:
    """Both roots, numerically stable (no cancellation between -b and the square root)."""
    _check(a, b, c)
    disc = b * b - 4 * a * c
    if disc > 0:
        q = -(b + math.copysign(math.sqrt(disc), b)) / 2
        r = sorted([q / a, c / q]) if q != 0 else [0.0, 0.0]
        return {"x1": r[0], "x2": r[1], "discriminant": disc}
    if disc == 0:
        x = -b / (2 * a)
        return {"x1": x, "x2": x, "discriminant": disc}
    re_, im = -b / (2 * a) + 0.0, math.sqrt(-disc) / (2 * abs(a))     # + 0.0: no negative zero
    return {"x1": complex(re_, im), "x2": complex(re_, -im), "discriminant": disc}


def discriminant(a, b, c) -> dict:
    _check(a, b, c)
    d = b * b - 4 * a * c
    nature = "two distinct real roots" if d > 0 else ("one repeated real root" if d == 0 else "two complex conjugate roots")
    return {"discriminant": d, "nature": nature}


def verify_roots(inputs, outputs) -> float:
    """Residual: each root substituted back into a*x^2 + b*x + c, relative to the size of the terms."""
    a, b, c = inputs["a"], inputs["b"], inputs["c"]
    worst = 0.0
    for x in (outputs["x1"], outputs["x2"]):
        terms = [a * x * x, b * x, c]
        scale = sum(abs(t) for t in terms) or 1.0
        worst = max(worst, abs(sum(terms)) / scale)
    return worst


def verify_discriminant(inputs, outputs) -> float:
    a, b, c = inputs["a"], inputs["b"], inputs["c"]
    return abs(outputs["discriminant"] - (b * b - 4 * a * c)) / max(1.0, b * b + abs(4 * a * c))


def _canonical(r1, r2):
    """Real roots ascending; complex roots with the positive imaginary part first."""
    if isinstance(r1, complex) and abs(r1.imag) > 0 or isinstance(r2, complex) and abs(r2.imag) > 0:
        r1, r2 = complex(r1), complex(r2)
        return (r1, r2) if r1.imag > 0 else (r2, r1)
    return tuple(sorted([float(complex(r1).real), float(complex(r2).real)]))


def reference_roots(a, b, c):
    """INDEPENDENT reference for certification: sympy's exact solver on rational coefficients."""
    import sympy as sp
    x = sp.Symbol("x")
    A, B, C = (sp.Rational(Fraction(v).numerator, Fraction(v).denominator) for v in (a, b, c))   # the exact inputs
    roots = []
    for r, mult in sp.roots(sp.Poly(A * x**2 + B * x + C, x)).items():
        roots += [complex(sp.N(r, 30))] * mult
    r1, r2 = _canonical(*[r if abs(r.imag) > 0 else r.real for r in roots])
    return {"x1": r1, "x2": r2, "discriminant": float(B ** 2 - 4 * A * C)}


def wrong_roots(a, b, c):
    """A deliberately wrong variant (a sign error in the formula): certification must reject it."""
    d = cmath.sqrt(b * b - 4 * a * c)
    r = [(b - d) / (2 * a), (b + d) / (2 * a)]
    r1, r2 = _canonical(*[x if abs(x.imag) > 0 else x.real for x in r])   # same output order as the procedure
    return {"x1": r1, "x2": r2, "discriminant": b * b - 4 * a * c}


_IN = [{"symbol": "a", "kind": "number", "meaning": "coefficient of x^2", "required": True, "condition": "a != 0"},
       {"symbol": "b", "kind": "number", "meaning": "coefficient of x", "required": True},
       {"symbol": "c", "kind": "number", "meaning": "constant term", "required": True}]

PROCEDURES = {
    "quadratic_roots": {
        "name": "Roots of a quadratic equation", "topic": "polynomial equations / quadratic equations",
        "purpose": "Solve a*x^2 + b*x + c = 0 for x: both roots, real or complex.",
        "inputs": _IN,
        "outputs": [{"symbol": "x1", "kind": "number", "meaning": "first root (the smaller real root, or the one with positive imaginary part)"},
                    {"symbol": "x2", "kind": "number", "meaning": "second root"},
                    {"symbol": "discriminant", "kind": "number", "meaning": "b^2 - 4ac"}],
        "primary": "x1", "conditions": ["a != 0 (otherwise the equation is linear)", "real coefficients"],
        "method": "quadratic formula in a numerically stable form (q = -(b + sign(b)*sqrt(b^2 - 4ac))/2, x = q/a and c/q)",
        "latex": r"x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}",
        "run": quadratic_roots, "verify": verify_roots, "parse": parse_quadratic,
        "reference": reference_roots, "wrong": wrong_roots,
        "templates": ["Solve x^2 - 5x + 6 = 0.", "Find the roots of 2x^2 + 3x - 2 = 0.", "Solve the quadratic with a = 1, b = -5, c = 6."],
        "examples": [
            {"inputs": {"a": 1, "b": -5, "c": 6}, "outputs": {"x1": 2.0, "x2": 3.0}},
            {"inputs": {"a": 2, "b": 3, "c": -2}, "outputs": {"x1": -2.0, "x2": 0.5}},
            {"inputs": {"a": 1, "b": 2, "c": 1}, "outputs": {"x1": -1.0, "x2": -1.0}},
            {"inputs": {"a": 1, "b": 0, "c": 4}, "outputs": {"x1": complex(0, 2), "x2": complex(0, -2)}},
            {"inputs": {"a": 1, "b": -1e8, "c": 1}, "outputs": {"x1": 1e-8, "x2": 1e8}}],
        "questions": [
            {"q": "Solve x^2 - 5x + 6 = 0.", "outputs": {"x1": 2.0, "x2": 3.0}},
            {"q": "Find the roots of 2x^2 + 3x - 2 = 0.", "outputs": {"x1": -2.0, "x2": 0.5}},
            {"q": "Solve the quadratic with a = 1, b = -5, c = 6.", "outputs": {"x1": 2.0, "x2": 3.0}},
            {"q": "Solve x^2 + 4 = 0.", "outputs": {"x1": complex(0, 2), "x2": complex(0, -2)}},
            {"q": "Solve 3x^2 = 12.", "outputs": {"x1": -2.0, "x2": 2.0}},
            {"q": "Solve x^5 - 1 = 0.", "outcome": "OUT_OF_SCOPE"},
            {"q": "Solve the quadratic with a = 0, b = 2, c = 1.", "outcome": "INVALID_INPUT"},
            {"q": "Solve the quadratic with a = 1, b = 2.", "outcome": "NEEDS_INPUT"}]},
    "quadratic_discriminant": {
        "name": "Discriminant of a quadratic", "topic": "polynomial equations / quadratic equations",
        "purpose": "The discriminant b^2 - 4ac of a*x^2 + b*x + c and the nature of the roots it implies.",
        "inputs": _IN,
        "outputs": [{"symbol": "discriminant", "kind": "number", "meaning": "b^2 - 4ac"},
                    {"symbol": "nature", "kind": "text", "meaning": "two distinct real / one repeated real / two complex conjugate roots"}],
        "primary": "discriminant", "conditions": ["a != 0", "real coefficients"],
        "method": "direct evaluation of b^2 - 4ac and its sign", "latex": r"\Delta = b^2 - 4ac",
        "run": discriminant, "verify": verify_discriminant, "parse": parse_discriminant,
        "reference": lambda a, b, c: {"discriminant": float(__import__("fractions").Fraction(b) ** 2 - 4 * __import__("fractions").Fraction(a) * __import__("fractions").Fraction(c))},
        "wrong": lambda a, b, c: {"discriminant": b * b + 4 * a * c, "nature": "?"},
        "templates": ["What is the discriminant of x^2 + 4x + 4?", "Find the discriminant with a = 2, b = 1, c = -3."],
        "examples": [
            {"inputs": {"a": 1, "b": 4, "c": 4}, "outputs": {"discriminant": 0.0}},
            {"inputs": {"a": 2, "b": 1, "c": -3}, "outputs": {"discriminant": 25.0}},
            {"inputs": {"a": 1, "b": 1, "c": 1}, "outputs": {"discriminant": -3.0}}],
        "questions": [
            {"q": "What is the discriminant of x^2 + 4x + 4?", "outputs": {"discriminant": 0.0}},
            {"q": "Find the discriminant with a = 2, b = 1, c = -3.", "outputs": {"discriminant": 25.0}}]},
}


# --- counting: exact integers (a closed-form spec cannot write a factorial) --------------------------------------
def _whole(v, name):
    if float(v) != int(float(v)) or float(v) < 0:
        raise ValueError(f"{name} must be a whole number >= 0")
    return int(float(v))


def counting_choices(n, k) -> dict:
    n, k = _whole(n, "n"), _whole(k, "k")
    if k > n:
        raise ValueError("k must not exceed n")
    if n > COUNTING_MAX_N:                   # results are floats: 170! is the largest factorial a float can hold
        raise ValueError(f"n must be at most {COUNTING_MAX_N} (the result would not fit in a floating-point number)")
    return {"combinations": float(math.comb(n, k)), "permutations": float(math.perm(n, k))}


COUNTING_MAX_N = 170


def verify_counting(inputs, outputs) -> float:
    """C(n, k) * k! = P(n, k) and P(n, k) * (n - k)! = n!, independent of comb/perm; exact integers, compared at
    the precision of the float outputs (above 2**53 a float is the rounded true value)."""
    n, k = int(inputs["n"]), int(inputs["k"])
    from fractions import Fraction                 # the exact value of each float output: nothing is truncated
    c, p = Fraction(outputs["combinations"]), Fraction(outputs["permutations"])
    rel = lambda a, b: abs(a - b) / max(1, abs(b))
    return max(rel(c * math.factorial(k), p), rel(p * math.factorial(n - k), math.factorial(n)))


def reference_counting(n, k):
    """INDEPENDENT reference for certification: sympy's binomial and falling factorial."""
    import sympy as sp
    return {"combinations": float(sp.binomial(int(n), int(k))), "permutations": float(sp.ff(int(n), int(k)))}


def parse_counting(question: str):
    q = question.lower()
    m = re.search(r"\b(?:choose|select|pick)\s+(\d+)\s+(?:\w+\s+)?(?:from|out of|of)\s+(\d+)\b", q) or \
        re.search(r"\bc\(\s*(\d+)\s*,\s*(\d+)\s*\)", q)
    if m and "c(" in m.group(0):
        return {"n": float(m.group(1)), "k": float(m.group(2))}
    if m and re.search(r"\b(ways|combinations?|arrangements?|permutations?|how many)\b", q):
        return {"k": float(m.group(1)), "n": float(m.group(2))}
    return None


PROCEDURES["counting_choices"] = {
    "name": "Combinations and permutations", "topic": "counting and probability",
    "purpose": "The number of ways to choose k items from n without order (combinations) and with order (permutations).",
    "inputs": [{"symbol": "n", "kind": "number", "meaning": "number of items to choose from (a whole number)", "required": True},
               {"symbol": "k", "kind": "number", "meaning": "number of items chosen (a whole number, k <= n)", "required": True}],
    "outputs": [{"symbol": "combinations", "kind": "number", "meaning": "C(n, k) = n! / (k! (n - k)!)"},
                {"symbol": "permutations", "kind": "number", "meaning": "P(n, k) = n! / (n - k)!"}],
    "primary": "combinations", "conditions": ["n and k whole numbers", "k <= n", "n <= 170"],
    "random_inputs": lambda rng: (lambda n: {"n": n, "k": rng.randint(0, n)})(
        rng.choice([rng.randint(0, 12), rng.randint(0, 60), rng.randint(0, COUNTING_MAX_N)])),
    "method": "exact integer arithmetic (math.comb, math.perm)", "latex": r"\binom{n}{k} = \frac{n!}{k!\,(n-k)!},\quad P(n,k) = \frac{n!}{(n-k)!}",
    "run": counting_choices, "verify": verify_counting, "parse": parse_counting, "reference": reference_counting,
    "wrong": lambda n, k: {"combinations": float(int(n) ** int(k)), "permutations": float(math.perm(int(n), int(k)))},
    "templates": ["How many ways are there to choose 3 books from 10?", "Compute C(10, 3)."],
    "examples": [{"inputs": {"n": 10, "k": 3}, "outputs": {"combinations": 120.0, "permutations": 720.0}},
                 {"inputs": {"n": 5, "k": 5}, "outputs": {"combinations": 1.0, "permutations": 120.0}},
                 {"inputs": {"n": 52, "k": 5}, "outputs": {"combinations": 2598960.0}}],
    "questions": [{"q": "How many ways are there to choose 3 books from 10?", "outputs": {"combinations": 120.0}},
                  {"q": "Compute C(6, 2).", "outputs": {"combinations": 15.0, "permutations": 30.0}},
                  {"q": "How many ways are there to choose 7 books from 4?", "outcome": "INVALID_INPUT"},
                  {"q": "Compute C(1000, 3).", "outcome": "INVALID_INPUT"}]}


# --- cubic and quartic equations: every root, real or complex (owner, 2026-10-17: "all the polynomial degrees") -----
_LETTERS = "abcde"


def _poly_side(expr: str, var: str, maxdeg: int):
    """Coefficients [c_0 .. c_maxdeg] of a polynomial in `var` written term by term, or None if it is not one."""
    s = expr.replace("²", "^2").replace("³", "^3").replace("⁴", "^4").replace("**", "^").replace("·", "*") \
        .replace("−", "-").replace(" ", "")
    s = re.sub(r"(?<=\d)\*(?=" + var + ")", "", s)
    if not s:
        return None
    if not s.startswith(("+", "-")):
        s = "+" + s
    terms = re.findall(r"[+-][^+-]+", s)
    if "".join(terms) != s:
        return None
    co = [0.0] * (maxdeg + 1)
    num = r"([+-](?:\d+(?:\.\d+)?(?:/\d+)?|\.\d+)?)"
    for t in terms:
        m = re.fullmatch(num + var + r"\^(\d+)", t)
        if m:
            deg = int(m.group(2))
            if deg > maxdeg:
                return None
            co[deg] += _num(m.group(1))
            continue
        m = re.fullmatch(num + var, t)
        if m:
            co[1] += _num(m.group(1))
            continue
        m = re.fullmatch(r"([+-](?:\d+(?:\.\d+)?(?:/\d+)?|\.\d+))", t)
        if m:
            co[0] += _num(m.group(1))
            continue
        return None
    return co


def _degree_written(q: str) -> int:
    degs = [int(d) for d in re.findall(r"[a-z]\s*(?:\^|\*\*)\s*(\d+)", q)] + [3] * q.count("³") + [4] * q.count("⁴")
    return max(degs, default=0)


def _parse_polynomial(question: str, degree: int):
    """'Solve x^3 - 6x^2 + 11x - 6 = 0' -> {'a':1,'b':-6,'c':11,'d':-6}; 'with a = .., b = ..' when the degree is
    named; None otherwise. The whole equation must be read (never a part of a longer expression)."""
    q = question.lower()
    if not re.search(r"\b(solve|roots?|zeros?|solutions?|cubic|quartic)\b", q) or re.search(r"\brational (?:roots?|zeros?)\b", q):
        return None
    if re.search(r"\b(minimum|maximum|turning point|inflection|derivative|factor(?:ed|ise|ize)?|value of|evaluate|"
                 r"sum|product|graph)\b", q) or _asks_for_a_coefficient(q):
        return None
    # an evaluation, "determine f(-4)" or "at x = 2", is not a request for roots (evaluate_function answers it;
    # tools/ask_sweep.py found the roots given instead, 2026-10-23)
    if re.search(r"\b[a-z]\s*\(\s*[-+]?\d", q) or re.search(r"\bat\s+[a-z]\s*=\s*[-+]?\d", q):
        return None
    name = {3: "cubic", 4: "quartic"}[degree]
    written = _degree_written(q)
    if written == degree:
        m = re.search(r"([-+0-9.\s/]*[a-z](?:\^\d|\*\*\d|[³⁴])[-+0-9a-z.\s/^²³⁴*]*?)(?:=\s*([-+0-9a-z.\s/^²³⁴*]+?))?"
                      r"(?:\s*(?:[?,;]|\.(?!\d)|$|for|where))", q)
        if not m:
            return None
        pre = q[:m.start(1)].rstrip()
        if re.search(r"[0-9a-z)]\s*[-+*/]$", pre):
            return None                              # the expression continues to the left: never read a part
        var = re.search(r"([a-z])(?:\^|\*\*|[³⁴])", m.group(1)).group(1)
        left = _poly_side(m.group(1).strip(), var, degree)
        right = _poly_side(m.group(2).strip(), var, degree) if m.group(2) else [0.0] * (degree + 1)
        if left is None or right is None:
            return None
        co = [l - r for l, r in zip(left, right)]
        return {_LETTERS[i]: co[degree - i] for i in range(degree + 1)}
    if written == 0 and name in q:
        found = {k: float(Fraction(v.replace(" ", ""))) for k, v in
                 re.findall(r"\b([" + _LETTERS[:degree + 1] + r"])\s*=\s*(" + _NUM + r")", q)}
        return found or None
    return None


def _canonical_roots(roots):
    """Real roots ascending, then complex roots by real part, the positive imaginary part first."""
    real = sorted(float(r.real) for r in roots if abs(r.imag) <= 1e-12 * max(1.0, abs(r)))
    cplx = sorted((complex(r) for r in roots if abs(r.imag) > 1e-12 * max(1.0, abs(r))), key=lambda z: (round(z.real, 12), -z.imag))
    return real + cplx


def _poly_roots(coeffs):
    """All roots of c_n x^n + ... + c_0 (highest first), to 30 digits, then rounded to floats."""
    import mpmath
    for name, v in zip(_LETTERS, coeffs):
        if not (isinstance(v, (int, float)) and math.isfinite(v)):
            raise ValueError(f"{name} must be a finite real number")
    if coeffs[0] == 0:
        raise ValueError(f"a must not be 0: with a = 0 the equation has a lower degree")
    with mpmath.workdps(30):
        roots = mpmath.polyroots([mpmath.mpf(str(c)) for c in coeffs], maxsteps=200, extraprec=60)
    return _canonical_roots([complex(r) for r in roots])


def _roots_out(roots):
    return {f"x{i + 1}": (r if isinstance(r, complex) else float(r)) for i, r in enumerate(roots)}


def cubic_roots(a, b, c, d) -> dict:
    return _roots_out(_poly_roots([a, b, c, d]))


def quartic_roots(a, b, c, d, e) -> dict:
    return _roots_out(_poly_roots([a, b, c, d, e]))


def _verify_poly(inputs, outputs, n) -> float:
    co = [inputs[k] for k in _LETTERS[:n + 1]]
    worst = 0.0
    for i in range(n):
        x = complex(outputs[f"x{i + 1}"])
        terms = [c * x ** (n - j) for j, c in enumerate(co)]
        worst = max(worst, abs(sum(terms)) / (sum(abs(t) for t in terms) or 1.0))
    return worst


def _reference_poly(coeffs):
    """INDEPENDENT reference for certification: sympy's exact roots of the rational polynomial, evaluated to 30 digits."""
    import sympy as sp
    x = sp.Symbol("x")
    poly = sp.Poly(sum(sp.Rational(Fraction(v).numerator, Fraction(v).denominator) * x ** (len(coeffs) - 1 - i)
                       for i, v in enumerate(coeffs)), x)
    roots = [complex(sp.N(r, 30)) for r in poly.nroots(n=30, maxsteps=200)]
    return _roots_out(_canonical_roots(roots))


def _wrong_poly(coeffs):
    """A deliberately wrong variant (the sign of the second coefficient flipped): certification must reject it."""
    flipped = list(coeffs)
    flipped[1] = -flipped[1]
    return _roots_out(_poly_roots(flipped))


def _poly_inputs(n):
    out = [{"symbol": "a", "kind": "number", "meaning": f"coefficient of x^{n}", "required": True, "condition": "a != 0"}]
    for i, k in enumerate(_LETTERS[1:n + 1], start=1):
        p = n - i
        out.append({"symbol": k, "kind": "number", "required": True,
                    "meaning": "constant term" if p == 0 else ("coefficient of x" if p == 1 else f"coefficient of x^{p}")})
    return out


PROCEDURES["cubic_roots"] = {
    "name": "Roots of a cubic equation", "topic": "polynomial equations / cubic and quartic equations",
    "purpose": "Solve a*x^3 + b*x^2 + c*x + d = 0 for x: all three roots, real or complex.",
    "inputs": _poly_inputs(3),
    "outputs": [{"symbol": f"x{i}", "kind": "number", "meaning": m} for i, m in
                ((1, "first root (real roots ascending, then complex roots)"), (2, "second root"), (3, "third root"))],
    "primary": "x1", "conditions": ["a != 0 (otherwise the equation is not cubic)", "real coefficients"],
    "method": "high-precision polynomial root finding (Durand-Kerner, 30 digits), each root substituted back",
    "latex": r"a x^3 + b x^2 + c x + d = 0",
    "run": cubic_roots, "verify": lambda i, o: _verify_poly(i, o, 3), "parse": lambda q: _parse_polynomial(q, 3),
    "reference": lambda a, b, c, d: _reference_poly([a, b, c, d]), "wrong": lambda a, b, c, d: _wrong_poly([a, b, c, d]),
    "templates": ["Solve x^3 - 6x^2 + 11x - 6 = 0.", "Find the roots of 2x^3 - 4x^2 - 22x + 24 = 0.",
                  "Solve the cubic with a = 1, b = 0, c = 0, d = -1."],
    "examples": [
        {"inputs": {"a": 1, "b": -6, "c": 11, "d": -6}, "outputs": {"x1": 1.0, "x2": 2.0, "x3": 3.0}},
        {"inputs": {"a": 2, "b": -4, "c": -22, "d": 24}, "outputs": {"x1": -3.0, "x2": 1.0, "x3": 4.0}},
        {"inputs": {"a": 1, "b": 0, "c": 0, "d": -1}, "outputs": {"x1": 1.0, "x2": complex(-0.5, 0.8660254037844386),
                                                                    "x3": complex(-0.5, -0.8660254037844386)}}],
    "questions": [
        {"q": "Solve x^3 - 6x^2 + 11x - 6 = 0.", "outputs": {"x1": 1.0, "x2": 2.0, "x3": 3.0}},
        {"q": "Solve x^3 - 1 = 0.", "outputs": {"x1": 1.0}},
        {"q": "Find the roots of 2x^3 - 4x^2 - 22x + 24 = 0.", "outputs": {"x1": -3.0, "x2": 1.0, "x3": 4.0}},
        {"q": "Solve the cubic with a = 1, b = 0, c = 0, d = -1.", "outputs": {"x1": 1.0}},
        {"q": "Solve the cubic with a = 0, b = 1, c = 2, d = 1.", "outcome": "INVALID_INPUT"},
        {"q": "Solve the cubic with a = 1, b = 2.", "outcome": "NEEDS_INPUT"}]}

PROCEDURES["quartic_roots"] = {
    "name": "Roots of a quartic equation", "topic": "polynomial equations / cubic and quartic equations",
    "purpose": "Solve a*x^4 + b*x^3 + c*x^2 + d*x + e = 0 for x: all four roots, real or complex.",
    "inputs": _poly_inputs(4),
    "outputs": [{"symbol": f"x{i}", "kind": "number", "meaning": m} for i, m in
                ((1, "first root (real roots ascending, then complex roots)"), (2, "second root"), (3, "third root"),
                 (4, "fourth root"))],
    "primary": "x1", "conditions": ["a != 0 (otherwise the equation is not quartic)", "real coefficients"],
    "method": "high-precision polynomial root finding (Durand-Kerner, 30 digits), each root substituted back",
    "latex": r"a x^4 + b x^3 + c x^2 + d x + e = 0",
    "run": quartic_roots, "verify": lambda i, o: _verify_poly(i, o, 4), "parse": lambda q: _parse_polynomial(q, 4),
    "reference": lambda a, b, c, d, e: _reference_poly([a, b, c, d, e]),
    "wrong": lambda a, b, c, d, e: _wrong_poly([a, b, c, d, e]),
    "templates": ["Solve x^4 - 5x^2 + 4 = 0.", "Find the roots of x^4 - 1 = 0.",
                  "Solve the quartic with a = 1, b = 0, c = -5, d = 0, e = 4."],
    "examples": [
        {"inputs": {"a": 1, "b": 0, "c": -5, "d": 0, "e": 4}, "outputs": {"x1": -2.0, "x2": -1.0, "x3": 1.0, "x4": 2.0}},
        {"inputs": {"a": 1, "b": 0, "c": 0, "d": 0, "e": -1}, "outputs": {"x1": -1.0, "x2": 1.0, "x3": complex(0, 1), "x4": complex(0, -1)}},
        {"inputs": {"a": 1, "b": -10, "c": 35, "d": -50, "e": 24}, "outputs": {"x1": 1.0, "x2": 2.0, "x3": 3.0, "x4": 4.0}}],
    "questions": [
        {"q": "Solve x^4 - 5x^2 + 4 = 0.", "outputs": {"x1": -2.0, "x2": -1.0, "x3": 1.0, "x4": 2.0}},
        {"q": "Find the roots of x^4 - 1 = 0.", "outputs": {"x1": -1.0, "x2": 1.0}},
        {"q": "Solve the quartic with a = 1, b = 0, c = -5, d = 0, e = 4.", "outputs": {"x1": -2.0, "x4": 2.0}},
        {"q": "Solve the quartic with a = 1, b = 2.", "outcome": "NEEDS_INPUT"}]}


# =====================================================================================================================
# MATH FOUNDATIONS (2026-10-22): the capabilities added to the folded algebra procedures above. Same contract:
# plain-Python `run`, `verify` and `parse` (documented templates only, read whole: never a part of a question), an
# INDEPENDENT `reference` (sympy, mpmath, numpy or scipy, imported inside the function: certification only) and a
# deliberately `wrong` variant that certification shows the checks reject.
# =====================================================================================================================
import ast as _ast
from decimal import ROUND_HALF_UP as _HALF_UP, Decimal as _Dec

_W = r"[-+]?\d+(?:\.\d+)?"                   # a decimal number
_Q = r"[-+]?\d+(?:\.\d+)?(?:/\d+)?"         # a decimal number or a fraction p/q
_E = r"\s*[.?!]?"                            # the end of a question


def _clean(q: str) -> str:
    """Typography only (never meaning): minus signs, multiplication dots, pi, roots, degree signs, spaces."""
    q = q.strip().replace("−", "-").replace("–", "-").replace("×", "*").replace("·", "*").replace("÷", " divided by ")
    q = q.replace("π", "pi").replace("√", "sqrt").replace("≤", "<=").replace("≥", ">=").replace("°", " degrees")
    return re.sub(r"\s+", " ", q).strip()


def _q(t) -> float:
    return float(Fraction(str(t).replace(" ", "")))


def _rel(x, y) -> float:
    x, y = complex(x), complex(y)
    return abs(x - y) / max(1.0, abs(x), abs(y))


def _finite(**kw):
    for k, v in kw.items():
        if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v):
            raise ValueError(f"{k} must be a finite number")


def _templated(patterns, build):
    """A parser for documented question forms: each regex must match the WHOLE (cleaned) question."""
    rx = [re.compile(p, re.I) for p in patterns]

    def parse(question):
        if not isinstance(question, str):
            return None
        q = _clean(question)
        for r in rx:
            m = r.fullmatch(q)
            if m:
                try:
                    return build(m.groupdict())
                except (ValueError, ZeroDivisionError, KeyError, TypeError):
                    return None
        return None
    return parse


def _num_in(sym, meaning, condition=None, required=True):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}
    if condition:
        d["condition"] = condition
    return d


def _out(sym, meaning, kind="number"):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


def _register(key, **spec):
    PROCEDURES[key] = spec


def _guarded(fn, fallback=None):
    """A wrong variant must return a result, never raise: an undefined value becomes a clearly wrong number."""
    try:
        return fn()
    except (ValueError, ZeroDivisionError, OverflowError):
        return fallback if fallback is not None else {"value": 1e9}


# --- exact arithmetic of written expressions (two independent readers: Python's ast, and a Pratt parser) -------------
_MAX_DIGITS = 400


def _prep_expr(text: str, var: str | None = None) -> str:
    """Written arithmetic -> Python arithmetic: ^ -> **, implicit products (2x, 3(x + 1), (x + 1)(x - 1), 2sqrt(x))."""
    s = _clean(text).lower().replace("^", "**").replace("[", "(").replace("]", ")").replace(" divided by ", "/")
    s = re.sub(r"\bsqrt\s*(\d+(?:\.\d+)?)", r"sqrt(\1)", s)
    s = re.sub(r"(\d)\s*(?=[a-z(])", r"\1*", s)
    s = re.sub(r"\)\s*(?=[a-z0-9(])", ")*", s)
    if var:
        s = re.sub(rf"\b{var}\s*(?=\()", f"{var}*", s)
    return s


def _big(v):
    if isinstance(v, Fraction) and (len(str(abs(v.numerator))) > _MAX_DIGITS or len(str(v.denominator)) > _MAX_DIGITS):
        raise ValueError("the numbers become too large")
    if isinstance(v, float) and not math.isfinite(v):
        raise ValueError("the value is not finite")
    return v


def _sqrt(v):
    if v < 0:
        raise ValueError("square root of a negative number")
    if isinstance(v, Fraction):
        n, d = math.isqrt(v.numerator), math.isqrt(v.denominator)
        if n * n == v.numerator and d * d == v.denominator:
            return Fraction(n, d)
    return math.sqrt(v)


def _pow(a, b):
    if isinstance(b, Fraction) and b.denominator == 1:
        if abs(b) > 64:
            raise ValueError("exponents are limited to 64 in size")
        if a == 0 and b < 0:
            raise ValueError("division by zero (zero to a negative power)")
        return _big(a ** int(b)) if isinstance(a, Fraction) else _big(float(a) ** int(b))
    if isinstance(b, Fraction) and b.denominator == 2 and a >= 0:
        r = _sqrt(a if isinstance(a, Fraction) else Fraction(a))
        return _pow(r, Fraction(b.numerator))
    if a <= 0:
        raise ValueError("a non-integer power of a number that is not positive")
    return _big(float(a) ** float(b))


def _fn(name, v):
    if name == "sqrt":
        return _sqrt(v)
    if name == "abs":
        return abs(v)
    if name == "ln":
        if v <= 0:
            raise ValueError("logarithm of a number that is not positive")
        return math.log(v)
    raise ValueError(f"unknown function {name}")


_FUNC_NAMES = ("sqrt", "abs", "ln")


def _eval_ast(text: str, env: dict | None = None, var: str | None = None):
    """Exact value (Fraction; float once a root or logarithm is irrational) of a written expression, read with
    Python's ast and walked node by node (nothing is evaluated as code)."""
    env = env or {}
    if not isinstance(text, str) or not text.strip() or len(text) > 300:
        raise ValueError("the expression is missing or too long")
    try:
        tree = _ast.parse(_prep_expr(text, var), mode="eval")
    except SyntaxError:
        raise ValueError(f"cannot read the expression {text!r}") from None
    if sum(1 for _ in _ast.walk(tree)) > 200:
        raise ValueError("the expression is too long")

    def walk(n):
        if isinstance(n, _ast.Expression):
            return walk(n.body)
        if isinstance(n, _ast.Constant) and type(n.value) in (int, float):
            return Fraction(n.value) if isinstance(n.value, int) else Fraction(repr(n.value))
        if isinstance(n, _ast.Name) and n.id in env:
            v = env[n.id]
            return Fraction(repr(float(v))) if isinstance(v, float) else Fraction(v)
        if isinstance(n, _ast.UnaryOp) and isinstance(n.op, (_ast.USub, _ast.UAdd)):
            v = walk(n.operand)
            return -v if isinstance(n.op, _ast.USub) else v
        if isinstance(n, _ast.BinOp) and isinstance(n.op, (_ast.Add, _ast.Sub, _ast.Mult, _ast.Div, _ast.Pow)):
            a, b = walk(n.left), walk(n.right)
            if isinstance(n.op, _ast.Add):
                return _big(a + b)
            if isinstance(n.op, _ast.Sub):
                return _big(a - b)
            if isinstance(n.op, _ast.Mult):
                return _big(a * b)
            if isinstance(n.op, _ast.Div):
                if b == 0:
                    raise ValueError("division by zero")
                return _big(a / b)
            return _pow(a, b if isinstance(b, Fraction) else Fraction(b))
        if isinstance(n, _ast.Call) and isinstance(n.func, _ast.Name) and n.func.id in _FUNC_NAMES \
                and len(n.args) == 1 and not n.keywords:
            return _fn(n.func.id, walk(n.args[0]))
        raise ValueError(f"unsupported notation in {text!r}")
    return walk(tree)


_TOK = re.compile(r"\s*(?:(\d+(?:\.\d+)?)|([a-z_]+)|(\*\*|[-+*/()]))")


def _tokens(s: str) -> list:
    out, pos = [], 0
    s = s.strip()
    while pos < len(s):
        m = _TOK.match(s, pos)
        if not m or m.end() == pos:
            raise ValueError(f"cannot read the expression at {s[pos:pos + 10]!r}")
        out.append(("num", m.group(1)) if m.group(1) else (("name", m.group(2)) if m.group(2) else ("op", m.group(3))))
        pos = m.end()
        while pos < len(s) and s[pos] == " ":
            pos += 1
    return out


def _eval_pratt(text: str, env: dict | None = None, var: str | None = None, flat: bool = False):
    """The same value read by a Pratt (precedence-climbing) parser: the verifier's second reading. With flat=True every
    binary operator has the same precedence (left to right): the classic order-of-operations mistake."""
    env = env or {}
    toks = _tokens(_prep_expr(text, var))
    pos = [0]
    lbp = {"+": 10, "-": 10, "*": 20, "/": 20, "**": 30} if not flat else {"+": 10, "-": 10, "*": 10, "/": 10, "**": 10}

    def peek():
        return toks[pos[0]] if pos[0] < len(toks) else ("end", None)

    def take():
        t = peek()
        pos[0] += 1
        return t

    def parse(rbp):
        t = take()
        left = nud(t)
        while True:
            p = peek()
            if p[0] != "op" or p[1] not in lbp or lbp[p[1]] <= rbp:
                break
            take()
            if p[1] == "**":
                right = parse(29 if not flat else 10)
                left = _pow(left, right if isinstance(right, Fraction) else Fraction(right))
                continue
            right = parse(lbp[p[1]])
            if p[1] == "+":
                left = _big(left + right)
            elif p[1] == "-":
                left = _big(left - right)
            elif p[1] == "*":
                left = _big(left * right)
            else:
                if right == 0:
                    raise ValueError("division by zero")
                left = _big(left / right)
        return left

    def nud(t):
        kind, v = t
        if kind == "num":
            return Fraction(v)
        if kind == "name":
            if v in env:
                x = env[v]
                return Fraction(repr(float(x))) if isinstance(x, float) else Fraction(x)
            if v in _FUNC_NAMES:
                if take() != ("op", "("):
                    raise ValueError("a function needs parentheses")
                inner = parse(0)
                if take() != ("op", ")"):
                    raise ValueError("unbalanced parentheses")
                return _fn(v, inner)
            raise ValueError(f"unknown name {v!r}")
        if t == ("op", "("):
            inner = parse(0)
            if take() != ("op", ")"):
                raise ValueError("unbalanced parentheses")
            return inner
        if t == ("op", "-"):
            return -parse(25 if not flat else 30)
        if t == ("op", "+"):
            return parse(25 if not flat else 30)
        raise ValueError("cannot read the expression")
    value = parse(0)
    if pos[0] != len(toks):
        raise ValueError("cannot read the expression (unexpected text at the end)")
    return value


def _sym_expr(text: str, var: str | None = None):
    """sympy's own reading of the same written expression (certification only)."""
    import sympy as sp
    from sympy.parsing.sympy_parser import parse_expr, standard_transformations, implicit_multiplication_application, \
        convert_xor, rationalize
    s = _clean(text).lower().replace("[", "(").replace("]", ")").replace(" divided by ", "/").replace("ln(", "log(")
    s = re.sub(r"(\d)\s*(?=[a-z(])", r"\1*", s)            # 2x, 0x (not a hexadecimal literal), 3(x + 1)
    loc = {"x": sp.Symbol("x"), "sqrt": sp.sqrt, "abs": sp.Abs, "log": sp.log}
    return parse_expr(s, local_dict=loc, transformations=standard_transformations + (convert_xor, rationalize,
                      implicit_multiplication_application), evaluate=True)


def _exact_outputs(v):
    if isinstance(v, Fraction):
        return {"value": float(v), "numerator": float(v.numerator), "denominator": float(v.denominator)}
    return {"value": float(v)}


# --- 1. fraction arithmetic ----------------------------------------------------------------------------------------
_OPS = {"+": "+", "plus": "+", "-": "-", "minus": "-", "*": "*", "times": "*", "x": "*", "divided by": "/", "/": "/"}


def fraction_arithmetic(a, b, c, d, op) -> dict:
    if b == 0 or d == 0:
        raise ValueError("a denominator must not be 0")
    f1, f2 = Fraction(int(a), int(b)), Fraction(int(c), int(d))
    if op == "/" and f2 == 0:
        raise ValueError("division by the fraction 0")
    r = {"+": f1 + f2, "-": f1 - f2, "*": f1 * f2, "/": f1 / f2 if f2 else None}[op]
    return {"numerator": float(r.numerator), "denominator": float(r.denominator), "value": float(r)}


def _cross(a, b, c, d, op):
    """The exact result by the textbook cross-multiplication rules (numerator, denominator), unreduced."""
    return {"+": (a * d + c * b, b * d), "-": (a * d - c * b, b * d), "*": (a * c, b * d), "/": (a * d, b * c)}[op]


def verify_fraction(i, o) -> float:
    n, m = _cross(int(i["a"]), int(i["b"]), int(i["c"]), int(i["d"]), i["op"])
    num, den = o["numerator"], o["denominator"]
    if den <= 0 or num != int(num) or den != int(den) or math.gcd(int(num), int(den)) != 1:
        return 1.0
    return abs(num * m - n * den) / max(1.0, abs(n * den)) + _rel(o["value"], num / den)


def _wrong_fraction(a, b, c, d, op):
    if op in "+-":
        den = b + d if op == "+" else b - d
        num = a + c if op == "+" else a - c
        r = Fraction(num, den) if den else Fraction(num, b * d)
    else:
        r = Fraction(a * d, b * c) if (op == "*" and c) else Fraction(a * c, b * d)
    return {"numerator": float(r.numerator), "denominator": float(r.denominator), "value": float(r)}


def _ref_fraction(a, b, c, d, op):
    import sympy as sp
    f1, f2 = sp.Rational(int(a), int(b)), sp.Rational(int(c), int(d))
    r = {"+": sp.Add(f1, f2), "-": sp.Add(f1, -f2), "*": sp.Mul(f1, f2), "/": sp.Mul(f1, sp.Pow(f2, -1))}[op]
    return {"numerator": float(r.p), "denominator": float(r.q), "value": float(r)}


def _rand_fraction(rng):
    op = rng.choice(["+", "-", "*", "/"])
    c = rng.choice([v for v in range(-12, 13) if v != 0])
    return {"a": rng.randint(-12, 12), "b": rng.randint(1, 12), "c": c, "d": rng.randint(1, 12), "op": op}


_FR = r"(?P<a>-?\d+)\s*/\s*(?P<b>-?\d+)\s*(?P<op>[-+*]|plus|minus|times|divided by)\s*\(?(?P<c>-?\d+)\s*/\s*(?P<d>\d+)\)?"
_register(
    "fraction_arithmetic", name="Arithmetic with fractions", topic="arithmetic / fractions",
    purpose="Add, subtract, multiply or divide two fractions a/b and c/d; the result in lowest terms.",
    inputs=[{"symbol": "a", "type": "integer", "kind": "number", "meaning": "numerator of the first fraction", "required": True},
            {"symbol": "b", "type": "integer", "kind": "number", "meaning": "denominator of the first fraction (not 0)", "required": True},
            {"symbol": "c", "type": "integer", "kind": "number", "meaning": "numerator of the second fraction", "required": True},
            {"symbol": "d", "type": "integer", "kind": "number", "meaning": "denominator of the second fraction (not 0)", "required": True},
            {"symbol": "op", "type": "choice", "choices": ["+", "-", "*", "/"], "kind": "text", "meaning": "the operation", "required": True}],
    outputs=[_out("numerator", "numerator of the result in lowest terms"), _out("denominator", "denominator of the result (positive)"),
             _out("value", "the result as a decimal")],
    primary="value", conditions=["whole-number numerators and denominators", "denominators not 0", "no division by 0"],
    method="common denominator (a/b ± c/d = (ad ± cb)/bd), product of numerators over product of denominators, "
           "division as multiplication by the reciprocal; then divide by the greatest common divisor",
    latex=r"\frac{a}{b} \pm \frac{c}{d} = \frac{ad \pm cb}{bd},\quad \frac{a}{b}\cdot\frac{c}{d} = \frac{ac}{bd},\quad \frac{a}{b} \div \frac{c}{d} = \frac{ad}{bc}",
    run=fraction_arithmetic, verify=verify_fraction, reference=_ref_fraction, wrong=_wrong_fraction, random_inputs=_rand_fraction,
    parse=_templated([r"(?:compute|calculate|find|work out|simplify) " + _FR + r"(?: as a (?:single )?fraction)?(?: in (?:lowest|simplest) terms)?" + _E,
                      r"what is " + _FR + r"(?: as a fraction)?(?: in (?:lowest|simplest) terms)?" + _E],
                     lambda g: {"a": int(g["a"]), "b": int(g["b"]), "c": int(g["c"]), "d": int(g["d"]), "op": _OPS[g["op"].lower()]}),
    templates=["Compute 3/4 + 5/6 as a fraction in lowest terms.", "What is 2/3 times 9/10?", "Calculate 7/8 - 1/6."],
    examples=[{"inputs": {"a": 3, "b": 4, "c": 5, "d": 6, "op": "+"}, "outputs": {"numerator": 19.0, "denominator": 12.0}},
              {"inputs": {"a": 2, "b": 3, "c": 9, "d": 10, "op": "*"}, "outputs": {"numerator": 3.0, "denominator": 5.0, "value": 0.6}},
              {"inputs": {"a": 1, "b": 2, "c": 3, "d": 4, "op": "-"}, "outputs": {"numerator": -1.0, "denominator": 4.0}},
              {"inputs": {"a": 3, "b": 4, "c": 9, "d": 8, "op": "/"}, "outputs": {"numerator": 2.0, "denominator": 3.0}}],
    questions=[{"q": "Compute 3/4 + 5/6 as a fraction in lowest terms.", "outputs": {"numerator": 19.0, "denominator": 12.0}},
               {"q": "What is 2/3 times 9/10?", "outputs": {"numerator": 3.0, "denominator": 5.0}},
               {"q": "Calculate 7/8 - 1/6.", "outputs": {"numerator": 17.0, "denominator": 24.0}},
               {"q": "Find 5/6 divided by 0/3.", "outcome": "INVALID_INPUT"},
               {"q": "Compute 1/0 + 1/2.", "outcome": "INVALID_INPUT"}])


# --- 2-5. percentages ----------------------------------------------------------------------------------------------
_PCT = r"\s*(?:%|percent|per cent)"


def percent_of_number(p, x) -> dict:
    _finite(p=p, x=x)
    return {"result": p * x / 100}


def _ref_percent_of(p, x):
    import sympy as sp
    y = sp.Symbol("y")
    P, X = (sp.Rational(repr(float(v))) for v in (p, x))
    if X == 0:
        return {"result": 0.0}
    return {"result": float(sp.solve(sp.Eq(y / X, P / 100), y)[0])}      # per cent: y is to x as p is to 100


_register(
    "percent_of_number", name="Percent of a number", topic="arithmetic / percentages",
    purpose="p% of x: the part that p per cent of x amounts to.",
    inputs=[_num_in("p", "the percentage p (per hundred)"), _num_in("x", "the whole x")],
    outputs=[_out("result", "p% of x")], primary="result", conditions=["finite numbers"],
    method="per cent means per hundred: p% of x = (p/100)·x", latex=r"\frac{p}{100}\,x",
    run=percent_of_number, verify=lambda i, o: abs(o["result"] * 100 - i["p"] * i["x"]) / max(1.0, abs(i["p"] * i["x"])),
    reference=_ref_percent_of, wrong=lambda p, x: {"result": p * x / 10},
    random_inputs=lambda rng: {"p": round(rng.uniform(0, 200), rng.choice([0, 1, 2])), "x": round(rng.uniform(-1000, 1000), rng.choice([0, 2]))},
    parse=_templated([r"(?:what is|find|calculate|compute|work out) (?P<p>" + _W + r")" + _PCT + r" of (?P<x>" + _W + r")" + _E],
                     lambda g: {"p": float(g["p"]), "x": float(g["x"])}),
    templates=["What is 15% of 80?", "Find 12.5 percent of 240."],
    examples=[{"inputs": {"p": 15, "x": 80}, "outputs": {"result": 12.0}}, {"inputs": {"p": 12.5, "x": 240}, "outputs": {"result": 30.0}},
              {"inputs": {"p": 150, "x": 40}, "outputs": {"result": 60.0}}],
    questions=[{"q": "What is 15% of 80?", "outputs": {"result": 12.0}}, {"q": "Find 12.5 percent of 240.", "outputs": {"result": 30.0}},
               {"q": "Calculate 150% of 40.", "outputs": {"result": 60.0}}])


def percent_of_what(part, p) -> dict:
    _finite(part=part, p=p)
    if p == 0:
        raise ValueError("p must not be 0 (0% of every number is 0)")
    return {"whole": part * 100 / p}


def _ref_percent_whole(part, p):
    import sympy as sp
    w = sp.Symbol("w")
    A, P = (sp.Rational(repr(float(v))) for v in (part, p))
    return {"whole": float(sp.solve(sp.Eq(A * 100, P * w), w)[0])}


_register(
    "percent_of_what", name="Whole from a part and its percentage", topic="arithmetic / percentages",
    purpose="The whole w of which `part` is p per cent (part is p% of what number?).",
    inputs=[_num_in("part", "the known part"), _num_in("p", "the percentage that the part is of the whole", "p != 0")],
    outputs=[_out("whole", "the whole")], primary="whole", conditions=["p != 0"],
    method="part = (p/100)·whole, so whole = 100·part/p", latex=r"w = \frac{100\,\mathrm{part}}{p}",
    run=percent_of_what, verify=lambda i, o: abs(o["whole"] * i["p"] - 100 * i["part"]) / max(1.0, abs(100 * i["part"]), abs(o["whole"] * i["p"])),
    reference=_ref_percent_whole, wrong=lambda part, p: {"whole": part * p / 100},
    random_inputs=lambda rng: {"part": round(rng.uniform(-500, 500), rng.choice([0, 2])), "p": rng.choice([round(rng.uniform(0.5, 150), 1), rng.randint(1, 99)])},
    parse=_templated([r"(?P<part>" + _W + r") is (?P<p>" + _W + r")" + _PCT + r" of what(?: number)?" + _E,
                      r"(?P<p>" + _W + r")" + _PCT + r" of what(?: number)? is (?P<part>" + _W + r")" + _E],
                     lambda g: {"part": float(g["part"]), "p": float(g["p"])}),
    templates=["18 is 15% of what number?", "40% of what number is 26?"],
    examples=[{"inputs": {"part": 18, "p": 15}, "outputs": {"whole": 120.0}}, {"inputs": {"part": 26, "p": 40}, "outputs": {"whole": 65.0}}],
    questions=[{"q": "18 is 15% of what number?", "outputs": {"whole": 120.0}}, {"q": "40% of what number is 26?", "outputs": {"whole": 65.0}},
               {"q": "5 is 0% of what number?", "outcome": "INVALID_INPUT"}])


def what_percent(part, whole) -> dict:
    _finite(part=part, whole=whole)
    if whole == 0:
        raise ValueError("the whole must not be 0")
    return {"percent": 100 * part / whole}


def _ref_what_percent(part, whole):
    import sympy as sp
    p = sp.Symbol("p")
    A, W = (sp.Rational(repr(float(v))) for v in (part, whole))
    return {"percent": float(sp.solve(sp.Eq(A / W, p / 100), p)[0])}


_register(
    "what_percent", name="What percent one number is of another", topic="arithmetic / percentages",
    purpose="The percentage p such that `part` is p% of `whole`.",
    inputs=[_num_in("part", "the part"), _num_in("whole", "the whole (not 0)", "whole != 0")],
    outputs=[_out("percent", "part as a percentage of whole")], primary="percent", conditions=["whole != 0"],
    method="p = 100·part/whole", latex=r"p = 100\,\frac{\mathrm{part}}{\mathrm{whole}}",
    run=what_percent, verify=lambda i, o: abs(o["percent"] * i["whole"] - 100 * i["part"]) / max(1.0, abs(100 * i["part"]), abs(o["percent"] * i["whole"])),
    reference=_ref_what_percent, wrong=lambda part, whole: {"percent": 100 * whole / part if part else 100.0},
    random_inputs=lambda rng: {"part": round(rng.uniform(0, 500), rng.choice([0, 1])), "whole": round(rng.uniform(1, 800), rng.choice([0, 1]))},
    parse=_templated([r"what percent(?:age)? of (?P<whole>" + _W + r") is (?P<part>" + _W + r")" + _E,
                      r"(?P<part>" + _W + r") is what percent(?:age)? of (?P<whole>" + _W + r")" + _E],
                     lambda g: {"part": float(g["part"]), "whole": float(g["whole"])}),
    templates=["What percent of 80 is 12?", "45 is what percent of 60?"],
    examples=[{"inputs": {"part": 12, "whole": 80}, "outputs": {"percent": 15.0}}, {"inputs": {"part": 45, "whole": 60}, "outputs": {"percent": 75.0}}],
    questions=[{"q": "What percent of 80 is 12?", "outputs": {"percent": 15.0}}, {"q": "45 is what percent of 60?", "outputs": {"percent": 75.0}},
               {"q": "What percent of 0 is 5?", "outcome": "INVALID_INPUT"}])


def percent_change(old, new) -> dict:
    _finite(old=old, new=new)
    if old <= 0:
        raise ValueError("the original value must be positive (a percent change is measured from it)")
    pc = 100 * (new - old) / old
    return {"percent_change": pc, "difference": new - old,
            "direction": "increase" if new > old else ("decrease" if new < old else "no change")}


def _verify_pchange(i, o):
    want_dir = "increase" if i["new"] > i["old"] else ("decrease" if i["new"] < i["old"] else "no change")
    return (abs(o["percent_change"] * i["old"] - 100 * (i["new"] - i["old"])) / max(1.0, abs(100 * (i["new"] - i["old"])), abs(o["percent_change"] * i["old"]))
            + _rel(o["difference"], i["new"] - i["old"]) + (o["direction"] != want_dir))


def _ref_pchange(old, new):
    import sympy as sp
    p = sp.Symbol("p")
    O, N = (sp.Rational(repr(float(v))) for v in (old, new))
    return {"percent_change": float(sp.solve(sp.Eq(N, O * (1 + p / 100)), p)[0]), "difference": float(N - O)}


_register(
    "percent_change", name="Percent change (increase or decrease)", topic="arithmetic / percentages",
    purpose="The percent change from an original value to a new value (negative for a decrease).",
    inputs=[_num_in("old", "the original value (positive)"), _num_in("new", "the new value")],
    outputs=[_out("percent_change", "percent change (negative = decrease)"), _out("difference", "new - old"),
             _out("direction", "increase, decrease or no change", "text")],
    primary="percent_change", conditions=["old > 0"],
    method="percent change = (new - old)/old · 100", latex=r"\Delta\% = \frac{\mathrm{new}-\mathrm{old}}{\mathrm{old}}\cdot 100",
    run=percent_change, verify=_verify_pchange, reference=_ref_pchange,
    wrong=lambda old, new: {"percent_change": 100 * (new - old) / new if new else 100.0, "difference": new - old, "direction": "?"},
    random_inputs=lambda rng: {"old": round(rng.uniform(1, 1000), rng.choice([0, 2])), "new": round(rng.uniform(1, 1500), rng.choice([0, 2]))},
    parse=_templated([r"(?:find|what is|calculate|compute) the percent(?:age)? (?:change|increase|decrease) from (?P<old>" + _W + r") to (?P<new>" + _W + r")" + _E,
                      r"a (?:price|value|quantity|population|number) (?:rises|falls|increases|decreases|changes|goes) from (?P<old>" + _W
                      + r") to (?P<new>" + _W + r")\. what is the percent(?:age)? (?:change|increase|decrease)" + _E],
                     lambda g: {"old": float(g["old"]), "new": float(g["new"])}),
    templates=["Find the percent change from 80 to 100.", "A price rises from 40 to 50. What is the percent increase?"],
    examples=[{"inputs": {"old": 80, "new": 100}, "outputs": {"percent_change": 25.0, "direction": "increase"}},
              {"inputs": {"old": 50, "new": 40}, "outputs": {"percent_change": -20.0, "direction": "decrease"}}],
    questions=[{"q": "Find the percent change from 80 to 100.", "outputs": {"percent_change": 25.0}},
               {"q": "What is the percent decrease from 50 to 40?", "outputs": {"percent_change": -20.0, "direction": "decrease"}},
               {"q": "A price rises from 40 to 50. What is the percent increase?", "outputs": {"percent_change": 25.0}},
               {"q": "Find the percent change from 0 to 10.", "outcome": "INVALID_INPUT"}])


# --- 6-8. ratios, proportions and unit rates -----------------------------------------------------------------------
def ratio_split(total, a, b, c=None) -> dict:
    _finite(total=total, a=a, b=b)
    parts = [a, b] + ([c] if c is not None else [])
    if any(not math.isfinite(p) or p <= 0 for p in parts):
        raise ValueError("every term of the ratio must be positive")
    s = sum(parts)
    return {f"part{i + 1}": total * p / s for i, p in enumerate(parts)}


def _verify_ratio(i, o):
    parts = [i["a"], i["b"]] + ([i["c"]] if i.get("c") is not None else [])
    got = [o.get(f"part{k + 1}") for k in range(len(parts))]
    if any(g is None for g in got):
        return 1.0
    r = _rel(sum(got), i["total"])
    for k in range(1, len(parts)):
        r += abs(got[k] * parts[0] - got[0] * parts[k]) / max(1.0, abs(got[0] * parts[k]), abs(got[k] * parts[0]))
    return r


def _ref_ratio(total, a, b, c=None):
    import sympy as sp
    terms = [a, b] + ([c] if c is not None else [])
    R = [sp.Rational(repr(float(t))) for t in terms]
    P = sp.symbols(f"p1:{len(R) + 1}")
    eqs = [sp.Eq(sum(P), sp.Rational(repr(float(total))))] + [sp.Eq(P[k] * R[0], P[0] * R[k]) for k in range(1, len(R))]
    sol = sp.solve(eqs, P, dict=True)[0]
    return {f"part{k + 1}": float(sol[P[k]]) for k in range(len(R))}


def _rand_ratio(rng):
    x = {"total": round(rng.uniform(10, 5000), rng.choice([0, 2])), "a": rng.randint(1, 12), "b": rng.randint(1, 12)}
    if rng.random() < 0.4:
        x["c"] = rng.randint(1, 12)
    return x


def _wrong_ratio(total, a, b, c=None):
    parts = [a, b] + ([c] if c is not None else [])
    return {f"part{i + 1}": total / len(parts) * p for i, p in enumerate(parts)}


_register(
    "ratio_split", name="Divide a quantity in a given ratio", topic="arithmetic / ratios and proportions",
    purpose="Share a total in the ratio a:b (or a:b:c).",
    inputs=[_num_in("total", "the quantity to share"), _num_in("a", "first term of the ratio (positive)"),
            _num_in("b", "second term of the ratio (positive)"), _num_in("c", "third term of the ratio (optional, positive)", required=False)],
    outputs=[_out("part1", "first share"), _out("part2", "second share"), _out("part3", "third share (a three-term ratio only)")],
    primary="part1", conditions=["every term of the ratio positive"],
    method="each share = total · term / (sum of the terms)", latex=r"\mathrm{part}_i = \mathrm{total}\cdot\frac{r_i}{\sum_j r_j}",
    run=ratio_split, verify=_verify_ratio, reference=_ref_ratio, wrong=_wrong_ratio, random_inputs=_rand_ratio,
    parse=_templated([r"(?:divide|split|share) (?P<total>" + _W + r") in the ratio (?P<a>" + _W + r")\s*:\s*(?P<b>" + _W + r")(?:\s*:\s*(?P<c>" + _W + r"))?" + _E],
                     lambda g: {k: float(v) for k, v in g.items() if v is not None}),
    templates=["Divide 120 in the ratio 3:5.", "Share 900 in the ratio 2:3:4."],
    examples=[{"inputs": {"total": 120, "a": 3, "b": 5}, "outputs": {"part1": 45.0, "part2": 75.0}},
              {"inputs": {"total": 900, "a": 2, "b": 3, "c": 4}, "outputs": {"part1": 200.0, "part2": 300.0, "part3": 400.0}}],
    questions=[{"q": "Divide 120 in the ratio 3:5.", "outputs": {"part1": 45.0, "part2": 75.0}},
               {"q": "Share 900 in the ratio 2:3:4.", "outputs": {"part1": 200.0, "part2": 300.0, "part3": 400.0}},
               {"q": "Split 50 in the ratio 0:5.", "outcome": "INVALID_INPUT"}])


_POSITIONS = ["first numerator", "first denominator", "second numerator", "second denominator"]


def _prop_terms(p, q, r, position):
    """The four terms n1/d1 = n2/d2 with None at the unknown's position; p, q, r are the known terms in order."""
    t = [p, q, r]
    k = _POSITIONS.index(position)
    return t[:k] + [None] + t[k:]


def proportion_solve(p, q, r, position) -> dict:
    _finite(p=p, q=q, r=r)
    n1, d1, n2, d2 = _prop_terms(p, q, r, position)
    if d1 == 0 or d2 == 0:
        raise ValueError("a denominator must not be 0")
    k = _POSITIONS.index(position)
    # cross products: n1*d2 = n2*d1
    if k == 0:
        x = n2 * d1 / d2
    elif k == 1:
        if n2 == 0:
            raise ValueError("no solution: the unknown denominator would have to make a nonzero ratio equal to 0")
        x = n1 * d2 / n2
    elif k == 2:
        x = n1 * d2 / d1
    else:
        if n1 == 0:
            raise ValueError("no solution: the unknown denominator would have to make a nonzero ratio equal to 0")
        x = n2 * d1 / n1
    if k in (1, 3) and x == 0:
        raise ValueError("no solution (the unknown denominator would be 0)")
    return {"x": x}


def _verify_prop(i, o):
    t = _prop_terms(i["p"], i["q"], i["r"], i["position"])
    t[t.index(None)] = o["x"]
    n1, d1, n2, d2 = t
    return abs(n1 * d2 - n2 * d1) / max(1.0, abs(n1 * d2), abs(n2 * d1))


def _ref_prop(p, q, r, position):
    import sympy as sp
    X = sp.Symbol("X")
    t = [sp.Rational(repr(float(v))) if v is not None else X for v in _prop_terms(p, q, r, position)]
    return {"x": float(sp.solve(sp.Eq(t[0] / t[1], t[2] / t[3]), X)[0])}


def _wrong_prop(p, q, r, position):
    return {"x": p * q / r if r else p * q}


def _rand_prop(rng):
    nz = lambda: rng.choice([v for v in range(-30, 31) if v != 0]) if rng.random() < 0.7 else round(rng.uniform(0.5, 50), 2)
    return {"p": nz(), "q": nz(), "r": nz(), "position": rng.choice(_POSITIONS)}


def _parse_prop(g):
    terms = [g["n1"], g["d1"], g["n2"], g["d2"]]
    if sum(1 for t in terms if t.lower() == "x") != 1:
        raise ValueError("exactly one unknown")
    k = [t.lower() for t in terms].index("x")
    known = [float(t) for t in terms if t.lower() != "x"]
    return {"p": known[0], "q": known[1], "r": known[2], "position": _POSITIONS[k]}


_PT = r"(" + _W + r"|x)"
_register(
    "proportion_solve", name="Solve a proportion", topic="arithmetic / ratios and proportions",
    purpose="Solve n1/d1 = n2/d2 for the one unknown term x (in any of the four positions).",
    inputs=[_num_in("p", "the first known term (reading n1, d1, n2, d2 in order)"), _num_in("q", "the second known term"),
            _num_in("r", "the third known term"),
            {"symbol": "position", "type": "choice", "choices": _POSITIONS, "kind": "text", "meaning": "where the unknown x stands", "required": True}],
    outputs=[_out("x", "the unknown term")], primary="x", conditions=["denominators not 0", "the proportion has a solution"],
    method="cross-multiplication: n1·d2 = n2·d1, then divide", latex=r"\frac{n_1}{d_1} = \frac{n_2}{d_2} \iff n_1 d_2 = n_2 d_1",
    run=proportion_solve, verify=_verify_prop, reference=_ref_prop, wrong=_wrong_prop, random_inputs=_rand_prop,
    parse=_templated([r"solve(?: the proportion)? (?P<n1>" + _W + r"|x)\s*/\s*(?P<d1>" + _W + r"|x)\s*=\s*(?P<n2>" + _W + r"|x)\s*/\s*(?P<d2>" + _W + r"|x)(?: for x)?" + _E],
                     _parse_prop),
    templates=["Solve 3/4 = x/20.", "Solve the proportion x/6 = 10/15.", "Solve 5/x = 2/7."],
    examples=[{"inputs": {"p": 3, "q": 4, "r": 20, "position": "second numerator"}, "outputs": {"x": 15.0}},
              {"inputs": {"p": 6, "q": 10, "r": 15, "position": "first numerator"}, "outputs": {"x": 4.0}},
              {"inputs": {"p": 5, "q": 2, "r": 7, "position": "first denominator"}, "outputs": {"x": 17.5}}],
    questions=[{"q": "Solve 3/4 = x/20.", "outputs": {"x": 15.0}}, {"q": "Solve the proportion x/6 = 10/15.", "outputs": {"x": 4.0}},
               {"q": "Solve 5/x = 2/7.", "outputs": {"x": 17.5}}, {"q": "Solve 3/0 = x/5.", "outcome": "INVALID_INPUT"}])


def unit_rate_compare(n1, c1, n2=None, c2=None) -> dict:
    _finite(n1=n1, c1=c1)
    if n1 <= 0 or c1 < 0:
        raise ValueError("quantities must be positive and amounts not negative")
    out = {"unit_rate_1": c1 / n1}
    if n2 is None and c2 is None:
        return out
    if n2 is None or c2 is None:
        raise ValueError("the second offer needs both its quantity and its amount")
    _finite(n2=n2, c2=c2)
    if n2 <= 0 or c2 < 0:
        raise ValueError("quantities must be positive and amounts not negative")
    out["unit_rate_2"] = c2 / n2
    a, b = Fraction(repr(float(c1))) / Fraction(repr(float(n1))), Fraction(repr(float(c2))) / Fraction(repr(float(n2)))
    out["better"] = "first" if a < b else ("second" if b < a else "equal")
    return out


def _verify_unit(i, o):
    r = abs(o["unit_rate_1"] * i["n1"] - i["c1"]) / max(1.0, abs(i["c1"]))
    if i.get("n2") is not None:
        if "unit_rate_2" not in o:
            return 1.0
        r += abs(o["unit_rate_2"] * i["n2"] - i["c2"]) / max(1.0, abs(i["c2"]))
        cmp = Fraction(repr(float(i["c1"]))) * Fraction(repr(float(i["n2"]))) - Fraction(repr(float(i["c2"]))) * Fraction(repr(float(i["n1"])))
        r += o.get("better") != ("first" if cmp < 0 else ("second" if cmp > 0 else "equal"))
    return r


def _ref_unit(n1, c1, n2=None, c2=None):
    import sympy as sp
    u = sp.Symbol("u")
    out = {"unit_rate_1": float(sp.solve(sp.Eq(u * sp.Rational(repr(float(n1))), sp.Rational(repr(float(c1)))), u)[0])}
    if n2 is not None:
        out["unit_rate_2"] = float(sp.solve(sp.Eq(u * sp.Rational(repr(float(n2))), sp.Rational(repr(float(c2)))), u)[0])
    return out


def _wrong_unit(n1, c1, n2=None, c2=None):
    out = {"unit_rate_1": n1 / c1 if c1 else 0.0}
    if n2 is not None:
        out["unit_rate_2"] = n2 / c2 if c2 else 0.0
    return out


def _rand_unit(rng):
    x = {"n1": rng.choice([rng.randint(1, 50), round(rng.uniform(0.5, 30), 1)]), "c1": round(rng.uniform(0.5, 200), 2)}
    if rng.random() < 0.7:
        x.update(n2=rng.choice([rng.randint(1, 50), round(rng.uniform(0.5, 30), 1)]), c2=round(rng.uniform(0.5, 200), 2))
    return x


_UNITW = r"(?: ?(?:items?|units?|ounces?|oz|grams?|g|kg|pounds?|lb|liters?|litres?|l|ml|cans?|packs?|bottles?|boxes|eggs|apples|pens|pencils|miles|km|kilometres|kilometers|hours?))?"
_register(
    "unit_rate_compare", name="Unit rates and the better buy", topic="arithmetic / ratios and proportions",
    purpose="The unit rate (amount per one unit) of an offer, and of a second offer with which is cheaper per unit.",
    inputs=[_num_in("n1", "quantity of the first offer (positive)"), _num_in("c1", "amount (price, distance, ...) of the first offer"),
            _num_in("n2", "quantity of the second offer (optional)", required=False), _num_in("c2", "amount of the second offer (optional)", required=False)],
    outputs=[_out("unit_rate_1", "amount per unit, first offer"), _out("unit_rate_2", "amount per unit, second offer"),
             _out("better", "the offer with the lower amount per unit: first, second or equal", "text")],
    primary="unit_rate_1", conditions=["quantities positive", "amounts not negative"],
    method="unit rate = amount / quantity; the lower unit price is the better buy", latex=r"\text{unit rate} = \frac{\text{amount}}{\text{quantity}}",
    run=unit_rate_compare, verify=_verify_unit, reference=_ref_unit, wrong=_wrong_unit, random_inputs=_rand_unit,
    parse=_templated([r"which is (?:the )?better (?:buy|deal)[:,]? (?P<n1>" + _W + r")" + _UNITW + r" for \$?(?P<c1>" + _W + r") or (?P<n2>" + _W + r")" + _UNITW + r" for \$?(?P<c2>" + _W + r")" + _E,
                      r"(?:find|what is) the unit (?:rate|price) (?:of|for) (?P<n1>" + _W + r")" + _UNITW + r" (?:for|costing|at) \$?(?P<c1>" + _W + r")" + _E],
                     lambda g: {k: float(v) for k, v in g.items() if v is not None}),
    templates=["Which is the better buy: 12 items for 30 or 20 items for 45?", "Find the unit price of 6 cans for 4.5."],
    examples=[{"inputs": {"n1": 12, "c1": 30, "n2": 20, "c2": 45}, "outputs": {"unit_rate_1": 2.5, "unit_rate_2": 2.25, "better": "second"}},
              {"inputs": {"n1": 6, "c1": 4.5}, "outputs": {"unit_rate_1": 0.75}}],
    questions=[{"q": "Which is the better buy: 12 items for 30 or 20 items for 45?", "outputs": {"unit_rate_1": 2.5, "unit_rate_2": 2.25, "better": "second"}},
               {"q": "Find the unit price of 6 cans for 4.5.", "outputs": {"unit_rate_1": 0.75}},
               {"q": "Which is the better deal: 3 kg for $7.50 or 5 kg for $12?", "outputs": {"unit_rate_1": 2.5, "unit_rate_2": 2.4, "better": "second"}},
               {"q": "Find the unit price of 0 items for 5.", "outcome": "INVALID_INPUT"}])


# --- 9-11. rounding, significant figures and scientific notation (exact decimal arithmetic) ------------------------
def _dec(x) -> Fraction:
    """The decimal number as written (its shortest repr), exactly."""
    return Fraction(repr(float(x)))


def _round_half_up(q: Fraction, n: int) -> Fraction:
    """Round the exact decimal q to n decimal places (n may be negative: tens, hundreds), halves away from zero."""
    scale = Fraction(10) ** n
    m = abs(q) * scale
    k = math.floor(m + Fraction(1, 2))
    return (k if q >= 0 else -k) / scale


_PLACES = {"whole number": 0, "integer": 0, "unit": 0, "one": 0, "ones": 0, "tenth": 1, "hundredth": 2, "thousandth": 3,
           "ten-thousandth": 4, "ten thousandth": 4, "ten": -1, "hundred": -2, "thousand": -3}


def round_decimal_places(x, n) -> dict:
    _finite(x=x)
    n = int(n)
    if not -9 <= n <= 12:
        raise ValueError("the number of decimal places must be between -9 and 12")
    return {"rounded": float(_round_half_up(_dec(x), n))}


def _verify_round_dp(i, o):
    q, n = _dec(i["x"]), int(i["n"])
    r = _dec(o["rounded"])
    unit = Fraction(1, 10 ** n) if n >= 0 else Fraction(10 ** -n)
    if (r / unit).denominator != 1:
        return 1.0 + float(abs(r - q))
    gap = abs(r - q)
    if gap > unit / 2 or (gap == unit / 2 and abs(r) < abs(q)):
        return 1.0 + float(gap)
    return 0.0


def _ref_round_dp(x, n):
    d = _Dec(repr(float(x)))
    return {"rounded": float(d.quantize(_Dec(1).scaleb(-int(n)), rounding=_HALF_UP))}


def _sig_exponent(q: Fraction) -> int:
    """e with 10^e <= |q| < 10^(e + 1), exactly."""
    a = abs(q)
    e = len(str(a.numerator)) - len(str(a.denominator))
    while Fraction(10) ** e > a:
        e -= 1
    while Fraction(10) ** (e + 1) <= a:
        e += 1
    return e


def round_significant_figures(x, n) -> dict:
    _finite(x=x)
    n = int(n)
    if x == 0:
        raise ValueError("0 has no significant figures to round to")
    if not 1 <= n <= 15:
        raise ValueError("the number of significant figures must be between 1 and 15")
    q = _dec(x)
    return {"rounded": float(_round_half_up(q, n - 1 - _sig_exponent(q)))}


def _verify_round_sf(i, o):
    q, n = _dec(i["x"]), int(i["n"])
    r = _dec(o["rounded"])
    places = n - 1 - _sig_exponent(q)
    unit = Fraction(10) ** (-places)
    if (r / unit).denominator != 1:
        return 1.0 + float(abs(r - q))
    gap = abs(r - q)
    if gap > unit / 2 or (gap == unit / 2 and abs(r) < abs(q)):
        return 1.0 + float(gap)
    return 0.0


def _ref_round_sf(x, n):
    d = _Dec(repr(float(x)))
    return {"rounded": float(d.quantize(_Dec(1).scaleb(d.adjusted() - int(n) + 1), rounding=_HALF_UP))}


def _rand_round(rng, sig):
    x = round(rng.uniform(-5000, 5000) * rng.choice([1, 1e-3, 1e-1]), rng.randint(2, 7))
    if x == 0:
        x = 0.125
    if rng.random() < 0.2:                       # an exact half, to exercise the tie rule
        x = rng.randint(-999, 999) + 0.5 if not sig else rng.choice([2.5, 0.125, 1.235, 46.5, 0.0045])
    return {"x": x, "n": rng.randint(1, 5) if sig else rng.randint(0, 4)}


def _parse_places(g):
    if g.get("n") is not None:
        return {"x": float(g["x"]), "n": int(g["n"])}
    return {"x": float(g["x"]), "n": _PLACES[g["place"].lower()]}


_register(
    "round_decimal_places", name="Round to decimal places", topic="arithmetic / rounding",
    purpose="Round a number to n decimal places (or to the nearest tenth, hundredth, ten, hundred ...), halves away from zero.",
    inputs=[_num_in("x", "the number to round"),
            {"symbol": "n", "type": "integer", "kind": "number", "meaning": "decimal places (negative: tens, hundreds)", "required": True, "min": -9, "max": 12}],
    outputs=[_out("rounded", "the rounded number")], primary="rounded", conditions=["-9 <= n <= 12"],
    method="exact decimal arithmetic on the number as written; a 5 in the first dropped place rounds away from zero",
    latex=r"\mathrm{round}(x, n) = \operatorname{sgn}(x)\,\frac{\lfloor |x|\,10^{n} + \tfrac12 \rfloor}{10^{n}}",
    run=round_decimal_places, verify=_verify_round_dp, reference=_ref_round_dp,
    wrong=lambda x, n: round_significant_figures(x, max(1, int(n))) if x else {"rounded": 1.0},
    random_inputs=lambda rng: _rand_round(rng, False),
    parse=_templated([r"round (?P<x>" + _W + r") to (?P<n>\d+) decimal places?" + _E,
                      r"round (?P<x>" + _W + r") to the nearest (?P<place>whole number|integer|tenth|hundredth|thousandth|ten-thousandth|ten thousandth|ten|hundred|thousand)" + _E],
                     _parse_places),
    templates=["Round 3.14159 to 2 decimal places.", "Round 2.675 to the nearest hundredth.", "Round 1234.5 to the nearest hundred."],
    examples=[{"inputs": {"x": 3.14159, "n": 2}, "outputs": {"rounded": 3.14}}, {"inputs": {"x": 2.675, "n": 2}, "outputs": {"rounded": 2.68}},
              {"inputs": {"x": 1234.5, "n": -2}, "outputs": {"rounded": 1200.0}}, {"inputs": {"x": -0.5, "n": 0}, "outputs": {"rounded": -1.0}}],
    questions=[{"q": "Round 3.14159 to 2 decimal places.", "outputs": {"rounded": 3.14}},
               {"q": "Round 2.675 to the nearest hundredth.", "outputs": {"rounded": 2.68}},
               {"q": "Round 1234.5 to the nearest hundred.", "outputs": {"rounded": 1200.0}},
               {"q": "Round 7.45 to the nearest whole number.", "outputs": {"rounded": 7.0}}])

_register(
    "round_significant_figures", name="Round to significant figures", topic="arithmetic / rounding",
    purpose="Round a nonzero number to n significant figures, halves away from zero.",
    inputs=[_num_in("x", "the number to round (not 0)", "x != 0"),
            {"symbol": "n", "type": "integer", "kind": "number", "meaning": "number of significant figures", "required": True, "min": 1, "max": 15}],
    outputs=[_out("rounded", "the rounded number")], primary="rounded", conditions=["x != 0", "1 <= n <= 15"],
    method="find the exponent e of the leading digit (10^e <= |x| < 10^(e+1)), round to 10^(e-n+1) exactly",
    latex=r"\mathrm{round}\big(x,\ n - 1 - \lfloor \log_{10}|x| \rfloor\big)",
    run=round_significant_figures, verify=_verify_round_sf, reference=_ref_round_sf,
    wrong=lambda x, n: round_decimal_places(x, int(n)),
    random_inputs=lambda rng: _rand_round(rng, True),
    parse=_templated([r"(?:round|write|express) (?P<x>" + _W + r") (?:to|correct to|with) (?P<n>\d+) significant (?:figures?|digits?)" + _E,
                      r"round (?P<x>" + _W + r") to (?P<n>\d+) sig(?:\.|nificant)? ?figs?" + _E],
                     lambda g: {"x": float(g["x"]), "n": int(g["n"])}),
    templates=["Round 0.0045678 to 3 significant figures.", "Write 45678 to 2 significant figures."],
    examples=[{"inputs": {"x": 0.0045678, "n": 3}, "outputs": {"rounded": 0.00457}}, {"inputs": {"x": 45678, "n": 2}, "outputs": {"rounded": 46000.0}},
              {"inputs": {"x": 9.996, "n": 3}, "outputs": {"rounded": 10.0}}],
    questions=[{"q": "Round 0.0045678 to 3 significant figures.", "outputs": {"rounded": 0.00457}},
               {"q": "Write 45678 to 2 significant figures.", "outputs": {"rounded": 46000.0}},
               {"q": "Round 0 to 2 significant figures.", "outcome": "INVALID_INPUT"}])


def scientific_notation(x) -> dict:
    _finite(x=x)
    if x == 0:
        raise ValueError("0 has no scientific notation with a leading digit from 1 to 9")
    q = _dec(x)
    e = _sig_exponent(q)
    return {"mantissa": float(q / Fraction(10) ** e), "exponent": float(e)}


def _verify_sci(i, o):
    m, e = o["mantissa"], o["exponent"]
    if e != int(e) or not (1 <= abs(m) < 10):
        return 1.0
    return float(abs(_dec(m) * Fraction(10) ** int(e) - _dec(i["x"])) / max(Fraction(1), abs(_dec(i["x"]))))


def _ref_sci(x):
    d = _Dec(repr(float(x)))
    e = d.adjusted()
    return {"mantissa": float(d.scaleb(-e)), "exponent": float(e)}


_register(
    "scientific_notation", name="Scientific notation", topic="arithmetic / scientific notation",
    purpose="Write a nonzero number as m × 10^e with 1 <= |m| < 10.",
    inputs=[_num_in("x", "the number (not 0)", "x != 0")],
    outputs=[_out("mantissa", "m, with 1 <= |m| < 10"), _out("exponent", "the integer power of ten e")],
    primary="mantissa", conditions=["x != 0"], method="exact decimal: e = position of the leading digit, m = x / 10^e",
    latex=r"x = m \times 10^{e},\quad 1 \le |m| < 10", run=scientific_notation, verify=_verify_sci, reference=_ref_sci,
    wrong=lambda x: (lambda r: {"mantissa": r["mantissa"] / 10, "exponent": r["exponent"] + 1})(scientific_notation(x)),
    random_inputs=lambda rng: {"x": round(rng.uniform(-9999, 9999), rng.randint(0, 4)) * rng.choice([1, 1e-6, 1e-3, 1e3, 1e8]) or 7.0},
    parse=_templated([r"(?:write|express|convert|put) (?P<x>" + _W + r") (?:in|into|to) scientific notation" + _E,
                      r"what is (?P<x>" + _W + r") in scientific notation" + _E],
                     lambda g: {"x": float(g["x"])}),
    templates=["Write 45600 in scientific notation.", "Express 0.00072 in scientific notation."],
    examples=[{"inputs": {"x": 45600}, "outputs": {"mantissa": 4.56, "exponent": 4.0}}, {"inputs": {"x": 0.00072}, "outputs": {"mantissa": 7.2, "exponent": -4.0}},
              {"inputs": {"x": -3.0}, "outputs": {"mantissa": -3.0, "exponent": 0.0}}],
    questions=[{"q": "Write 45600 in scientific notation.", "outputs": {"mantissa": 4.56, "exponent": 4.0}},
               {"q": "Express 0.00072 in scientific notation.", "outputs": {"mantissa": 7.2, "exponent": -4.0}},
               {"q": "Write 0 in scientific notation.", "outcome": "INVALID_INPUT"}])


# --- 12. order of operations ---------------------------------------------------------------------------------------
_ARITH = re.compile(r"^[\d\s.+\-*/^()\[\]]+$")


def order_of_operations(expression) -> dict:
    if not isinstance(expression, str) or not _ARITH.match(_clean(expression).replace(" divided by ", "/")):
        raise ValueError("only numbers, + - * / ^ and parentheses are allowed")
    return _exact_outputs(_eval_ast(expression))


def _verify_ooo(i, o):
    v = _eval_pratt(i["expression"])
    r = _rel(o["value"], float(v))
    if isinstance(v, Fraction):
        if o.get("numerator") is None or o["denominator"] <= 0:
            return 1.0
        r += abs(Fraction(int(o["numerator"]), int(o["denominator"])) - v) / max(1, abs(v))
    return float(r)


def _ref_ooo(expression):
    import sympy as sp
    v = sp.nsimplify(_sym_expr(expression))
    if v.is_Rational:
        return {"value": float(v), "numerator": float(v.p), "denominator": float(v.q)}
    return {"value": float(v)}


def _wrong_ooo(expression):
    try:
        return _exact_outputs(_eval_pratt(expression, flat=True))
    except ValueError:
        return {"value": 0.123456789}


def _rand_ooo(rng):
    a, b, c, d, e, f = (rng.randint(1, 12) for _ in range(6))
    shapes = [f"{a} + {b} * ({c} - {d})^2 / {f}", f"{a} - {b} * {c} + {d} / {e}", f"({a} + {b}) * {c} - {d}^2",
              f"{a} * {b} - {c} / ({d} + {e})", f"{a}^2 - {b} * {c} + {d}", f"{a} + {b}^2 * {c} / {e} - {f}",
              f"-{a}^2 + {b} * ({c} - {d})", f"{a} / {b} / {c} + {d} * {e}", f"{a} - ({b} - {c}) * {d}^2 / {e}",
              f"{a}.5 * {b} + {c}^3 / {d}"]
    return {"expression": rng.choice(shapes)}


_register(
    "order_of_operations", name="Evaluate an arithmetic expression (order of operations)", topic="arithmetic / order of operations",
    purpose="The exact value of an arithmetic expression with + - * / ^ and parentheses, following the order of operations.",
    inputs=[{"symbol": "expression", "type": "text", "kind": "text", "meaning": "the expression, e.g. 3 + 4 * (2 - 5)^2 / 6", "required": True,
             "min_length": 1, "max_length": 200}],
    outputs=[_out("value", "the value"), _out("numerator", "numerator of the exact value in lowest terms"),
             _out("denominator", "denominator of the exact value")],
    primary="value", conditions=["numbers, + - * / ^ and parentheses only", "no division by 0", "integer exponents up to 64"],
    method="parentheses first, then powers (right to left), then multiplication and division, then addition and subtraction "
           "(left to right); exact rational arithmetic",
    latex=r"\text{(  )} \;\to\; a^{b} \;\to\; \times\,\div \;\to\; +\,-",
    run=order_of_operations, verify=_verify_ooo, reference=_ref_ooo, wrong=_wrong_ooo, random_inputs=_rand_ooo,
    parse=_templated([r"(?:evaluate|use the order of operations to (?:evaluate|simplify)|simplify the expression) (?P<e>[\d\s.+\-*/^()\[\]]+?)" + _E],
                     lambda g: {"expression": g["e"].strip()} if re.search(r"[-+*/^]", g["e"].strip()[1:]) else None),
    templates=["Evaluate 3 + 4 * (2 - 5)^2 / 6.", "Use the order of operations to evaluate 8 - 2 * 3^2 + 10 / 5."],
    examples=[{"inputs": {"expression": "3 + 4 * (2 - 5)^2 / 6"}, "outputs": {"value": 9.0}},
              {"inputs": {"expression": "8 - 2 * 3^2 + 10 / 5"}, "outputs": {"value": -8.0}},
              {"inputs": {"expression": "2^3^2"}, "outputs": {"value": 512.0}},
              {"inputs": {"expression": "1/3 + 1/6"}, "outputs": {"numerator": 1.0, "denominator": 2.0}}],
    questions=[{"q": "Evaluate 3 + 4 * (2 - 5)^2 / 6.", "outputs": {"value": 9.0}},
               {"q": "Use the order of operations to evaluate 8 - 2 * 3^2 + 10 / 5.", "outputs": {"value": -8.0}},
               {"q": "Evaluate -2^2 + 3 * (4 - 1).", "outputs": {"value": 5.0}},
               {"q": "Evaluate 6 / (3 - 3) + 1.", "outcome": "INVALID_INPUT"}])


# =====================================================================================================================
# more algebra: systems, absolute value, rational and radical equations, polynomial division, factoring, series
# =====================================================================================================================
def _poly_text(expr: str, var: str = "x", maxdeg: int = 10):
    """Coefficients, highest power first, of a polynomial written term by term ('2x^3 - x + 5'); None otherwise."""
    co = _poly_side(_clean(expr).lower(), var, maxdeg)
    if co is None:
        return None
    while len(co) > 1 and co[-1] == 0:
        co.pop()
    return list(reversed(co))


def _fr(v) -> Fraction:
    return Fraction(repr(float(v)))


def _lin_side(side: str, names: str):
    """{'x': 2, 'y': -1, '': 5} for '2x - y + 5' (names: the allowed letters); None if it is not linear in them."""
    s = _clean(side).lower().replace(" ", "").replace("*", "")
    if not s:
        return None
    if s[0] not in "+-":
        s = "+" + s
    terms = re.findall(r"[+-][^+-]+", s)
    if "".join(terms) != s:
        return None
    out = {}
    for t in terms:
        m = re.fullmatch(r"([+-])(\d+(?:\.\d+)?(?:/\d+)?)?([" + names + r"])?", t)
        if not m or (m.group(2) is None and m.group(3) is None):
            return None
        v = Fraction(m.group(2)) if m.group(2) else Fraction(1)
        v = -v if m.group(1) == "-" else v
        k = m.group(3) or ""
        out[k] = out.get(k, Fraction(0)) + v
    return out


def _lin_equation(eq: str, names: str):
    if eq.count("=") != 1:
        return None
    l, r = (_lin_side(p, names) for p in eq.split("="))
    if l is None or r is None:
        return None
    row = [float(l.get(n, 0) - r.get(n, 0)) for n in names] + [float(r.get("", 0) - l.get("", 0))]
    return row


# --- linear system 3x3 --------------------------------------------------------------------------------------------
def _gauss(rows):
    """Exact Gaussian elimination on rational entries; ValueError when there is no unique solution."""
    n = len(rows)
    m = [[_fr(v) for v in r] for r in rows]
    for col in range(n):
        piv = next((r for r in range(col, n) if m[r][col] != 0), None)
        if piv is None:
            raise ValueError("the system has no unique solution (its determinant is 0)")
        m[col], m[piv] = m[piv], m[col]
        for r in range(n):
            if r != col and m[r][col] != 0:
                f = m[r][col] / m[col][col]
                m[r] = [a - f * b for a, b in zip(m[r], m[col])]
    return [m[i][n] / m[i][i] for i in range(n)]


def linear_system_3x3(augmented) -> dict:
    if len(augmented) != 3 or any(len(r) != 4 for r in augmented):
        raise ValueError("the augmented matrix must have 3 rows of 4 numbers (a, b, c | d)")
    x, y, z = _gauss(augmented)
    return {"x": float(x), "y": float(y), "z": float(z)}


def _verify_sys3(i, o):
    worst = 0.0
    for r in i["augmented"]:
        terms = [r[0] * o["x"], r[1] * o["y"], r[2] * o["z"]]
        worst = max(worst, abs(sum(terms) - r[3]) / max(1.0, sum(abs(t) for t in terms) + abs(r[3])))
    return worst


def _ref_sys3(augmented):
    import sympy as sp                                   # LU decomposition in exact rational arithmetic
    R = lambda v: sp.Rational(repr(float(v)))
    A = sp.Matrix([[R(v) for v in r[:3]] for r in augmented])
    x = A.LUsolve(sp.Matrix([R(r[3]) for r in augmented]))
    return {"x": float(x[0]), "y": float(x[1]), "z": float(x[2])}


def _wrong_sys3(augmented):
    t = [[augmented[j][i] for j in range(3)] + [augmented[i][3]] for i in range(3)]      # coefficients read by column
    try:
        x, y, z = _gauss(t)
        return {"x": float(x), "y": float(y), "z": float(z)}
    except ValueError:
        return {"x": 0.0, "y": 0.0, "z": 0.0}


def _rand_sys3(rng):
    sol = [rng.randint(-6, 6) for _ in range(3)]
    rows = []
    for _ in range(3):
        a = [rng.randint(-9, 9) for _ in range(3)]
        rows.append(a + [sum(p * q for p, q in zip(a, sol)) + rng.choice([0, 0, rng.randint(-3, 3)])])
    return {"augmented": rows}


def _parse_sys3(g):
    eqs = [e.strip() for e in re.split(r"\s*(?:,|;| and )\s*", g["eqs"]) if e.strip()]
    if len(eqs) != 3:
        raise ValueError("three equations")
    rows = [_lin_equation(e, "xyz") for e in eqs]
    if any(r is None for r in rows):
        raise ValueError("not linear in x, y, z")
    return {"augmented": rows}


_register(
    "linear_system_3x3", name="System of three linear equations", topic="algebra / systems of linear equations",
    purpose="Solve three linear equations in x, y and z (a unique solution, or a refusal when there is none).",
    inputs=[{"symbol": "augmented", "type": "matrix", "kind": "number", "required": True,
             "meaning": "augmented matrix, one row a b c d per equation a*x + b*y + c*z = d"}],
    outputs=[_out("x", "x"), _out("y", "y"), _out("z", "z")], primary="x",
    conditions=["determinant of the coefficients != 0 (a unique solution)"],
    method="Gaussian elimination in exact rational arithmetic", latex=r"A\,\mathbf{x} = \mathbf{d},\quad \mathbf{x} = A^{-1}\mathbf{d}",
    run=linear_system_3x3, verify=_verify_sys3, reference=_ref_sys3, wrong=_wrong_sys3, random_inputs=_rand_sys3,
    parse=_templated([r"solve the system(?: of equations)?:? (?P<eqs>[^:]+?)(?: for x, y,? and z)?" + _E], _parse_sys3),
    templates=["Solve the system x + y + z = 6, 2x - y + z = 3, x + 2y - z = 2.", "Solve the system 2x + y - z = 8; -3x - y + 2z = -11; -2x + y + 2z = -3."],
    examples=[{"inputs": {"augmented": "1 1 1 6; 2 -1 1 3; 1 2 -1 2"}, "outputs": {"x": 1.0, "y": 2.0, "z": 3.0}},
              {"inputs": {"augmented": [[2, 1, -1, 8], [-3, -1, 2, -11], [-2, 1, 2, -3]]}, "outputs": {"x": 2.0, "y": 3.0, "z": -1.0}}],
    questions=[{"q": "Solve the system x + y + z = 6, 2x - y + z = 3, x + 2y - z = 2.", "outputs": {"x": 1.0, "y": 2.0, "z": 3.0}},
               {"q": "Solve the system 2x + y - z = 8; -3x - y + 2z = -11; -2x + y + 2z = -3.", "outputs": {"x": 2.0, "y": 3.0, "z": -1.0}},
               {"q": "Solve the system x + y + z = 1, 2x + 2y + 2z = 2, x - y = 0.", "outcome": "INVALID_INPUT"}])


# --- |a x + b| compared with c ------------------------------------------------------------------------------------
_RELS = ["<", "<=", ">", ">="]


def _ineq_text(lo, hi, rel, between):
    f = lambda v: f"{v:.10g}"
    if between:
        r = "<" if rel in ("<", ">") else "<="
        return f"{f(lo)} {r} x {r} {f(hi)}"
    lt, gt = ("<", ">") if rel in ("<", ">") else ("<=", ">=")
    return f"x {lt} {f(lo)} or x {gt} {f(hi)}"


def abs_linear_inequality(a, b, c, rel) -> dict:
    _finite(a=a, b=b, c=c)
    if a == 0:
        raise ValueError("a must not be 0 (no x inside the absolute value)")
    if c <= 0:
        raise ValueError("c must be positive (with c <= 0 the solution set is empty, a point or every x)")
    lo, hi = sorted([(-c - b) / a, (c - b) / a])
    between = rel in ("<", "<=")
    return {"lower": lo, "upper": hi, "region": "between" if between else "outside", "solution": _ineq_text(lo, hi, rel, between)}


def _verify_absineq(i, o):
    a, b, c = i["a"], i["b"], i["c"]
    lo, hi = o["lower"], o["upper"]
    r = abs(abs(a * lo + b) - c) / max(1.0, c) + abs(abs(a * hi + b) - c) / max(1.0, c)
    r += abs((a * lo + b) + (a * hi + b)) / max(1.0, c)                # the two boundaries are +c and -c
    r += (lo > hi) + (o["region"] != ("between" if i["rel"] in ("<", "<=") else "outside"))
    return r


def _ref_absineq(a, b, c, rel):
    import sympy as sp
    x = sp.Symbol("x", real=True)
    A, B, C = (sp.Rational(repr(float(v))) for v in (a, b, c))
    ex = {"<": sp.Lt, "<=": sp.Le, ">": sp.Gt, ">=": sp.Ge}[rel](sp.Abs(A * x + B), C)
    s = sp.solve_univariate_inequality(ex, x, relational=False)
    if isinstance(s, sp.Interval):
        return {"lower": float(s.inf), "upper": float(s.sup)}
    left, right = sorted(s.args, key=lambda I: float(I.inf) if I.inf.is_finite else -1e300)
    return {"lower": float(left.sup), "upper": float(right.inf)}


def _wrong_absineq(a, b, c, rel):
    lo, hi = sorted([(c - b) / a, (-c + b) / a])
    return {"lower": lo, "upper": hi, "region": "between" if rel in ("<", "<=") else "outside"}


def _parse_absineq(g):
    co = _poly_text(g["lin"], "x", 1)
    if co is None or len(co) != 2:
        raise ValueError("linear inside the bars")
    return {"a": co[0], "b": co[1], "c": float(Fraction(g["c"])), "rel": g["rel"]}


_register(
    "abs_linear_inequality", name="Absolute value inequality |ax + b| compared with c", topic="algebra / absolute value",
    purpose="Solve |a*x + b| < c, <= c, > c or >= c (c > 0): the two boundary points and whether x lies between or outside them.",
    inputs=[_num_in("a", "coefficient of x inside the bars", "a != 0"), _num_in("b", "constant inside the bars"),
            _num_in("c", "the bound (positive)"),
            {"symbol": "rel", "type": "choice", "choices": _RELS, "kind": "text", "meaning": "the comparison", "required": True}],
    outputs=[_out("lower", "the smaller boundary point"), _out("upper", "the larger boundary point"),
             _out("region", "between (for < and <=) or outside (for > and >=)", "text"), _out("solution", "the solution set", "text")],
    primary="lower", conditions=["a != 0", "c > 0"],
    method="|u| < c means -c < u < c; |u| > c means u < -c or u > c; solve each linear piece for x",
    latex=r"|ax+b| < c \iff -c < ax+b < c,\qquad |ax+b| > c \iff ax+b < -c \ \text{or}\ ax+b > c",
    run=abs_linear_inequality, verify=_verify_absineq, reference=_ref_absineq, wrong=_wrong_absineq,
    random_inputs=lambda rng: {"a": rng.choice([v for v in range(-9, 10) if v != 0]), "b": rng.choice([v for v in range(-20, 21) if v != 0]),
                               "c": rng.choice([rng.randint(1, 25), round(rng.uniform(0.5, 30), 2)]), "rel": rng.choice(_RELS)},
    parse=_templated([r"solve(?: the inequality)? \|(?P<lin>[^|]+)\|\s*(?P<rel><=|>=|<|>)\s*(?P<c>" + _Q + r")" + _E,
                      r"for which x is \|(?P<lin>[^|]+)\|\s*(?P<rel><=|>=|<|>)\s*(?P<c>" + _Q + r")" + _E],
                     _parse_absineq),
    templates=["Solve |2x + 3| < 7.", "Solve the inequality |3x - 1| >= 5."],
    examples=[{"inputs": {"a": 2, "b": 3, "c": 7, "rel": "<"}, "outputs": {"lower": -5.0, "upper": 2.0, "region": "between"}},
              {"inputs": {"a": 3, "b": -1, "c": 5, "rel": ">="}, "outputs": {"lower": -4 / 3, "upper": 2.0, "region": "outside"}}],
    questions=[{"q": "Solve |2x + 3| < 7.", "outputs": {"lower": -5.0, "upper": 2.0, "region": "between"}},
               {"q": "Solve the inequality |3x - 1| >= 5.", "outputs": {"lower": -4 / 3, "upper": 2.0, "region": "outside"}},
               {"q": "Solve |x + 4| <= 0.", "outcome": "INVALID_INPUT"}])


# --- rational equation a/(x + b) = c/(x + d) ----------------------------------------------------------------------
def rational_equation(a, b, c, d) -> dict:
    _finite(a=a, b=b, c=c, d=d)
    A, B, C, D = (_fr(v) for v in (a, b, c, d))
    if A == C:
        raise ValueError("no unique solution (equal numerators: no solution or the same expression on both sides)")
    x = (C * B - A * D) / (A - C)
    if x == -B or x == -D:
        raise ValueError(f"no solution: the only candidate x = {float(x):g} makes a denominator 0 (extraneous)")
    return {"x": float(x)}


def _verify_rateq(i, o):
    x = o["x"]
    a, b, c, d = i["a"], i["b"], i["c"], i["d"]
    if x + b == 0 or x + d == 0:
        return 1.0
    return abs(a * (x + d) - c * (x + b)) / max(1.0, abs(a * (x + d)), abs(c * (x + b)))


def _ref_rateq(a, b, c, d):
    import sympy as sp
    x = sp.Symbol("x")
    A, B, C, D = (sp.Rational(repr(float(v))) for v in (a, b, c, d))
    return {"x": float(sp.solve(sp.Eq(A / (x + B), C / (x + D)), x)[0])}


def _parse_rateq(g):
    def den(t):
        co = _poly_text(t, "x", 1)
        if co is None or len(co) != 2 or co[0] != 1:
            raise ValueError("a monic linear denominator")
        return co[1]
    return {"a": float(Fraction(g["a"])), "b": den(g["d1"]), "c": float(Fraction(g["c"])), "d": den(g["d2"])}


_register(
    "rational_equation", name="Rational equation a/(x + b) = c/(x + d)", topic="algebra / rational equations",
    purpose="Solve a/(x + b) = c/(x + d) for x, rejecting a candidate that makes a denominator 0.",
    inputs=[_num_in("a", "numerator on the left"), _num_in("b", "constant in the left denominator x + b"),
            _num_in("c", "numerator on the right"), _num_in("d", "constant in the right denominator x + d")],
    outputs=[_out("x", "the solution")], primary="x", conditions=["a != c", "the candidate is not an excluded value"],
    method="multiply by both denominators: a(x + d) = c(x + b), solve the linear equation, check the excluded values x = -b, x = -d",
    latex=r"\frac{a}{x+b} = \frac{c}{x+d} \;\Rightarrow\; x = \frac{cb - ad}{a - c}",
    run=rational_equation, verify=_verify_rateq, reference=_ref_rateq,
    wrong=lambda a, b, c, d: {"x": (a * d - c * b) / (a - c) if a != c else 0.0},
    random_inputs=lambda rng: {"a": rng.choice([v for v in range(-9, 10) if v]), "b": rng.randint(-9, 9),
                               "c": rng.choice([v for v in range(-9, 10) if v]), "d": rng.randint(-9, 9)},
    parse=_templated([r"solve (?P<a>" + _Q + r")\s*/\s*\((?P<d1>[^()]+)\)\s*=\s*(?P<c>" + _Q + r")\s*/\s*\((?P<d2>[^()]+)\)(?: for x)?" + _E],
                     _parse_rateq),
    templates=["Solve 3/(x + 1) = 2/(x - 4).", "Solve 5/(x - 2) = 3/(x + 6)."],
    examples=[{"inputs": {"a": 3, "b": 1, "c": 2, "d": -4}, "outputs": {"x": 14.0}},
              {"inputs": {"a": 5, "b": -2, "c": 3, "d": 6}, "outputs": {"x": -18.0}}],
    questions=[{"q": "Solve 3/(x + 1) = 2/(x - 4).", "outputs": {"x": 14.0}}, {"q": "Solve 5/(x - 2) = 3/(x + 6).", "outputs": {"x": -18.0}},
               {"q": "Solve 2/(x + 1) = 2/(x + 3).", "outcome": "INVALID_INPUT"},
               {"q": "Solve 0/(x + 1) = 4/(x - 2).", "outcome": "INVALID_INPUT"}])


# --- radical equation sqrt(a x + b) = c x + d ---------------------------------------------------------------------
def _sign_root_expr(p: Fraction, q: Fraction, D: Fraction) -> int:
    """The exact sign of p + q*sqrt(D) (D >= 0)."""
    if q == 0 or D == 0:
        return (p > 0) - (p < 0)
    if p >= 0 and q > 0:
        return 1
    if p <= 0 and q < 0:
        return -1
    big = p * p - q * q * D                            # |p| vs |q| sqrt(D)
    if big == 0:
        return 0
    return ((p > 0) - (p < 0)) if big > 0 else ((q > 0) - (q < 0))


def radical_equation(a, b, c, d) -> dict:
    _finite(a=a, b=b, c=c, d=d)
    A0, B0, C0, D0 = (_fr(v) for v in (a, b, c, d))
    if A0 == 0:
        raise ValueError("a must not be 0 (no x under the root)")
    # square both sides: a x + b = (c x + d)^2  ->  c^2 x^2 + (2cd - a) x + (d^2 - b) = 0
    qa, qb, qc = C0 * C0, 2 * C0 * D0 - A0, D0 * D0 - B0
    cands = []                                          # (value, exact-or-None, (p, q, D, den) for the sign test)
    if qa == 0:
        x = -qc / qb
        cands.append(x)
        valid = [x] if C0 * x + D0 >= 0 else []
        bad = [] if valid else [x]
        sols, ext = [float(v) for v in valid], [float(v) for v in bad]
    else:
        disc = qb * qb - 4 * qa * qc
        sols, ext = [], []
        if disc >= 0:
            n, m = math.isqrt(disc.numerator), math.isqrt(disc.denominator)
            for s in (-1, 1):
                if n * n == disc.numerator and m * m == disc.denominator:
                    x = (-qb + s * Fraction(n, m)) / (2 * qa)
                    ok = C0 * x + D0 >= 0
                    xv = float(x)
                else:
                    # c x + d = (c(-qb + s sqrt(disc)) + 2 qa d) / (2 qa); 2 qa > 0
                    ok = _sign_root_expr(-C0 * qb + 2 * qa * D0, s * C0, disc) >= 0
                    xv = (float(-qb) + s * math.sqrt(float(disc))) / float(2 * qa)
                (sols if ok else ext).append(xv)
            if disc == 0:
                sols, ext = sorted(set(sols)), sorted(set(ext))
    sols, ext = sorted(sols), sorted(ext)
    return {"count": float(len(sols)), "solutions": sols, "extraneous": ext}


def _verify_radical(i, o):
    a, b, c, d = i["a"], i["b"], i["c"], i["d"]
    if o["count"] != len(o["solutions"]):
        return 1.0
    worst = 0.0
    for x in o["solutions"]:
        lhs2 = a * x + b
        if lhs2 < -1e-12 * max(1.0, abs(a * x), abs(b)):
            return 1.0
        lhs = math.sqrt(max(0.0, lhs2))
        worst = max(worst, abs(lhs - (c * x + d)) / max(1.0, abs(lhs), abs(c * x), abs(d)))
    # completeness: every root of the squared equation is either a solution or extraneous
    qa, qb, qc = c * c, 2 * c * d - a, d * d - b
    n_roots = (1 if qb != 0 else 0) if qa == 0 else (lambda D: 0 if D < 0 else (1 if D == 0 else 2))(_fr(qb) ** 2 - 4 * _fr(qa) * _fr(qc))
    if len(o["solutions"]) + len(o.get("extraneous", [])) != n_roots:
        return 1.0
    return worst


def _ref_radical(a, b, c, d):
    import sympy as sp
    x = sp.Symbol("x", real=True)
    A, B, C, D = (sp.Rational(repr(float(v))) for v in (a, b, c, d))
    sols = sorted(float(s) for s in sp.solve(sp.Eq(sp.sqrt(A * x + B), C * x + D), x))
    return {"count": float(len(sols)), "solutions": sols}


def _wrong_radical(a, b, c, d):
    r = radical_equation(a, b, c, d)
    allr = sorted(r["solutions"] + r["extraneous"])
    return {"count": float(len(allr)), "solutions": allr, "extraneous": []}


def _rand_radical(rng):
    if rng.random() < 0.8:          # a valid root x0 and an extraneous root x1 (c*x1 + d < 0) of the squared equation
        while True:
            c, x0, x1 = rng.choice([1, 2, -1, 3, -2]), rng.randint(-6, 8), rng.randint(-6, 8)
            if x0 == x1:
                continue
            d = -c * x0 + rng.randint(0, 3)
            a = 2 * c * d + c * c * (x0 + x1)
            if c * x1 + d < 0 and a != 0:
                return {"a": a, "b": d * d - c * c * x0 * x1, "c": c, "d": d}
    x0 = rng.randint(-6, 8)
    c = rng.choice([0, 1, 1, 2, -1, 3])
    d = rng.randint(-5, 9)
    if c * x0 + d < 0:
        d = -c * x0 + rng.randint(0, 4)
    a = rng.choice([v for v in range(-6, 7) if v])
    b = (c * x0 + d) ** 2 - a * x0
    return {"a": a, "b": b, "c": c, "d": d}


def _parse_radical(g):
    inner, rhs = _poly_text(g["inner"], "x", 1), _poly_text(g["rhs"], "x", 1)
    if inner is None or rhs is None or len(inner) != 2:
        raise ValueError("linear expressions")
    c, d = (rhs if len(rhs) == 2 else [0.0, rhs[0]])
    return {"a": inner[0], "b": inner[1], "c": c, "d": d}


_register(
    "radical_equation", name="Radical equation sqrt(ax + b) = cx + d", topic="algebra / radical equations",
    purpose="Solve sqrt(a*x + b) = c*x + d: square, solve, and keep only the roots that satisfy the original equation.",
    inputs=[_num_in("a", "coefficient of x under the root", "a != 0"), _num_in("b", "constant under the root"),
            _num_in("c", "coefficient of x on the right"), _num_in("d", "constant on the right")],
    outputs=[_out("count", "number of solutions (0, 1 or 2)"), _out("solutions", "the solutions, ascending", "list"),
             _out("extraneous", "roots of the squared equation that fail the original (extraneous)", "list")],
    primary="count", conditions=["a != 0", "real numbers"],
    method="square both sides (a x + b = (c x + d)^2), solve the quadratic exactly, keep a root only if c x + d >= 0",
    latex=r"\sqrt{ax+b} = cx+d \;\Rightarrow\; ax+b = (cx+d)^2,\ cx+d \ge 0",
    run=radical_equation, verify=_verify_radical, reference=_ref_radical, wrong=_wrong_radical, random_inputs=_rand_radical,
    parse=_templated([r"solve sqrt\s*\((?P<inner>[^()]+)\)\s*=\s*(?P<rhs>[^=()]+?)(?: for x)?" + _E], _parse_radical),
    templates=["Solve sqrt(2x + 3) = x.", "Solve sqrt(x + 7) = x - 5."],
    examples=[{"inputs": {"a": 2, "b": 3, "c": 1, "d": 0}, "outputs": {"count": 1.0, "solutions": [3.0], "extraneous": [-1.0]}},
              {"inputs": {"a": 1, "b": 7, "c": 1, "d": -5}, "outputs": {"count": 1.0, "solutions": [9.0], "extraneous": [2.0]}},
              {"inputs": {"a": 1, "b": -1, "c": 1, "d": -1}, "outputs": {"count": 2.0, "solutions": [1.0, 2.0]}}],
    questions=[{"q": "Solve sqrt(2x + 3) = x.", "outputs": {"count": 1.0, "solutions": [3.0]}},
               {"q": "Solve √(x + 7) = x - 5.", "outputs": {"count": 1.0, "solutions": [9.0]}},
               {"q": "Solve sqrt(x - 1) = x - 1.", "outputs": {"count": 2.0, "solutions": [1.0, 2.0]}},
               {"q": "Solve sqrt(x + 4) = -2.", "outputs": {"count": 0.0, "solutions": []}}])


# --- polynomial division ------------------------------------------------------------------------------------------
def _strip(co):
    co = list(co)
    while len(co) > 1 and co[0] == 0:
        co.pop(0)
    return co


def synthetic_division(coefficients, r) -> dict:
    co = _strip([_fr(c) for c in coefficients])
    _finite(r=r)
    if len(co) < 2:
        raise ValueError("the dividend must have degree at least 1")
    R = _fr(r)
    out = [co[0]]
    for c in co[1:]:
        out.append(c + out[-1] * R)
    return {"quotient": [float(v) for v in out[:-1]], "remainder": float(out[-1])}


def _verify_synth(i, o):
    co = _strip([float(c) for c in i["coefficients"]])
    q, rem, r = o["quotient"], o["remainder"], i["r"]
    if len(q) != len(co) - 1:
        return 1.0
    back = [q[0]] + [q[k] - r * q[k - 1] for k in range(1, len(q))] + [rem - r * q[-1]]    # q(x)(x - r) + rem
    scale = max(1.0, max(abs(c) for c in co), max(abs(v) for v in q) * max(1.0, abs(r)))
    return max(abs(u - v) for u, v in zip(back, co)) / scale


def _ref_synth(coefficients, r):
    import numpy as np
    co = _strip([float(c) for c in coefficients])
    q, rem = np.polydiv(np.array(co, dtype=float), np.array([1.0, -float(r)]))
    q = list(q) + [0.0] * (len(co) - 1 - len(q))
    return {"quotient": [float(v) for v in q], "remainder": float(rem[-1]) if len(rem) else 0.0}


def _rand_synth(rng):
    n = rng.randint(2, 6)
    return {"coefficients": [float(rng.choice([v for v in range(-9, 10) if v]))] + [float(rng.randint(-12, 12)) for _ in range(n)],
            "r": float(rng.randint(-5, 5))}


def _divisor_r(t):
    m = re.fullmatch(r"\(?\s*x\s*(?:(?P<s>[-+])\s*(?P<n>" + _W + r"))?\s*\)?", t.strip().lower())
    if not m:
        return None
    if not m.group("s"):
        return 0.0
    return float(m.group("n")) * (1 if m.group("s") == "-" else -1)


def _parse_synth(g):
    co = _poly_text(g["p"], "x", 10)
    r = _divisor_r(g["dv"])
    if co is None or r is None or len(co) < 2:
        raise ValueError("a polynomial and x - r")
    return {"coefficients": co, "r": r}


_POLY = r"[-+0-9a-z.\s/^*²³⁴]+?"
_register(
    "synthetic_division", name="Synthetic division and the remainder theorem", topic="algebra / polynomial division",
    purpose="Divide a polynomial by x - r: the quotient's coefficients and the remainder, which equals p(r).",
    inputs=[{"symbol": "coefficients", "type": "vector", "kind": "number", "required": True, "min_length": 2, "max_length": 11,
             "meaning": "coefficients of the dividend, highest power first (2x^3 - 5x + 1 -> 2, 0, -5, 1)"},
            _num_in("r", "the number r of the divisor x - r")],
    outputs=[_out("quotient", "coefficients of the quotient, highest power first", "list"), _out("remainder", "the remainder = p(r)")],
    primary="remainder", conditions=["dividend of degree >= 1"],
    method="synthetic division (Horner's scheme): bring down, multiply by r, add; the last number is the remainder p(r)",
    latex=r"p(x) = (x - r)\,q(x) + p(r)",
    run=synthetic_division, verify=_verify_synth, reference=_ref_synth,
    wrong=lambda coefficients, r: synthetic_division(coefficients, -r), random_inputs=_rand_synth,
    parse=_templated([r"(?:use synthetic division to )?divide (?P<p>" + _POLY + r") by (?P<dv>\(?x(?:\s*[-+]\s*" + _W + r")?\)?)(?: using synthetic division)?" + _E,
                      r"(?:find|what is) the remainder when (?P<p>" + _POLY + r") is divided by (?P<dv>\(?x(?:\s*[-+]\s*" + _W + r")?\)?)" + _E],
                     _parse_synth),
    templates=["Divide x^3 - 6x^2 + 11x - 6 by x - 1.", "Find the remainder when x^4 - 3x^2 + 2 is divided by x - 3.",
               "Use synthetic division to divide 2x^3 + 3x - 4 by x + 2."],
    examples=[{"inputs": {"coefficients": "1, -6, 11, -6", "r": 1}, "outputs": {"quotient": [1.0, -5.0, 6.0], "remainder": 0.0}},
              {"inputs": {"coefficients": [1, 0, -3, 0, 2], "r": 3}, "outputs": {"quotient": [1.0, 3.0, 6.0, 18.0], "remainder": 56.0}},
              {"inputs": {"coefficients": [2, 0, 3, -4], "r": -2}, "outputs": {"quotient": [2.0, -4.0, 11.0], "remainder": -26.0}}],
    questions=[{"q": "Divide x^3 - 6x^2 + 11x - 6 by x - 1.", "outputs": {"quotient": [1.0, -5.0, 6.0], "remainder": 0.0}},
               {"q": "Find the remainder when x^4 - 3x^2 + 2 is divided by x - 3.", "outputs": {"remainder": 56.0}},
               {"q": "Use synthetic division to divide 2x^3 + 3x - 4 by x + 2.", "outputs": {"quotient": [2.0, -4.0, 11.0], "remainder": -26.0}}])


def polynomial_long_division(dividend, divisor) -> dict:
    p, d = _strip([_fr(c) for c in dividend]), _strip([_fr(c) for c in divisor])
    if d[0] == 0:
        raise ValueError("the divisor must not be the zero polynomial")
    if len(d) < 2:
        raise ValueError("the divisor must have degree at least 1")
    if len(p) < len(d):
        return {"quotient": [0.0], "remainder": [float(v) for v in p]}
    rem, q = list(p), []
    for k in range(len(p) - len(d) + 1):
        f = rem[k] / d[0]
        q.append(f)
        for j in range(len(d)):
            rem[k + j] -= f * d[j]
    r = _strip(rem[len(p) - len(d) + 1:]) or [Fraction(0)]
    return {"quotient": [float(v) for v in q], "remainder": [float(v) for v in r]}


def _verify_longdiv(i, o):
    p, d = _strip([float(c) for c in i["dividend"]]), _strip([float(c) for c in i["divisor"]])
    q, r = o["quotient"], o["remainder"]
    if len(r) >= len(d) and any(r[:len(r) - len(d) + 1]):
        return 1.0
    prod = [0.0] * (len(q) + len(d) - 1)
    for a_i, a in enumerate(q):
        for b_i, b in enumerate(d):
            prod[a_i + b_i] += a * b
    n = max(len(prod), len(r), len(p))
    pad = lambda v: [0.0] * (n - len(v)) + list(v)
    back = [u + v for u, v in zip(pad(prod), pad(r))]
    scale = max(1.0, max(abs(c) for c in p), max(abs(v) for v in prod))
    return max(abs(u - v) for u, v in zip(back, pad(p))) / scale


def _ref_longdiv(dividend, divisor):
    import sympy as sp
    x = sp.Symbol("x")
    P = sp.Poly([sp.Rational(repr(float(c))) for c in dividend], x)
    D = sp.Poly([sp.Rational(repr(float(c))) for c in divisor], x)
    q, r = sp.div(P, D)
    qc = [float(c) for c in q.all_coeffs()] if not q.is_zero else [0.0]
    rc = [float(c) for c in r.all_coeffs()] if not r.is_zero else [0.0]
    return {"quotient": qc, "remainder": rc}


def _wrong_longdiv(dividend, divisor):
    p, d = _strip([_fr(c) for c in dividend]), _strip([_fr(c) for c in divisor])
    if len(p) < len(d):
        return {"quotient": [1.0], "remainder": [0.0]}
    rem, q = list(p), []
    for k in range(len(p) - len(d) + 1):
        f = rem[k] / d[0]
        q.append(f)
        for j in range(1, len(d)):
            rem[k + j] += f * d[j]                     # the sign error: adding instead of subtracting
    r = _strip(rem[len(p) - len(d) + 1:]) or [Fraction(0)]
    return {"quotient": [float(v) for v in q], "remainder": [float(v) for v in r]}


def _rand_longdiv(rng):
    nd = rng.randint(1, 3)
    n = nd + rng.randint(0, 3)
    nz = lambda: float(rng.choice([v for v in range(-6, 7) if v]))
    return {"dividend": [nz()] + [float(rng.randint(-9, 9)) for _ in range(n)],
            "divisor": [nz()] + [float(rng.randint(-5, 5)) for _ in range(nd - 1)] + [nz()]}


def _parse_longdiv(g):
    p, d = _poly_text(g["p"], "x", 10), _poly_text(g["d"], "x", 10)
    if p is None or d is None or len(d) < 2 or len(p) < 2:
        raise ValueError("two polynomials")
    if not g.get("long") and _divisor_r(g["d"]) is not None:
        raise ValueError("x - r: synthetic division reads it")
    return {"dividend": p, "divisor": d}


_register(
    "polynomial_long_division", name="Polynomial long division", topic="algebra / polynomial division",
    purpose="Divide one polynomial by another: quotient and remainder, with p = q*d + r and deg r < deg d.",
    inputs=[{"symbol": "dividend", "type": "vector", "kind": "number", "required": True, "min_length": 1, "max_length": 11,
             "meaning": "coefficients of the dividend, highest power first"},
            {"symbol": "divisor", "type": "vector", "kind": "number", "required": True, "min_length": 2, "max_length": 11,
             "meaning": "coefficients of the divisor, highest power first (degree >= 1)"}],
    outputs=[_out("quotient", "coefficients of the quotient, highest power first", "list"),
             _out("remainder", "coefficients of the remainder, highest power first", "list")],
    primary="quotient", conditions=["divisor of degree >= 1"],
    method="long division: divide leading terms, multiply back, subtract, repeat; exact rational arithmetic",
    latex=r"p(x) = q(x)\,d(x) + r(x),\quad \deg r < \deg d",
    run=polynomial_long_division, verify=_verify_longdiv, reference=_ref_longdiv, wrong=_wrong_longdiv, random_inputs=_rand_longdiv,
    parse=_templated([r"(?P<long>use (?:polynomial )?long division to )?divide (?P<p>" + _POLY + r") by (?P<d>" + _POLY + r")" + _E,
                      r"(?:find|what are) the quotient and (?:the )?remainder when (?P<p>" + _POLY + r") is divided by (?P<d>" + _POLY + r")" + _E],
                     _parse_longdiv),
    templates=["Divide 2x^3 + 3x^2 - x + 5 by x^2 + 1.", "Use long division to divide 6x^3 - 5x^2 + 4 by 2x - 3."],
    examples=[{"inputs": {"dividend": "2, 3, -1, 5", "divisor": "1, 0, 1"}, "outputs": {"quotient": [2.0, 3.0], "remainder": [-3.0, 2.0]}},
              {"inputs": {"dividend": [6, -5, 0, 4], "divisor": [2, -3]}, "outputs": {"quotient": [3.0, 2.0, 3.0], "remainder": [13.0]}}],
    questions=[{"q": "Divide 2x^3 + 3x^2 - x + 5 by x^2 + 1.", "outputs": {"quotient": [2.0, 3.0], "remainder": [-3.0, 2.0]}},
               {"q": "Use long division to divide 6x^3 - 5x^2 + 4 by 2x - 3.", "outputs": {"quotient": [3.0, 2.0, 3.0], "remainder": [13.0]}},
               {"q": "Find the quotient and remainder when x^4 - 1 is divided by x^2 + x + 1.", "outputs": {"quotient": [1.0, -1.0, 0.0], "remainder": [1.0, -1.0]}}])


# --- factoring a quadratic over the integers -----------------------------------------------------------------------
def factor_quadratic_integer(a, b, c) -> dict:
    a, b, c = int(a), int(b), int(c)
    if a == 0:
        raise ValueError("a must not be 0 (not a quadratic)")
    g = math.gcd(math.gcd(a, b), c) * (1 if a > 0 else -1)
    A, B, C = a // g, b // g, c // g
    D = B * B - 4 * A * C
    if D < 0 or math.isqrt(D) ** 2 != D:
        raise ValueError("this quadratic does not factor over the integers (its discriminant is not a perfect square)")
    s = math.isqrt(D)
    r1, r2 = Fraction(-B - s, 2 * A), Fraction(-B + s, 2 * A)          # r1 <= r2 since A > 0
    p, q, r, t = r1.denominator, -r1.numerator, r2.denominator, -r2.numerator
    if p * r != A:
        raise ValueError("internal: the factors do not reproduce the leading coefficient")
    fac = lambda m, n: ("x" if m == 1 else f"{m}x") + (f" + {n}" if n > 0 else (f" - {-n}" if n < 0 else ""))
    text = ("" if g == 1 else ("-" if g == -1 else f"{g}")) + f"({fac(p, q)})({fac(r, t)})"
    return {"g": float(g), "p": float(p), "q": float(q), "r": float(r), "s": float(t), "factored": text}


def _verify_factor(i, o):
    g, p, q, r, s = o["g"], o["p"], o["q"], o["r"], o["s"]
    if any(v != int(v) for v in (g, p, q, r, s)) or p <= 0 or r <= 0:
        return 1.0
    if math.gcd(int(p), int(q)) != 1 or math.gcd(int(r), int(s)) != 1 or Fraction(int(-q), int(p)) > Fraction(int(-s), int(r)):
        return 1.0
    want = (g * p * r, g * (p * s + q * r), g * q * s)
    return max(abs(u - v) / max(1.0, abs(v)) for u, v in zip(want, (i["a"], i["b"], i["c"])))


def _ref_factor(a, b, c):
    import sympy as sp
    x = sp.Symbol("x")
    coef, facs = sp.factor_list(sp.Integer(int(a)) * x**2 + sp.Integer(int(b)) * x + sp.Integer(int(c)))
    lin = []
    for f, mult in facs:
        pc = sp.Poly(f, x).all_coeffs()
        if len(pc) != 2:
            raise ValueError("not a product of linear factors")
        m, n = int(pc[0]), int(pc[1])
        if m < 0:
            m, n, coef = -m, -n, -coef
        lin += [(m, n)] * mult
    lin.sort(key=lambda t: sp.Rational(-t[1], t[0]))
    (p, q), (r, s) = lin
    return {"g": float(coef), "p": float(p), "q": float(q), "r": float(r), "s": float(s)}


def _wrong_factor(a, b, c):
    disc = b * b - 4 * a * c
    x1, x2 = sorted([(-b - math.sqrt(max(disc, 0))) / (2 * a), (-b + math.sqrt(max(disc, 0))) / (2 * a)])
    return {"g": 1.0, "p": 1.0, "q": -x1, "r": 1.0, "s": -x2}


def _rand_factor(rng):
    p, r = rng.randint(1, 6), rng.randint(1, 6)
    q, s = rng.randint(-9, 9), rng.randint(-9, 9)
    k = rng.choice([1, 1, 2, 3, -1, -2])
    return {"a": k * p * r, "b": k * (p * s + q * r), "c": k * q * s}


def _parse_factor(g):
    co = _poly_text(g["p"], "x", 2)
    if co is None or len(co) != 3 or any(v != int(v) for v in co):
        raise ValueError("a quadratic with whole-number coefficients")
    return {"a": int(co[0]), "b": int(co[1]), "c": int(co[2])}


_register(
    "factor_quadratic_integer", name="Factor a quadratic over the integers", topic="algebra / factoring",
    purpose="Factor a*x^2 + b*x + c with whole-number coefficients as g(px + q)(rx + s), or say that it does not factor over the integers.",
    inputs=[{"symbol": "a", "type": "integer", "kind": "number", "meaning": "coefficient of x^2 (not 0)", "required": True},
            {"symbol": "b", "type": "integer", "kind": "number", "meaning": "coefficient of x", "required": True},
            {"symbol": "c", "type": "integer", "kind": "number", "meaning": "constant term", "required": True}],
    outputs=[_out("g", "common factor (sign of a)"), _out("p", "x-coefficient of the first factor"), _out("q", "constant of the first factor"),
             _out("r", "x-coefficient of the second factor"), _out("s", "constant of the second factor"), _out("factored", "the factored form", "text")],
    primary="q", conditions=["whole-number coefficients", "a != 0", "the discriminant is a perfect square"],
    method="take out the greatest common factor; the rational roots -q/p and -s/r of the primitive quadratic give its factors (Gauss's lemma)",
    latex=r"ax^2+bx+c = g\,(px+q)(rx+s)",
    run=factor_quadratic_integer, verify=_verify_factor, reference=_ref_factor, wrong=_wrong_factor, random_inputs=_rand_factor,
    parse=_templated([r"factor(?: completely)? (?P<p>" + _POLY + r")(?: completely)?(?: over the integers)?" + _E], _parse_factor),
    templates=["Factor 6x^2 + 5x - 6 over the integers.", "Factor 4x^2 - 9.", "Factor completely 2x^2 - 8."],
    examples=[{"inputs": {"a": 6, "b": 5, "c": -6}, "outputs": {"g": 1.0, "p": 2.0, "q": 3.0, "r": 3.0, "s": -2.0}},
              {"inputs": {"a": 4, "b": 0, "c": -9}, "outputs": {"g": 1.0, "p": 2.0, "q": 3.0, "r": 2.0, "s": -3.0}},
              {"inputs": {"a": 2, "b": 0, "c": -8}, "outputs": {"g": 2.0, "p": 1.0, "q": 2.0, "r": 1.0, "s": -2.0}}],
    questions=[{"q": "Factor 6x^2 + 5x - 6 over the integers.", "outputs": {"g": 1.0, "p": 2.0, "q": 3.0, "r": 3.0, "s": -2.0}},
               {"q": "Factor 4x^2 - 9.", "outputs": {"p": 2.0, "q": 3.0, "r": 2.0, "s": -3.0}},
               {"q": "Factor completely 2x^2 - 8.", "outputs": {"g": 2.0, "q": 2.0, "s": -2.0}},
               {"q": "Factor 2x^2 + 3x + 5 over the integers.", "outcome": "INVALID_INPUT"}])


# --- quadratic inequality -----------------------------------------------------------------------------------------
def quadratic_inequality(a, b, c, rel) -> dict:
    _finite(a=a, b=b, c=c)
    if a == 0:
        raise ValueError("a must not be 0 (not a quadratic)")
    disc = b * b - 4 * a * c
    if disc <= 0:
        raise ValueError("refused: the quadratic has no two distinct real roots (this capability answers that case only)")
    q = -(b + math.copysign(math.sqrt(disc), b)) / 2
    r1, r2 = sorted([q / a, c / q])
    between = (rel in ("<", "<=")) == (a > 0)
    return {"r1": r1, "r2": r2, "region": "between" if between else "outside", "solution": _ineq_text(r1, r2, rel, between)}


def _verify_qineq(i, o):
    a, b, c = i["a"], i["b"], i["c"]
    worst = 0.0
    for x in (o["r1"], o["r2"]):
        t = [a * x * x, b * x, c]
        worst = max(worst, abs(sum(t)) / (sum(abs(v) for v in t) or 1.0))
    worst += abs(o["r1"] + o["r2"] + b / a) / max(1.0, abs(b / a))          # the two DIFFERENT roots (Vieta)
    worst += (o["r1"] >= o["r2"]) + (o["region"] != ("between" if (i["rel"] in ("<", "<=")) == (a > 0) else "outside"))
    return worst


def _ref_qineq(a, b, c, rel):
    import sympy as sp
    x = sp.Symbol("x", real=True)
    A, B, C = (sp.Rational(repr(float(v))) for v in (a, b, c))
    ex = {"<": sp.Lt, "<=": sp.Le, ">": sp.Gt, ">=": sp.Ge}[rel](A * x**2 + B * x + C, 0)
    s = sp.solve_univariate_inequality(ex, x, relational=False)
    if isinstance(s, sp.Interval):
        return {"r1": float(s.inf), "r2": float(s.sup), "region": "between"}
    left, right = sorted(s.args, key=lambda I: float(I.sup))
    return {"r1": float(left.sup), "r2": float(right.inf), "region": "outside"}


def _wrong_qineq(a, b, c, rel):
    r = quadratic_inequality(a, b, c, rel)
    return dict(r, r1=-r["r2"], r2=-r["r1"])                 # the sign error: (b ± sqrt(D))/(2a)


def _rand_qineq(rng):
    while True:
        a = rng.choice([v for v in range(-5, 6) if v])
        x1, x2 = rng.randint(-8, 8), rng.randint(-8, 8)
        if x1 != x2:
            break
    if rng.random() < 0.5:
        return {"a": a, "b": -a * (x1 + x2), "c": a * x1 * x2, "rel": rng.choice(_RELS)}
    return {"a": a, "b": float(rng.randint(-9, 9)), "c": float(rng.randint(-20, 20)), "rel": rng.choice(_RELS)}


def _parse_qineq(g):
    l, r = _poly_text(g["l"], "x", 2), _poly_text(g["r"], "x", 2)
    if l is None or r is None:
        raise ValueError("polynomials")
    pad = lambda v: [0.0] * (3 - len(v)) + v
    a, b, c = (u - v for u, v in zip(pad(l), pad(r)))
    if a == 0:
        raise ValueError("not quadratic")
    if b * b - 4 * a * c <= 0:
        return {"_out_of_scope": "a quadratic inequality whose quadratic has no two distinct real roots"}
    return {"a": a, "b": b, "c": c, "rel": g["rel"]}


_register(
    "quadratic_inequality", name="Quadratic inequality", topic="algebra / inequalities",
    purpose="Solve a*x^2 + b*x + c < 0 (or <=, >, >=) when the quadratic has two distinct real roots.",
    inputs=[_num_in("a", "coefficient of x^2", "a != 0"), _num_in("b", "coefficient of x"), _num_in("c", "constant term"),
            {"symbol": "rel", "type": "choice", "choices": _RELS, "kind": "text", "meaning": "the comparison with 0", "required": True}],
    outputs=[_out("r1", "the smaller root"), _out("r2", "the larger root"),
             _out("region", "between or outside the roots", "text"), _out("solution", "the solution set", "text")],
    primary="r1", conditions=["a != 0", "b^2 - 4ac > 0"],
    method="the roots split the line into three intervals; the sign of a decides whether the quadratic is negative between or outside them",
    latex=r"a(x - r_1)(x - r_2) < 0 \iff r_1 < x < r_2 \ \ (a > 0)",
    run=quadratic_inequality, verify=_verify_qineq, reference=_ref_qineq, wrong=_wrong_qineq, random_inputs=_rand_qineq,
    parse=_templated([r"solve(?: the(?: quadratic)? inequality)? (?P<l>" + _POLY + r")\s*(?P<rel><=|>=|<|>)\s*(?P<r>" + _POLY + r")" + _E], _parse_qineq),
    templates=["Solve x^2 - 5x + 6 < 0.", "Solve the quadratic inequality 2x^2 - x - 3 >= 0."],
    examples=[{"inputs": {"a": 1, "b": -5, "c": 6, "rel": "<"}, "outputs": {"r1": 2.0, "r2": 3.0, "region": "between"}},
              {"inputs": {"a": 2, "b": -1, "c": -3, "rel": ">="}, "outputs": {"r1": -1.0, "r2": 1.5, "region": "outside"}},
              {"inputs": {"a": -1, "b": 0, "c": 4, "rel": ">"}, "outputs": {"r1": -2.0, "r2": 2.0, "region": "between"}}],
    questions=[{"q": "Solve x^2 - 5x + 6 < 0.", "outputs": {"r1": 2.0, "r2": 3.0, "region": "between"}},
               {"q": "Solve the quadratic inequality 2x^2 - x - 3 >= 0.", "outputs": {"r1": -1.0, "r2": 1.5, "region": "outside"}},
               {"q": "Solve -x^2 + 4 > 0.", "outputs": {"r1": -2.0, "r2": 2.0, "region": "between"}},
               {"q": "Solve x^2 + 1 < 0.", "outcome": "OUT_OF_SCOPE"}])


# --- infinite geometric series ------------------------------------------------------------------------------------
def infinite_geometric_series(a, r) -> dict:
    _finite(a=a, r=r)
    if not abs(r) < 1:
        raise ValueError("the series diverges unless |r| < 1")
    return {"sum": a / (1 - r)}


def _ref_geo_inf(a, r):
    import mpmath as mp
    with mp.workdps(30):
        return {"sum": float(mp.nsum(lambda k: mp.mpf(repr(float(a))) * mp.mpf(repr(float(r))) ** k, [0, mp.inf]))}


def _parse_geo_terms(g):
    terms = [Fraction(t.replace(" ", "")) for t in re.findall(r"[-+]?\s*\d+(?:\.\d+)?(?:/\d+)?", g["terms"])]
    if len(terms) < 3 or terms[0] == 0:
        raise ValueError("at least three terms")
    r = terms[1] / terms[0]
    if any(terms[k + 1] != terms[k] * r for k in range(len(terms) - 1)):
        raise ValueError("not geometric")
    return {"a": float(terms[0]), "r": float(r)}


_register(
    "infinite_geometric_series", name="Sum of an infinite geometric series", topic="algebra / sequences and series",
    purpose="a + a*r + a*r^2 + ... = a/(1 - r) for |r| < 1 (a refusal when the series diverges).",
    inputs=[_num_in("a", "the first term"), _num_in("r", "the common ratio, |r| < 1")],
    outputs=[_out("sum", "the sum to infinity")], primary="sum", conditions=["|r| < 1"],
    method="S_n = a(1 - r^n)/(1 - r) and r^n -> 0 when |r| < 1", latex=r"\sum_{k=0}^{\infty} a r^k = \frac{a}{1-r},\quad |r| < 1",
    run=infinite_geometric_series, verify=lambda i, o: abs(o["sum"] * (1 - i["r"]) - i["a"]) / max(1.0, abs(i["a"])),
    reference=_ref_geo_inf, wrong=lambda a, r: {"sum": a / (1 + r)},
    random_inputs=lambda rng: {"a": round(rng.uniform(-100, 100), rng.choice([0, 2])), "r": rng.choice([round(rng.uniform(-0.95, 0.95), 3), rng.choice([0.5, -0.5, 1 / 3, 0.25, -0.2])]) or 0.5},
    parse=_templated([r"(?:find|what is) the sum of the infinite geometric series with first term (?P<a>" + _Q + r") and (?:common )?ratio (?P<r>" + _Q + r")" + _E,
                      r"(?:find|what is) the sum to infinity of (?P<terms>(?:[-+]?\s*\d+(?:\.\d+)?(?:/\d+)?\s*[-+]\s*){2,}[-+]?\s*\d+(?:\.\d+)?(?:/\d+)?)\s*[-+]\s*(?:\.\.\.|…)" + _E],
                     lambda g: _parse_geo_terms(g) if g.get("terms") else {"a": _q(g["a"]), "r": _q(g["r"])}),
    templates=["Find the sum of the infinite geometric series with first term 6 and ratio 1/3.", "Find the sum to infinity of 8 + 4 + 2 + 1 + ..."],
    examples=[{"inputs": {"a": 6, "r": 1 / 3}, "outputs": {"sum": 9.0}}, {"inputs": {"a": 8, "r": 0.5}, "outputs": {"sum": 16.0}},
              {"inputs": {"a": 9, "r": -1 / 3}, "outputs": {"sum": 6.75}}],
    questions=[{"q": "Find the sum of the infinite geometric series with first term 6 and ratio 1/3.", "outputs": {"sum": 9.0}},
               {"q": "Find the sum to infinity of 8 + 4 + 2 + 1 + ...", "outputs": {"sum": 16.0}},
               {"q": "Find the sum to infinity of 9 - 3 + 1 - ...", "outputs": {"sum": 6.75}},
               {"q": "Find the sum of the infinite geometric series with first term 2 and ratio 3/2.", "outcome": "INVALID_INPUT"}])


# --- partial fractions with two distinct linear factors -----------------------------------------------------------
def partial_fractions_linear(p, q, r1, r2) -> dict:
    _finite(p=p, q=q, r1=r1, r2=r2)
    if r1 == r2:
        raise ValueError("the two linear factors must be different (repeated factors need another form)")
    return {"A": (p * r1 + q) / (r1 - r2), "B": (p * r2 + q) / (r2 - r1)}


def _verify_pf(i, o):
    A, B = o["A"], o["B"]
    p, q, r1, r2 = i["p"], i["q"], i["r1"], i["r2"]
    return (abs(A + B - p) / max(1.0, abs(p), abs(A), abs(B))
            + abs(-A * r2 - B * r1 - q) / max(1.0, abs(q), abs(A * r2), abs(B * r1)))


def _ref_pf(p, q, r1, r2):
    import sympy as sp
    x = sp.Symbol("x")
    P, Q, R1, R2 = (sp.Rational(repr(float(v))) for v in (p, q, r1, r2))
    f = (P * x + Q) / ((x - R1) * (x - R2))
    return {"A": float(sp.residue(f, x, R1)), "B": float(sp.residue(f, x, R2))}


def _parse_pf(g):
    num = _poly_text(g["num"], "x", 1)
    if num is None:
        raise ValueError("linear numerator")
    p, q = (num if len(num) == 2 else [0.0, num[0]])
    if g.get("f1"):
        r1, r2 = _divisor_r(g["f1"]), _divisor_r(g["f2"])
        if r1 is None or r2 is None:
            raise ValueError("factors x - r")
    else:
        den = _poly_text(g["den"], "x", 2)
        if den is None or len(den) != 3 or den[0] != 1:
            raise ValueError("a monic quadratic denominator")
        disc = den[1] ** 2 - 4 * den[2]
        if disc <= 0:
            raise ValueError("the denominator has no two distinct real linear factors")
        r1, r2 = (-den[1] - math.sqrt(disc)) / 2, (-den[1] + math.sqrt(disc)) / 2
    return {"p": p, "q": q, "r1": r1, "r2": r2}


_register(
    "partial_fractions_linear", name="Partial fractions (two distinct linear factors)", topic="algebra / rational expressions",
    purpose="Write (p*x + q)/((x - r1)(x - r2)) as A/(x - r1) + B/(x - r2).",
    inputs=[_num_in("p", "coefficient of x in the numerator"), _num_in("q", "constant of the numerator"),
            _num_in("r1", "root of the first factor x - r1"), _num_in("r2", "root of the second factor x - r2 (r2 != r1)")],
    outputs=[_out("A", "numerator over x - r1"), _out("B", "numerator over x - r2")], primary="A", conditions=["r1 != r2"],
    method="cover-up rule: A = (p r1 + q)/(r1 - r2), B = (p r2 + q)/(r2 - r1)",
    latex=r"\frac{px+q}{(x-r_1)(x-r_2)} = \frac{A}{x-r_1} + \frac{B}{x-r_2},\quad A = \frac{pr_1+q}{r_1-r_2}",
    run=partial_fractions_linear, verify=_verify_pf, reference=_ref_pf,
    wrong=lambda p, q, r1, r2: {"A": (p * r1 + q) / (r2 - r1), "B": (p * r2 + q) / (r1 - r2)},
    random_inputs=lambda rng: (lambda r1, r2: {"p": rng.randint(-9, 9), "q": rng.randint(-12, 12), "r1": r1, "r2": r2 if r2 != r1 else r1 + 1})(
        rng.randint(-8, 8), rng.randint(-8, 8)),
    parse=_templated([r"(?:decompose|write|express) \((?P<num>[^()]+)\)\s*/\s*\((?P<f1>\(?x\s*[-+]?\s*[\d.]*\)?)\s*\*?\s*(?P<f2>\(x\s*[-+]?\s*[\d.]*\)|x)\) (?:into|as|in) partial fractions" + _E,
                      r"(?:find )?the partial fraction decomposition of \((?P<num>[^()]+)\)\s*/\s*\((?P<den>[^()]+)\)" + _E],
                     _parse_pf),
    templates=["Decompose (5x + 1)/((x - 1)(x + 2)) into partial fractions.", "Find the partial fraction decomposition of (3x + 5)/(x^2 - x - 2)."],
    examples=[{"inputs": {"p": 5, "q": 1, "r1": 1, "r2": -2}, "outputs": {"A": 2.0, "B": 3.0}},
              {"inputs": {"p": 3, "q": 5, "r1": -1, "r2": 2}, "outputs": {"A": -2 / 3, "B": 11 / 3}}],
    questions=[{"q": "Decompose (5x + 1)/((x - 1)(x + 2)) into partial fractions.", "outputs": {"A": 2.0, "B": 3.0}},
               {"q": "Find the partial fraction decomposition of (3x + 5)/(x^2 - x - 2).", "outputs": {"A": -2 / 3, "B": 11 / 3}},
               {"q": "Decompose (2x + 3)/((x - 4)(x - 4)) into partial fractions.", "outcome": "INVALID_INPUT"}])


# --- rational roots of an integer polynomial ----------------------------------------------------------------------
def _divisors(n: int) -> list:
    n = abs(n)
    out = set()
    for k in range(1, math.isqrt(n) + 1):
        if n % k == 0:
            out.update((k, n // k))
    return sorted(out)


def _horner(co, x):
    v = Fraction(0)
    for c in co:
        v = v * x + c
    return v


def _candidates_roots(co):
    """Distinct rational roots of the integer polynomial co (highest first), by the rational root theorem."""
    co = list(co)
    roots = set()
    while len(co) > 1 and co[-1] == 0:                 # x = 0 is a root: divide by x
        roots.add(Fraction(0))
        co.pop()
    if len(co) > 1:
        for p in _divisors(int(co[-1])):
            for q in _divisors(int(co[0])):
                for cand in (Fraction(p, q), Fraction(-p, q)):
                    if cand not in roots and _horner(co, cand) == 0:
                        roots.add(cand)
    return sorted(roots)


def rational_roots(coefficients) -> dict:
    co = _strip([float(c) for c in coefficients])
    if any(c != int(c) for c in co):
        raise ValueError("the coefficients must be whole numbers")
    co = [int(c) for c in co]
    if len(co) < 2:
        raise ValueError("the polynomial must have degree at least 1")
    if max(abs(c) for c in co) > 10**7:
        raise ValueError("coefficients are limited to 10^7 in size")
    roots = _candidates_roots(co)
    return {"count": float(len(roots)), "roots": [float(r) for r in roots],
            "roots_exact": [str(r) for r in roots]}


def _verify_ratroots(i, o):
    co = [int(c) for c in _strip([float(c) for c in i["coefficients"]])]
    ex = [Fraction(s) for s in o.get("roots_exact", [])]
    if o["count"] != len(ex) or len(o["roots"]) != len(ex) or len(set(ex)) != len(ex):
        return 1.0
    if any(_horner(co, r) != 0 for r in ex) or any(_rel(float(r), v) > 1e-15 for r, v in zip(ex, o["roots"])):
        return 1.0
    rest = [Fraction(c) for c in co]                   # deflate by every root found (with its multiplicity) ...
    for r in ex:
        while len(rest) > 1 and _horner(rest, r) == 0:
            out = [rest[0]]
            for c in rest[1:]:
                out.append(c + out[-1] * r)
            rest = out[:-1]
    if len(rest) > 1:                                 # ... and check that what is left has no rational root
        den = math.lcm(*[f.denominator for f in rest])
        ints = [int(f * den) for f in rest]
        g = math.gcd(*ints)
        ints = [v // g for v in ints]
        while len(ints) > 1 and ints[-1] == 0:
            return 1.0
        for p in _divisors(ints[-1]):
            for q in _divisors(ints[0]):
                if _horner(ints, Fraction(p, q)) == 0 or _horner(ints, Fraction(-p, q)) == 0:
                    return 1.0
    return 0.0


def _ref_ratroots(coefficients):
    import sympy as sp
    x = sp.Symbol("x")
    P = sp.Poly([sp.Integer(int(c)) for c in _strip([float(c) for c in coefficients])], x)
    rs = sorted(r for r in sp.roots(P, filter="Q"))
    return {"count": float(len(rs)), "roots": [float(r) for r in rs]}


def _wrong_ratroots(coefficients):
    co = [int(c) for c in _strip([float(c) for c in coefficients])]
    rs = set()
    if co[-1] == 0:
        rs.add(Fraction(0))
    for p in _divisors(co[-1]) if co[-1] else []:
        for cand in (Fraction(p), Fraction(-p)):
            if _horner(co, cand) == 0:
                rs.add(cand)
    rs = sorted(rs)
    return {"count": float(len(rs)), "roots": [float(r) for r in rs], "roots_exact": [str(r) for r in rs]}


def _rand_ratroots(rng):
    co = [1]
    for _ in range(rng.randint(1, 3)):
        p, q = rng.randint(-6, 6), rng.choice([1, 2, 2, 3, 3, 4, 5])
        co = [a - b for a, b in zip([q * c for c in co] + [0], [0] + [p * c for c in co])]     # times (q x - p)
    if rng.random() < 0.4:
        co = [a + b for a, b in zip(co + [0, 0], [0, 0] + co)]                              # times (x^2 + 1)
    return {"coefficients": [float(c) for c in co]}


def _parse_ratroots(g):
    co = _poly_text(g["p"], "x", 8)
    if co is None or len(co) < 2 or any(v != int(v) for v in co):
        raise ValueError("an integer polynomial")
    return {"coefficients": co}


_register(
    "rational_roots", name="Rational roots of a polynomial (rational root theorem)", topic="algebra / polynomial equations",
    purpose="Every rational root of a polynomial with whole-number coefficients (degree up to 8).",
    inputs=[{"symbol": "coefficients", "type": "vector", "kind": "number", "required": True, "min_length": 2, "max_length": 9,
             "meaning": "whole-number coefficients, highest power first"}],
    outputs=[_out("count", "number of distinct rational roots"), _out("roots", "the rational roots, ascending", "list"),
             _out("roots_exact", "the rational roots as fractions", "list")],
    primary="count", conditions=["whole-number coefficients", "degree 1 to 8"],
    method="rational root theorem: a root p/q in lowest terms has p dividing the constant term and q dividing the leading "
           "coefficient; test every candidate exactly",
    latex=r"a_n x^n + \dots + a_0 = 0,\ x = \tfrac{p}{q} \Rightarrow p \mid a_0,\ q \mid a_n",
    run=rational_roots, verify=_verify_ratroots, reference=_ref_ratroots, wrong=_wrong_ratroots, random_inputs=_rand_ratroots,
    parse=_templated([r"(?:find|list|what are) (?:all )?the rational (?:roots|zeros) of (?P<p>" + _POLY + r")(?:\s*=\s*0)?" + _E,
                      r"(?:find|list) all rational (?:roots|zeros) of (?P<p>" + _POLY + r")(?:\s*=\s*0)?" + _E],
                     _parse_ratroots),
    templates=["Find all rational roots of 2x^3 - 3x^2 - 11x + 6 = 0.", "List the rational zeros of x^3 - 7x + 6."],
    examples=[{"inputs": {"coefficients": "2, -3, -11, 6"}, "outputs": {"count": 3.0, "roots": [-2.0, 0.5, 3.0]}},
              {"inputs": {"coefficients": [1, 0, -7, 6]}, "outputs": {"count": 3.0, "roots": [-3.0, 1.0, 2.0]}},
              {"inputs": {"coefficients": [1, 0, -2]}, "outputs": {"count": 0.0, "roots": []}}],
    questions=[{"q": "Find all rational roots of 2x^3 - 3x^2 - 11x + 6 = 0.", "outputs": {"count": 3.0, "roots": [-2.0, 0.5, 3.0]}},
               {"q": "List the rational zeros of x^3 - 7x + 6.", "outputs": {"roots": [-3.0, 1.0, 2.0]}},
               {"q": "Find the rational roots of x^2 - 2.", "outputs": {"count": 0.0, "roots": []}},
               {"q": "Find all rational roots of 0.5x^2 - 2.", "outcome": "UNRECOGNIZED_FORMAT"}])


# =====================================================================================================================
# functions: evaluation, composition, domain, inverse, piecewise, transformations, lines
# =====================================================================================================================
_FEXPR = re.compile(r"^[\dx\s.+\-*/^()\[\]a-z]+$")


def _fval(expression, x):
    if not isinstance(expression, str) or not _FEXPR.match(_clean(expression).lower()):
        raise ValueError("the function must be written with numbers, x, + - * / ^, parentheses, sqrt, abs or ln")
    if not isinstance(x, Fraction):
        _finite(x=x)
    return _eval_ast(expression, {"x": x}, "x")


def evaluate_function(expression, x) -> dict:
    try:
        return _exact_outputs(_fval(expression, x))
    except ValueError as exc:
        raise ValueError(f"f({x:g}) is not defined: {exc}") from None


def _verify_feval(i, o):
    v = _eval_pratt(i["expression"], {"x": i["x"]}, "x")
    r = _rel(o["value"], float(v))
    if isinstance(v, Fraction) and o.get("denominator"):
        r += float(abs(Fraction(int(o["numerator"]), int(o["denominator"])) - v) / max(1, abs(v)))
    return r


def _ref_feval(expression, x):
    import sympy as sp
    e = _sym_expr(expression, "x").subs(sp.Symbol("x"), sp.Rational(repr(float(x))))
    v = sp.nsimplify(sp.simplify(e)) if e.is_Rational else e
    if v.is_Rational:
        return {"value": float(v), "numerator": float(v.p), "denominator": float(v.q)}
    return {"value": float(sp.N(v, 30))}


_SHAPES_F = ["{a}x^2 + {b}x + {c}", "{a}x^3 - {b}x + {c}", "({a}x + {b})/(x - {c})", "{a}(x - {b})^2 + {c}",
             "x^2 - {a}x + {b} - {c}/2", "{a}/(x^2 + {b}) + {c}x", "sqrt(x^2 + {a}) + {b}", "{a}abs(x - {b}) - {c}"]


def _rand_fexpr(rng):
    return rng.choice(_SHAPES_F).format(a=rng.randint(1, 9), b=rng.randint(1, 9), c=rng.randint(-9, 9)).replace("+ -", "- ")


_FX = r"f\((?P<x0>" + _Q + r")\)"
_register(
    "evaluate_function", name="Evaluate a function at a number", topic="functions / evaluation",
    purpose="The value f(x0) of a function given by a formula in x (exact when it is rational).",
    inputs=[{"symbol": "expression", "type": "text", "kind": "text", "required": True, "min_length": 1, "max_length": 200,
             "meaning": "the formula of f in x: numbers, x, + - * / ^, parentheses, sqrt, abs, ln"},
            _num_in("x", "the number x0 to put in")],
    outputs=[_out("value", "f(x0)"), _out("numerator", "numerator of the exact value"), _out("denominator", "denominator of the exact value")],
    primary="value", conditions=["x0 in the domain of f (no division by 0, no root of a negative number, no logarithm of a number <= 0)"],
    method="substitute x0 for x and evaluate with the order of operations, in exact rational arithmetic",
    latex=r"f(x_0)", run=evaluate_function, verify=_verify_feval, reference=_ref_feval,
    wrong=lambda expression, x: _guarded(lambda: _exact_outputs(_fval(expression, -x)) if x else {"value": 1e9}),
    random_inputs=lambda rng: {"expression": _rand_fexpr(rng), "x": float(rng.randint(-6, 6)) or 2.0},
    parse=_templated([r"(?:evaluate|find|compute|what is) " + _FX + r" (?:for|if|when|where|given) f\(x\) = (?P<e>[^=]+?)" + _E,
                      r"(?:if|given|let) f\(x\) = (?P<e>[^=]+?), (?:find|evaluate|compute|what is) " + _FX + _E],
                     lambda g: {"expression": g["e"].strip(), "x": _q(g["x0"])} if _FEXPR.match(_clean(g["e"]).lower())
                     and not re.search(r"\b(?:if|for|when|and|or)\b|[<>]", g["e"]) else None),
    templates=["Evaluate f(3) for f(x) = 2x^2 - 3x + 1.", "If f(x) = x^3 - 2x + 5, find f(-2)."],
    examples=[{"inputs": {"expression": "2x^2 - 3x + 1", "x": 3}, "outputs": {"value": 10.0}},
              {"inputs": {"expression": "x^3 - 2x + 5", "x": -2}, "outputs": {"value": 1.0}},
              {"inputs": {"expression": "(x + 1)/(x - 3)", "x": 2}, "outputs": {"value": -3.0}},
              {"inputs": {"expression": "sqrt(x + 7)", "x": 2}, "outputs": {"value": 3.0}}],
    questions=[{"q": "Evaluate f(3) for f(x) = 2x^2 - 3x + 1.", "outputs": {"value": 10.0}},
               {"q": "If f(x) = x^3 - 2x + 5, find f(-2).", "outputs": {"value": 1.0}},
               {"q": "Find f(1/2) when f(x) = 4x^2 - 1.", "outputs": {"value": 0.0}},
               {"q": "Find f(3) when f(x) = (x + 1)/(x - 3).", "outcome": "INVALID_INPUT"}])

_ORDERS = ["f(g(x))", "g(f(x))"]


def composite_function_value(f, g, x, order) -> dict:
    first, second = (g, f) if order == "f(g(x))" else (f, g)
    try:
        inner = _fval(first, x)
        value = _fval(second, float(inner) if not isinstance(inner, Fraction) else inner)
    except ValueError as exc:
        raise ValueError(f"the composition is not defined at x = {x:g}: {exc}") from None
    return {"inner": float(inner), "value": float(value)}


def _verify_comp(i, o):
    first, second = (i["g"], i["f"]) if i["order"] == "f(g(x))" else (i["f"], i["g"])
    inner = _eval_pratt(first, {"x": i["x"]}, "x")
    v = _eval_pratt(second, {"x": inner if isinstance(inner, Fraction) else float(inner)}, "x")
    return _rel(o["inner"], float(inner)) + _rel(o["value"], float(v))


def _ref_comp(f, g, x, order):
    import sympy as sp
    X = sp.Symbol("x")
    F, G = _sym_expr(f, "x"), _sym_expr(g, "x")
    inner_e, outer_e = (G, F) if order == "f(g(x))" else (F, G)
    comp = outer_e.subs(X, inner_e)                     # the composite function first, then the number
    xv = sp.Rational(repr(float(x)))
    return {"inner": float(sp.N(inner_e.subs(X, xv), 30)), "value": float(sp.N(comp.subs(X, xv), 30))}


_register(
    "composite_function_value", name="Value of a composite function", topic="functions / composition",
    purpose="f(g(x0)) or g(f(x0)) for two functions given by formulas.",
    inputs=[{"symbol": "f", "type": "text", "kind": "text", "required": True, "min_length": 1, "max_length": 200, "meaning": "the formula of f in x"},
            {"symbol": "g", "type": "text", "kind": "text", "required": True, "min_length": 1, "max_length": 200, "meaning": "the formula of g in x"},
            _num_in("x", "the number x0"),
            {"symbol": "order", "type": "choice", "choices": _ORDERS, "kind": "text", "required": True, "meaning": "which composition"}],
    outputs=[_out("inner", "value of the inner function at x0"), _out("value", "value of the composite at x0")],
    primary="value", conditions=["each value in the domain of the function applied to it"],
    method="evaluate the inner function at x0, then the outer function at that value", latex=r"(f\circ g)(x_0) = f\big(g(x_0)\big)",
    run=composite_function_value, verify=_verify_comp, reference=_ref_comp,
    wrong=lambda f, g, x, order: _guarded(lambda: composite_function_value(f, g, x, _ORDERS[1 - _ORDERS.index(order)]), {"inner": 1e9, "value": 1e9}),
    random_inputs=lambda rng: {"f": _rand_fexpr(rng), "g": rng.choice(["{a}x - {b}", "x^2 - {b}", "{a}x + {b}", "{b} - x"]).format(
        a=rng.randint(2, 6), b=rng.randint(1, 9)), "x": float(rng.randint(-4, 5)), "order": rng.choice(_ORDERS)},
    parse=_templated([r"(?:if|given|let) f\(x\) = (?P<f>[^=]+?) and g\(x\) = (?P<g>[^=]+?), (?:find|evaluate|compute|what is) (?P<o>f\(g|g\(f)\((?P<x0>" + _Q + r")\)\)" + _E],
                     lambda g: {"f": g["f"].strip(), "g": g["g"].strip(), "x": _q(g["x0"]), "order": "f(g(x))" if g["o"].lower() == "f(g" else "g(f(x))"}),
    templates=["If f(x) = x^2 + 1 and g(x) = 2x - 3, find f(g(2)).", "Let f(x) = 3x + 2 and g(x) = x^2, evaluate g(f(-1))."],
    examples=[{"inputs": {"f": "x^2 + 1", "g": "2x - 3", "x": 2, "order": "f(g(x))"}, "outputs": {"inner": 1.0, "value": 2.0}},
              {"inputs": {"f": "3x + 2", "g": "x^2", "x": -1, "order": "g(f(x))"}, "outputs": {"inner": -1.0, "value": 1.0}}],
    questions=[{"q": "If f(x) = x^2 + 1 and g(x) = 2x - 3, find f(g(2)).", "outputs": {"value": 2.0}},
               {"q": "If f(x) = x^2 + 1 and g(x) = 2x - 3, find g(f(2)).", "outputs": {"inner": 5.0, "value": 7.0}},
               {"q": "Let f(x) = 3x + 2 and g(x) = x^2, evaluate g(f(-1)).", "outputs": {"value": 1.0}},
               {"q": "If f(x) = 1/x and g(x) = x - 2, find f(g(2)).", "outcome": "INVALID_INPUT"}])


def _domain_parts(a, b, c, d, strict):
    """boundary, side, excluded point (None when there is none inside the radicand's set), interval text."""
    B = -_fr(b) / _fr(a)
    up = a > 0
    side = (">" if strict else ">=") if up else ("<" if strict else "<=")
    E = None
    if c != 0:
        e = -_fr(d) / _fr(c)
        inside = (e > B if up else e < B) or (e == B and not strict)
        E = e if inside else None
    f = lambda v: f"{float(v):.10g}"
    lo_br, hi_br = ("(" if strict else "["), ("(" if strict else "]")
    if up:
        if E is None:
            text = f"{lo_br}{f(B)}, inf)"
        elif E == B:
            text = f"({f(B)}, inf)"
        else:
            text = f"{lo_br}{f(B)}, {f(E)}) U ({f(E)}, inf)"
    else:
        hi_br = ")" if strict else "]"
        if E is None:
            text = f"(-inf, {f(B)}{hi_br}"
        elif E == B:
            text = f"(-inf, {f(B)})"
        else:
            text = f"(-inf, {f(E)}) U ({f(E)}, {f(B)}{hi_br}"
    return B, side, E, text


def domain_sqrt_over_linear(a, b, c, d, root_in_denominator=False) -> dict:
    _finite(a=a, b=b, c=c, d=d)
    if a == 0:
        raise ValueError("a must not be 0 (no x under the root)")
    if c == 0 and d == 0:
        raise ValueError("the denominator is 0 for every x")
    B, side, E, text = _domain_parts(a, b, c, d, bool(root_in_denominator))
    return {"boundary": float(B), "side": side, "excluded": float(E) if E is not None else None, "domain": text}


def _verify_domain(i, o):
    B, side, E, text = _domain_parts(i["a"], i["b"], i["c"], i["d"], bool(i.get("root_in_denominator")))
    r = abs(i["a"] * o["boundary"] + i["b"]) / max(1.0, abs(i["b"]), abs(i["a"] * o["boundary"]))
    r += (o["side"] != side) + (o["domain"] != text)
    if (E is None) != (o["excluded"] is None):
        return r + 1.0
    if E is not None:
        r += abs(i["c"] * o["excluded"] + i["d"]) / max(1.0, abs(i["d"]))
    return r


def _ref_domain(a, b, c, d, root_in_denominator=False):
    import sympy as sp
    x = sp.Symbol("x", real=True)
    A, Bv, Cv, D = (sp.Rational(repr(float(v))) for v in (a, b, c, d))
    rad = sp.solve_univariate_inequality((sp.Gt if root_in_denominator else sp.Ge)(A * x + Bv, 0), x, relational=False)
    bound = rad.inf if rad.inf.is_finite else rad.sup
    out = {"boundary": float(bound), "excluded": None}
    if Cv != 0:
        e = sp.solve(sp.Eq(Cv * x + D, 0), x)[0]
        if e in rad:
            out["excluded"] = float(e)
    return out


def _rand_domain(rng):
    a = rng.choice([v for v in range(-6, 7) if v])
    c = rng.choice([0, 1, 1, 2, -1, 3])
    return {"a": a, "b": rng.randint(-12, 12), "c": c, "d": rng.randint(-9, 9) if c else 1, "root_in_denominator": rng.random() < 0.3}


def _parse_domain(g):
    rad = _poly_text(g["rad"], "x", 1)
    if rad is None or len(rad) != 2:
        raise ValueError("linear radicand")
    if g.get("den"):
        den = _poly_text(g["den"], "x", 1)
        if den is None:
            raise ValueError("linear denominator")
        c, d = (den if len(den) == 2 else [0.0, den[0]])
    else:
        c, d = 0.0, 1.0
    return {"a": rad[0], "b": rad[1], "c": c, "d": d, "root_in_denominator": bool(g.get("inv"))}


_register(
    "domain_sqrt_over_linear", name="Domain of a square-root function", topic="functions / domain",
    purpose="The domain of f(x) = sqrt(a*x + b)/(c*x + d) (or 1/sqrt(a*x + b)): where the radicand is allowed and the denominator is not 0.",
    inputs=[_num_in("a", "coefficient of x under the root", "a != 0"), _num_in("b", "constant under the root"),
            _num_in("c", "coefficient of x in the denominator (0 for none)"), _num_in("d", "constant in the denominator (1 for none)"),
            {"symbol": "root_in_denominator", "type": "boolean", "kind": "text", "required": False,
             "meaning": "true when the root itself is in the denominator (the radicand must then be positive)"}],
    outputs=[_out("boundary", "where the radicand is 0"), _out("side", "the inequality x must satisfy with the boundary", "text"),
             _out("excluded", "a point removed because the denominator is 0 there (None if none)"), _out("domain", "the domain in interval notation", "text")],
    primary="boundary", conditions=["a != 0", "the denominator is not identically 0"],
    method="radicand a*x + b >= 0 (> 0 when the root is in a denominator); remove the zero of c*x + d",
    latex=r"ax + b \ge 0,\quad cx + d \ne 0",
    run=domain_sqrt_over_linear, verify=_verify_domain, reference=_ref_domain,
    wrong=lambda a, b, c, d, root_in_denominator=False: dict(domain_sqrt_over_linear(a, b, c, d, root_in_denominator), boundary=b / a if b else 1.0),
    random_inputs=_rand_domain,
    parse=_templated([r"(?:find|what is) the domain of (?:f\(x\) = |y = )?sqrt\((?P<rad>[^()]+)\)(?:\s*/\s*\((?P<den>[^()]+)\))?" + _E,
                      r"(?:find|what is) the domain of (?:f\(x\) = |y = )?(?P<inv>1\s*/\s*)sqrt\((?P<rad>[^()]+)\)" + _E],
                     _parse_domain),
    templates=["Find the domain of f(x) = sqrt(2x - 4).", "Find the domain of f(x) = sqrt(x + 3)/(x - 1).", "Find the domain of f(x) = 1/sqrt(5 - x)."],
    examples=[{"inputs": {"a": 2, "b": -4, "c": 0, "d": 1}, "outputs": {"boundary": 2.0, "side": ">=", "excluded": None, "domain": "[2, inf)"}},
              {"inputs": {"a": 1, "b": 3, "c": 1, "d": -1}, "outputs": {"boundary": -3.0, "excluded": 1.0, "domain": "[-3, 1) U (1, inf)"}},
              {"inputs": {"a": -1, "b": 5, "c": 0, "d": 1, "root_in_denominator": True}, "outputs": {"boundary": 5.0, "side": "<", "domain": "(-inf, 5)"}}],
    questions=[{"q": "Find the domain of f(x) = sqrt(2x - 4).", "outputs": {"boundary": 2.0, "side": ">="}},
               {"q": "Find the domain of f(x) = sqrt(x + 3)/(x - 1).", "outputs": {"boundary": -3.0, "excluded": 1.0}},
               {"q": "Find the domain of f(x) = 1/sqrt(5 - x).", "outputs": {"boundary": 5.0, "side": "<"}},
               {"q": "Find the domain of f(x) = sqrt(x - 4)/(x - 2).", "outputs": {"boundary": 4.0, "excluded": None}}])


def inverse_linear_function(a, b, y=None) -> dict:
    _finite(a=a, b=b)
    if a == 0:
        raise ValueError("a must not be 0 (a constant function has no inverse)")
    out = {"inv_slope": 1 / a, "inv_intercept": -b / a}
    if y is not None:
        _finite(y=y)
        out["value"] = (y - b) / a
    return out


def _verify_inv(i, o):
    a, b = i["a"], i["b"]
    r = abs(a * o["inv_slope"] - 1) + abs(a * o["inv_intercept"] + b) / max(1.0, abs(b))
    if i.get("y") is not None:
        r += abs(a * o.get("value", float("inf")) + b - i["y"]) / max(1.0, abs(i["y"]), abs(b)) if "value" in o else 1.0
    return r


def _ref_inv(a, b, y=None):
    import mpmath as mp
    with mp.workdps(30):
        f = lambda t: mp.mpf(repr(float(a))) * t + mp.mpf(repr(float(b)))
        x0, x1 = mp.findroot(lambda t: f(t) - 0, 0), mp.findroot(lambda t: f(t) - 1, 0)     # f^-1(0), f^-1(1)
        out = {"inv_slope": float(x1 - x0), "inv_intercept": float(x0)}
        if y is not None:
            out["value"] = float(mp.findroot(lambda t: f(t) - mp.mpf(repr(float(y))), 0))
    return out


def _parse_inv(g):
    co = _poly_text(g["e"], "x", 1)
    if co is None or len(co) != 2:
        raise ValueError("a linear function")
    out = {"a": co[0], "b": co[1]}
    if g.get("y"):
        out["y"] = _q(g["y"])
    return out


_register(
    "inverse_linear_function", name="Inverse of a linear function", topic="functions / inverse functions",
    purpose="For f(x) = a*x + b (a != 0): f^-1(x) = x/a - b/a, and f^-1(y) for a given y.",
    inputs=[_num_in("a", "slope of f", "a != 0"), _num_in("b", "intercept of f"), _num_in("y", "a value to invert (optional)", required=False)],
    outputs=[_out("inv_slope", "slope of the inverse, 1/a"), _out("inv_intercept", "intercept of the inverse, -b/a"), _out("value", "f^-1(y)")],
    primary="inv_slope", conditions=["a != 0"], method="swap x and y in y = a*x + b and solve for y",
    latex=r"f^{-1}(x) = \frac{x - b}{a}", run=inverse_linear_function, verify=_verify_inv, reference=_ref_inv,
    wrong=lambda a, b, y=None: dict({"inv_slope": 1 / a, "inv_intercept": b / a}, **({"value": (y + b) / a} if y is not None else {})),
    random_inputs=lambda rng: dict({"a": rng.choice([v for v in range(-9, 10) if v]) / rng.choice([1, 2, 4]), "b": float(rng.choice([v for v in range(-20, 21) if v]))},
                                   **({"y": float(rng.randint(-30, 30))} if rng.random() < 0.5 else {})),
    parse=_templated([r"find the inverse(?: function)? of f\(x\) = (?P<e>[^=,]+?)" + _E,
                      r"(?:find|what is) f\^-1\((?P<y>" + _Q + r")\) (?:if|for|when) f\(x\) = (?P<e>[^=,]+?)" + _E],
                     _parse_inv),
    templates=["Find the inverse of f(x) = 3x - 6.", "Find f^-1(9) if f(x) = 3x - 6."],
    examples=[{"inputs": {"a": 3, "b": -6}, "outputs": {"inv_slope": 1 / 3, "inv_intercept": 2.0}},
              {"inputs": {"a": 3, "b": -6, "y": 9}, "outputs": {"value": 5.0}}, {"inputs": {"a": -2, "b": 5}, "outputs": {"inv_slope": -0.5, "inv_intercept": 2.5}}],
    questions=[{"q": "Find the inverse of f(x) = 3x - 6.", "outputs": {"inv_slope": 1 / 3, "inv_intercept": 2.0}},
               {"q": "Find f^-1(9) if f(x) = 3x - 6.", "outputs": {"value": 5.0}},
               {"q": "Find the inverse function of f(x) = -2x + 5.", "outputs": {"inv_slope": -0.5, "inv_intercept": 2.5}}])


def piecewise_evaluate(e1, left_rel, c, e2, x) -> dict:
    _finite(c=c, x=x)
    if left_rel not in ("<", "<="):
        raise ValueError("the first piece must be for x < c or x <= c")
    first = x < c if left_rel == "<" else x <= c
    try:
        v = _fval(e1 if first else e2, x)
    except ValueError as exc:
        raise ValueError(f"f({x:g}) is not defined: {exc}") from None
    return {"value": float(v), "piece": 1.0 if first else 2.0}


def _verify_pw(i, o):
    first = i["x"] < i["c"] if i["left_rel"] == "<" else i["x"] <= i["c"]
    if o["piece"] != (1.0 if first else 2.0):
        return 1.0
    return _rel(o["value"], float(_eval_pratt(i["e1"] if first else i["e2"], {"x": i["x"]}, "x")))


def _ref_pw(e1, left_rel, c, e2, x):
    import sympy as sp
    X = sp.Symbol("x", real=True)
    C = sp.Rational(repr(float(c)))
    cond1 = (X < C) if left_rel == "<" else (X <= C)
    cond2 = (X >= C) if left_rel == "<" else (X > C)
    f = sp.Piecewise((_sym_expr(e1, "x").subs(sp.Symbol("x"), X), cond1), (_sym_expr(e2, "x").subs(sp.Symbol("x"), X), cond2))
    return {"value": float(sp.N(f.subs(X, sp.Rational(repr(float(x)))), 30))}


def _parse_pw(g):
    r1, r2 = g["r1"], g["r2"]
    if _q(g["c1"]) != _q(g["c2"]) or {r1, r2} not in ({"<", ">="}, {"<=", ">"}):
        return {"_out_of_scope": "a piecewise function whose two pieces do not split the line at one point"}
    e1, e2, rel = g["e1"].strip(), g["e2"].strip(), r1
    if r1 in (">", ">="):
        e1, e2, rel = e2, e1, ("<" if r1 == ">=" else "<=")
    return {"e1": e1, "left_rel": rel, "c": _q(g["c1"]), "e2": e2, "x": _q(g["x0"])}


_PW = (r"f\(x\) = (?P<e1>[^=,]+?) (?:if|for) x\s*(?P<r1><=|>=|<|>)\s*(?P<c1>" + _Q + r")(?:,| and|, and)\s*(?P<e2>[^=,]+?) (?:if|for) x\s*(?P<r2><=|>=|<|>)\s*(?P<c2>" + _Q + r")")
_register(
    "piecewise_evaluate", name="Evaluate a piecewise function", topic="functions / piecewise functions",
    purpose="f(x0) for f(x) = e1 when x < c (or x <= c) and e2 otherwise.",
    inputs=[{"symbol": "e1", "type": "text", "kind": "text", "required": True, "min_length": 1, "max_length": 200, "meaning": "formula of the left piece"},
            {"symbol": "left_rel", "type": "choice", "choices": ["<", "<="], "kind": "text", "required": True, "meaning": "the left piece holds for x < c or x <= c"},
            _num_in("c", "the break point c"),
            {"symbol": "e2", "type": "text", "kind": "text", "required": True, "min_length": 1, "max_length": 200, "meaning": "formula of the right piece"},
            _num_in("x", "the number x0")],
    outputs=[_out("value", "f(x0)"), _out("piece", "which piece applies (1 = left, 2 = right)")], primary="value",
    conditions=["the two conditions split the line at c (one < and one >=, or <= and >)", "x0 in the domain of its piece"],
    method="decide which condition x0 satisfies (the break point belongs to the piece whose condition includes it), then evaluate that piece",
    latex=r"f(x) = \begin{cases} e_1(x) & x < c \\ e_2(x) & x \ge c \end{cases}",
    run=piecewise_evaluate, verify=_verify_pw, reference=_ref_pw,
    wrong=lambda e1, left_rel, c, e2, x: _guarded(lambda: (lambda first: {"value": float(_fval(e2 if first else e1, x)), "piece": 2.0 if first else 1.0})(
        x < c if left_rel == "<" else x <= c), {"value": 1e9, "piece": 0.0}),
    random_inputs=lambda rng: (lambda c: {"e1": rng.choice(["2x + 1", "x^2 - 1", "3 - x", "x^3"]), "left_rel": rng.choice(["<", "<="]), "c": c,
                                          "e2": rng.choice(["x^2 + 4", "5x - 2", "10 - 2x", "abs(x) + 7"]), "x": float(c + rng.randint(-3, 3))})(float(rng.randint(-4, 4))),
    parse=_templated([r"(?:evaluate|find|compute) " + _FX + r" (?:for|where|if|when) " + _PW + _E,
                      r"(?:if|given|let) " + _PW + r", (?:find|evaluate|compute|what is) " + _FX + _E],
                     _parse_pw),
    templates=["Evaluate f(3) for f(x) = 2x + 1 if x < 2, x^2 - 1 if x >= 2.", "If f(x) = 3 - x for x <= 0 and x^2 + 4 for x > 0, find f(0)."],
    examples=[{"inputs": {"e1": "2x + 1", "left_rel": "<", "c": 2, "e2": "x^2 - 1", "x": 3}, "outputs": {"value": 8.0, "piece": 2.0}},
              {"inputs": {"e1": "3 - x", "left_rel": "<=", "c": 0, "e2": "x^2 + 4", "x": 0}, "outputs": {"value": 3.0, "piece": 1.0}}],
    questions=[{"q": "Evaluate f(3) for f(x) = 2x + 1 if x < 2, x^2 - 1 if x >= 2.", "outputs": {"value": 8.0, "piece": 2.0}},
               {"q": "If f(x) = 3 - x for x <= 0 and x^2 + 4 for x > 0, find f(0).", "outputs": {"value": 3.0}},
               {"q": "Evaluate f(2) for f(x) = 2x + 1 if x < 2, x^2 - 1 if x >= 2.", "outputs": {"value": 3.0, "piece": 2.0}},
               {"q": "Evaluate f(1) for f(x) = x if x < 2, x^2 if x > 3.", "outcome": "OUT_OF_SCOPE"}])


def graph_transformation_point(x0, y0, A=1.0, B=1.0, h=0.0, k=0.0) -> dict:
    _finite(x0=x0, y0=y0, A=A, B=B, h=h, k=k)
    if A == 0 or B == 0:
        raise ValueError("A and B must not be 0 (the graph would collapse)")
    return {"x_new": x0 / B + h, "y_new": A * y0 + k}


def _ref_transform(x0, y0, A=1.0, B=1.0, h=0.0, k=0.0):
    import sympy as sp
    t, X = sp.symbols("t X")
    R = lambda v: sp.Rational(repr(float(v)))
    f = R(y0) + (t - R(x0)) ** 3 + 2 * (t - R(x0))          # any function whose graph passes through (x0, y0)
    xn = sp.solve(sp.Eq(R(B) * (X - R(h)), R(x0)), X)[0]
    yn = R(A) * f.subs(t, R(B) * (xn - R(h))) + R(k)
    return {"x_new": float(xn), "y_new": float(yn)}


def _parse_transform(g):
    A = g["A"].replace(" ", "") if g["A"] else ""
    A = 1.0 if A in ("", "+") else (-1.0 if A == "-" else _q(A))
    inner = g["inner"].replace(" ", "").lower()
    m = re.fullmatch(r"([-+]?\d+(?:\.\d+)?(?:/\d+)?)?\*?\(x([-+]\d+(?:\.\d+)?(?:/\d+)?)\)", inner)
    if m:
        B = _q(m.group(1)) if m.group(1) not in (None, "", "+") else (-1.0 if m.group(1) == "-" else 1.0)
        h = -_q(m.group(2))
    else:
        co = _poly_text(inner, "x", 1)
        if co is None or len(co) != 2:
            raise ValueError("a linear inside")
        B, h = co[0], -co[1] / co[0]
    k = _q(g["k"].replace(" ", "")) if g.get("k") else 0.0
    return {"x0": _q(g["x0"]), "y0": _q(g["y0"]), "A": A, "B": B, "h": h, "k": k}


_register(
    "graph_transformation_point", name="Image of a point under a graph transformation", topic="functions / transformations",
    purpose="If (x0, y0) is on y = f(x), the corresponding point on y = A*f(B*(x - h)) + k.",
    inputs=[_num_in("x0", "x-coordinate of the point on y = f(x)"), _num_in("y0", "y-coordinate of the point"),
            _num_in("A", "vertical stretch (negative: reflection in the x-axis)", required=False),
            _num_in("B", "horizontal compression factor (negative: reflection in the y-axis)", required=False),
            _num_in("h", "horizontal shift", required=False), _num_in("k", "vertical shift", required=False)],
    outputs=[_out("x_new", "x-coordinate of the image"), _out("y_new", "y-coordinate of the image")], primary="x_new",
    conditions=["A != 0", "B != 0"], method="B(x - h) must equal x0, so x = x0/B + h; the height becomes A*y0 + k",
    latex=r"(x_0, y_0) \mapsto \left(\frac{x_0}{B} + h,\; A y_0 + k\right)",
    run=graph_transformation_point, verify=lambda i, o: (abs(i.get("B", 1.0) * (o["x_new"] - i.get("h", 0.0)) - i["x0"]) / max(1.0, abs(i["x0"]))
                                                           + _rel(o["y_new"], i.get("A", 1.0) * i["y0"] + i.get("k", 0.0))),
    reference=_ref_transform, wrong=lambda x0, y0, A=1.0, B=1.0, h=0.0, k=0.0: {"x_new": x0 * B + h, "y_new": A * y0 + k},
    random_inputs=lambda rng: {"x0": float(rng.randint(-9, 9)), "y0": float(rng.randint(-9, 9)), "A": rng.choice([1.0, 2.0, -1.0, 3.0, 0.5, -2.0]),
                               "B": rng.choice([2.0, 3.0, -1.0, 0.5, -2.0, 4.0]), "h": float(rng.randint(-5, 5)), "k": float(rng.randint(-5, 5))},
    parse=_templated([r"the point \((?P<x0>" + _Q + r"),\s*(?P<y0>" + _Q + r")\) (?:is|lies) on the graph of y = f\(x\)\. (?:find|what is) the "
                      r"(?:corresponding|image) point on the graph of y = (?P<A>[-+]?\s*\d*(?:\.\d+)?(?:/\d+)?)\s*\*?\s*f\((?P<inner>[^=]+?)\)\s*(?P<k>[-+]\s*" + _Q + r")?" + _E],
                     _parse_transform),
    templates=["The point (2, 5) is on the graph of y = f(x). Find the corresponding point on the graph of y = 3f(2(x - 1)) + 4.",
               "The point (-4, 6) is on the graph of y = f(x). Find the corresponding point on the graph of y = -f(x + 3)."],
    examples=[{"inputs": {"x0": 2, "y0": 5, "A": 3, "B": 2, "h": 1, "k": 4}, "outputs": {"x_new": 2.0, "y_new": 19.0}},
              {"inputs": {"x0": -4, "y0": 6, "A": -1, "B": 1, "h": -3, "k": 0}, "outputs": {"x_new": -7.0, "y_new": -6.0}}],
    questions=[{"q": "The point (2, 5) is on the graph of y = f(x). Find the corresponding point on the graph of y = 3f(2(x - 1)) + 4.",
                "outputs": {"x_new": 2.0, "y_new": 19.0}},
               {"q": "The point (-4, 6) is on the graph of y = f(x). Find the corresponding point on the graph of y = -f(x + 3).",
                "outputs": {"x_new": -7.0, "y_new": -6.0}},
               {"q": "The point (3, 1) is on the graph of y = f(x). Find the corresponding point on the graph of y = f(3x) - 2.",
                "outputs": {"x_new": 1.0, "y_new": -1.0}}])


def line_intercepts(a, b, c) -> dict:
    _finite(a=a, b=b, c=c)
    if a == 0 or b == 0:
        raise ValueError("a and b must both be nonzero (otherwise the line is parallel to an axis and misses one of them)")
    return {"x_intercept": c / a, "y_intercept": c / b, "slope": -a / b}


def _ref_intercepts(a, b, c):
    import sympy as sp
    x, y = sp.symbols("x y")
    R = lambda v: sp.Rational(repr(float(v)))
    L = sp.Line(R(a) * x + R(b) * y - R(c), x=x, y=y)
    px = L.intersection(sp.Line(sp.Point(0, 0), sp.Point(1, 0)))[0]
    py = L.intersection(sp.Line(sp.Point(0, 0), sp.Point(0, 1)))[0]
    return {"x_intercept": float(px.x), "y_intercept": float(py.y), "slope": float(L.slope)}


def _parse_intercepts(g):
    row = _lin_equation(g["eq"], "xy")
    if row is None:
        raise ValueError("a linear equation in x and y")
    return {"a": row[0], "b": row[1], "c": row[2]}


_register(
    "line_intercepts", name="Intercepts and slope of a line", topic="functions / lines",
    purpose="The x- and y-intercepts and the slope of the line a*x + b*y = c.",
    inputs=[_num_in("a", "coefficient of x", "a != 0"), _num_in("b", "coefficient of y", "b != 0"), _num_in("c", "right-hand side")],
    outputs=[_out("x_intercept", "where the line meets the x-axis (y = 0)"), _out("y_intercept", "where it meets the y-axis (x = 0)"),
             _out("slope", "the slope -a/b")], primary="x_intercept", conditions=["a != 0", "b != 0"],
    method="set y = 0 for the x-intercept and x = 0 for the y-intercept", latex=r"ax + by = c:\quad x_0 = \frac{c}{a},\ y_0 = \frac{c}{b},\ m = -\frac{a}{b}",
    run=line_intercepts, verify=lambda i, o: (abs(i["a"] * o["x_intercept"] - i["c"]) + abs(i["b"] * o["y_intercept"] - i["c"])) / max(1.0, abs(i["c"]))
                                             + abs(o["slope"] * i["b"] + i["a"]) / max(1.0, abs(i["a"])),
    reference=_ref_intercepts, wrong=lambda a, b, c: {"x_intercept": c / b, "y_intercept": c / a, "slope": -b / a},
    random_inputs=lambda rng: {"a": float(rng.choice([v for v in range(-9, 10) if v])), "b": float(rng.choice([v for v in range(-9, 10) if v])),
                               "c": float(rng.randint(-30, 30))},
    parse=_templated([r"(?:find|what are) the (?:x- and y-intercepts|intercepts) of (?:the line )?(?P<eq>[^=]+=[^=]+?)" + _E], _parse_intercepts),
    templates=["Find the x- and y-intercepts of 2x + 3y = 12.", "Find the intercepts of the line y = 2x - 6."],
    examples=[{"inputs": {"a": 2, "b": 3, "c": 12}, "outputs": {"x_intercept": 6.0, "y_intercept": 4.0, "slope": -2 / 3}},
              {"inputs": {"a": -2, "b": 1, "c": -6}, "outputs": {"x_intercept": 3.0, "y_intercept": -6.0, "slope": 2.0}}],
    questions=[{"q": "Find the x- and y-intercepts of 2x + 3y = 12.", "outputs": {"x_intercept": 6.0, "y_intercept": 4.0}},
               {"q": "Find the intercepts of the line y = 2x - 6.", "outputs": {"x_intercept": 3.0, "y_intercept": -6.0, "slope": 2.0}},
               {"q": "Find the intercepts of the line y = 5.", "outcome": "INVALID_INPUT"}])


def distance_point_to_line(x0, y0, a, b, c) -> dict:
    _finite(x0=x0, y0=y0, a=a, b=b, c=c)
    if a == 0 and b == 0:
        raise ValueError("a and b must not both be 0 (that is not a line)")
    return {"distance": abs(a * x0 + b * y0 + c) / math.hypot(a, b)}


def _verify_dist_line(i, o):
    a, b, c, x0, y0 = i["a"], i["b"], i["c"], i["x0"], i["y0"]
    n = math.hypot(a, b)
    t = -(a * x0 + b * y0 + c) / (n * n)                  # the foot of the perpendicular
    fx, fy = x0 + a * t, y0 + b * t
    return abs(o["distance"] - math.hypot(fx - x0, fy - y0)) / max(1.0, o["distance"]) + abs(a * fx + b * fy + c) / max(1.0, abs(c), abs(a * fx), abs(b * fy))


def _ref_dist_line(x0, y0, a, b, c):
    import sympy as sp
    R = lambda v: sp.Rational(repr(float(v)))
    x, y = sp.symbols("x y")
    return {"distance": float(sp.N(sp.Point(R(x0), R(y0)).distance(sp.Line(R(a) * x + R(b) * y + R(c), x=x, y=y)), 30))}


def _parse_dist_line(g):
    row = _lin_equation(g["eq"], "xy")
    if row is None:
        raise ValueError("a linear equation in x and y")
    return {"x0": _q(g["x0"]), "y0": _q(g["y0"]), "a": row[0], "b": row[1], "c": -row[2]}


_register(
    "distance_point_to_line", name="Distance from a point to a line", topic="functions / lines",
    purpose="The perpendicular distance from (x0, y0) to the line a*x + b*y + c = 0.",
    inputs=[_num_in("x0", "x-coordinate of the point"), _num_in("y0", "y-coordinate of the point"), _num_in("a", "coefficient of x"),
            _num_in("b", "coefficient of y"), _num_in("c", "constant term")],
    outputs=[_out("distance", "the perpendicular distance")], primary="distance", conditions=["a and b not both 0"],
    method="|a x0 + b y0 + c| / sqrt(a^2 + b^2)", latex=r"d = \frac{|a x_0 + b y_0 + c|}{\sqrt{a^2 + b^2}}",
    run=distance_point_to_line, verify=_verify_dist_line, reference=_ref_dist_line,
    wrong=lambda x0, y0, a, b, c: {"distance": abs(a * x0 + b * y0 + c) / (a * a + b * b)},
    random_inputs=lambda rng: {"x0": float(rng.randint(-9, 9)), "y0": float(rng.randint(-9, 9)), "a": float(rng.choice([v for v in range(-6, 7) if v])),
                               "b": float(rng.randint(-6, 6)), "c": float(rng.randint(-15, 15))},
    parse=_templated([r"(?:find|what is) the distance from the point \((?P<x0>" + _Q + r"),\s*(?P<y0>" + _Q + r")\) to the line (?P<eq>[^=]+=[^=]+?)" + _E],
                     _parse_dist_line),
    templates=["Find the distance from the point (1, 2) to the line 3x + 4y - 5 = 0.", "Find the distance from the point (0, 0) to the line y = 2x + 5."],
    examples=[{"inputs": {"x0": 1, "y0": 2, "a": 3, "b": 4, "c": -5}, "outputs": {"distance": 1.2}},
              {"inputs": {"x0": 0, "y0": 0, "a": -2, "b": 1, "c": -5}, "outputs": {"distance": 5 / math.sqrt(5)}}],
    questions=[{"q": "Find the distance from the point (1, 2) to the line 3x + 4y - 5 = 0.", "outputs": {"distance": 1.2}},
               {"q": "Find the distance from the point (0, 0) to the line y = 2x + 5.", "outputs": {"distance": 5 / math.sqrt(5)}}])


# =====================================================================================================================
# exponentials and logarithms
# =====================================================================================================================
def log_evaluate(b, x) -> dict:
    _finite(b=b, x=x)
    if b <= 0 or b == 1:
        raise ValueError("the base must be positive and not 1")
    if x <= 0:
        raise ValueError("the logarithm is defined only for positive numbers")
    v = math.log2(x) / math.log2(b) if b == 2 else (math.log10(x) / math.log10(b) if b == 10 else math.log(x) / math.log(b))
    n = round(v)
    if abs(v - n) < 1e-12 and b == int(b) and abs(n) <= 60 and Fraction(int(b)) ** n == _fr(x):
        v = float(n)                                   # an exact power of the base
    return {"value": v}


def _ref_log(b, x):
    import mpmath as mp
    with mp.workdps(30):
        return {"value": float(mp.log(mp.mpf(repr(float(x))), mp.mpf(repr(float(b)))))}


def _parse_log(g):
    if g.get("ln"):
        return {"b": math.e, "x": _q(g["x"])}
    return {"b": _q(g["b"]) if g.get("b") else 10.0, "x": _q(g["x"])}


_LOGQ = r"(?:evaluate|find|compute|calculate|what is) "
_register(
    "log_evaluate", name="Evaluate a logarithm", topic="exponentials and logarithms / logarithms",
    purpose="log_b(x) for a positive base b != 1 and x > 0 (log is base 10, ln base e).",
    inputs=[_num_in("b", "the base (positive, not 1)"), _num_in("x", "the number (positive)")],
    outputs=[_out("value", "log_b(x)")], primary="value", conditions=["b > 0", "b != 1", "x > 0"],
    method="change of base: log_b(x) = ln(x)/ln(b)", latex=r"\log_b x = \frac{\ln x}{\ln b}",
    run=log_evaluate, verify=lambda i, o: abs(math.log(i["x"]) - o["value"] * math.log(i["b"])) / max(1.0, abs(math.log(i["x"]))),
    reference=_ref_log, wrong=lambda b, x: {"value": math.log(b) / math.log(x) if x != 1 else 1.0},
    random_inputs=lambda rng: {"b": rng.choice([2.0, 3.0, 10.0, math.e, 0.5, 1.5, 7.0, 5.0]), "x": rng.choice([round(rng.uniform(0.01, 10000), 3), float(rng.choice([8, 81, 125, 1000, 0.25, 49]))]) or 2.0},
    parse=_templated([_LOGQ + r"log base (?P<b>" + _Q + r") of (?P<x>" + _Q + r")" + _E,
                      _LOGQ + r"log_\{?(?P<b>" + _Q + r")\}?\s*\(?(?P<x>" + _Q + r")\)?" + _E,
                      _LOGQ + r"(?P<ln>ln)\s*\(?(?P<x>" + _Q + r")\)?" + _E,
                      _LOGQ + r"log\s*\((?P<x>" + _Q + r")\)" + _E],
                     _parse_log),
    templates=["Evaluate log base 2 of 32.", "Find log_3(81).", "Compute ln(20).", "Find log(1000)."],
    examples=[{"inputs": {"b": 2, "x": 32}, "outputs": {"value": 5.0}}, {"inputs": {"b": 3, "x": 81}, "outputs": {"value": 4.0}},
              {"inputs": {"b": math.e, "x": 20}, "outputs": {"value": math.log(20)}}, {"inputs": {"b": 10, "x": 1000}, "outputs": {"value": 3.0}},
              {"inputs": {"b": 2, "x": 0.125}, "outputs": {"value": -3.0}}],
    questions=[{"q": "Evaluate log base 2 of 32.", "outputs": {"value": 5.0}}, {"q": "Find log_3(81).", "outputs": {"value": 4.0}},
               {"q": "Compute ln(20).", "outputs": {"value": math.log(20)}}, {"q": "Find log(1000).", "outputs": {"value": 3.0}},
               {"q": "Evaluate log base 2 of -8.", "outcome": "INVALID_INPUT"}, {"q": "Evaluate log base 1 of 5.", "outcome": "INVALID_INPUT"}])


def log_laws_combine(u, v, p, q) -> dict:
    _finite(u=u, v=v, p=p, q=q)
    return {"value": p * u + q * v}


def _ref_loglaws(u, v, p, q):
    import mpmath as mp
    with mp.workdps(40):                                 # base 2 with M = 2^u, N = 2^v: the logarithm of the actual number
        M, N = mp.mpf(2) ** mp.mpf(repr(float(u))), mp.mpf(2) ** mp.mpf(repr(float(v)))
        return {"value": float(mp.log(M ** mp.mpf(repr(float(p))) * N ** mp.mpf(repr(float(q))), 2))}


def _parse_logarg(arg: str):
    a = arg.replace(" ", "").replace("*", "")
    num, _, den = a.partition("/")
    exps = {"m": Fraction(0), "n": Fraction(0)}
    for part, sign in ((num, 1), (den, -1)):
        if not part:
            continue
        pos = 0
        for m in re.finditer(r"sqrt\(([mn])\)|([mn])(?:\^\(?(-?\d+(?:/\d+)?)\)?)?", part):
            if m.start() != pos:
                raise ValueError("unreadable argument")
            pos = m.end()
            if m.group(1):
                exps[m.group(1)] += sign * Fraction(1, 2)
            else:
                exps[m.group(2)] += sign * Fraction(m.group(3) or 1)
        if pos != len(part):
            raise ValueError("unreadable argument")
    return float(exps["m"]), float(exps["n"])


def _parse_loglaws(g):
    p, q = _parse_logarg(g["arg"].lower())
    return {"u": _q(g["u"]), "v": _q(g["v"]), "p": p, "q": q}


_register(
    "log_laws_combine", name="Laws of logarithms (product, quotient, power)", topic="exponentials and logarithms / laws of logarithms",
    purpose="Given log_b(M) = u and log_b(N) = v, the value of log_b(M^p * N^q) (quotients and roots included).",
    inputs=[_num_in("u", "log_b(M)"), _num_in("v", "log_b(N)"), _num_in("p", "power of M"), _num_in("q", "power of N (negative: N in the denominator)")],
    outputs=[_out("value", "log_b(M^p N^q)")], primary="value", conditions=["M, N > 0"],
    method="product law log(MN) = log M + log N, quotient law, power law log(M^p) = p log M",
    latex=r"\log_b\!\left(M^{p} N^{q}\right) = p\log_b M + q\log_b N",
    run=log_laws_combine, verify=lambda i, o: abs(o["value"] - (i["p"] * i["u"] + i["q"] * i["v"])) / max(1.0, abs(i["p"] * i["u"]) + abs(i["q"] * i["v"])),
    reference=_ref_loglaws, wrong=lambda u, v, p, q: {"value": (p * u) * (q * v)},
    random_inputs=lambda rng: {"u": float(rng.randint(-8, 8)) or 1.5, "v": float(rng.randint(-8, 8)) or -2.5, "p": float(rng.choice([1, 2, 3, -1, 0.5, 4])),
                               "q": float(rng.choice([1, 2, -1, -2, 0.5, 3, -3]))},
    parse=_templated([r"(?:given|if) log_\{?b\}?\s*\(m\) = (?P<u>" + _Q + r") and log_\{?b\}?\s*\(n\) = (?P<v>" + _Q
                      + r"),? (?:find|evaluate|what is) log_\{?b\}?\s*\((?P<arg>[mn0-9^()/*\s\-sqrt]+)\)" + _E],
                     _parse_loglaws),
    templates=["Given log_b(M) = 2 and log_b(N) = 3, find log_b(M^2 N).", "If log_b(M) = 1.5 and log_b(N) = 4, find log_b(M^3/N)."],
    examples=[{"inputs": {"u": 2, "v": 3, "p": 2, "q": 1}, "outputs": {"value": 7.0}}, {"inputs": {"u": 1.5, "v": 4, "p": 3, "q": -1}, "outputs": {"value": 0.5}}],
    questions=[{"q": "Given log_b(M) = 2 and log_b(N) = 3, find log_b(M^2 N).", "outputs": {"value": 7.0}},
               {"q": "If log_b(M) = 1.5 and log_b(N) = 4, find log_b(M^3/N).", "outputs": {"value": 0.5}},
               {"q": "Given log_b(M) = 6 and log_b(N) = 2, find log_b(sqrt(M)/N^2).", "outputs": {"value": -1.0}}])


_TU = r"(?P<tu>years?|months?|weeks?|days?|hours?|minutes?|seconds?)"


def _same_unit(a, b):
    return a.lower().rstrip("s") == b.lower().rstrip("s")


def continuous_growth(n0, k, t) -> dict:
    _finite(n0=n0, k=k, t=t)
    if abs(k * t) > 700:
        raise ValueError("k*t is too large for a finite result")
    return {"amount": n0 * math.exp(k * t)}


def _ref_cgrowth(n0, k, t):
    import mpmath as mp
    with mp.workdps(25):                                 # dN/dt = k N, N(0) = n0, integrated numerically
        K = mp.mpf(repr(float(k)))
        sol = mp.odefun(lambda s, y: K * y, 0, mp.mpf(repr(float(n0))))
        return {"amount": float(sol(mp.mpf(repr(float(t)))))}


def _parse_cgrowth(g):
    if g.get("tu2") and not _same_unit(g["tu"], g["tu2"]):
        raise ValueError("the rate and the time must use the same unit")
    k = _q(g["r"]) / 100
    k = -k if (g.get("dir") or "").lower().startswith("decay") else k
    return {"n0": _q(g["n0"]), "k": k, "t": _q(g["t"])}


_register(
    "continuous_growth", name="Continuous exponential growth or decay", topic="exponentials and logarithms / growth and decay",
    purpose="N(t) = N0 * e^(k t) for continuous growth (k > 0) or decay (k < 0), including continuous compounding.",
    inputs=[_num_in("n0", "initial amount N0"), _num_in("k", "continuous rate per unit time, as a decimal (0.03 for 3%; negative for decay)"),
            _num_in("t", "elapsed time, in the rate's unit")],
    outputs=[_out("amount", "N(t)")], primary="amount", conditions=["|k t| <= 700"],
    method="the solution of dN/dt = kN: N = N0 e^(kt)", latex=r"N(t) = N_0\,e^{kt}",
    run=continuous_growth, verify=lambda i, o: _rel(o["amount"], i["n0"] * math.exp(i["k"] * i["t"])), reference=_ref_cgrowth,
    wrong=lambda n0, k, t: {"amount": n0 * (1 + k) ** t},
    random_inputs=lambda rng: {"n0": round(rng.uniform(1, 5000), 1), "k": rng.choice([1, -1]) * round(rng.uniform(0.01, 0.25), 3), "t": round(rng.uniform(1, 30), 1)},
    parse=_templated([r"a (?:population|quantity|culture|colony|sample|amount) of (?P<n0>" + _W + r") (?P<dir>grows|decays) continuously at (?:a rate of )?(?P<r>" + _W
                      + r")\s*% per " + _TU + r"\. (?:find|what is) the (?:population|amount|quantity|size) after (?P<t>" + _W + r") (?P<tu2>years?|months?|weeks?|days?|hours?|minutes?|seconds?)" + _E,
                      r"(?:find|what is) the amount (?:when|if) (?P<n0>" + _W + r") is compounded continuously at (?P<r>" + _W + r")\s*% (?:a year |per year |annually )?for (?P<t>" + _W + r") (?P<tu>years?)" + _E],
                     _parse_cgrowth),
    templates=["A population of 500 grows continuously at 3% per year. Find the population after 10 years.",
               "Find the amount when 1000 is compounded continuously at 5% for 3 years."],
    examples=[{"inputs": {"n0": 500, "k": 0.03, "t": 10}, "outputs": {"amount": 500 * math.exp(0.3)}},
              {"inputs": {"n0": 1000, "k": 0.05, "t": 3}, "outputs": {"amount": 1000 * math.exp(0.15)}},
              {"inputs": {"n0": 200, "k": -0.05, "t": 8}, "outputs": {"amount": 200 * math.exp(-0.4)}}],
    questions=[{"q": "A population of 500 grows continuously at 3% per year. Find the population after 10 years.", "outputs": {"amount": 500 * math.exp(0.3)}},
               {"q": "Find the amount when 1000 is compounded continuously at 5% for 3 years.", "outputs": {"amount": 1000 * math.exp(0.15)}},
               {"q": "A quantity of 200 decays continuously at a rate of 5% per hour. Find the amount after 8 hours.", "outputs": {"amount": 200 * math.exp(-0.4)}},
               {"q": "A population of 500 grows continuously at 3% per year. Find the population after 10 days.", "outcome": "UNRECOGNIZED_FORMAT"}])


def half_life_remaining(n0, half_life, t) -> dict:
    _finite(n0=n0, half_life=half_life, t=t)
    if n0 <= 0 or half_life <= 0 or t < 0:
        raise ValueError("the amount and the half-life must be positive and the time not negative")
    frac = 0.5 ** (t / half_life)
    return {"remaining": n0 * frac, "fraction": frac}


def _ref_halflife(n0, half_life, t):
    from scipy.integrate import solve_ivp                # first-order decay dN/dt = -(ln 2 / T) N, integrated numerically
    k = -math.log(2) / float(half_life)
    sol = solve_ivp(lambda s, y: k * y, (0.0, float(t)), [1.0], method="DOP853", rtol=1e-13, atol=1e-300)
    frac = float(sol.y[0][-1])
    return {"remaining": float(n0) * frac, "fraction": frac}


def _parse_halflife(g):
    if not _same_unit(g["tu"], g["tu2"]):
        raise ValueError("the half-life and the time must use the same unit")
    return {"n0": _q(g["n0"]), "half_life": _q(g["h"]), "t": _q(g["t"])}


_MASS = r"(?: ?(?:g|mg|kg|grams?|milligrams?|kilograms?|units?|atoms|nuclei|mol|moles))?"
_register(
    "half_life_remaining", name="Amount remaining after radioactive decay (half-life)", topic="exponentials and logarithms / growth and decay",
    purpose="The amount left after time t of a substance with half-life T: N0 * (1/2)^(t/T).",
    inputs=[_num_in("n0", "initial amount (positive)"), _num_in("half_life", "the half-life T (positive)"), _num_in("t", "elapsed time, in the unit of T")],
    outputs=[_out("remaining", "amount remaining (in the unit of N0)"), _out("fraction", "fraction remaining")], primary="remaining",
    conditions=["n0 > 0", "T > 0", "t >= 0", "t and T in the same unit"],
    method="each half-life halves the amount: N = N0 (1/2)^(t/T)", latex=r"N(t) = N_0 \left(\tfrac12\right)^{t/T}",
    run=half_life_remaining, verify=lambda i, o: _rel(o["remaining"], i["n0"] * 2 ** (-i["t"] / i["half_life"])) + _rel(o["fraction"] * i["n0"], o["remaining"]),
    reference=_ref_halflife, wrong=lambda n0, half_life, t: {"remaining": n0 * (1 - 0.5 * t / half_life), "fraction": 1 - 0.5 * t / half_life},
    random_inputs=lambda rng: {"n0": round(rng.uniform(1, 1000), 1), "half_life": round(rng.uniform(0.5, 50), 1), "t": round(rng.uniform(0.5, 100), 1)},
    parse=_templated([r"an? (?P<n0>" + _W + r")" + _MASS + r" sample(?: of [a-z0-9\- ]+?)? has a half-life of (?P<h>" + _W + r") " + _TU
                      + r"\. how much (?:of it )?(?:remains|is left) after (?P<t>" + _W + r") (?P<tu2>years?|months?|weeks?|days?|hours?|minutes?|seconds?)" + _E,
                      r"a sample of (?P<n0>" + _W + r")" + _MASS + r"(?: of [a-z0-9\- ]+?)? has a half-life of (?P<h>" + _W + r") " + _TU
                      + r"\. how much (?:of it )?(?:remains|is left) after (?P<t>" + _W + r") (?P<tu2>years?|months?|weeks?|days?|hours?|minutes?|seconds?)" + _E],
                     _parse_halflife),
    templates=["An 80 g sample has a half-life of 5 years. How much remains after 15 years?",
               "A sample of 200 mg of iodine-131 has a half-life of 8 days. How much is left after 20 days?"],
    examples=[{"inputs": {"n0": 80, "half_life": 5, "t": 15}, "outputs": {"remaining": 10.0, "fraction": 0.125}},
              {"inputs": {"n0": 200, "half_life": 8, "t": 20}, "outputs": {"remaining": 200 * 0.5 ** 2.5}}],
    questions=[{"q": "An 80 g sample has a half-life of 5 years. How much remains after 15 years?", "outputs": {"remaining": 10.0}},
               {"q": "A sample of 200 mg of iodine-131 has a half-life of 8 days. How much is left after 20 days?", "outputs": {"remaining": 200 * 0.5 ** 2.5}},
               {"q": "An 80 g sample has a half-life of 5 years. How much remains after 15 days?", "outcome": "UNRECOGNIZED_FORMAT"}])


def doubling_time_percent(r) -> dict:
    _finite(r=r)
    if r <= 0:
        raise ValueError("the growth rate must be positive")
    return {"doubling_time": math.log(2) / math.log1p(r / 100), "rule_of_70_estimate": 70 / r}


def _ref_doubling(r):
    import mpmath as mp
    with mp.workdps(30):
        g = 1 + mp.mpf(repr(float(r))) / 100
        return {"doubling_time": float(mp.findroot(lambda t: g ** t - 2, 70 / float(r)))}


_register(
    "doubling_time_percent", name="Doubling time at a percentage growth rate", topic="exponentials and logarithms / growth and decay",
    purpose="The time for a quantity growing by r% per period (compounded once per period) to double.",
    inputs=[_num_in("r", "growth rate per period, in percent (positive)")],
    outputs=[_out("doubling_time", "periods to double (exact)"), _out("rule_of_70_estimate", "the rule-of-70 approximation 70/r")],
    primary="doubling_time", conditions=["r > 0"], method="solve (1 + r/100)^T = 2: T = ln 2 / ln(1 + r/100)",
    latex=r"T = \frac{\ln 2}{\ln\left(1 + \frac{r}{100}\right)}",
    run=doubling_time_percent, verify=lambda i, o: abs((1 + i["r"] / 100) ** o["doubling_time"] - 2) / 2 + _rel(o["rule_of_70_estimate"], 70 / i["r"]),
    reference=_ref_doubling, wrong=lambda r: {"doubling_time": 70 / r, "rule_of_70_estimate": 70 / r},
    random_inputs=lambda rng: {"r": round(rng.uniform(0.5, 40), 2)},
    parse=_templated([r"how long does it take (?:an investment|a population|a quantity|money|it) to double at (?P<r>" + _W + r")\s*% (?:interest |growth )?(?:compounded annually|per year|a year|annually)" + _E,
                      r"what is the doubling time at a growth rate of (?P<r>" + _W + r")\s*% per (?:year|period|month|day)" + _E],
                     lambda g: {"r": _q(g["r"])}),
    templates=["How long does it take an investment to double at 7% interest compounded annually?", "What is the doubling time at a growth rate of 3% per year?"],
    examples=[{"inputs": {"r": 7}, "outputs": {"doubling_time": math.log(2) / math.log(1.07)}}, {"inputs": {"r": 100}, "outputs": {"doubling_time": 1.0}}],
    questions=[{"q": "How long does it take an investment to double at 7% interest compounded annually?", "outputs": {"doubling_time": math.log(2) / math.log(1.07)}},
               {"q": "What is the doubling time at a growth rate of 3% per year?", "outputs": {"doubling_time": math.log(2) / math.log(1.03)}}])


def exponential_equation_general(a, b, c, d, k) -> dict:
    _finite(a=a, b=b, c=c, d=d, k=k)
    if a == 0 or c == 0:
        raise ValueError("a and c must not be 0")
    if b <= 0 or b == 1:
        raise ValueError("the base must be positive and not 1")
    if k / a <= 0:
        raise ValueError("no real solution: b^(cx + d) is positive, so k/a must be positive")
    return {"x": (math.log(k / a) / math.log(b) - d) / c}


def _ref_expeq(a, b, c, d, k):
    import sympy as sp
    x = sp.Symbol("x", real=True)
    R = lambda v: sp.Rational(repr(float(v)))
    base = sp.E if float(b) == math.e else R(b)
    s = sp.solveset(sp.Eq(R(a) * base ** (R(c) * x + R(d)), R(k)), x, sp.S.Reals)
    return {"x": float(sp.N(list(s)[0], 30))}


def _parse_expeq(g):
    lin = (g.get("lin") or g.get("lin2")).strip()
    co = _poly_text(lin, "x", 1)
    if co is None or len(co) != 2:
        raise ValueError("a linear exponent")
    a = g.get("a")
    return {"a": _q(a) if a else 1.0, "b": math.e if g["b"].lower() == "e" else _q(g["b"]), "c": co[0], "d": co[1], "k": _q(g["k"])}


_register(
    "exponential_equation_general", name="Exponential equation a*b^(cx + d) = k", topic="exponentials and logarithms / exponential equations",
    purpose="Solve a * b^(c*x + d) = k for x by taking logarithms.",
    inputs=[_num_in("a", "the coefficient a", "a != 0"), _num_in("b", "the base (positive, not 1; e for the natural base)"),
            _num_in("c", "coefficient of x in the exponent", "c != 0"), _num_in("d", "constant in the exponent"), _num_in("k", "the right-hand side")],
    outputs=[_out("x", "the solution")], primary="x", conditions=["a != 0", "c != 0", "b > 0, b != 1", "k/a > 0"],
    method="divide by a, take log base b: c x + d = log_b(k/a)", latex=r"x = \frac{\log_b(k/a) - d}{c}",
    run=exponential_equation_general, verify=lambda i, o: abs(i["a"] * i["b"] ** (i["c"] * o["x"] + i["d"]) - i["k"]) / max(1.0, abs(i["k"])),
    reference=_ref_expeq, wrong=lambda a, b, c, d, k: {"x": (math.log(abs(k)) / math.log(b) - d) / c},
    random_inputs=lambda rng: {"a": rng.choice([2.0, 3.0, 5.0, 0.5, 4.0, 1.5]), "b": rng.choice([2.0, 3.0, 5.0, 10.0, math.e, 0.5, 1.5]),
                               "c": float(rng.choice([1, 2, 3, -1, -2, 4])), "d": float(rng.randint(-5, 5)), "k": round(rng.uniform(0.5, 1000), 2)},
    parse=_templated([r"solve (?:(?P<a>" + _Q + r")\s*\*?\s*)?(?P<b>" + _W + r"|e)\s*\^\s*\((?P<lin>[^()]+)\)\s*=\s*(?P<k>" + _Q + r")(?: for x)?" + _E,
                      r"solve (?P<a>" + _Q + r")\s*\*?\s*(?P<b>" + _W + r"|e)\s*\^\s*(?P<lin2>[-+]?\d*(?:\.\d+)?x)\s*=\s*(?P<k>" + _Q + r")(?: for x)?" + _E],
                     _parse_expeq),
    templates=["Solve 3 * 2^(x + 1) = 48.", "Solve 5e^(2x) = 20.", "Solve 2^(3x - 1) = 16."],
    examples=[{"inputs": {"a": 3, "b": 2, "c": 1, "d": 1, "k": 48}, "outputs": {"x": 3.0}}, {"inputs": {"a": 5, "b": math.e, "c": 2, "d": 0, "k": 20}, "outputs": {"x": math.log(4) / 2}},
              {"inputs": {"a": 1, "b": 2, "c": 3, "d": -1, "k": 16}, "outputs": {"x": 5 / 3}}],
    questions=[{"q": "Solve 3 * 2^(x + 1) = 48.", "outputs": {"x": 3.0}}, {"q": "Solve 5e^(2x) = 20.", "outputs": {"x": math.log(4) / 2}},
               {"q": "Solve 2^(3x - 1) = 16.", "outputs": {"x": 5 / 3}}, {"q": "Solve 4 * 3^x = 324.", "outputs": {"x": 4.0}},
               {"q": "Solve 2 * 3^(x - 1) = -6.", "outcome": "INVALID_INPUT"}])


def log_equation_linear_argument(b, c, d, k) -> dict:
    _finite(b=b, c=c, d=d, k=k)
    if b <= 0 or b == 1:
        raise ValueError("the base must be positive and not 1")
    if c == 0:
        raise ValueError("c must not be 0 (no x inside the logarithm)")
    if abs(k * math.log(b)) > 700:
        raise ValueError("the solution is too large for a finite number")
    return {"x": (b ** k - d) / c}


def _ref_logeq(b, c, d, k):
    import sympy as sp
    x = sp.Symbol("x", real=True)
    R = lambda v: sp.Rational(repr(float(v)))
    base = sp.E if float(b) == math.e else R(b)
    s = sp.solve(sp.Eq(sp.log(R(c) * x + R(d), base), R(k)), x)
    return {"x": float(sp.N(s[0], 30))}


def _parse_logeq(g):
    co = _poly_text(g["arg"], "x", 1)
    if co is None or len(co) != 2:
        raise ValueError("a linear argument")
    b = math.e if g.get("ln") else (_q(g["b"]) if g.get("b") else 10.0)
    return {"b": b, "c": co[0], "d": co[1], "k": _q(g["k"])}


def _verify_logeq(i, o):
    arg = i["c"] * o["x"] + i["d"]
    if arg <= 0:
        return 1.0
    return abs(math.log(arg) / math.log(i["b"]) - i["k"]) / max(1.0, abs(i["k"]))


_register(
    "log_equation_linear_argument", name="Logarithmic equation log_b(cx + d) = k", topic="exponentials and logarithms / logarithmic equations",
    purpose="Solve log_b(c*x + d) = k for x by rewriting it in exponential form.",
    inputs=[_num_in("b", "the base (positive, not 1; 10 for log, e for ln)"), _num_in("c", "coefficient of x in the argument", "c != 0"),
            _num_in("d", "constant in the argument"), _num_in("k", "the right-hand side")],
    outputs=[_out("x", "the solution")], primary="x", conditions=["b > 0, b != 1", "c != 0"],
    method="log_b(u) = k means u = b^k; the argument b^k is positive, so the solution is always admissible",
    latex=r"\log_b(cx+d) = k \iff cx + d = b^{k}", run=log_equation_linear_argument, verify=_verify_logeq, reference=_ref_logeq,
    wrong=lambda b, c, d, k: {"x": (b * k - d) / c},
    random_inputs=lambda rng: {"b": rng.choice([2.0, 3.0, 10.0, math.e, 5.0, 0.5]), "c": float(rng.choice([1, 2, 3, -1, 5, -2])),
                               "d": float(rng.randint(-9, 9)), "k": float(rng.choice([rng.randint(-3, 5), round(rng.uniform(-2, 4), 2)]))},
    parse=_templated([r"solve log base (?P<b>" + _Q + r") of \((?P<arg>[^()]+)\)\s*=\s*(?P<k>" + _Q + r")(?: for x)?" + _E,
                      r"solve log_\{?(?P<b>" + _Q + r")\}?\s*\((?P<arg>[^()]+)\)\s*=\s*(?P<k>" + _Q + r")(?: for x)?" + _E,
                      r"solve (?P<ln>ln)\s*\((?P<arg>[^()]+)\)\s*=\s*(?P<k>" + _Q + r")(?: for x)?" + _E,
                      r"solve log\s*\((?P<arg>[^()]+)\)\s*=\s*(?P<k>" + _Q + r")(?: for x)?" + _E,
                      r"solve for x in the equation log_?(?:base)?_?\{?(?P<b>" + _Q + r")\}?\s*\((?P<arg>[^()]+)\)\s*=\s*(?P<k>" + _Q + r")" + _E],
                     _parse_logeq),
    templates=["Solve log base 2 of (3x + 1) = 4.", "Solve ln(2x - 1) = 3.", "Solve log(5x) = 2."],
    examples=[{"inputs": {"b": 2, "c": 3, "d": 1, "k": 4}, "outputs": {"x": 5.0}}, {"inputs": {"b": math.e, "c": 2, "d": -1, "k": 3}, "outputs": {"x": (math.exp(3) + 1) / 2}},
              {"inputs": {"b": 10, "c": 5, "d": 0, "k": 2}, "outputs": {"x": 20.0}}],
    questions=[{"q": "Solve log base 2 of (3x + 1) = 4.", "outputs": {"x": 5.0}}, {"q": "Solve log_3(x - 4) = 2.", "outputs": {"x": 13.0}},
               {"q": "Solve ln(2x - 1) = 3.", "outputs": {"x": (math.exp(3) + 1) / 2}}, {"q": "Solve log(5x) = 2.", "outputs": {"x": 20.0}},
               {"q": "Solve for x in the equation log_3(x - 1) = 2.", "outputs": {"x": 10.0}},
               {"q": "Solve log base 1 of (2x) = 3.", "outcome": "INVALID_INPUT"}])


# =====================================================================================================================
# trigonometry (angles in degrees unless stated; a triangle's angles A, B, C are opposite its sides a, b, c)
# =====================================================================================================================
def _pi_angle(text: str) -> float:
    """'3pi/4', 'pi', '-pi/6', '2.5' (radians) -> a float in radians."""
    t = text.replace(" ", "").lower()
    m = re.fullmatch(r"([-+]?\d*(?:\.\d+)?)\*?pi(?:/(\d+))?", t)
    if m:
        k = m.group(1)
        k = 1.0 if k in ("", "+") else (-1.0 if k == "-" else float(k))
        return k * math.pi / (float(m.group(2)) if m.group(2) else 1.0)
    return float(Fraction(t))


def _pi_degrees(text: str) -> float:
    """'3pi/4' -> 135.0 exactly (a multiple of pi converted with exact arithmetic)."""
    t = text.replace(" ", "").lower()
    m = re.fullmatch(r"([-+]?\d*)\*?pi(?:/(\d+))?", t)
    if not m:
        raise ValueError("a multiple of pi")
    k = m.group(1)
    k = 1 if k in ("", "+") else (-1 if k == "-" else int(k))
    return float(Fraction(180 * k, int(m.group(2)) if m.group(2) else 1))


def deg_to_rad(deg) -> dict:
    _finite(deg=deg)
    f = _fr(deg) / 180
    exact = ""
    if f.denominator <= 360:
        n, d = f.numerator, f.denominator
        exact = "0" if n == 0 else (("-" if n < 0 else "") + ("" if abs(n) == 1 else str(abs(n))) + "pi" + ("" if d == 1 else f"/{d}"))
    return {"radians": deg * math.pi / 180, "pi_multiple": float(f), "exact": exact}


def _verify_d2r(i, o):
    r = abs(o["radians"] * 180 / math.pi - i["deg"]) / max(1.0, abs(i["deg"])) + abs(o["pi_multiple"] * 180 - i["deg"]) / max(1.0, abs(i["deg"]))
    if o["exact"]:
        m = re.fullmatch(r"(-?)(\d*)pi(?:/(\d+))?|0", o["exact"])
        if not m:
            return 1.0 + r
        v = Fraction(0) if o["exact"] == "0" else Fraction((-1 if m.group(1) else 1) * int(m.group(2) or 1), int(m.group(3) or 1))
        r += float(abs(v * 180 - _fr(i["deg"])))
    return r


def _ref_d2r(deg):
    import mpmath as mp
    with mp.workdps(30):
        return {"radians": float(mp.radians(mp.mpf(repr(float(deg))))), "pi_multiple": float(mp.mpf(repr(float(deg))) / 180)}


_register(
    "deg_to_rad", name="Degrees to radians", topic="trigonometry / angle measure",
    purpose="Convert an angle in degrees to radians (also as a multiple of pi).",
    inputs=[_num_in("deg", "the angle in degrees")],
    outputs=[_out("radians", "the angle in radians"), _out("pi_multiple", "the angle as a multiple of pi"), _out("exact", "exact form, e.g. 3pi/4", "text")],
    primary="radians", conditions=["a finite angle"], method="multiply by pi/180", latex=r"\theta_{\mathrm{rad}} = \theta_{\deg}\cdot\frac{\pi}{180}",
    run=deg_to_rad, verify=_verify_d2r, reference=_ref_d2r, wrong=lambda deg: {"radians": deg * 180 / math.pi, "pi_multiple": deg / math.pi, "exact": ""},
    random_inputs=lambda rng: {"deg": rng.choice([float(rng.randint(-720, 720)), round(rng.uniform(-400, 400), 2), float(rng.choice([30, 45, 60, 135, 210, 330]))]) or 15.0},
    parse=_templated([r"(?:convert|change|express) (?P<d>" + _W + r") degrees? (?:to|into|in) radians" + _E,
                      r"what is (?P<d>" + _W + r") degrees? in radians" + _E],
                     lambda g: {"deg": float(g["d"])}),
    templates=["Convert 135 degrees to radians.", "What is 210° in radians?"],
    examples=[{"inputs": {"deg": 135}, "outputs": {"radians": 3 * math.pi / 4, "pi_multiple": 0.75, "exact": "3pi/4"}},
              {"inputs": {"deg": 210}, "outputs": {"radians": 7 * math.pi / 6, "exact": "7pi/6"}}, {"inputs": {"deg": -90}, "outputs": {"exact": "-pi/2"}}],
    questions=[{"q": "Convert 135 degrees to radians.", "outputs": {"radians": 3 * math.pi / 4, "exact": "3pi/4"}},
               {"q": "What is 210° in radians?", "outputs": {"radians": 7 * math.pi / 6}}, {"q": "Express 1 degree in radians.", "outputs": {"radians": math.pi / 180}}])


def rad_to_deg(rad) -> dict:
    _finite(rad=rad)
    d = rad * 180 / math.pi
    snapped = round(d, 9)
    if abs(d - snapped) <= 1e-11 * max(1.0, abs(d)):
        d = snapped + 0.0
    return {"degrees": d}


def _ref_r2d(rad):
    import mpmath as mp
    with mp.workdps(30):
        return {"degrees": float(mp.degrees(mp.mpf(repr(float(rad)))))}


_register(
    "rad_to_deg", name="Radians to degrees", topic="trigonometry / angle measure",
    purpose="Convert an angle in radians (a number, or a multiple of pi such as 3pi/4) to degrees.",
    inputs=[_num_in("rad", "the angle in radians")], outputs=[_out("degrees", "the angle in degrees")], primary="degrees",
    conditions=["a finite angle"], method="multiply by 180/pi", latex=r"\theta_{\deg} = \theta_{\mathrm{rad}}\cdot\frac{180}{\pi}",
    run=rad_to_deg, verify=lambda i, o: abs(o["degrees"] * math.pi / 180 - i["rad"]) / max(1.0, abs(i["rad"])), reference=_ref_r2d,
    wrong=lambda rad: {"degrees": rad * math.pi / 180},
    random_inputs=lambda rng: {"rad": rng.choice([round(rng.uniform(-10, 10), 3), rng.randint(-12, 12) * math.pi / rng.choice([1, 2, 3, 4, 6])]) or 1.0},
    parse=_templated([r"(?:convert|change|express) (?P<r>[-+]?\d*(?:\.\d+)?\s*\*?\s*pi(?:\s*/\s*\d+)?|" + _W + r") radians? (?:to|into|in) degrees" + _E,
                      r"what is (?P<r>[-+]?\d*(?:\.\d+)?\s*\*?\s*pi(?:\s*/\s*\d+)?) (?:radians )?in degrees" + _E],
                     lambda g: {"rad": _pi_angle(g["r"])}),
    templates=["Convert 3pi/4 radians to degrees.", "Convert 2.5 radians to degrees.", "What is 5π/6 in degrees?"],
    examples=[{"inputs": {"rad": 3 * math.pi / 4}, "outputs": {"degrees": 135.0}}, {"inputs": {"rad": 2.5}, "outputs": {"degrees": 450 / math.pi}},
              {"inputs": {"rad": math.pi}, "outputs": {"degrees": 180.0}}],
    questions=[{"q": "Convert 3pi/4 radians to degrees.", "outputs": {"degrees": 135.0}}, {"q": "Convert 2.5 radians to degrees.", "outputs": {"degrees": 450 / math.pi}},
               {"q": "What is 5π/6 in degrees?", "outputs": {"degrees": 150.0}}])


# --- right triangles ----------------------------------------------------------------------------------------------
def _mp_angle(P, Q, R):
    """The angle at P of the triangle P, Q, R (mpmath points), in degrees, from the two side vectors."""
    import mpmath as mp
    u = (Q[0] - P[0], Q[1] - P[1])
    v = (R[0] - P[0], R[1] - P[1])
    return mp.degrees(mp.atan2(abs(u[0] * v[1] - u[1] * v[0]), u[0] * v[0] + u[1] * v[1]))


def _mp_dist(P, Q):
    import mpmath as mp
    return mp.sqrt((P[0] - Q[0]) ** 2 + (P[1] - Q[1]) ** 2)


def right_triangle_legs(a, b) -> dict:
    _finite(a=a, b=b)
    if a <= 0 or b <= 0:
        raise ValueError("the legs must be positive")
    A = math.degrees(math.atan2(a, b))
    return {"c": math.hypot(a, b), "A": A, "B": 90.0 - A}


def _verify_rt_legs(i, o):
    a, b = i["a"], i["b"]
    return (abs(o["c"] ** 2 - a * a - b * b) / max(1.0, a * a + b * b) + abs(math.tan(math.radians(o["A"])) * b - a) / max(1.0, a)
            + abs(o["A"] + o["B"] - 90) / 90)


def _ref_rt_legs(a, b):
    import mpmath as mp
    with mp.workdps(30):
        a, b = mp.mpf(repr(float(a))), mp.mpf(repr(float(b)))
        C, A, B = (0, 0), (b, 0), (0, a)                   # right angle at C; side a = BC opposite A, side b = CA
        return {"c": float(_mp_dist(A, B)), "A": float(_mp_angle(A, B, C)), "B": float(_mp_angle(B, A, C))}


_RT = r"(?:a )?right(?:-angled)? triangle "
_register(
    "right_triangle_legs", name="Right triangle from its two legs", topic="trigonometry / right triangles",
    purpose="Given the legs a and b of a right triangle: the hypotenuse c and the acute angles A (opposite a) and B.",
    inputs=[_num_in("a", "the leg opposite angle A (positive)"), _num_in("b", "the leg opposite angle B (positive)")],
    outputs=[_out("c", "the hypotenuse"), _out("A", "angle opposite a, degrees"), _out("B", "angle opposite b, degrees")],
    primary="c", conditions=["a > 0", "b > 0"], method="Pythagoras c = sqrt(a^2 + b^2); tan A = a/b; B = 90 - A",
    latex=r"c = \sqrt{a^2 + b^2},\quad \tan A = \frac{a}{b}", run=right_triangle_legs, verify=_verify_rt_legs, reference=_ref_rt_legs,
    wrong=lambda a, b: (lambda r: dict(r, A=r["B"], B=r["A"]))(right_triangle_legs(a, b)),
    random_inputs=lambda rng: {"a": rng.choice([float(rng.randint(1, 30)), round(rng.uniform(0.5, 50), 2)]), "b": rng.choice([float(rng.randint(1, 30)), round(rng.uniform(0.5, 50), 2)])},
    parse=_templated([_RT + r"has legs (?:a = )?(?P<a>" + _W + r") and (?:b = )?(?P<b>" + _W + r")\. find (?:the hypotenuse and the (?:acute )?angles|the hypotenuse|the other side and the angles)" + _E,
                      r"solve the right triangle with legs a = (?P<a>" + _W + r") and b = (?P<b>" + _W + r")" + _E],
                     lambda g: {"a": float(g["a"]), "b": float(g["b"])}),
    templates=["A right triangle has legs 3 and 4. Find the hypotenuse and the acute angles.", "Solve the right triangle with legs a = 5 and b = 12."],
    examples=[{"inputs": {"a": 3, "b": 4}, "outputs": {"c": 5.0, "A": math.degrees(math.atan(0.75))}},
              {"inputs": {"a": 5, "b": 12}, "outputs": {"c": 13.0}}, {"inputs": {"a": 1, "b": 1}, "outputs": {"A": 45.0, "B": 45.0}}],
    questions=[{"q": "A right triangle has legs 3 and 4. Find the hypotenuse and the acute angles.", "outputs": {"c": 5.0, "A": math.degrees(math.atan(0.75))}},
               {"q": "Solve the right triangle with legs a = 5 and b = 12.", "outputs": {"c": 13.0}},
               {"q": "Solve the right triangle with legs a = 0 and b = 12.", "outcome": "INVALID_INPUT"}])


def right_triangle_leg_hypotenuse(a, c) -> dict:
    _finite(a=a, c=c)
    if not 0 < a < c:
        raise ValueError("the leg must be positive and shorter than the hypotenuse")
    A = math.degrees(math.asin(a / c))
    return {"b": math.sqrt((c - a) * (c + a)), "A": A, "B": 90.0 - A}


def _ref_rt_lh(a, c):
    import mpmath as mp
    with mp.workdps(30):
        a, c = mp.mpf(repr(float(a))), mp.mpf(repr(float(c)))
        B = (0, a)                                          # right angle at C = (0, 0); A on the x-axis at distance c from B
        bx = mp.findroot(lambda t: _mp_dist((t, 0), B) - c, c)
        A = (bx, 0)
        return {"b": float(bx), "A": float(_mp_angle(A, B, (0, 0))), "B": float(_mp_angle(B, A, (0, 0)))}


_register(
    "right_triangle_leg_hypotenuse", name="Right triangle from a leg and the hypotenuse", topic="trigonometry / right triangles",
    purpose="Given a leg a and the hypotenuse c of a right triangle: the other leg b and the acute angles A (opposite a) and B.",
    inputs=[_num_in("a", "the known leg (positive)"), _num_in("c", "the hypotenuse (longer than the leg)")],
    outputs=[_out("b", "the other leg"), _out("A", "angle opposite a, degrees"), _out("B", "angle opposite b, degrees")],
    primary="b", conditions=["0 < a < c"], method="b = sqrt(c^2 - a^2); sin A = a/c; B = 90 - A",
    latex=r"b = \sqrt{c^2 - a^2},\quad \sin A = \frac{a}{c}", run=right_triangle_leg_hypotenuse,
    verify=lambda i, o: abs(o["b"] ** 2 + i["a"] ** 2 - i["c"] ** 2) / max(1.0, i["c"] ** 2) + abs(math.sin(math.radians(o["A"])) * i["c"] - i["a"]) / max(1.0, i["a"])
                        + abs(o["A"] + o["B"] - 90) / 90,
    reference=_ref_rt_lh, wrong=lambda a, c: dict(right_triangle_leg_hypotenuse(a, c), b=math.sqrt(c * c + a * a)),
    random_inputs=lambda rng: (lambda c: {"c": c, "a": round(c * rng.uniform(0.05, 0.95), 2) or 0.5})(rng.choice([float(rng.randint(2, 40)), round(rng.uniform(1, 60), 2)])),
    parse=_templated([_RT + r"has (?:a )?hypotenuse (?:of )?(?P<c>" + _W + r") and (?:one|a) leg (?:of )?(?P<a>" + _W + r")\. find the other leg(?: and the (?:acute )?angles)?" + _E],
                     lambda g: {"a": float(g["a"]), "c": float(g["c"])}),
    templates=["A right triangle has hypotenuse 13 and one leg 5. Find the other leg and the acute angles."],
    examples=[{"inputs": {"a": 5, "c": 13}, "outputs": {"b": 12.0, "A": math.degrees(math.asin(5 / 13))}}, {"inputs": {"a": 3, "c": 6}, "outputs": {"A": 30.0, "B": 60.0}}],
    questions=[{"q": "A right triangle has hypotenuse 13 and one leg 5. Find the other leg and the acute angles.", "outputs": {"b": 12.0}},
               {"q": "A right triangle has hypotenuse 6 and one leg 3. Find the other leg and the angles.", "outputs": {"A": 30.0, "B": 60.0}},
               {"q": "A right triangle has hypotenuse 5 and one leg 7. Find the other leg.", "outcome": "INVALID_INPUT"}])

_SIDES = ["hypotenuse", "opposite", "adjacent"]


def right_triangle_angle_side(A, side, which) -> dict:
    _finite(A=A, side=side)
    if not 0 < A < 90:
        raise ValueError("the acute angle must be between 0 and 90 degrees")
    if side <= 0:
        raise ValueError("the side must be positive")
    t = math.radians(A)
    if which == "hypotenuse":
        c = side
        a, b = c * math.sin(t), c * math.cos(t)
    elif which == "opposite":
        a = side
        c, b = a / math.sin(t), a / math.tan(t)
    else:
        b = side
        c, a = b / math.cos(t), b * math.tan(t)
    return {"opposite": a, "adjacent": b, "hypotenuse": c, "B": 90.0 - A}


def _verify_rt_as(i, o):
    a, b, c = o["opposite"], o["adjacent"], o["hypotenuse"]
    given = {"hypotenuse": c, "opposite": a, "adjacent": b}[i["which"]]
    return (abs(a * a + b * b - c * c) / max(1.0, c * c) + abs(a - c * math.sin(math.radians(i["A"]))) / max(1.0, c)
            + _rel(given, i["side"]) + abs(o["B"] + i["A"] - 90) / 90)


def _ref_rt_as(A, side, which):
    import mpmath as mp
    with mp.workdps(30):                                 # a similar triangle on the unit circle, scaled to the given side
        t = mp.radians(mp.mpf(repr(float(A))))
        P = (mp.cos(t), mp.sin(t))                        # hypotenuse 1 from the origin; opposite = P_y, adjacent = P_x
        unit = {"hypotenuse": _mp_dist((0, 0), P), "opposite": P[1], "adjacent": P[0]}
        s = mp.mpf(repr(float(side))) / unit[which]
        return {"opposite": float(s * P[1]), "adjacent": float(s * P[0]), "hypotenuse": float(s * unit["hypotenuse"]), "B": float(90 - mp.mpf(repr(float(A))))}


def _wrong_rt_as(A, side, which):
    r = right_triangle_angle_side(A, side, which)
    return dict(r, opposite=r["adjacent"], adjacent=r["opposite"]) if which == "hypotenuse" else \
        dict(r, adjacent=r["opposite"] * math.tan(math.radians(A))) if which == "opposite" else dict(r, opposite=r["adjacent"] / math.tan(math.radians(A)))


def _parse_rt_as(g):
    w = g["w"].lower()
    which = "hypotenuse" if w.startswith("hyp") else ("opposite" if "opposite" in w else "adjacent")
    return {"A": float(g["A"]), "side": float(g["s"]), "which": which}


_register(
    "right_triangle_angle_side", name="Right triangle from an acute angle and a side", topic="trigonometry / right triangles",
    purpose="Given an acute angle A and one side (the hypotenuse, the side opposite A or the side adjacent to A): every side and the other angle.",
    inputs=[_num_in("A", "the acute angle A in degrees"), _num_in("side", "the length of the known side"),
            {"symbol": "which", "type": "choice", "choices": _SIDES, "kind": "text", "required": True, "meaning": "which side is known"}],
    outputs=[_out("opposite", "side opposite A"), _out("adjacent", "side adjacent to A"), _out("hypotenuse", "the hypotenuse"), _out("B", "the other acute angle, degrees")],
    primary="hypotenuse", conditions=["0 < A < 90", "side > 0"], method="SOH-CAH-TOA: sin A = opp/hyp, cos A = adj/hyp, tan A = opp/adj",
    latex=r"\sin A = \frac{\mathrm{opp}}{\mathrm{hyp}},\ \cos A = \frac{\mathrm{adj}}{\mathrm{hyp}},\ \tan A = \frac{\mathrm{opp}}{\mathrm{adj}}",
    run=right_triangle_angle_side, verify=_verify_rt_as, reference=_ref_rt_as, wrong=_wrong_rt_as,
    random_inputs=lambda rng: {"A": round(rng.uniform(5, 85), 1), "side": round(rng.uniform(0.5, 50), 2), "which": rng.choice(_SIDES)},
    parse=_templated([r"in a right triangle,? (?:angle )?a (?:is|=) (?P<A>" + _W + r") degrees and the (?P<w>hypotenuse|side opposite (?:angle )?a|side adjacent to (?:angle )?a|opposite side|adjacent side) (?:is|=) (?P<s>"
                      + _W + r")\. find the (?:other|remaining|missing) sides(?: and angle)?" + _E],
                     _parse_rt_as),
    templates=["In a right triangle, angle A is 30 degrees and the hypotenuse is 10. Find the other sides.",
               "In a right triangle, angle A is 40 degrees and the side adjacent to A is 8. Find the other sides."],
    examples=[{"inputs": {"A": 30, "side": 10, "which": "hypotenuse"}, "outputs": {"opposite": 5.0, "adjacent": 5 * math.sqrt(3), "B": 60.0}},
              {"inputs": {"A": 45, "side": 7, "which": "opposite"}, "outputs": {"adjacent": 7.0, "hypotenuse": 7 * math.sqrt(2)}}],
    questions=[{"q": "In a right triangle, angle A is 30 degrees and the hypotenuse is 10. Find the other sides.", "outputs": {"opposite": 5.0, "adjacent": 5 * math.sqrt(3)}},
               {"q": "In a right triangle, angle A is 45 degrees and the side opposite A is 7. Find the other sides.", "outputs": {"adjacent": 7.0}},
               {"q": "In a right triangle, angle A is 95 degrees and the hypotenuse is 10. Find the other sides.", "outcome": "INVALID_INPUT"}])


# --- oblique triangles: law of sines, law of cosines, Heron ---------------------------------------------------------
def _tri_given(question: str):
    """'In triangle ABC, a = 8, b = 10 and A = 40 degrees. Solve the triangle.' -> {'a': 8, 'b': 10, 'A': 40}; an angle
    is a capital letter or carries 'degrees'. None unless every number of the question is one of these values."""
    if not isinstance(question, str):
        return None
    q = _clean(question)
    m = re.fullmatch(r"(?i:in (?:a )?triangle abc),? (?P<body>.+?)[.,;]? (?i:(?:solve the triangle|find (?:all )?(?:the )?(?:remaining|missing|other|unknown)? ?(?:sides and angles|angles and sides|parts|angles|sides|area)(?: of the triangle)?))" + _E, q)
    if not m:
        return None
    body = m.group("body")
    found = {}
    for g in re.finditer(r"\b(?:side |angle )?([abcABC])\s*=\s*(" + _W + r")\s*(degrees|deg)?", body):
        letter = g.group(1).upper() if (g.group(1).isupper() or g.group(3)) else g.group(1)
        if letter in found:
            return None
        found[letter] = float(g.group(2))
    if len(re.findall(r"\d+(?:\.\d+)?", body)) != len(found):
        return None
    return found


def _tri_parser(keys, extra=None):
    def parse(question):
        g = _tri_given(question)
        if g is None or set(g) != set(keys):
            return None
        if extra and not re.search(extra, question, re.I):
            return None
        return g
    return parse


_TRIQ = "In triangle ABC, "


def _deg_sin(d):
    return math.sin(math.radians(d))


def law_of_sines_aas(A, B, a) -> dict:
    _finite(A=A, B=B, a=a)
    if A <= 0 or B <= 0 or A + B >= 180:
        raise ValueError("the two angles must be positive with a sum below 180 degrees")
    if a <= 0:
        raise ValueError("the side must be positive")
    C = 180.0 - A - B
    k = a / _deg_sin(A)
    return {"C": C, "b": k * _deg_sin(B), "c": k * _deg_sin(C)}


def _verify_aas(i, o):
    A, B, a = i["A"], i["B"], i["a"]
    b, c, C = o["b"], o["c"], o["C"]
    cosA = math.cos(math.radians(A))
    return (abs(A + B + C - 180) / 180 + abs(a * a - (b * b + c * c - 2 * b * c * cosA)) / max(1.0, a * a, b * b + c * c)
            + abs(b * _deg_sin(A) - a * _deg_sin(B)) / max(1.0, a, b))


def _ref_aas(A, B, a):
    import mpmath as mp
    with mp.workdps(30):                                  # B at the origin, C at (a, 0): intersect the rays from B and C
        A_, B_, a_ = (mp.mpf(repr(float(v))) for v in (A, B, a))
        Cang = 180 - A_ - B_
        rb, rc = mp.radians(B_), mp.radians(Cang)
        t, s = mp.lu_solve(mp.matrix([[mp.cos(rb), mp.cos(rc)], [mp.sin(rb), -mp.sin(rc)]]), mp.matrix([a_, 0]))
        Apt = (t * mp.cos(rb), t * mp.sin(rb))
        return {"C": float(Cang), "b": float(_mp_dist(Apt, (a_, 0))), "c": float(_mp_dist(Apt, (0, 0)))}


_register(
    "law_of_sines_aas", name="Law of sines (two angles and a side)", topic="trigonometry / oblique triangles",
    purpose="Given angles A and B and the side a opposite A: the third angle C and the sides b and c.",
    inputs=[_num_in("A", "angle A, degrees"), _num_in("B", "angle B, degrees"), _num_in("a", "side a, opposite A")],
    outputs=[_out("C", "angle C, degrees"), _out("b", "side b"), _out("c", "side c")], primary="b",
    conditions=["A, B > 0", "A + B < 180", "a > 0"], method="C = 180 - A - B; law of sines a/sin A = b/sin B = c/sin C",
    latex=r"\frac{a}{\sin A} = \frac{b}{\sin B} = \frac{c}{\sin C}", run=law_of_sines_aas, verify=_verify_aas, reference=_ref_aas,
    wrong=lambda A, B, a: dict(law_of_sines_aas(A, B, a), b=a * _deg_sin(A) / _deg_sin(B)),
    random_inputs=lambda rng: (lambda A: {"A": A, "B": round(rng.uniform(5, 170 - A), 1), "a": round(rng.uniform(1, 50), 2)})(round(rng.uniform(10, 150), 1)),
    parse=_tri_parser(("A", "B", "a")),
    templates=["In triangle ABC, A = 40 degrees, B = 60 degrees and a = 10. Solve the triangle."],
    examples=[{"inputs": {"A": 40, "B": 60, "a": 10}, "outputs": {"C": 80.0, "b": 10 * _deg_sin(60) / _deg_sin(40), "c": 10 * _deg_sin(80) / _deg_sin(40)}},
              {"inputs": {"A": 30, "B": 90, "a": 5}, "outputs": {"C": 60.0, "b": 10.0}}],
    questions=[{"q": "In triangle ABC, A = 40 degrees, B = 60 degrees and a = 10. Solve the triangle.", "outputs": {"C": 80.0, "b": 10 * _deg_sin(60) / _deg_sin(40)}},
               {"q": "In triangle ABC, A = 30°, B = 90° and a = 5. Find the remaining sides and angles.", "outputs": {"b": 10.0}},
               {"q": "In triangle ABC, A = 100 degrees, B = 90 degrees and a = 5. Solve the triangle.", "outcome": "INVALID_INPUT"}])


def law_of_sines_ssa(a, b, A) -> dict:
    """The ambiguous case: 0, 1 or 2 triangles. Two are both reported (B1 acute first); none is a refusal."""
    _finite(a=a, b=b, A=A)
    if a <= 0 or b <= 0 or not 0 < A < 180:
        raise ValueError("the sides must be positive and A between 0 and 180 degrees")
    s = b * _deg_sin(A) / a                               # sin B
    if s > 1 + 1e-12:
        raise ValueError("no triangle exists: side a is shorter than the height b*sin(A)")
    sols = []
    B1 = 90.0 if s >= 1 - 1e-12 else math.degrees(math.asin(s))
    for B in ([B1] if B1 == 90.0 else [B1, 180.0 - B1]):
        C = 180.0 - A - B
        if C > 1e-9:
            sols.append((B, C, a * _deg_sin(C) / _deg_sin(A)))
    if not sols:
        raise ValueError("no triangle exists with these measurements")
    out = {"count": float(len(sols))}
    for k, (B, C, c) in enumerate(sols, 1):
        out.update({f"B{k}": B, f"C{k}": C, f"c{k}": c})
    return out


def _verify_ssa(i, o):
    a, b, A = i["a"], i["b"], i["A"]
    n = int(o["count"])
    if n != o["count"] or n not in (1, 2) or any(f"B{k}" not in o for k in range(1, n + 1)) or f"B{n + 1}" in o:
        return 1.0
    worst = 0.0
    for k in range(1, n + 1):
        B, C, c = o[f"B{k}"], o[f"C{k}"], o[f"c{k}"]
        worst += abs(A + B + C - 180) / 180 + abs(a * a - (b * b + c * c - 2 * b * c * math.cos(math.radians(A)))) / max(1.0, a * a, b * b)
        worst += abs(b * _deg_sin(A) - a * _deg_sin(B)) / max(1.0, a, b)
    # how many triangles: positive roots of c^2 - 2bc cos A + (b^2 - a^2) = 0
    p, q = -2 * b * math.cos(math.radians(A)), b * b - a * a
    disc = p * p - 4 * q
    if abs(disc) <= 1e-11 * max(1.0, p * p, abs(4 * q)):
        disc = 0.0                                       # the right-angled boundary case: one triangle
    roots = [] if disc < 0 else [(-p - math.sqrt(disc)) / 2, (-p + math.sqrt(disc)) / 2]
    want = len({round(r, 9) for r in roots if r > 1e-9})
    return worst + (want != n)


def _ref_ssa(a, b, A):
    import mpmath as mp
    with mp.workdps(30):                                  # c from the law of cosines, then the triangle drawn: A at the origin
        a_, b_, A_ = (mp.mpf(repr(float(v))) for v in (a, b, A))
        t = mp.radians(A_)
        cs = sorted({mp.re(r) for r in mp.polyroots([1, -2 * b_ * mp.cos(t), b_ * b_ - a_ * a_]) if abs(mp.im(r)) < mp.mpf(10) ** -20 and mp.re(r) > 0})
        sols = []
        for c in cs:
            Bpt, Cpt = (c * mp.cos(t), c * mp.sin(t)), (b_, 0)
            Bang = _mp_angle(Bpt, (0, 0), Cpt)
            sols.append((Bang, 180 - A_ - Bang, c))
        sols.sort(key=lambda s: s[0])
        out = {"count": float(len(sols))}
        for k, (B, C, c) in enumerate(sols, 1):
            out.update({f"B{k}": float(B), f"C{k}": float(C), f"c{k}": float(c)})
        return out


def _wrong_ssa(a, b, A):
    r = law_of_sines_ssa(a, b, A)
    return {k: v for k, v in r.items() if not k.endswith("2")} | {"count": 1.0}


def _rand_ssa(rng):
    A = round(rng.uniform(15, 75), 1)
    b = round(rng.uniform(5, 40), 2)
    h = b * _deg_sin(A)
    if rng.random() < 0.7:
        a = round(rng.uniform(h * 1.02, b * 0.98), 2)        # the ambiguous case with two triangles
    else:
        a = round(rng.uniform(b * 1.02, b * 2), 2)
    return {"a": a, "b": b, "A": A}


_register(
    "law_of_sines_ssa", name="Law of sines, the ambiguous case (two sides and a non-included angle)", topic="trigonometry / oblique triangles",
    purpose="Given sides a and b and the angle A opposite a: how many triangles exist (0, 1 or 2) and every one of them.",
    inputs=[_num_in("a", "side a, opposite A"), _num_in("b", "side b"), _num_in("A", "angle A, degrees")],
    outputs=[_out("count", "number of triangles (1 or 2; none is refused)"), _out("B1", "angle B of the first triangle (the acute one)"),
             _out("C1", "angle C of the first triangle"), _out("c1", "side c of the first triangle"),
             _out("B2", "angle B of the second triangle (obtuse), when there are two"), _out("C2", "angle C of the second triangle"), _out("c2", "side c of the second triangle")],
    primary="count", conditions=["a, b > 0", "0 < A < 180", "a triangle exists (else refused)"],
    method="sin B = b sin A / a; B and 180 - B both give a triangle when A + B < 180 (the ambiguous case)",
    latex=r"\sin B = \frac{b\sin A}{a},\quad B_2 = 180^\circ - B_1", run=law_of_sines_ssa, verify=_verify_ssa, reference=_ref_ssa,
    wrong=_wrong_ssa, random_inputs=_rand_ssa, parse=_tri_parser(("a", "b", "A")),
    templates=["In triangle ABC, a = 8, b = 10 and A = 40 degrees. Solve the triangle."],
    examples=[{"inputs": {"a": 8, "b": 10, "A": 40}, "outputs": {"count": 2.0, "B1": math.degrees(math.asin(10 * _deg_sin(40) / 8))}},
              {"inputs": {"a": 12, "b": 10, "A": 40}, "outputs": {"count": 1.0}}, {"inputs": {"a": 5, "b": 10, "A": 30}, "outputs": {"count": 1.0, "B1": 90.0}}],
    questions=[{"q": "In triangle ABC, a = 8, b = 10 and A = 40 degrees. Solve the triangle.", "outputs": {"count": 2.0}},
               {"q": "In triangle ABC, a = 12, b = 10 and A = 40°. Solve the triangle.", "outputs": {"count": 1.0}},
               {"q": "In triangle ABC, a = 3, b = 10 and A = 40 degrees. Solve the triangle.", "outcome": "INVALID_INPUT"}])


def law_of_cosines_sas(b, c, A) -> dict:
    _finite(b=b, c=c, A=A)
    if b <= 0 or c <= 0 or not 0 < A < 180:
        raise ValueError("the sides must be positive and A between 0 and 180 degrees")
    a = math.sqrt(b * b + c * c - 2 * b * c * math.cos(math.radians(A)))
    B = math.degrees(math.acos(max(-1.0, min(1.0, (a * a + c * c - b * b) / (2 * a * c)))))
    return {"a": a, "B": B, "C": 180.0 - A - B}


def _ref_sas(b, c, A):
    import mpmath as mp
    with mp.workdps(30):                                  # A at the origin, B = (c, 0), C on the ray at angle A
        b_, c_, A_ = (mp.mpf(repr(float(v))) for v in (b, c, A))
        t = mp.radians(A_)
        P, Q, R = (0, 0), (c_, 0), (b_ * mp.cos(t), b_ * mp.sin(t))
        return {"a": float(_mp_dist(Q, R)), "B": float(_mp_angle(Q, P, R)), "C": float(_mp_angle(R, P, Q))}


_register(
    "law_of_cosines_sas", name="Law of cosines (two sides and the included angle)", topic="trigonometry / oblique triangles",
    purpose="Given sides b and c and the included angle A: the third side a and the angles B and C.",
    inputs=[_num_in("b", "side b"), _num_in("c", "side c"), _num_in("A", "the included angle A, degrees")],
    outputs=[_out("a", "side a"), _out("B", "angle B, degrees"), _out("C", "angle C, degrees")], primary="a",
    conditions=["b, c > 0", "0 < A < 180"], method="a^2 = b^2 + c^2 - 2bc cos A; B from the law of cosines (no ambiguity)",
    latex=r"a^2 = b^2 + c^2 - 2bc\cos A", run=law_of_cosines_sas,
    verify=lambda i, o: (abs(o["a"] ** 2 - (i["b"] ** 2 + i["c"] ** 2 - 2 * i["b"] * i["c"] * math.cos(math.radians(i["A"])))) / max(1.0, i["b"] ** 2 + i["c"] ** 2)
                         + abs(o["a"] * _deg_sin(o["B"]) - i["b"] * _deg_sin(i["A"])) / max(1.0, o["a"], i["b"]) + abs(i["A"] + o["B"] + o["C"] - 180) / 180),
    reference=_ref_sas, wrong=lambda b, c, A: (lambda a: {"a": a, "B": 0.0, "C": 0.0})(math.sqrt(b * b + c * c + 2 * b * c * math.cos(math.radians(A)))),
    random_inputs=lambda rng: {"b": round(rng.uniform(1, 40), 2), "c": round(rng.uniform(1, 40), 2), "A": round(rng.uniform(5, 175), 1)},
    parse=_tri_parser(("b", "c", "A")),
    templates=["In triangle ABC, b = 5, c = 7 and A = 49 degrees. Solve the triangle."],
    examples=[{"inputs": {"b": 5, "c": 7, "A": 49}, "outputs": {"a": math.sqrt(74 - 70 * math.cos(math.radians(49)))}},
              {"inputs": {"b": 3, "c": 4, "A": 90}, "outputs": {"a": 5.0}}],
    questions=[{"q": "In triangle ABC, b = 5, c = 7 and A = 49 degrees. Solve the triangle.", "outputs": {"a": math.sqrt(74 - 70 * math.cos(math.radians(49)))}},
               {"q": "In triangle ABC, b = 3, c = 4 and A = 90°. Find the remaining sides and angles.", "outputs": {"a": 5.0}},
               {"q": "In triangle ABC, b = 3, c = 4 and A = 190 degrees. Solve the triangle.", "outcome": "INVALID_INPUT"}])


def _tri_check(a, b, c):
    _finite(a=a, b=b, c=c)
    if min(a, b, c) <= 0 or a + b <= c or a + c <= b or b + c <= a:
        raise ValueError("these lengths do not form a triangle (each side must be shorter than the sum of the other two)")


def law_of_cosines_sss(a, b, c) -> dict:
    _tri_check(a, b, c)
    ang = lambda x, y, z: math.degrees(math.acos(max(-1.0, min(1.0, (y * y + z * z - x * x) / (2 * y * z)))))
    A, B = ang(a, b, c), ang(b, a, c)
    return {"A": A, "B": B, "C": 180.0 - A - B}


def _mp_sss_apex(a, b, c):
    """A = (x, y), y > 0, with |AB| = c, |AC| = b, B = (0, 0), C = (a, 0): found numerically (not by the law of cosines)."""
    import mpmath as mp
    x, y = mp.findroot(lambda x, y: [x * x + y * y - c * c, (x - a) ** 2 + y * y - b * b], (a / 2, (b + c) / 2))
    return x, abs(y)


def _ref_sss(a, b, c):
    import mpmath as mp
    with mp.workdps(30):
        a_, b_, c_ = (mp.mpf(repr(float(v))) for v in (a, b, c))
        P = _mp_sss_apex(a_, b_, c_)
        Bp, Cp = (0, 0), (a_, 0)
        return {"A": float(_mp_angle(P, Bp, Cp)), "B": float(_mp_angle(Bp, P, Cp)), "C": float(_mp_angle(Cp, P, Bp))}


def _rand_tri(rng):
    while True:
        a, b, c = (round(rng.uniform(1, 30), 2) for _ in range(3))
        if a + b > c * 1.05 and a + c > b * 1.05 and b + c > a * 1.05:
            return {"a": a, "b": b, "c": c}


_register(
    "law_of_cosines_sss", name="Law of cosines (three sides)", topic="trigonometry / oblique triangles",
    purpose="Given the three sides a, b, c: the angles A, B, C.",
    inputs=[_num_in("a", "side a"), _num_in("b", "side b"), _num_in("c", "side c")],
    outputs=[_out("A", "angle A, degrees"), _out("B", "angle B, degrees"), _out("C", "angle C, degrees")], primary="A",
    conditions=["positive sides", "triangle inequality (each side shorter than the sum of the other two)"],
    method="cos A = (b^2 + c^2 - a^2)/(2bc), likewise B; C = 180 - A - B", latex=r"\cos A = \frac{b^2 + c^2 - a^2}{2bc}",
    run=law_of_cosines_sss,
    verify=lambda i, o: (abs(math.cos(math.radians(o["A"])) * 2 * i["b"] * i["c"] - (i["b"] ** 2 + i["c"] ** 2 - i["a"] ** 2)) / max(1.0, i["b"] * i["c"])
                         + abs(math.cos(math.radians(o["B"])) * 2 * i["a"] * i["c"] - (i["a"] ** 2 + i["c"] ** 2 - i["b"] ** 2)) / max(1.0, i["a"] * i["c"])
                         + abs(o["A"] + o["B"] + o["C"] - 180) / 180),
    reference=_ref_sss, wrong=lambda a, b, c: {k: math.radians(v) for k, v in law_of_cosines_sss(a, b, c).items()}, random_inputs=_rand_tri,
    parse=_tri_parser(("a", "b", "c"), r"angles|solve"),
    templates=["In triangle ABC, a = 7, b = 8 and c = 9. Find the angles."],
    examples=[{"inputs": {"a": 7, "b": 8, "c": 9}, "outputs": {"A": math.degrees(math.acos(96 / 144))}}, {"inputs": {"a": 3, "b": 4, "c": 5}, "outputs": {"C": 90.0}},
              {"inputs": {"a": 2, "b": 2, "c": 2}, "outputs": {"A": 60.0, "B": 60.0, "C": 60.0}}],
    questions=[{"q": "In triangle ABC, a = 7, b = 8 and c = 9. Find the angles.", "outputs": {"A": math.degrees(math.acos(96 / 144))}},
               {"q": "In triangle ABC, a = 3, b = 4 and c = 5. Solve the triangle.", "outputs": {"C": 90.0}},
               {"q": "In triangle ABC, a = 1, b = 2 and c = 5. Find the angles.", "outcome": "INVALID_INPUT"}])


def heron_area(a, b, c) -> dict:
    _tri_check(a, b, c)
    s = (a + b + c) / 2
    # a numerically stable arrangement of Heron's formula (sides sorted a >= b >= c)
    x, y, z = sorted((a, b, c), reverse=True)
    area = 0.25 * math.sqrt((x + (y + z)) * (z - (x - y)) * (z + (x - y)) * (x + (y - z)))
    return {"area": area, "s": s}


def _ref_heron(a, b, c):
    import mpmath as mp
    with mp.workdps(30):                                  # base a on the x-axis; the apex found numerically; area = base * height / 2
        a_, b_, c_ = (mp.mpf(repr(float(v))) for v in (a, b, c))
        _x, y = _mp_sss_apex(a_, b_, c_)
        return {"area": float(a_ * y / 2), "s": float((a_ + b_ + c_) / 2)}


_register(
    "heron_area", name="Area of a triangle from its three sides (Heron)", topic="trigonometry / triangle area",
    purpose="The area of a triangle with sides a, b, c, and its semi-perimeter s.",
    inputs=[_num_in("a", "side a"), _num_in("b", "side b"), _num_in("c", "side c")],
    outputs=[_out("area", "the area"), _out("s", "the semi-perimeter (a + b + c)/2")], primary="area",
    conditions=["positive sides", "triangle inequality"], method="Heron's formula, in its numerically stable arrangement",
    latex=r"K = \sqrt{s(s-a)(s-b)(s-c)},\quad s = \frac{a+b+c}{2}", run=heron_area,
    verify=lambda i, o: (lambda s: abs(o["area"] ** 2 - s * (s - i["a"]) * (s - i["b"]) * (s - i["c"])) / max(1.0, o["area"] ** 2) + _rel(o["s"], s))((i["a"] + i["b"] + i["c"]) / 2),
    reference=_ref_heron,
    wrong=lambda a, b, c: (lambda p: {"area": math.sqrt(p * (p - a) * (p - b) * (p - c)), "s": p})(a + b + c), random_inputs=_rand_tri,
    parse=_templated([r"(?:find|what is) the area of (?:a|the) triangle with sides (?:of length )?(?P<a>" + _W + r"),\s*(?P<b>" + _W + r"),? and (?P<c>" + _W + r")" + _E,
                      r"use heron'?s formula to find the area of (?:a|the) triangle with sides (?P<a>" + _W + r"),\s*(?P<b>" + _W + r"),? and (?P<c>" + _W + r")" + _E],
                     lambda g: {"a": float(g["a"]), "b": float(g["b"]), "c": float(g["c"])}),
    templates=["Find the area of a triangle with sides 13, 14 and 15.", "Use Heron's formula to find the area of a triangle with sides 5, 6 and 7."],
    examples=[{"inputs": {"a": 13, "b": 14, "c": 15}, "outputs": {"area": 84.0, "s": 21.0}}, {"inputs": {"a": 3, "b": 4, "c": 5}, "outputs": {"area": 6.0}},
              {"inputs": {"a": 5, "b": 6, "c": 7}, "outputs": {"area": 6 * math.sqrt(6)}}],
    questions=[{"q": "Find the area of a triangle with sides 13, 14 and 15.", "outputs": {"area": 84.0}},
               {"q": "Use Heron's formula to find the area of a triangle with sides 5, 6 and 7.", "outputs": {"area": 6 * math.sqrt(6)}},
               {"q": "Find the area of a triangle with sides 1, 2 and 3.", "outcome": "INVALID_INPUT"}])


# --- the unit circle, trigonometric equations and values --------------------------------------------------------
_EXACT_SIN = {0: ("0", 0.0), 30: ("1/2", 0.5), 45: ("sqrt(2)/2", math.sqrt(2) / 2), 60: ("sqrt(3)/2", math.sqrt(3) / 2), 90: ("1", 1.0)}
_EXACT_TAN = {0: ("0", 0.0), 30: ("sqrt(3)/3", math.sqrt(3) / 3), 45: ("1", 1.0), 60: ("sqrt(3)", math.sqrt(3)), 90: ("undefined", None)}


def _signed(text_val, sign):
    t, v = text_val
    if v is None:
        return t, None
    if sign < 0 and v != 0:
        return "-" + t, -v
    return t, v + 0.0


def unit_circle_exact(deg) -> dict:
    _finite(deg=deg)
    if deg != int(deg) or (int(deg) % 30 and int(deg) % 45):
        raise ValueError("exact values are given for multiples of 30 or 45 degrees (pi/6 or pi/4)")
    t = int(deg) % 360
    ref, ss, cs = (t, 1, 1) if t <= 90 else ((180 - t, 1, -1) if t <= 180 else ((t - 180, -1, -1) if t <= 270 else (360 - t, -1, 1)))
    st, sv = _signed(_EXACT_SIN[ref], ss)
    ct, cv = _signed(_EXACT_SIN[90 - ref], cs)
    tt, tv = _signed(_EXACT_TAN[ref], ss * cs)
    return {"sin": sv, "cos": cv, "tan": tv, "sin_exact": st, "cos_exact": ct, "tan_exact": tt, "reference_angle": float(ref)}


def _exact_text_value(t):
    if t == "undefined":
        return None
    sign = -1 if t.startswith("-") else 1
    t = t.lstrip("-")
    m = re.fullmatch(r"(?:sqrt\((\d)\)|(\d))(?:/(\d))?", t)
    v = (math.sqrt(int(m.group(1))) if m.group(1) else int(m.group(2))) / (int(m.group(3)) if m.group(3) else 1)
    return sign * v


def _verify_unit(i, o):
    t = math.radians(i["deg"])
    r = abs(o["sin"] - math.sin(t)) + abs(o["cos"] - math.cos(t))
    if (o["tan"] is None) != (abs(math.cos(t)) < 1e-12):
        return 1.0
    if o["tan"] is not None:
        r += abs(o["tan"] - math.tan(t)) / max(1.0, abs(math.tan(t)))
    for k in ("sin", "cos", "tan"):
        tv = _exact_text_value(o[k + "_exact"])
        if (tv is None) != (o[k] is None) or (tv is not None and abs(tv - o[k]) > 1e-15):
            return 1.0
    return r


def _ref_unit_circle(deg):
    import sympy as sp
    t = sp.pi * sp.Integer(int(deg)) / 180
    tn = sp.tan(t)
    return {"sin": float(sp.sin(t)), "cos": float(sp.cos(t)), "tan": None if tn.has(sp.zoo) or not tn.is_finite else float(tn)}


def _parse_unit_circle(g):
    a = g["a"].replace(" ", "").lower()
    deg = _pi_degrees(a) if "pi" in a else float(a)
    if "pi" not in a and not g.get("deg"):
        raise ValueError("say degrees, or give a multiple of pi")
    return {"deg": deg}


_ANG = r"(?P<a>[-+]?\d*\s*\*?\s*pi(?:\s*/\s*\d+)?|" + _W + r")\s*(?P<deg>degrees)?"
_register(
    "unit_circle_exact", name="Exact trigonometric values on the unit circle", topic="trigonometry / unit circle",
    purpose="The exact sin, cos and tan of an angle that is a multiple of 30 or 45 degrees (pi/6, pi/4), with the reference angle.",
    inputs=[_num_in("deg", "the angle in degrees (a multiple of 30 or 45)")],
    outputs=[_out("sin", "sin of the angle"), _out("cos", "cos of the angle"), _out("tan", "tan of the angle (None where undefined)"),
             _out("sin_exact", "exact sin", "text"), _out("cos_exact", "exact cos", "text"), _out("tan_exact", "exact tan", "text"),
             _out("reference_angle", "the reference angle, degrees")],
    primary="sin", conditions=["a whole multiple of 30 or 45 degrees"],
    method="the reference angle's exact values from the 30-60-90 and 45-45-90 triangles, with the signs of the quadrant (ASTC)",
    latex=r"\sin 30^\circ = \tfrac12,\ \sin 45^\circ = \tfrac{\sqrt2}{2},\ \sin 60^\circ = \tfrac{\sqrt3}{2}",
    run=unit_circle_exact, verify=_verify_unit, reference=_ref_unit_circle,
    wrong=lambda deg: (lambda r: dict(r, sin=r["cos"], cos=r["sin"], sin_exact=r["cos_exact"], cos_exact=r["sin_exact"]))(unit_circle_exact(deg)),
    random_inputs=lambda rng: {"deg": float(rng.choice([30, 45]) * rng.randint(-16, 16))},
    parse=_templated([r"(?:find|give|what (?:is|are)) the exact values? of sin,? cos,? and tan (?:of|at) " + _ANG + _E,
                      r"(?:find|give|what is) the exact value of (?:sin|cos|tan)\s*\(?" + _ANG + r"\)?" + _E,
                      r"what is (?:sin|cos|tan)\s*\(?" + _ANG + r"\)? exactly" + _E],
                     _parse_unit_circle),
    templates=["Find the exact values of sin, cos and tan of 150 degrees.", "Find the exact value of sin(5pi/6).", "What is cos(225°) exactly?"],
    examples=[{"inputs": {"deg": 150}, "outputs": {"sin": 0.5, "sin_exact": "1/2", "cos_exact": "-sqrt(3)/2", "tan_exact": "-sqrt(3)/3"}},
              {"inputs": {"deg": 225}, "outputs": {"cos_exact": "-sqrt(2)/2", "tan": 1.0}}, {"inputs": {"deg": 90}, "outputs": {"tan": None, "tan_exact": "undefined"}},
              {"inputs": {"deg": -60}, "outputs": {"sin_exact": "-sqrt(3)/2", "cos_exact": "1/2"}}],
    questions=[{"q": "Find the exact values of sin, cos and tan of 150 degrees.", "outputs": {"sin": 0.5, "cos_exact": "-sqrt(3)/2"}},
               {"q": "Find the exact value of sin(5pi/6).", "outputs": {"sin_exact": "1/2"}},
               {"q": "What is cos(225°) exactly?", "outputs": {"cos_exact": "-sqrt(2)/2"}},
               {"q": "Find the exact value of tan(20 degrees).", "outcome": "INVALID_INPUT"}])

_TRIG = ["sin", "cos", "tan"]
_UNITS = ["degrees", "radians"]


def trig_equation_interval(func, k, n=1, unit="degrees") -> dict:
    _finite(k=k)
    n = int(n)
    if func not in _TRIG or unit not in _UNITS:
        raise ValueError("func is sin, cos or tan; unit is degrees or radians")
    if not 1 <= n <= 12:
        raise ValueError("the multiple n of x must be a whole number from 1 to 12")
    if func == "tan":
        base = [math.degrees(math.atan(k)), math.degrees(math.atan(k)) + 180]
    elif abs(k) > 1:
        base = []
    elif func == "sin":
        a = 90.0 if k == 1 else (-90.0 if k == -1 else math.degrees(math.asin(k)))
        base = [a, 180 - a]
    else:
        a = 0.0 if k == 1 else (180.0 if k == -1 else math.degrees(math.acos(k)))
        base = [a, -a]
    us = []
    for b in base:
        v = b % 360.0
        if v > 360.0 - 1e-9:
            v = 0.0
        if not any(abs(v - w) < 1e-9 for w in us):
            us.append(v)
    sols = sorted((u + 360.0 * m) / n for u in us for m in range(n))
    if unit == "radians":
        sols = [math.radians(s) for s in sols]
    return {"count": float(len(sols)), "solutions": sols}


def _trig_f(func):
    return {"sin": math.sin, "cos": math.cos, "tan": math.tan}[func]


def _verify_trigeq(i, o):
    func, k, n = i["func"], i["k"], int(i.get("n", 1))
    unit = i.get("unit", "degrees")
    per = 2 if func == "tan" else (2 if abs(k) < 1 else (1 if abs(k) == 1 else 0))
    sols = o["solutions"]
    if o["count"] != len(sols) or len(sols) != per * n:
        return 1.0
    P = 360.0 if unit == "degrees" else 2 * math.pi
    worst = 0.0
    for j, x in enumerate(sols):
        if not 0 <= x < P or (j and x <= sols[j - 1] + 1e-12 * P):
            return 1.0
        rad = math.radians(x) if unit == "degrees" else x
        worst = max(worst, abs(_trig_f(func)(n * rad) - k) / max(1.0, abs(k)))
    return worst


def _ref_trigeq(func, k, n=1, unit="degrees"):
    import numpy as np
    from scipy.optimize import brentq
    n = int(n)
    f = {"sin": np.sin, "cos": np.cos, "tan": np.tan}[func]
    g = lambda x: float(f(n * x) - k)
    xs = np.linspace(0.0, 2 * np.pi, 40000 * n, endpoint=False)
    ys = f(n * xs) - k
    roots = []
    for j in range(len(xs)):
        if ys[j] == 0:
            roots.append(float(xs[j]))
        elif j + 1 < len(xs) and ys[j] * ys[j + 1] < 0:
            r = brentq(g, xs[j], xs[j + 1], xtol=1e-15, rtol=8.9e-16)
            if abs(g(r)) < 1e-7 * max(1.0, abs(k)):          # a sign change across an asymptote of tan is not a root
                roots.append(r)
    if ys[-1] * (f(n * 2 * np.pi) - k) < 0:
        r = brentq(g, xs[-1], 2 * np.pi, xtol=1e-15, rtol=8.9e-16)
        if abs(g(r)) < 1e-7 * max(1.0, abs(k)) and r < 2 * np.pi - 1e-12:
            roots.append(r)
    roots = sorted(roots)
    if unit == "degrees":
        roots = [float(np.degrees(r)) for r in roots]
    return {"count": float(len(roots)), "solutions": roots}


def _wrong_trigeq(func, k, n=1, unit="degrees"):
    r = trig_equation_interval(func, k, n, unit)
    return {"count": 1.0, "solutions": r["solutions"][:1]} if r["solutions"] else {"count": 1.0, "solutions": [0.0]}


def _parse_trigeq(g):
    coef = g.get("c")
    coef = 1.0 if coef in (None, "", "+") else (-1.0 if coef == "-" else _q(coef))
    n = g.get("n")
    n = 1 if not n else int(n)
    interval = g["iv"].replace(" ", "").lower()
    if interval in ("0<=x<360degrees", "0<=x<360", "[0,360)", "[0,360)degrees", "[0degrees,360degrees)"):
        unit = "degrees"
    elif interval in ("0<=x<2pi", "[0,2pi)"):
        unit = "radians"
    else:
        raise ValueError("the interval must be [0, 360) degrees or [0, 2pi)")
    if coef == 0:
        raise ValueError("no function left")
    return {"func": g["f"].lower(), "k": _q(g["k"]) / coef, "n": n, "unit": unit}


_register(
    "trig_equation_interval", name="Solve a trigonometric equation over one full turn", topic="trigonometry / trigonometric equations",
    purpose="Every solution of a*sin(n x) = k (or cos, tan) with 0 <= x < 360 degrees or 0 <= x < 2 pi.",
    inputs=[{"symbol": "func", "type": "choice", "choices": _TRIG, "kind": "text", "required": True, "meaning": "sin, cos or tan"},
            _num_in("k", "the value the function must take (after dividing by its coefficient)"),
            {"symbol": "n", "type": "integer", "kind": "number", "required": False, "min": 1, "max": 12, "meaning": "the multiple n in f(n x) (default 1)"},
            {"symbol": "unit", "type": "choice", "choices": _UNITS, "kind": "text", "required": False, "meaning": "degrees ([0, 360)) or radians ([0, 2pi))"}],
    outputs=[_out("count", "number of solutions in the interval"), _out("solutions", "the solutions, ascending", "list")],
    primary="count", conditions=["n a whole number from 1 to 12"],
    method="principal value (asin, acos or atan), the second solution of the period (180 - a, -a, a + 180), then every period: "
           "x = (u + 360 m)/n inside the interval",
    latex=r"\sin u = k \Rightarrow u = \arcsin k,\ 180^\circ - \arcsin k \pmod{360^\circ},\quad u = n x",
    run=trig_equation_interval, verify=_verify_trigeq, reference=_ref_trigeq, wrong=_wrong_trigeq,
    random_inputs=lambda rng: (lambda f: {"func": f, "k": round(rng.uniform(-0.97, 0.97), 3) if f != "tan" else round(rng.uniform(-5, 5), 3),
                                          "n": rng.randint(1, 4), "unit": rng.choice(_UNITS)})(rng.choice(_TRIG)),
    parse=_templated([r"solve (?P<c>[-+]?\d*(?:\.\d+)?)\s*\*?\s*(?P<f>sin|cos|tan)\s*\((?P<n>\d*)\s*\*?\s*x\)\s*=\s*(?P<k>" + _Q
                      + r") (?:for|on|in|over)(?: the interval)? (?P<iv>0\s*<=\s*x\s*<\s*(?:360(?: degrees)?|2\s*pi)|\[\s*0\s*,\s*(?:360|2\s*pi)\s*\)(?: degrees)?)" + _E],
                     _parse_trigeq),
    templates=["Solve sin(x) = 0.5 for 0 <= x < 360 degrees.", "Solve 2cos(x) = 1 on [0, 2pi).", "Solve tan(2x) = 1 for 0 <= x < 360 degrees."],
    examples=[{"inputs": {"func": "sin", "k": 0.5, "n": 1, "unit": "degrees"}, "outputs": {"count": 2.0, "solutions": [30.0, 150.0]}},
              {"inputs": {"func": "cos", "k": 0.5, "n": 1, "unit": "radians"}, "outputs": {"count": 2.0, "solutions": [math.pi / 3, 5 * math.pi / 3]}},
              {"inputs": {"func": "tan", "k": 1, "n": 2, "unit": "degrees"}, "outputs": {"count": 4.0, "solutions": [22.5, 112.5, 202.5, 292.5]}},
              {"inputs": {"func": "sin", "k": 1, "n": 1, "unit": "degrees"}, "outputs": {"count": 1.0, "solutions": [90.0]}},
              {"inputs": {"func": "cos", "k": 2, "n": 1, "unit": "degrees"}, "outputs": {"count": 0.0, "solutions": []}}],
    questions=[{"q": "Solve sin(x) = 0.5 for 0 <= x < 360 degrees.", "outputs": {"count": 2.0, "solutions": [30.0, 150.0]}},
               {"q": "Solve 2cos(x) = 1 on [0, 2pi).", "outputs": {"count": 2.0, "solutions": [math.pi / 3, 5 * math.pi / 3]}},
               {"q": "Solve tan(2x) = 1 for 0 <= x < 360 degrees.", "outputs": {"count": 4.0, "solutions": [22.5, 112.5, 202.5, 292.5]}},
               {"q": "Solve sin(x) = 0 for 0 ≤ x < 2π.", "outputs": {"count": 2.0, "solutions": [0.0, math.pi]}},
               {"q": "Solve 3sin(x) = 4 for 0 <= x < 360 degrees.", "outputs": {"count": 0.0, "solutions": []}}])

_QUAD = {"i": 1, "ii": 2, "iii": 3, "iv": 4, "1": 1, "2": 2, "3": 3, "4": 4, "first": 1, "second": 2, "third": 3, "fourth": 4}


def trig_values_from_one(func, value, quadrant) -> dict:
    _finite(value=value)
    q = int(quadrant)
    if func not in _TRIG or q not in (1, 2, 3, 4):
        raise ValueError("func is sin, cos or tan; the quadrant is 1, 2, 3 or 4")
    sin_sign, cos_sign = (1 if q in (1, 2) else -1), (1 if q in (1, 4) else -1)
    if func in ("sin", "cos") and not 0 < abs(value) < 1:
        raise ValueError("a sine or cosine strictly inside a quadrant lies strictly between -1 and 1 and is not 0")
    if func == "tan" and value == 0:
        raise ValueError("tan = 0 is on an axis, not inside a quadrant")
    if func == "sin":
        if (value > 0) != (sin_sign > 0):
            raise ValueError(f"sine is {'positive' if sin_sign > 0 else 'negative'} in quadrant {q}")
        s, c = value, cos_sign * math.sqrt((1 - value) * (1 + value))
    elif func == "cos":
        if (value > 0) != (cos_sign > 0):
            raise ValueError(f"cosine is {'positive' if cos_sign > 0 else 'negative'} in quadrant {q}")
        c, s = value, sin_sign * math.sqrt((1 - value) * (1 + value))
    else:
        if (value > 0) != (sin_sign * cos_sign > 0):
            raise ValueError(f"tangent is {'positive' if sin_sign * cos_sign > 0 else 'negative'} in quadrant {q}")
        c = cos_sign / math.sqrt(1 + value * value)
        s = value * c
    return {"sin": s, "cos": c, "tan": s / c, "sin2": 2 * s * c, "cos2": (c - s) * (c + s)}


def _verify_trigvals(i, o):
    q = int(i["quadrant"])
    s, c = o["sin"], o["cos"]
    if (s > 0) != (q in (1, 2)) or (c > 0) != (q in (1, 4)):
        return 1.0
    given = {"sin": s, "cos": c, "tan": o["tan"]}[i["func"]]
    return (abs(s * s + c * c - 1) + _rel(given, i["value"]) + _rel(o["tan"] * c, s) + abs(o["sin2"] - 2 * s * c)
            + abs(o["cos2"] - (c * c - s * s)))


def _ref_trigvals(func, value, quadrant):
    import mpmath as mp
    with mp.workdps(30):
        v, q, pi = mp.mpf(repr(float(value))), int(quadrant), mp.pi
        if func == "sin":
            b = mp.asin(v)
            t = b if q in (1, 4) else pi - b
        elif func == "cos":
            b = mp.acos(v)
            t = b if q in (1, 2) else -b
        else:
            b = mp.atan(v)
            t = b if q in (1, 4) else b + pi
        return {"sin": float(mp.sin(t)), "cos": float(mp.cos(t)), "tan": float(mp.tan(t)), "sin2": float(mp.sin(2 * t)), "cos2": float(mp.cos(2 * t))}


def _rand_trigvals(rng):
    q = rng.randint(1, 4)
    f = rng.choice(_TRIG)
    mag = round(rng.uniform(0.05, 0.95), 3) if f != "tan" else round(rng.uniform(0.1, 6), 3)
    sign = {"sin": 1 if q in (1, 2) else -1, "cos": 1 if q in (1, 4) else -1, "tan": 1 if q in (1, 3) else -1}[f]
    return {"func": f, "value": sign * mag, "quadrant": q}


_register(
    "trig_values_from_one", name="Trigonometric values from one value and the quadrant", topic="trigonometry / identities",
    purpose="Given sin, cos or tan of an angle and its quadrant: sin, cos, tan, sin 2t and cos 2t (Pythagorean and double-angle identities).",
    inputs=[{"symbol": "func", "type": "choice", "choices": _TRIG, "kind": "text", "required": True, "meaning": "the function whose value is known"},
            _num_in("value", "its value"),
            {"symbol": "quadrant", "type": "integer", "kind": "number", "required": True, "min": 1, "max": 4, "meaning": "the quadrant of the angle (1 to 4)"}],
    outputs=[_out("sin", "sin t"), _out("cos", "cos t"), _out("tan", "tan t"), _out("sin2", "sin 2t"), _out("cos2", "cos 2t")], primary="cos",
    conditions=["the value's sign agrees with the quadrant", "0 < |sin|, |cos| < 1"],
    method="sin^2 + cos^2 = 1 with the quadrant's signs (ASTC); sin 2t = 2 sin t cos t; cos 2t = cos^2 t - sin^2 t",
    latex=r"\cos t = \pm\sqrt{1 - \sin^2 t},\quad \sin 2t = 2\sin t\cos t,\quad \cos 2t = \cos^2 t - \sin^2 t",
    run=trig_values_from_one, verify=_verify_trigvals, reference=_ref_trigvals,
    wrong=lambda func, value, quadrant: (lambda r: dict(r, sin2=2 * r["sin"]))(trig_values_from_one(func, value, quadrant)),
    random_inputs=_rand_trigvals,
    parse=_templated([r"(?:if|given(?: that)?) (?P<f>sin|cos|tan)\s*\(?(?P<v>[a-z]+)\)?\s*=\s*(?P<val>" + _Q + r") and (?:(?P=v) )?(?:is|lies) in (?:the )?(?:quadrant (?P<q>iv|i{1,3}|[1-4])|(?P<q2>first|second|third|fourth) quadrant),? find (?:[a-z(),\s]|2(?=[a-z]))+" + _E],
                     lambda g: {"func": g["f"].lower(), "value": _q(g["val"]), "quadrant": _QUAD[(g.get("q") or g.get("q2")).lower()]}),
    templates=["If sin(t) = 3/5 and t is in quadrant II, find cos(t), tan(t), sin(2t) and cos(2t).", "Given cos(x) = -5/13 and x lies in the third quadrant, find sin(x) and tan(x)."],
    examples=[{"inputs": {"func": "sin", "value": 0.6, "quadrant": 2}, "outputs": {"cos": -0.8, "tan": -0.75, "sin2": -0.96, "cos2": 0.28}},
              {"inputs": {"func": "cos", "value": -5 / 13, "quadrant": 3}, "outputs": {"sin": -12 / 13, "tan": 2.4}},
              {"inputs": {"func": "tan", "value": -2, "quadrant": 4}, "outputs": {"cos": 1 / math.sqrt(5)}}],
    questions=[{"q": "If sin(t) = 3/5 and t is in quadrant II, find cos(t), tan(t), sin(2t) and cos(2t).", "outputs": {"cos": -0.8, "tan": -0.75, "sin2": -0.96}},
               {"q": "Given cos(x) = -5/13 and x lies in the third quadrant, find sin(x) and tan(x).", "outputs": {"sin": -12 / 13, "tan": 2.4}},
               {"q": "If sin(t) = 3/5 and t is in quadrant III, find cos(t).", "outcome": "INVALID_INPUT"}])


# =====================================================================================================================
# precalculus: vectors, polar coordinates, conic sections, the binomial theorem
# =====================================================================================================================
_VEC = r"[<(]\s*(?P<{0}1>" + _Q + r")\s*,\s*(?P<{0}2>" + _Q + r")(?:\s*,\s*(?P<{0}3>" + _Q + r"))?\s*[>)]"


def _vec(g, p):
    v = [_q(g[f"{p}1"]), _q(g[f"{p}2"])]
    if g.get(f"{p}3") is not None:
        v.append(_q(g[f"{p}3"]))
    return v


def _cross_mag(u, v):
    if len(u) == 2:
        return abs(u[0] * v[1] - u[1] * v[0])
    return math.sqrt((u[1] * v[2] - u[2] * v[1]) ** 2 + (u[2] * v[0] - u[0] * v[2]) ** 2 + (u[0] * v[1] - u[1] * v[0]) ** 2)


def vector_dot_angle(u, v) -> dict:
    if len(u) != len(v) or len(u) not in (2, 3):
        raise ValueError("two vectors of the same dimension (2 or 3)")
    for w in (u, v):
        if all(c == 0 for c in w):
            raise ValueError("the zero vector has no direction, so the angle is not defined")
    dot = math.fsum(a * b for a, b in zip(u, v))
    return {"dot": dot, "mag_u": math.sqrt(math.fsum(a * a for a in u)), "mag_v": math.sqrt(math.fsum(b * b for b in v)),
            "angle_deg": math.degrees(math.atan2(_cross_mag(u, v), dot))}


def _verify_vec(i, o):
    u, v = i["u"], i["v"]
    mu, mv = math.sqrt(sum(a * a for a in u)), math.sqrt(sum(b * b for b in v))
    dot = sum(a * b for a, b in zip(u, v))
    t = math.radians(o["angle_deg"])
    return (_rel(o["dot"], dot) + _rel(o["mag_u"], mu) + _rel(o["mag_v"], mv)
            + abs(math.cos(t) * mu * mv - dot) / max(1.0, mu * mv) + abs(math.sin(t) * mu * mv - _cross_mag(u, v)) / max(1.0, mu * mv))


def _ref_vec(u, v):
    import numpy as np
    U, V = np.array(u, dtype=float), np.array(v, dtype=float)
    nu, nv = float(np.linalg.norm(U)), float(np.linalg.norm(V))
    return {"dot": float(np.dot(U, V)), "mag_u": nu, "mag_v": nv,
            "angle_deg": float(np.degrees(np.arccos(np.clip(np.dot(U, V) / (nu * nv), -1.0, 1.0))))}


def _wrong_vec(u, v):
    w = list(v[1:]) + [v[0]]                              # components paired with the wrong partner
    r = vector_dot_angle(u, v)
    dot = sum(a * b for a, b in zip(u, w))
    return dict(r, dot=dot, angle_deg=math.degrees(math.acos(max(-1.0, min(1.0, dot / (r["mag_u"] * r["mag_v"]))))))


def _rand_vec(rng):
    n = rng.choice([2, 2, 3])
    while True:
        u, v = [float(rng.randint(-9, 9)) for _ in range(n)], [float(rng.randint(-9, 9)) for _ in range(n)]
        if any(u) and any(v) and _cross_mag(u, v) > 1e-9 * (1 + abs(sum(a * b for a, b in zip(u, v)))):
            return {"u": u, "v": v}


_VEC_IN = lambda s, m: {"symbol": s, "type": "vector", "kind": "number", "required": True, "min_length": 2, "max_length": 3, "meaning": m}
_register(
    "vector_dot_angle", name="Dot product and angle between two vectors", topic="precalculus / vectors",
    purpose="The dot product, the magnitudes and the angle between two vectors in the plane or in space.",
    inputs=[_VEC_IN("u", "the first vector, e.g. 3, 4"), _VEC_IN("v", "the second vector")],
    outputs=[_out("dot", "u . v"), _out("mag_u", "|u|"), _out("mag_v", "|v|"), _out("angle_deg", "the angle between them, degrees (0 to 180)")],
    primary="dot", conditions=["same dimension (2 or 3)", "neither vector is zero"],
    method="u . v = sum of products of components; the angle from atan2(|u x v|, u . v) (stable for nearly parallel vectors)",
    latex=r"\mathbf u\cdot\mathbf v = |\mathbf u||\mathbf v|\cos\theta", run=vector_dot_angle, verify=_verify_vec, reference=_ref_vec,
    wrong=_wrong_vec, random_inputs=_rand_vec,
    parse=_templated([r"find the (?:dot product and the )?angle between (?:the vectors )?(?:u = )?" + _VEC.format("u") + r" and (?:v = )?" + _VEC.format("v") + _E,
                      r"find the dot product of (?:the vectors )?(?:u = )?" + _VEC.format("u") + r" and (?:v = )?" + _VEC.format("v") + _E],
                     lambda g: {"u": _vec(g, "u"), "v": _vec(g, "v")}),
    templates=["Find the angle between u = <3, 4> and v = <4, -3>.", "Find the dot product of <1, 2, 3> and <4, -5, 6>."],
    examples=[{"inputs": {"u": "3, 4", "v": "4, -3"}, "outputs": {"dot": 0.0, "angle_deg": 90.0}},
              {"inputs": {"u": [1, 2, 3], "v": [4, -5, 6]}, "outputs": {"dot": 12.0, "mag_u": math.sqrt(14)}},
              {"inputs": {"u": [1, 0], "v": [1, 1]}, "outputs": {"angle_deg": 45.0}}],
    questions=[{"q": "Find the angle between u = <3, 4> and v = <4, -3>.", "outputs": {"dot": 0.0, "angle_deg": 90.0}},
               {"q": "Find the dot product of <1, 2, 3> and <4, -5, 6>.", "outputs": {"dot": 12.0}},
               {"q": "Find the dot product and the angle between the vectors (1, 0) and (1, 1).", "outputs": {"angle_deg": 45.0}},
               {"q": "Find the angle between u = <0, 0> and v = <1, 2>.", "outcome": "INVALID_INPUT"}])


def _direction(x, y):
    """Angle of (x, y) in [0, 360) degrees from acos(x/r) and the sign of y."""
    r = math.hypot(x, y)
    t = math.degrees(math.acos(max(-1.0, min(1.0, x / r))))
    return (360.0 - t if y < 0 else t) % 360.0


def vector_magnitude_direction(x, y) -> dict:
    _finite(x=x, y=y)
    if x == 0 and y == 0:
        raise ValueError("the zero vector has no direction")
    return {"magnitude": math.hypot(x, y), "direction_deg": _direction(x, y)}


def _ref_magdir(x, y):
    import numpy as np
    return {"magnitude": float(np.hypot(x, y)), "direction_deg": float(np.mod(np.degrees(np.arctan2(y, x)), 360.0))}


def _verify_magdir(i, o):
    t = math.radians(o["direction_deg"])
    if not 0 <= o["direction_deg"] < 360:
        return 1.0
    r = o["magnitude"]
    return (abs(r * r - i["x"] ** 2 - i["y"] ** 2) / max(1.0, r * r) + abs(r * math.cos(t) - i["x"]) / max(1.0, r)
            + abs(r * math.sin(t) - i["y"]) / max(1.0, r))


_register(
    "vector_magnitude_direction", name="Magnitude and direction of a vector", topic="precalculus / vectors",
    purpose="The length of the plane vector <x, y> and its direction angle from the positive x-axis (0 to 360 degrees).",
    inputs=[_num_in("x", "the x-component"), _num_in("y", "the y-component")],
    outputs=[_out("magnitude", "|v|"), _out("direction_deg", "the direction angle, degrees in [0, 360)")], primary="magnitude",
    conditions=["not the zero vector"], method="|v| = sqrt(x^2 + y^2); the angle in the quadrant of (x, y)",
    latex=r"|\mathbf v| = \sqrt{x^2 + y^2},\quad \theta = \operatorname{atan2}(y, x)", run=vector_magnitude_direction, verify=_verify_magdir,
    reference=_ref_magdir, wrong=lambda x, y: {"magnitude": math.hypot(x, y), "direction_deg": math.degrees(math.atan2(x, y)) % 360.0},
    random_inputs=lambda rng: {"x": float(rng.randint(-15, 15)) or 2.0, "y": float(rng.randint(-15, 15)) or -3.0},
    parse=_templated([r"find the magnitude and direction(?: angle)? of (?:the vector )?(?:v = )?<\s*(?P<x>" + _Q + r")\s*,\s*(?P<y>" + _Q + r")\s*>" + _E],
                     lambda g: {"x": _q(g["x"]), "y": _q(g["y"])}),
    templates=["Find the magnitude and direction of the vector <-3, 4>."],
    examples=[{"inputs": {"x": -3, "y": 4}, "outputs": {"magnitude": 5.0, "direction_deg": 180 - math.degrees(math.atan(4 / 3))}},
              {"inputs": {"x": 5, "y": -12}, "outputs": {"magnitude": 13.0}}, {"inputs": {"x": 0, "y": -2}, "outputs": {"direction_deg": 270.0}}],
    questions=[{"q": "Find the magnitude and direction of the vector <-3, 4>.", "outputs": {"magnitude": 5.0}},
               {"q": "Find the magnitude and direction angle of v = <0, -2>.", "outputs": {"magnitude": 2.0, "direction_deg": 270.0}},
               {"q": "Find the magnitude and direction of the vector <0, 0>.", "outcome": "INVALID_INPUT"}])


def polar_to_rectangular(r, theta, unit) -> dict:
    _finite(r=r, theta=theta)
    if unit not in _UNITS:
        raise ValueError("the angle unit is degrees or radians")
    t = math.radians(theta) if unit == "degrees" else theta
    x, y = r * math.cos(t), r * math.sin(t)
    snap = lambda v: 0.0 if abs(v) < 1e-15 * max(1.0, abs(r)) else v
    return {"x": snap(x), "y": snap(y)}


def _ref_p2r(r, theta, unit):
    import mpmath as mp
    with mp.workdps(30):
        t = mp.radians(mp.mpf(repr(float(theta)))) if unit == "degrees" else mp.mpf(repr(float(theta)))
        z = mp.mpf(repr(float(r))) * mp.expj(t)
        return {"x": float(mp.re(z)), "y": float(mp.im(z))}


def _parse_polar(g):
    t = g["t"].replace(" ", "").lower()
    if "pi" in t:
        th, unit = _pi_angle(t.replace("radians", "")), "radians"
    elif t.endswith("degrees"):
        th, unit = _q(t[:-7]), "degrees"
    elif t.endswith("radians"):
        th, unit = _q(t[:-7]), "radians"
    else:
        raise ValueError("say degrees or radians")
    return {"r": _q(g["r"]), "theta": th, "unit": unit}


_PANG = r"(?P<t>[-+]?\d*\s*\*?\s*pi(?:\s*/\s*\d+)?(?: radians)?|" + _W + r"\s*(?:degrees|radians))"
_register(
    "polar_to_rectangular", name="Polar to rectangular coordinates", topic="precalculus / polar coordinates",
    purpose="The point (x, y) with polar coordinates (r, theta).",
    inputs=[_num_in("r", "the distance r from the pole"), _num_in("theta", "the angle theta"),
            {"symbol": "unit", "type": "choice", "choices": _UNITS, "kind": "text", "required": True, "meaning": "degrees or radians"}],
    outputs=[_out("x", "x = r cos(theta)"), _out("y", "y = r sin(theta)")], primary="x", conditions=["the angle's unit stated"],
    method="x = r cos(theta), y = r sin(theta)", latex=r"x = r\cos\theta,\quad y = r\sin\theta", run=polar_to_rectangular,
    verify=lambda i, o: (lambda t: abs(o["x"] - i["r"] * math.cos(t)) / max(1.0, abs(i["r"])) + abs(o["y"] - i["r"] * math.sin(t)) / max(1.0, abs(i["r"])))(
        math.radians(i["theta"]) if i["unit"] == "degrees" else i["theta"]),
    reference=_ref_p2r, wrong=lambda r, theta, unit: (lambda p: {"x": p["y"], "y": p["x"]})(polar_to_rectangular(r, theta, unit)),
    random_inputs=lambda rng: (lambda u: {"r": round(rng.uniform(0.5, 20), 2), "theta": round(rng.uniform(-360, 360), 1) if u == "degrees" else round(rng.uniform(-7, 7), 3), "unit": u})(rng.choice(_UNITS)),
    parse=_templated([r"convert the polar (?:point|coordinates) \((?P<r>" + _Q + r"),\s*" + _PANG + r"\) (?:to|into) rectangular(?: coordinates)?" + _E,
                      r"convert \((?P<r>" + _Q + r"),\s*" + _PANG + r"\) from polar to rectangular(?: coordinates)?" + _E],
                     _parse_polar),
    templates=["Convert the polar point (4, 60 degrees) to rectangular coordinates.", "Convert (2, 3pi/4) from polar to rectangular coordinates."],
    examples=[{"inputs": {"r": 4, "theta": 60, "unit": "degrees"}, "outputs": {"x": 2.0, "y": 2 * math.sqrt(3)}},
              {"inputs": {"r": 2, "theta": 3 * math.pi / 4, "unit": "radians"}, "outputs": {"x": -math.sqrt(2), "y": math.sqrt(2)}}],
    questions=[{"q": "Convert the polar point (4, 60 degrees) to rectangular coordinates.", "outputs": {"x": 2.0, "y": 2 * math.sqrt(3)}},
               {"q": "Convert (2, 3pi/4) from polar to rectangular coordinates.", "outputs": {"x": -math.sqrt(2), "y": math.sqrt(2)}},
               {"q": "Convert the polar point (4, 60) to rectangular coordinates.", "outcome": "UNRECOGNIZED_FORMAT"}])


def rectangular_to_polar(x, y) -> dict:
    _finite(x=x, y=y)
    if x == 0 and y == 0:
        raise ValueError("the origin has r = 0 and no defined angle")
    d = _direction(x, y)
    return {"r": math.hypot(x, y), "theta_deg": d, "theta_rad": math.radians(d)}


def _ref_r2p(x, y):
    import numpy as np
    t = float(np.mod(np.arctan2(y, x), 2 * np.pi))
    return {"r": float(np.hypot(x, y)), "theta_deg": float(np.degrees(t)), "theta_rad": t}


_register(
    "rectangular_to_polar", name="Rectangular to polar coordinates", topic="precalculus / polar coordinates",
    purpose="Polar coordinates (r, theta) of the point (x, y), with r > 0 and theta in [0, 360) degrees / [0, 2 pi).",
    inputs=[_num_in("x", "x-coordinate"), _num_in("y", "y-coordinate")],
    outputs=[_out("r", "r = sqrt(x^2 + y^2)"), _out("theta_deg", "theta in degrees, [0, 360)"), _out("theta_rad", "theta in radians, [0, 2 pi)")],
    primary="r", conditions=["not the origin"], method="r = sqrt(x^2 + y^2); theta in the quadrant of (x, y)",
    latex=r"r = \sqrt{x^2+y^2},\quad \tan\theta = \frac{y}{x}", run=rectangular_to_polar,
    verify=lambda i, o: _verify_magdir(i, {"magnitude": o["r"], "direction_deg": o["theta_deg"]}) + abs(o["theta_rad"] - math.radians(o["theta_deg"])),
    reference=_ref_r2p, wrong=lambda x, y: (lambda d: {"r": math.hypot(x, y), "theta_deg": d, "theta_rad": math.radians(d)})(math.degrees(math.atan2(x, y)) % 360.0),
    random_inputs=lambda rng: {"x": float(rng.randint(-15, 15)) or 1.0, "y": float(rng.randint(-15, 15)) or -1.0},
    parse=_templated([r"convert (?:the point )?\((?P<x>" + _Q + r"),\s*(?P<y>" + _Q + r")\) (?:to|into) polar(?: coordinates)?" + _E,
                      r"(?:find|what are) the polar coordinates of (?:the point )?\((?P<x>" + _Q + r"),\s*(?P<y>" + _Q + r")\)" + _E],
                     lambda g: {"x": _q(g["x"]), "y": _q(g["y"])}),
    templates=["Convert the point (3, -3) to polar coordinates.", "Find the polar coordinates of (-1, 1)."],
    examples=[{"inputs": {"x": 3, "y": -3}, "outputs": {"r": 3 * math.sqrt(2), "theta_deg": 315.0}}, {"inputs": {"x": -1, "y": 1}, "outputs": {"theta_deg": 135.0}}],
    questions=[{"q": "Convert the point (3, -3) to polar coordinates.", "outputs": {"r": 3 * math.sqrt(2), "theta_deg": 315.0}},
               {"q": "Find the polar coordinates of (-1, 1).", "outputs": {"theta_deg": 135.0, "theta_rad": 3 * math.pi / 4}},
               {"q": "Convert the point (0, 0) to polar coordinates.", "outcome": "INVALID_INPUT"}])


# --- conic sections -----------------------------------------------------------------------------------------------
_ORIENT = ["vertical", "horizontal"]


def parabola_focus_directrix(orientation, m, h=0.0, k=0.0) -> dict:
    """(x - h)^2 = m (y - k) (vertical axis) or (y - k)^2 = m (x - h) (horizontal axis); m = 4p."""
    _finite(m=m, h=h, k=k)
    if m == 0:
        raise ValueError("m must not be 0 (no parabola)")
    if orientation not in _ORIENT:
        raise ValueError("orientation is vertical or horizontal")
    p = m / 4
    if orientation == "vertical":
        return {"p": p, "focus_x": h, "focus_y": k + p, "directrix": k - p, "vertex_x": h, "vertex_y": k}
    return {"p": p, "focus_x": h + p, "focus_y": k, "directrix": h - p, "vertex_x": h, "vertex_y": k}


def _verify_parabola(i, o):
    m, h, k = i["m"], i.get("h", 0.0), i.get("k", 0.0)
    worst = abs(4 * o["p"] - m) / max(1.0, abs(m))
    for t in (0.5, 1.0, 2.0, -3.0):                      # every point is as far from the focus as from the directrix
        if i["orientation"] == "vertical":
            px, py = h + t, k + t * t / m
            d_line = abs(py - o["directrix"])
        else:
            px, py = h + t * t / m, k + t
            d_line = abs(px - o["directrix"])
        worst = max(worst, abs(math.hypot(px - o["focus_x"], py - o["focus_y"]) - d_line) / max(1.0, d_line))
    return worst


def _ref_parabola(orientation, m, h=0.0, k=0.0):
    import sympy as sp
    f = sp.Symbol("f", real=True)
    M, H, K = (sp.Rational(repr(float(v))) for v in (m, h, k))
    # the focal distance f from the definition at the point t = 1: distance to the focus = distance to the directrix
    if orientation == "vertical":
        P = (H + 1, K + 1 / M)
        sol = sp.solve(sp.Eq((P[0] - H) ** 2 + (P[1] - (K + f)) ** 2, (P[1] - (K - f)) ** 2), f)[0]
        return {"p": float(sol), "focus_x": float(H), "focus_y": float(K + sol), "directrix": float(K - sol)}
    P = (H + 1 / M, K + 1)
    sol = sp.solve(sp.Eq((P[0] - (H + f)) ** 2 + (P[1] - K) ** 2, (P[0] - (H - f)) ** 2), f)[0]
    return {"p": float(sol), "focus_x": float(H + sol), "focus_y": float(K), "directrix": float(H - sol)}


def _shift(sign, num):
    if not sign:
        return 0.0
    return _q(num) if sign == "-" else -_q(num)


def _parse_parabola(g):
    if g.get("a") is not None or g.get("yform"):
        a = g.get("a")
        a = 1.0 if a in (None, "", "+") else (-1.0 if a == "-" else _q(a))
        return {"orientation": "vertical", "m": 1 / a, "h": 0.0, "k": 0.0}
    v1, v2 = g["v1"].lower(), g["v2"].lower()
    if v1 == v2:
        raise ValueError("one squared variable")
    m = g.get("m")
    m = 1.0 if m in (None, "", "+") else (-1.0 if m.replace(" ", "") == "-" else _q(m))
    s1, s2 = _shift(g.get("s1"), g.get("c1")), _shift(g.get("s2"), g.get("c2"))
    if v1 == "x":
        return {"orientation": "vertical", "m": m, "h": s1, "k": s2}
    return {"orientation": "horizontal", "m": m, "h": s2, "k": s1}


_SQ = r"\(?(?P<v1>[xy])\s*(?:(?P<s1>[-+])\s*(?P<c1>" + _W + r"))?\)?\s*\^\s*2"
_LIN = r"(?P<m>[-+]?\s*(?:\d+(?:\.\d+)?(?:/\d+)?)?)\s*\*?\s*\(?(?P<v2>[xy])\s*(?:(?P<s2>[-+])\s*(?P<c2>" + _W + r"))?\)?"
_register(
    "parabola_focus_directrix", name="Focus and directrix of a parabola", topic="precalculus / conic sections",
    purpose="Vertex, focal distance p, focus and directrix of (x - h)^2 = 4p(y - k) or (y - k)^2 = 4p(x - h).",
    inputs=[{"symbol": "orientation", "type": "choice", "choices": _ORIENT, "kind": "text", "required": True, "meaning": "vertical axis ((x-h)^2 = m(y-k)) or horizontal"},
            _num_in("m", "the coefficient m = 4p", "m != 0"), _num_in("h", "x of the vertex", required=False), _num_in("k", "y of the vertex", required=False)],
    outputs=[_out("p", "focal distance p = m/4 (signed)"), _out("focus_x", "x of the focus"), _out("focus_y", "y of the focus"),
             _out("directrix", "the directrix: y = this (vertical axis) or x = this (horizontal axis)"), _out("vertex_x", "x of the vertex"), _out("vertex_y", "y of the vertex")],
    primary="p", conditions=["m != 0"], method="compare with (x - h)^2 = 4p(y - k): p = m/4, focus at distance p from the vertex along the axis",
    latex=r"(x-h)^2 = 4p(y-k):\ F = (h, k+p),\ y = k - p", run=parabola_focus_directrix, verify=_verify_parabola, reference=_ref_parabola,
    wrong=lambda orientation, m, h=0.0, k=0.0: (lambda r: r)(dict(parabola_focus_directrix(orientation, 4 * m, h, k))),
    random_inputs=lambda rng: {"orientation": rng.choice(_ORIENT), "m": float(rng.choice([v for v in range(-20, 21) if v])), "h": float(rng.randint(-6, 6)), "k": float(rng.randint(-6, 6))},
    parse=_templated([r"find the (?:vertex, )?focus and (?:the )?directrix of (?:the parabola )?" + _SQ + r"\s*=\s*" + _LIN + _E,
                      r"find the (?:vertex, )?focus and (?:the )?directrix of (?:the parabola )?(?P<yform>y)\s*=\s*(?P<a>[-+]?\s*(?:\d+(?:\.\d+)?(?:/\d+)?)?)\s*\*?\s*x\s*\^\s*2" + _E],
                     _parse_parabola),
    templates=["Find the focus and directrix of the parabola x^2 = 12y.", "Find the focus and directrix of (y - 1)^2 = -8(x - 3).", "Find the focus and directrix of y = 2x^2."],
    examples=[{"inputs": {"orientation": "vertical", "m": 12}, "outputs": {"p": 3.0, "focus_x": 0.0, "focus_y": 3.0, "directrix": -3.0}},
              {"inputs": {"orientation": "horizontal", "m": -8, "h": 3, "k": 1}, "outputs": {"p": -2.0, "focus_x": 1.0, "focus_y": 1.0, "directrix": 5.0}},
              {"inputs": {"orientation": "vertical", "m": 0.5}, "outputs": {"p": 0.125, "focus_y": 0.125}}],
    questions=[{"q": "Find the focus and directrix of the parabola x^2 = 12y.", "outputs": {"p": 3.0, "focus_y": 3.0, "directrix": -3.0}},
               {"q": "Find the focus and directrix of (y - 1)^2 = -8(x - 3).", "outputs": {"focus_x": 1.0, "focus_y": 1.0, "directrix": 5.0}},
               {"q": "Find the focus and directrix of y = 2x^2.", "outputs": {"p": 0.125}},
               {"q": "Find the focus and directrix of the parabola (x + 2)^2 = 4(y - 5).", "outputs": {"focus_x": -2.0, "focus_y": 6.0, "directrix": 4.0}}])


def _conic_term(t):
    m = re.fullmatch(r"\(?\s*(?P<v>[xy])\s*(?:(?P<s>[-+])\s*(?P<c>" + _W + r"))?\s*\)?\s*\^\s*2\s*/\s*(?P<d>" + _W + r")(?:\s*\^\s*(?P<p>2))?", t.strip())
    if not m:
        return None
    den = _q(m.group("d")) ** (2 if m.group("p") else 1)
    return m.group("v").lower(), _shift(m.group("s"), m.group("c")), den


def ellipse_parameters(h, k, A, B) -> dict:
    """(x - h)^2/A + (y - k)^2/B = 1 with A, B > 0, A != B."""
    _finite(h=h, k=k, A=A, B=B)
    if A <= 0 or B <= 0:
        raise ValueError("both denominators must be positive")
    if A == B:
        raise ValueError("equal denominators give a circle (no distinct foci)")
    a, b = math.sqrt(max(A, B)), math.sqrt(min(A, B))
    c = math.sqrt(abs(A - B))
    if A > B:
        f = [(h - c, k), (h + c, k)]
    else:
        f = [(h, k - c), (h, k + c)]
    return {"a": a, "b": b, "c": c, "eccentricity": c / a, "f1x": f[0][0], "f1y": f[0][1], "f2x": f[1][0], "f2y": f[1][1],
            "orientation": "horizontal" if A > B else "vertical"}


def _verify_ellipse(i, o):
    h, k, A, B = i["h"], i["k"], i["A"], i["B"]
    r = abs(o["c"] ** 2 - (o["a"] ** 2 - o["b"] ** 2)) / max(1.0, o["a"] ** 2) + _rel(o["a"] ** 2, max(A, B)) + _rel(o["b"] ** 2, min(A, B))
    r += _rel(o["eccentricity"] * o["a"], o["c"])
    for t in (0.3, 1.7, 4.0):                            # every point: distances to the foci add to 2a
        px, py = h + math.sqrt(A) * math.cos(t), k + math.sqrt(B) * math.sin(t)
        s = math.hypot(px - o["f1x"], py - o["f1y"]) + math.hypot(px - o["f2x"], py - o["f2y"])
        r += abs(s - 2 * o["a"]) / max(1.0, o["a"])
    return r


def _ref_ellipse(h, k, A, B):
    import sympy as sp
    R = lambda v: sp.Rational(repr(float(v)))
    E = sp.Ellipse(sp.Point(R(h), R(k)), hradius=sp.sqrt(R(A)), vradius=sp.sqrt(R(B)))
    f1, f2 = sorted(E.foci, key=lambda P: (float(P.x), float(P.y)))
    return {"a": float(E.major), "b": float(E.minor), "c": float(E.focus_distance), "eccentricity": float(E.eccentricity),
            "f1x": float(f1.x), "f1y": float(f1.y), "f2x": float(f2.x), "f2y": float(f2.y)}


def _parse_ellipse(g):
    t1, t2 = _conic_term(g["t1"]), _conic_term(g["t2"])
    if not t1 or not t2 or {t1[0], t2[0]} != {"x", "y"}:
        raise ValueError("one x term and one y term")
    tx, ty = (t1, t2) if t1[0] == "x" else (t2, t1)
    return {"h": tx[1], "k": ty[1], "A": tx[2], "B": ty[2]}


_CT = r"\(?\s*[xy](?:\s*[-+]\s*\d+(?:\.\d+)?)?\s*\)?\s*\^\s*2\s*/\s*\d+(?:\.\d+)?(?:\s*\^\s*2)?"
_register(
    "ellipse_parameters", name="Foci, axes and eccentricity of an ellipse", topic="precalculus / conic sections",
    purpose="For (x - h)^2/A + (y - k)^2/B = 1: semi-axes a and b, focal distance c, eccentricity and the two foci.",
    inputs=[_num_in("h", "x of the centre"), _num_in("k", "y of the centre"), _num_in("A", "denominator under the x term (positive)"),
            _num_in("B", "denominator under the y term (positive)")],
    outputs=[_out("a", "semi-major axis"), _out("b", "semi-minor axis"), _out("c", "centre-to-focus distance"), _out("eccentricity", "e = c/a"),
             _out("f1x", "first focus x"), _out("f1y", "first focus y"), _out("f2x", "second focus x"), _out("f2y", "second focus y"),
             _out("orientation", "the major axis is horizontal or vertical", "text")],
    primary="c", conditions=["A, B > 0", "A != B (otherwise a circle)"], method="a^2 = the larger denominator, b^2 = the smaller, c^2 = a^2 - b^2; foci on the major axis",
    latex=r"\frac{(x-h)^2}{A} + \frac{(y-k)^2}{B} = 1,\quad c^2 = a^2 - b^2,\quad e = \frac{c}{a}",
    run=ellipse_parameters, verify=_verify_ellipse, reference=_ref_ellipse,
    wrong=lambda h, k, A, B: dict(ellipse_parameters(h, k, A, B), c=math.sqrt(A + B)),
    random_inputs=lambda rng: (lambda A, B: {"h": float(rng.randint(-5, 5)), "k": float(rng.randint(-5, 5)), "A": A, "B": B if B != A else A + 3})(
        float(rng.randint(1, 60)), float(rng.randint(1, 60))),
    parse=_templated([r"find the (?:foci|centre|center|axes)(?:,? (?:and )?(?:the )?(?:foci|vertices|axes|eccentricity|centre|center))* of (?:the ellipse )?(?P<t1>" + _CT + r")\s*\+\s*(?P<t2>" + _CT + r")\s*=\s*1" + _E],
                     _parse_ellipse),
    templates=["Find the foci and eccentricity of the ellipse x^2/25 + y^2/9 = 1.", "Find the foci of (x - 1)^2/16 + (y + 2)^2/25 = 1."],
    examples=[{"inputs": {"h": 0, "k": 0, "A": 25, "B": 9}, "outputs": {"a": 5.0, "b": 3.0, "c": 4.0, "eccentricity": 0.8, "f1x": -4.0, "f2x": 4.0}},
              {"inputs": {"h": 1, "k": -2, "A": 16, "B": 25}, "outputs": {"c": 3.0, "f1y": -5.0, "f2y": 1.0, "orientation": "vertical"}}],
    questions=[{"q": "Find the foci and eccentricity of the ellipse x^2/25 + y^2/9 = 1.", "outputs": {"c": 4.0, "eccentricity": 0.8}},
               {"q": "Find the foci of (x - 1)^2/16 + (y + 2)^2/25 = 1.", "outputs": {"f1x": 1.0, "f1y": -5.0, "f2y": 1.0}},
               {"q": "Find the foci of the ellipse x^2/4 + y^2/4 = 1.", "outcome": "INVALID_INPUT"}])


def hyperbola_parameters(h, k, A, B, orientation) -> dict:
    """horizontal: (x - h)^2/A - (y - k)^2/B = 1; vertical: (y - k)^2/A - (x - h)^2/B = 1; A, B > 0."""
    _finite(h=h, k=k, A=A, B=B)
    if A <= 0 or B <= 0:
        raise ValueError("both denominators must be positive")
    if orientation not in _ORIENT:
        raise ValueError("orientation is horizontal or vertical")
    a, b = math.sqrt(A), math.sqrt(B)
    c = math.sqrt(A + B)
    f = [(h - c, k), (h + c, k)] if orientation == "horizontal" else [(h, k - c), (h, k + c)]
    return {"a": a, "b": b, "c": c, "eccentricity": c / a, "asymptote_slope": b / a if orientation == "horizontal" else a / b,
            "f1x": f[0][0], "f1y": f[0][1], "f2x": f[1][0], "f2y": f[1][1]}


def _hyp_point(i, t):
    a, b = math.sqrt(i["A"]), math.sqrt(i["B"])
    if i["orientation"] == "horizontal":
        return i["h"] + a * math.cosh(t), i["k"] + b * math.sinh(t)
    return i["h"] + b * math.sinh(t), i["k"] + a * math.cosh(t)


def _verify_hyperbola(i, o):
    r = abs(o["c"] ** 2 - i["A"] - i["B"]) / max(1.0, o["c"] ** 2) + _rel(o["a"] ** 2, i["A"]) + _rel(o["b"] ** 2, i["B"]) + _rel(o["eccentricity"] * o["a"], o["c"])
    r += _rel(o["asymptote_slope"], o["b"] / o["a"] if i["orientation"] == "horizontal" else o["a"] / o["b"])
    for t in (0.4, 1.1, -2.0):                           # every point: the distances to the foci differ by 2a
        px, py = _hyp_point(i, t)
        d = abs(math.hypot(px - o["f1x"], py - o["f1y"]) - math.hypot(px - o["f2x"], py - o["f2y"]))
        r += abs(d - 2 * o["a"]) / max(1.0, o["a"])
    return r


def _ref_hyperbola(h, k, A, B, orientation):
    import mpmath as mp
    with mp.workdps(40):
        h_, k_, A_, B_ = (mp.mpf(repr(float(v))) for v in (h, k, A, B))
        a, b = mp.sqrt(A_), mp.sqrt(B_)
        hor = orientation == "horizontal"
        P = (h_ + a * mp.cosh(1), k_ + b * mp.sinh(1)) if hor else (h_ + b * mp.sinh(1), k_ + a * mp.cosh(1))
        foci = lambda c: ((h_ - c, k_), (h_ + c, k_)) if hor else ((h_, k_ - c), (h_, k_ + c))
        gap = lambda c: abs(_mp_dist(P, foci(c)[0]) - _mp_dist(P, foci(c)[1])) - 2 * a        # the defining property
        c = mp.findroot(gap, a + b)
        T = 60                                            # the asymptote's slope, read far out on the branch
        Q = (a * mp.cosh(T), b * mp.sinh(T)) if hor else (b * mp.sinh(T), a * mp.cosh(T))
        f1, f2 = foci(c)
        return {"a": float(a), "b": float(b), "c": float(c), "eccentricity": float(c / a), "asymptote_slope": float(Q[1] / Q[0]),
                "f1x": float(f1[0]), "f1y": float(f1[1]), "f2x": float(f2[0]), "f2y": float(f2[1])}


def _parse_hyperbola(g):
    t1, t2 = _conic_term(g["t1"]), _conic_term(g["t2"])
    if not t1 or not t2 or {t1[0], t2[0]} != {"x", "y"}:
        raise ValueError("one x term and one y term")
    tx, ty = (t1, t2) if t1[0] == "x" else (t2, t1)
    orient = "horizontal" if t1[0] == "x" else "vertical"
    return {"h": tx[1], "k": ty[1], "A": t1[2], "B": t2[2], "orientation": orient}


_register(
    "hyperbola_parameters", name="Foci, eccentricity and asymptotes of a hyperbola", topic="precalculus / conic sections",
    purpose="For (x - h)^2/A - (y - k)^2/B = 1 (or the vertical form): a, b, c, eccentricity, the asymptotes' slope and the foci.",
    inputs=[_num_in("h", "x of the centre"), _num_in("k", "y of the centre"), _num_in("A", "denominator of the positive term"),
            _num_in("B", "denominator of the negative term"),
            {"symbol": "orientation", "type": "choice", "choices": _ORIENT, "kind": "text", "required": True, "meaning": "horizontal (x term positive) or vertical"}],
    outputs=[_out("a", "a = sqrt(A)"), _out("b", "b = sqrt(B)"), _out("c", "centre-to-focus distance"), _out("eccentricity", "e = c/a"),
             _out("asymptote_slope", "slope of the asymptotes (plus or minus this)"), _out("f1x", "first focus x"), _out("f1y", "first focus y"),
             _out("f2x", "second focus x"), _out("f2y", "second focus y")],
    primary="c", conditions=["A, B > 0"], method="c^2 = a^2 + b^2; asymptotes y - k = ±(b/a)(x - h) (horizontal) or ±(a/b) (vertical)",
    latex=r"\frac{(x-h)^2}{a^2} - \frac{(y-k)^2}{b^2} = 1,\quad c^2 = a^2 + b^2", run=hyperbola_parameters, verify=_verify_hyperbola,
    reference=_ref_hyperbola, wrong=lambda h, k, A, B, orientation: dict(hyperbola_parameters(h, k, A, B, orientation), c=math.sqrt(abs(A - B)) + 0.5),
    random_inputs=lambda rng: {"h": float(rng.randint(-5, 5)), "k": float(rng.randint(-5, 5)), "A": float(rng.randint(1, 50)), "B": float(rng.randint(1, 50)),
                               "orientation": rng.choice(_ORIENT)},
    parse=_templated([r"find the (?:foci|asymptotes|eccentricity|vertices)(?:,? (?:and )?(?:the )?(?:foci|asymptotes|eccentricity|vertices))* of (?:the hyperbola )?(?P<t1>" + _CT + r")\s*-\s*(?P<t2>" + _CT + r")\s*=\s*1" + _E],
                     _parse_hyperbola),
    templates=["Find the foci and asymptotes of the hyperbola x^2/16 - y^2/9 = 1.", "Find the foci of (y - 2)^2/9 - (x + 1)^2/16 = 1."],
    examples=[{"inputs": {"h": 0, "k": 0, "A": 16, "B": 9, "orientation": "horizontal"}, "outputs": {"c": 5.0, "eccentricity": 1.25, "asymptote_slope": 0.75, "f2x": 5.0}},
              {"inputs": {"h": -1, "k": 2, "A": 9, "B": 16, "orientation": "vertical"}, "outputs": {"c": 5.0, "f1y": -3.0, "f2y": 7.0, "asymptote_slope": 0.75}}],
    questions=[{"q": "Find the foci and asymptotes of the hyperbola x^2/16 - y^2/9 = 1.", "outputs": {"c": 5.0, "asymptote_slope": 0.75}},
               {"q": "Find the foci of (y - 2)^2/9 - (x + 1)^2/16 = 1.", "outputs": {"f1x": -1.0, "f1y": -3.0, "f2y": 7.0}}])


def binomial_coefficient_term(a, b, n, k) -> dict:
    _finite(a=a, b=b)
    n, k = int(n), int(k)
    if not 0 <= n <= 60 or k < 0:
        raise ValueError("n must be a whole number from 0 to 60 and k a whole number >= 0")
    if k > n:
        return {"coefficient": 0.0, "binomial": 0.0}
    A, B = _fr(a), _fr(b)
    return {"coefficient": float(math.comb(n, k) * A ** k * B ** (n - k)), "binomial": float(math.comb(n, k))}


def _verify_binom(i, o):
    n, k = int(i["n"]), int(i["k"])
    co = [Fraction(1)]                                  # expand (a x + b)^n by repeated multiplication
    A, B = _fr(i["a"]), _fr(i["b"])
    for _ in range(n):
        co = [(co[j] * B if j < len(co) else 0) + (co[j - 1] * A if j >= 1 else 0) for j in range(len(co) + 1)]
    want = co[k] if k < len(co) else Fraction(0)
    pas = [1]
    for _ in range(n):
        pas = [x + y for x, y in zip([0] + pas, pas + [0])]
    return _rel(o["coefficient"], float(want)) + _rel(o["binomial"], pas[k] if k <= n else 0)


def _ref_binom(a, b, n, k):
    import sympy as sp
    x = sp.Symbol("x")
    e = sp.expand((sp.Rational(repr(float(a))) * x + sp.Rational(repr(float(b)))) ** int(n))
    return {"coefficient": float(e.coeff(x, int(k))), "binomial": float(sp.binomial(int(n), int(k)))}


def _parse_binom(g):
    co = _poly_text(g["lin"], "x", 1)
    if co is None or len(co) != 2:
        raise ValueError("a binomial in x")
    k = 0 if g.get("const") else (int(g["k"]) if g.get("k") else 1)
    return {"a": co[0], "b": co[1], "n": int(g["n"]), "k": k}


_register(
    "binomial_coefficient_term", name="Coefficient of a term in a binomial expansion", topic="precalculus / binomial theorem",
    purpose="The coefficient of x^k in (a*x + b)^n, by the binomial theorem.",
    inputs=[_num_in("a", "coefficient of x in the binomial"), _num_in("b", "constant of the binomial"),
            {"symbol": "n", "type": "integer", "kind": "number", "required": True, "min": 0, "max": 60, "meaning": "the power n"},
            {"symbol": "k", "type": "integer", "kind": "number", "required": True, "min": 0, "meaning": "the power of x whose coefficient is wanted"}],
    outputs=[_out("coefficient", "coefficient of x^k"), _out("binomial", "the binomial coefficient C(n, k)")], primary="coefficient",
    conditions=["0 <= n <= 60", "k >= 0"], method="binomial theorem: the x^k term is C(n, k) (ax)^k b^(n-k)",
    latex=r"(ax + b)^n = \sum_{k=0}^{n} \binom{n}{k} a^k b^{n-k} x^k", run=binomial_coefficient_term, verify=_verify_binom, reference=_ref_binom,
    wrong=lambda a, b, n, k: {"coefficient": float(math.comb(int(n), int(k)) * _fr(a) ** int(k)) if int(k) <= int(n) else 1.0, "binomial": float(math.comb(int(n), int(k)))},
    random_inputs=lambda rng: (lambda n: {"a": float(rng.choice([1, 2, 3, -1, -2, 0.5])), "b": float(rng.choice([2, 3, -1, -2, 4, -3, 5])), "n": n, "k": rng.randint(0, n - 1)})(rng.randint(2, 15)),
    parse=_templated([r"(?:find|what is) the coefficient of x(?:\^(?P<k>\d+))? in (?:the expansion of )?\((?P<lin>[^()]+)\)\^(?P<n>\d+)" + _E,
                      r"(?:find|what is) the (?P<const>constant term) (?:in|of) (?:the expansion of )?\((?P<lin>[^()]+)\)\^(?P<n>\d+)" + _E],
                     _parse_binom),
    templates=["Find the coefficient of x^3 in (2x - 1)^5.", "What is the coefficient of x^2 in the expansion of (x + 3)^6?", "Find the constant term in (3 - 2x)^4."],
    examples=[{"inputs": {"a": 2, "b": -1, "n": 5, "k": 3}, "outputs": {"coefficient": 80.0, "binomial": 10.0}},
              {"inputs": {"a": 1, "b": 3, "n": 6, "k": 2}, "outputs": {"coefficient": 1215.0}}, {"inputs": {"a": -2, "b": 3, "n": 4, "k": 0}, "outputs": {"coefficient": 81.0}}],
    questions=[{"q": "Find the coefficient of x^3 in (2x - 1)^5.", "outputs": {"coefficient": 80.0}},
               {"q": "What is the coefficient of x^2 in the expansion of (x + 3)^6?", "outputs": {"coefficient": 1215.0}},
               {"q": "Find the constant term in (3 - 2x)^4.", "outputs": {"coefficient": 81.0}},
               {"q": "Find the coefficient of x in (x + 2)^3.", "outputs": {"coefficient": 12.0}}])


# =====================================================================================================================
# complex numbers
# =====================================================================================================================
def _cplx(text: str):
    """'3 - 2i', '-i', '4', 'sqrt(3) + i', '2i' -> (real, imaginary) as exact Fractions where possible."""
    s = _clean(text).lower().replace(" ", "").replace("*", "")
    if not s or not re.fullmatch(r"[0-9.+\-/()sqrti]+", s):
        raise ValueError("not a complex number")
    if s[0] not in "+-":
        s = "+" + s
    parts, depth, cur = [], 0, ""
    for ch in s:
        if ch in "+-" and depth == 0 and cur:
            parts.append(cur)
            cur = ""
        depth += {"(": 1, ")": -1}.get(ch, 0)
        cur += ch
    parts.append(cur)
    re_, im = Fraction(0), Fraction(0)
    for p in parts:
        sign, body = (-1 if p[0] == "-" else 1), p[1:]
        if body.endswith("i") and not body.endswith("sqrti"):
            c = body[:-1]
            v = Fraction(1) if c == "" else _eval_ast(c)
            im += sign * (v if isinstance(v, Fraction) else _fr(v))
        else:
            if not body:
                raise ValueError("an empty term")
            v = _eval_ast(body)
            re_ += sign * (v if isinstance(v, Fraction) else _fr(v))
    return re_, im


def complex_arithmetic(a, b, c, d, op) -> dict:
    _finite(a=a, b=b, c=c, d=d)
    z, w = complex(a, b), complex(c, d)
    if op == "/" and w == 0:
        raise ValueError("division by 0")
    A, B, C, D = (_fr(v) for v in (a, b, c, d))
    if op == "+":
        re_, im = A + C, B + D
    elif op == "-":
        re_, im = A - C, B - D
    elif op == "*":
        re_, im = A * C - B * D, A * D + B * C
    else:
        den = C * C + D * D
        re_, im = (A * C + B * D) / den, (B * C - A * D) / den
    return {"re": float(re_), "im": float(im)}


def _verify_carith(i, o):
    z, w, r = complex(i["a"], i["b"]), complex(i["c"], i["d"]), complex(o["re"], o["im"])
    op = i["op"]
    if op == "+":
        return abs(r - (z + w)) / max(1.0, abs(z), abs(w))
    if op == "-":
        return abs(r - (z - w)) / max(1.0, abs(z), abs(w))
    if op == "*":
        return abs(r / w - z) / max(1.0, abs(z)) if w else abs(r)
    return abs(r * w - z) / max(1.0, abs(z), abs(r * w))


def _ref_carith(a, b, c, d, op):
    import sympy as sp
    R = lambda v: sp.Rational(repr(float(v)))
    z, w = R(a) + R(b) * sp.I, R(c) + R(d) * sp.I
    r = sp.expand({"+": z + w, "-": z - w, "*": z * w, "/": sp.simplify(z / w)}[op])
    r = sp.nsimplify(sp.expand(sp.simplify(r)))
    return {"re": float(sp.re(r)), "im": float(sp.im(r))}


def _wrong_carith(a, b, c, d, op):
    if op == "*":
        return {"re": a * c + b * d, "im": a * d + b * c}                 # i^2 taken as +1
    if op == "/":
        den = c * c + d * d
        return {"re": (a * c - b * d) / den, "im": (a * d + b * c) / den}     # multiplied by w instead of its conjugate
    if op == "-":
        return {"re": a - c, "im": b + d}                                   # the minus not distributed
    return complex_arithmetic(a, b, c, d, op)


_COPS = {"+": "+", "plus": "+", "-": "-", "minus": "-", "*": "*", "times": "*", "/": "/", "divided by": "/", "": "*"}


def _parse_carith(g):
    if "i" not in (g["z1"] + g["z2"]).lower().replace("sqrt", ""):
        raise ValueError("not complex")
    (a, b), (c, d) = _cplx(g["z1"]), _cplx(g["z2"])
    return {"a": float(a), "b": float(b), "c": float(c), "d": float(d), "op": _COPS[(g.get("op") or "").strip().lower()]}


_register(
    "complex_arithmetic", name="Arithmetic with complex numbers", topic="complex numbers / arithmetic",
    purpose="(a + bi) + - * / (c + di) in the form re + im*i.",
    inputs=[_num_in("a", "real part of the first number"), _num_in("b", "imaginary part of the first number"),
            _num_in("c", "real part of the second number"), _num_in("d", "imaginary part of the second number"),
            {"symbol": "op", "type": "choice", "choices": ["+", "-", "*", "/"], "kind": "text", "required": True, "meaning": "the operation"}],
    outputs=[_out("re", "real part of the result"), _out("im", "imaginary part of the result")], primary="re",
    conditions=["no division by 0"], method="i^2 = -1; division multiplies numerator and denominator by the conjugate c - di",
    latex=r"\frac{a+bi}{c+di} = \frac{(ac+bd) + (bc-ad)i}{c^2+d^2}", run=complex_arithmetic, verify=_verify_carith, reference=_ref_carith,
    wrong=_wrong_carith,
    random_inputs=lambda rng: {"a": float(rng.randint(-9, 9)), "b": float(rng.choice([v for v in range(-9, 10) if v])), "c": float(rng.randint(-9, 9)),
                               "d": float(rng.choice([v for v in range(-9, 10) if v])), "op": rng.choice(["+", "-", "-", "*", "*", "*", "/", "/", "/"])},
    parse=_templated([r"(?:compute|calculate|simplify|evaluate|find|what is) \((?P<z1>[^()]*(?:\([^()]*\)[^()]*)*)\)\s*(?P<op>[-+*/]|plus|minus|times|divided by)?\s*\((?P<z2>[^()]*(?:\([^()]*\)[^()]*)*)\)" + _E],
                     _parse_carith),
    templates=["Compute (3 + 2i) / (1 - i).", "Simplify (2 - 3i)(4 + i).", "Find (2 - 3i) + (4 + i)."],
    examples=[{"inputs": {"a": 3, "b": 2, "c": 1, "d": -1, "op": "/"}, "outputs": {"re": 0.5, "im": 2.5}},
              {"inputs": {"a": 2, "b": -3, "c": 4, "d": 1, "op": "*"}, "outputs": {"re": 11.0, "im": -10.0}},
              {"inputs": {"a": 2, "b": -3, "c": 4, "d": 1, "op": "+"}, "outputs": {"re": 6.0, "im": -2.0}}],
    questions=[{"q": "Compute (3 + 2i) / (1 - i).", "outputs": {"re": 0.5, "im": 2.5}}, {"q": "Simplify (2 - 3i)(4 + i).", "outputs": {"re": 11.0, "im": -10.0}},
               {"q": "Find (2 - 3i) + (4 + i).", "outputs": {"re": 6.0, "im": -2.0}}, {"q": "Compute (5 + i) - (2 - 4i).", "outputs": {"re": 3.0, "im": 5.0}},
               {"q": "Compute (1 + i) / (0 + 0i).", "outcome": "INVALID_INPUT"}])


def complex_modulus_argument(a, b) -> dict:
    _finite(a=a, b=b)
    if a == 0 and b == 0:
        raise ValueError("0 has modulus 0 and no argument")
    r = math.hypot(a, b)
    t = math.degrees(math.acos(max(-1.0, min(1.0, a / r))))
    t = -t if b < 0 else t
    return {"modulus": r, "argument_deg": t, "argument_rad": math.radians(t),
            "polar": f"{r:.10g}(cos({t:.10g} deg) + i sin({t:.10g} deg))"}


def _ref_cmodarg(a, b):
    import sympy as sp
    z = sp.Rational(repr(float(a))) + sp.Rational(repr(float(b))) * sp.I
    t = sp.arg(z)
    return {"modulus": float(sp.Abs(z)), "argument_deg": float(t * 180 / sp.pi), "argument_rad": float(t)}


_register(
    "complex_modulus_argument", name="Modulus, argument and polar form of a complex number", topic="complex numbers / polar form",
    purpose="|z| and the principal argument (-180, 180] degrees of z = a + bi, and z in polar form.",
    inputs=[_num_in("a", "real part"), _num_in("b", "imaginary part")],
    outputs=[_out("modulus", "|z|"), _out("argument_deg", "principal argument, degrees in (-180, 180]"), _out("argument_rad", "principal argument, radians"),
             _out("polar", "z = r(cos t + i sin t)", "text")],
    primary="modulus", conditions=["z != 0"], method="|z| = sqrt(a^2 + b^2); the argument in the quadrant of (a, b)",
    latex=r"z = r(\cos\theta + i\sin\theta),\quad r = \sqrt{a^2+b^2}", run=complex_modulus_argument,
    verify=lambda i, o: (lambda t: abs(o["modulus"] * math.cos(t) - i["a"]) / max(1.0, o["modulus"]) + abs(o["modulus"] * math.sin(t) - i["b"]) / max(1.0, o["modulus"])
                         + (not -180 < o["argument_deg"] <= 180) + abs(o["argument_rad"] - t) + abs(o["modulus"] ** 2 - i["a"] ** 2 - i["b"] ** 2) / max(1.0, o["modulus"] ** 2))(
        math.radians(o["argument_deg"])),
    reference=_ref_cmodarg, wrong=lambda a, b: (lambda t: {"modulus": math.hypot(a, b), "argument_deg": t, "argument_rad": math.radians(t)})(math.degrees(math.atan2(a, b))),
    random_inputs=lambda rng: {"a": float(rng.randint(-12, 12)) or 3.0, "b": float(rng.randint(-12, 12)) or -2.0},
    parse=_templated([r"find the modulus and (?:the )?argument of (?:z = )?(?P<z>[^?]+?)" + _E, r"(?:write|express) (?:z = )?(?P<z>[^?]+?) in polar form" + _E],
                     lambda g: (lambda ab: {"a": float(ab[0]), "b": float(ab[1])})(_cplx(g["z"]))),
    templates=["Find the modulus and argument of -1 + i.", "Write 1 - i in polar form."],
    examples=[{"inputs": {"a": -1, "b": 1}, "outputs": {"modulus": math.sqrt(2), "argument_deg": 135.0}}, {"inputs": {"a": 1, "b": -1}, "outputs": {"argument_deg": -45.0}},
              {"inputs": {"a": 3, "b": -4}, "outputs": {"modulus": 5.0}}, {"inputs": {"a": -2, "b": 0}, "outputs": {"argument_deg": 180.0}}],
    questions=[{"q": "Find the modulus and argument of -1 + i.", "outputs": {"modulus": math.sqrt(2), "argument_deg": 135.0}},
               {"q": "Write 1 - i in polar form.", "outputs": {"argument_deg": -45.0}}, {"q": "Find the modulus and argument of z = 3 - 4i.", "outputs": {"modulus": 5.0}},
               {"q": "Find the modulus and argument of 0.", "outcome": "INVALID_INPUT"}])


def _cpow_exact(A: Fraction, B: Fraction, n: int):
    """(A + Bi)^n exactly, by binary exponentiation (negative n through the reciprocal)."""
    if n < 0:
        den = A * A + B * B
        A, B, n = A / den, -B / den, -n
    rr, ri = Fraction(1), Fraction(0)
    while n:
        if n & 1:
            rr, ri = rr * A - ri * B, rr * B + ri * A
        A, B = A * A - B * B, 2 * A * B
        n >>= 1
    return rr, ri


def de_moivre_power(a, b, n) -> dict:
    _finite(a=a, b=b)
    n = int(n)
    if abs(n) > 40:
        raise ValueError("the power is limited to 40 in size")
    if a == 0 and b == 0 and n <= 0:
        raise ValueError("0 to a power that is not positive is undefined")
    rr, ri = _cpow_exact(_fr(a), _fr(b), n)
    re_, im = float(rr), float(ri)
    re_, im = (0.0 if abs(re_) < 1e-12 else re_), (0.0 if abs(im) < 1e-12 else im)
    mod = math.hypot(re_, im)
    return {"re": re_, "im": im, "modulus": mod, "argument_deg": math.degrees(math.atan2(im, re_)) if mod else 0.0}


def _verify_demoivre(i, o):
    n = int(i["n"])
    A, B = _fr(i["a"]), _fr(i["b"])
    if n < 0:
        den = A * A + B * B
        A, B = A / den, -B / den
    rr, ri = Fraction(1), Fraction(0)
    for _ in range(abs(n)):                              # straightforward repeated multiplication
        rr, ri = rr * A - ri * B, rr * B + ri * A
    r = abs(o["re"] - float(rr)) / max(1.0, abs(float(rr))) + abs(o["im"] - float(ri)) / max(1.0, abs(float(ri)))
    mod = math.hypot(float(rr), float(ri))
    r += _rel(o["modulus"], mod)
    if mod:
        z = o["modulus"] * complex(math.cos(math.radians(o["argument_deg"])), math.sin(math.radians(o["argument_deg"])))
        r += abs(z - complex(o["re"], o["im"])) / max(1.0, mod)
    return r


def _ref_demoivre(a, b, n):
    import sympy as sp
    z = sp.Rational(repr(float(a))) + sp.Rational(repr(float(b))) * sp.I
    w = sp.expand(z ** int(n))
    re_, im = float(sp.re(w)), float(sp.im(w))
    return {"re": re_, "im": im, "modulus": float(sp.Abs(w))}


def _parse_demoivre(g):
    a, b = _cplx(g["z"])
    return {"a": float(a), "b": float(b), "n": int(g["n"])}


_register(
    "de_moivre_power", name="Power of a complex number (De Moivre)", topic="complex numbers / De Moivre's theorem",
    purpose="(a + bi)^n for a whole number n (|n| <= 40): real and imaginary parts, modulus and argument.",
    inputs=[_num_in("a", "real part"), _num_in("b", "imaginary part"),
            {"symbol": "n", "type": "integer", "kind": "number", "required": True, "min": -40, "max": 40, "meaning": "the power n"}],
    outputs=[_out("re", "real part of z^n"), _out("im", "imaginary part of z^n"), _out("modulus", "|z^n| = |z|^n"), _out("argument_deg", "argument of z^n, degrees")],
    primary="re", conditions=["|n| <= 40", "z != 0 when n <= 0"],
    method="De Moivre: z^n = r^n (cos n t + i sin n t); computed exactly (binary exponentiation in rational arithmetic)",
    latex=r"\left[r(\cos\theta + i\sin\theta)\right]^n = r^n(\cos n\theta + i\sin n\theta)", run=de_moivre_power, verify=_verify_demoivre,
    reference=_ref_demoivre, wrong=lambda a, b, n: (lambda r, i: {"re": r, "im": i, "modulus": math.hypot(r, i), "argument_deg": math.degrees(math.atan2(i, r))})(float(_fr(a) ** int(n)) if a or int(n) > 0 else 1.0, float(_fr(b) ** int(n)) if b or int(n) > 0 else 1.0),
    random_inputs=lambda rng: {"a": float(rng.randint(-4, 4)) or 1.0, "b": float(rng.choice([v for v in range(-4, 5) if v])), "n": rng.choice([v for v in range(-6, 13) if v not in (0, 1)])},
    parse=_templated([r"(?:use de moivre'?s theorem to )?(?:compute|find|calculate|evaluate|simplify) \((?P<z>[^()]*(?:\([^()]*\)[^()]*)*)\)\s*\^\s*\(?(?P<n>-?\d+)\)?(?: using de moivre'?s theorem)?" + _E],
                     _parse_demoivre),
    templates=["Use De Moivre's theorem to compute (1 + i)^8.", "Compute (sqrt(3) + i)^6."],
    examples=[{"inputs": {"a": 1, "b": 1, "n": 8}, "outputs": {"re": 16.0, "im": 0.0, "modulus": 16.0}},
              {"inputs": {"a": 0, "b": 2, "n": 3}, "outputs": {"re": 0.0, "im": -8.0}}, {"inputs": {"a": 1, "b": -1, "n": -2}, "outputs": {"re": 0.0, "im": 0.5}}],
    questions=[{"q": "Use De Moivre's theorem to compute (1 + i)^8.", "outputs": {"re": 16.0, "im": 0.0}},
               {"q": "Compute (sqrt(3) + i)^6.", "outputs": {"re": -64.0, "im": 0.0}}, {"q": "Find (2i)^3.", "outputs": {"im": -8.0}},
               {"q": "Compute (0 + 0i)^-2.", "outcome": "INVALID_INPUT"}])


def _arg360(z):
    t = math.degrees(math.atan2(z.imag, z.real)) % 360.0
    return 0.0 if t > 360.0 - 1e-9 else t


def complex_nth_roots(a, b, n) -> dict:
    _finite(a=a, b=b)
    n = int(n)
    if not 2 <= n <= 12:
        raise ValueError("n must be a whole number from 2 to 12")
    if a == 0 and b == 0:
        raise ValueError("0 has only the root 0")
    r = math.hypot(a, b) ** (1.0 / n)
    t = math.atan2(b, a) % (2 * math.pi)
    roots = [r * complex(math.cos((t + 2 * math.pi * k) / n), math.sin((t + 2 * math.pi * k) / n)) for k in range(n)]
    # 12 significant digits: cos/sin differ in the last bit between platforms (Windows vs Linux libm), and the answer
    # must be identical everywhere; exact zeros are snapped first
    snap = lambda x: 0.0 if abs(x) < 1e-12 * r else float(f"{x:.12g}") + 0.0   # noqa: E731
    roots = [complex(snap(z.real), snap(z.imag)) for z in roots]
    roots.sort(key=_arg360)
    return {"modulus": r, "count": float(n), "roots": roots}


def _verify_nroots(i, o):
    n, z = int(i["n"]), complex(i["a"], i["b"])
    rs = o["roots"]
    if len(rs) != n or o["count"] != n:
        return 1.0
    args = [_arg360(w) for w in rs]
    if any(args[j] >= args[j + 1] for j in range(n - 1)):
        return 1.0
    worst = _rel(o["modulus"] ** n, abs(z))
    for w in rs:
        worst = max(worst, abs(w ** n - z) / abs(z), abs(abs(w) - o["modulus"]) / o["modulus"])
    return worst


def _ref_nroots(a, b, n):
    import mpmath as mp
    with mp.workdps(30):
        z = mp.mpc(repr(float(a)), repr(float(b)))
        rs = mp.polyroots([1] + [0] * (int(n) - 1) + [-z], maxsteps=400, extraprec=80)
        roots = sorted((complex(w) for w in rs), key=lambda w: (float(mp.arg(mp.mpc(w.real, w.imag))) % (2 * float(mp.pi))) if
                       (float(mp.arg(mp.mpc(w.real, w.imag))) % (2 * float(mp.pi))) < 2 * float(mp.pi) - 1e-9 else 0.0)
        return {"modulus": float(abs(z) ** (mp.mpf(1) / int(n))), "count": float(n), "roots": roots}


_NTH = {"square": 2, "cube": 3, "fourth": 4, "fifth": 5, "sixth": 6}


def _parse_nroots(g):
    w = g["n"].lower()
    n = _NTH.get(w) or int(re.match(r"\d+", w).group(0))
    a, b = _cplx(g["z"])
    return {"a": float(a), "b": float(b), "n": n}


_register(
    "complex_nth_roots", name="The n-th roots of a complex number", topic="complex numbers / roots",
    purpose="All n complex n-th roots of z = a + bi (n from 2 to 12), ordered by argument from 0 to 360 degrees.",
    inputs=[_num_in("a", "real part of z"), _num_in("b", "imaginary part of z"),
            {"symbol": "n", "type": "integer", "kind": "number", "required": True, "min": 2, "max": 12, "meaning": "which roots (2 = square roots)"}],
    outputs=[_out("modulus", "the common modulus |z|^(1/n)"), _out("count", "the number of roots, n"), _out("roots", "the roots, by argument in [0, 360)", "list")],
    primary="modulus", conditions=["z != 0", "2 <= n <= 12"],
    method="w_k = |z|^(1/n) (cos((t + 2 pi k)/n) + i sin((t + 2 pi k)/n)), k = 0 .. n-1",
    latex=r"w_k = \sqrt[n]{r}\left(\cos\frac{\theta + 2\pi k}{n} + i\sin\frac{\theta + 2\pi k}{n}\right)", run=complex_nth_roots, verify=_verify_nroots,
    reference=_ref_nroots,
    wrong=lambda a, b, n: (lambda r, t: {"modulus": r, "count": float(n), "roots": sorted((r * complex(math.cos(t + 2 * math.pi * k / int(n)), math.sin(t + 2 * math.pi * k / int(n))) for k in range(int(n))), key=_arg360)})(
        math.hypot(a, b) ** (1.0 / int(n)), math.atan2(b, a) % (2 * math.pi)),
    random_inputs=lambda rng: {"a": float(rng.randint(-20, 20)), "b": float(rng.choice([v for v in range(-20, 21) if v])), "n": rng.randint(2, 8)},
    parse=_templated([r"find (?:all )?(?:of )?the (?P<n>square|cube|fourth|fifth|sixth|\d+(?:st|nd|rd|th)) roots of (?:z = )?(?P<z>[^?]+?)" + _E], _parse_nroots),
    templates=["Find the cube roots of 8i.", "Find all the fourth roots of -16."],
    examples=[{"inputs": {"a": 0, "b": 8, "n": 3}, "outputs": {"modulus": 2.0, "count": 3.0, "roots": [complex(math.sqrt(3), 1), complex(-math.sqrt(3), 1), complex(0, -2)]}},
              {"inputs": {"a": -16, "b": 0, "n": 4}, "outputs": {"modulus": 2.0, "roots": [complex(math.sqrt(2), math.sqrt(2)), complex(-math.sqrt(2), math.sqrt(2)),
                                                                                          complex(-math.sqrt(2), -math.sqrt(2)), complex(math.sqrt(2), -math.sqrt(2))]}}],
    questions=[{"q": "Find the cube roots of 8i.", "outputs": {"modulus": 2.0, "count": 3.0}}, {"q": "Find all the fourth roots of -16.", "outputs": {"modulus": 2.0}},
               {"q": "Find the square roots of 9.", "outputs": {"roots": [complex(3, 0), complex(-3, 0)]}},
               {"q": "Find the cube roots of 0.", "outcome": "INVALID_INPUT"}])
