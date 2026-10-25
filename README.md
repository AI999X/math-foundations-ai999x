# math-foundations-ai999x — offline, verified math foundations solver with step-by-step answers

[![tests](https://github.com/AI999X/math-foundations-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/math-foundations-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/math-foundations-ai999x)](https://pypi.org/project/math-foundations-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/math-foundations-ai999x)

**[Install](https://pypi.org/project/math-foundations-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/math-foundations-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/math-foundations-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

math-foundations-ai999x is an offline, deterministic Python solver that answers math foundations questions with verified results from 119 certified capabilities, gives 625 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**math-foundations-ai999x** is a compact, deterministic math foundations engine for Python. It reads a question in plain English (or a typed call), picks a certified math foundations capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified math foundations capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 625 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** arithmetic, algebra, functions, exponentials and logarithms, trigonometry, precalculus and complex numbers.

## Model overview

| Property | Value |
|---|---|
| Package | `math-foundations-ai999x` 1.0.0 (import `math_foundations_ai999x`) |
| Type | Deterministic neuro-symbolic math foundations engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 19 certified derivations, 100 certified procedures |
| Knowledge pack | version 1.0.2 |
| Golden answers | 625 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install math-foundations-ai999x
```

```python
import math_foundations_ai999x as math_foundations

math_foundations.ask("Solve x^2 - 5x + 6 = 0.").show()
# -> SOLVED — x1 = 2, x2 = 3, discriminant = 1  (quadratic_roots)
math_foundations.ask("In triangle ABC, a = 8, b = 10 and A = 40 degrees. Solve the triangle.").show()
# -> SOLVED — count = 2, B1 = 53.4641, C1 = 86.5359, c1 = 12.423  (law_of_sines_ssa)
math_foundations.ask("Compute (3 + 2i) / (1 - i).").show()
# -> SOLVED — re = 0.5, im = 2.5  (complex_arithmetic)
math_foundations.ask("Find the coefficient of x^3 in (2x - 1)^5.").show()
# -> SOLVED — coefficient = 80, binomial = 10  (binomial_coefficient_term)
math_foundations.solve("de_moivre_power", a=1, b=1, n=8).show()
# -> SOLVED — re = 16, im = 0, modulus = 16, argument_deg = 0  (de_moivre_power)
math_foundations.ask("Who created you?").message
# -> math-foundations-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `math-foundations-ai999x "<question>"`, `math-foundations-ai999x --capabilities`. The outputs above were produced by running this exact code on math-foundations-ai999x 1.0.0.

## Find the right capability, and draw it

`math_foundations.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "math-foundations-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import math_foundations_ai999x as math_foundations

math_foundations.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (algebra-ai999x, 0.67); Solution of two linear equations in two unknowns (algebra-ai999x, 0.57)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

19 certified derivations, 100 certified procedures. A few of them:

- **Derivations:** area of rectangle; associative property of addition; cramer rule x solution; determinant cramer two variables; difference of cubes factorization; difference of squares; distributive property verification; elimination method identity; elimination solution two variables; linear equation solution formula; partial fraction decomposition linear factors; perfect square trinomial; product of sums; quadratic vertex form conversion; radical equation squaring; square of difference; substitution method identity; substitution solution two variables; sum of cubes factorization
- **Procedures:** Absolute value inequality |ax + b| compared with c; Add rational expressions with same denominator; Amount remaining after radioactive decay (half-life); Area of a triangle from its three sides (Heron); Arithmetic with complex numbers; Arithmetic with fractions; Asymptotes of a linear rational function; Average rate of change between two points; Average rate of change of a function between two inputs; Centre and radius of a circle from its general equation; Coefficient of a term in a binomial expansion; Combinations and permutations; Composition of two linear functions; Compound interest; Continuous exponential growth or decay; Degrees to radians; Determinant and inverse of a 2 by 2 matrix; Determinant of 2x2 matrix; Discriminant of a quadratic; Distance between two points; Distance from a point to a line; Divide a quantity in a given ratio; Domain of a square-root function; Dot product and angle between two vectors …

`math_foundations_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified math foundations capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (arithmetic, algebra, functions, exponentials and logarithms, trigonometry, precalculus and complex numbers); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{math_foundations_ai999x_1_0_0,
  title  = {math-foundations-ai999x: an offline, verified math foundations solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/math-foundations-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/math-foundations-ai999x/discussions or visit https://ai999x.com.

## Support the project

math-foundations-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/math-foundations-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/math-foundations-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/math-foundations-ai999x/tree/main/onnx).
