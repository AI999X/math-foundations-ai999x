# Math Foundations (math-foundations-ai999x)

[![tests](https://github.com/AI999X/math-foundations-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/math-foundations-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/math-foundations-ai999x)](https://pypi.org/project/math-foundations-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/math-foundations-ai999x)

Math Foundations covers arithmetic, fractions, percentages, ratios, algebraic equations and inequalities, functions, exponentials and logarithms, trigonometry, precalculus and complex numbers: 106 calculations in 34 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Math Foundations desktop app: Degrees to radians, with its worked example and live scene](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/hero.webp)](https://www.ai999x.com/models/math-foundations-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `math-foundations` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/math-foundations-ai999x/#try) |
| **Python** 3.10+ | `pip install math-foundations-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import math_foundations_ai999x as math_foundations

a = math_foundations.ask("Compute (3 + 2i) / (1 - i).")
print(a.outcome, a.values)
# SOLVED {'re': 0.5, 'im': 2.5}

a = math_foundations.ask(
    "Find the centre and radius of x^2 + y^2 - 4x + 6y - 3 = 0."
)
print(a.outcome, a.values)
# SOLVED {'h': 2.0, 'k': -3.0, 'rad': 4.0}

a = math_foundations.ask("Find the distance between (1, 1) and (4, 5).")
print(a.outcome, a.values)
# SOLVED {'dist': 5.0}

a = math_foundations.ask(
    "How many ways are there to choose 3 books from 10?"
)
print(a.outcome, a.values)
# SOLVED {'combinations': 120.0, 'permutations': 720.0}

a = math_foundations.ask(
    "Find the amount after 2 years if 1000 is invested at rate 0.05 "
    "compounded 1 times a year."
)
print(a.outcome, a.values)
# SOLVED {'amount': 1102.5}

a = math_foundations.ask(
    "What is the doubling time for a growth constant of 0.1?"
)
print(a.outcome, a.values)
# SOLVED {'td': 6.931471805599452}

a = math_foundations.ask(
    "An 80 g sample has a half-life of 5 years. How much remains after "
    "15 years?"
)
print(a.outcome, a.values)
# SOLVED {'remaining': 10.0, 'fraction': 0.125}

a = math_foundations.ask("Solve 2^x = 32.")
print(a.outcome, a.values)
# SOLVED {'x': 5.0}

a = math_foundations.ask("Factor x^2 + 5x + 6.")
print(a.outcome, a.values)
# SOLVED {'p': 3.0, 'q': 2.0}

a = math_foundations.ask(
    "If f(0) = 1 and f(5) = 11, what is the average rate of change of "
    "f?"
)
print(a.outcome, a.values)
# SOLVED {'arc': 2.0}

a = math_foundations.ask("Solve the inequality 2x + 1 < 7.")
print(a.outcome, a.values)
# SOLVED {'x0': 3.0, 's': 1.0}

a = math_foundations.ask("Solve |x - 4| < 1.")
print(a.outcome, a.values)
# SOLVED {'lower': 3.0, 'upper': 5.0}

a = math_foundations.ask("Solve 3x + 6 = 0.")
print(a.outcome, a.values)
# SOLVED {'x': -2.0}

a = math_foundations.ask(
    "Find the average rate of change between the points (-2, 5) and "
    "(3, 0)."
)
print(a.outcome, a.values)
# SOLVED {'m': -1.0}

a = math_foundations.ask("Solve log base 3 of x = 4.")
print(a.outcome, a.values)
# SOLVED {'x': 81.0}
```

## Screenshots

[![The Math Foundations desktop app: Degrees to radians, with its worked example and live scene](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/hero-dark.webp)](https://www.ai999x.com/models/math-foundations-ai999x/)

[![Evaluate an arithmetic expression (order of operations): the live scene in the Math Foundations desktop app](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/see-1.webp)](https://www.ai999x.com/models/math-foundations-ai999x/)

*Evaluate an arithmetic expression (order of operations)*

[![Absolute value inequality |ax + b| compared with c: the live scene in the Math Foundations desktop app](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/see-2.webp)](https://www.ai999x.com/models/math-foundations-ai999x/)

*Absolute value inequality |ax + b| compared with c*

[![Polar to rectangular coordinates: the live scene in the Math Foundations desktop app](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/see-3.webp)](https://www.ai999x.com/models/math-foundations-ai999x/)

*Polar to rectangular coordinates*

Every calculation, with its formula, a worked example and its figures: **[Math Foundations on ai999x.com](https://www.ai999x.com/models/math-foundations-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/math-foundations-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/math-foundations-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `math-foundations` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/math-foundations`) and its data (`rust/models/math_foundations/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p math-foundations-ai999x
./target/release/math-foundations --help
cargo test -p math-foundations-ai999x
```
