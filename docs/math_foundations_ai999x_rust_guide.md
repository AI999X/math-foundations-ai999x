# math-foundations: the Math Foundations program — user guide

**math-foundations-ai999x 1.1.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Math Foundations covers arithmetic, fractions, percentages, ratios, algebraic equations and inequalities, functions, exponentials and logarithms, trigonometry, precalculus and complex numbers. This guide is about `math-foundations`, the model as a self-contained program for Windows and Linux: 100 capabilities in 31 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/math-foundations-ai999x/); the Python package (`pip install math-foundations-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.4.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Math Foundations | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-math-foundations-1.4.0-windows-x86_64.zip |
| Desktop app: Math Foundations | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-math-foundations-1.4.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz |

### Desktop app

Math Foundations in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-math-foundations-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-math-foundations.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-math-foundations-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-math-foundations-1.4.0-linux-x86_64.tar.gz
./ai999x-math-foundations-1.4.0-linux-x86_64/ai999x-math-foundations
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.4.0-linux-x86_64.tar.gz
./ai999x-1.4.0-linux-x86_64/ai999x
```

### Command line — Linux

The bundle holds one program per model; `math-foundations` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
math-foundations --version
```

### Command line — Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
math-foundations --version
```

### In your browser

Open https://www.ai999x.com/models/math-foundations-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install math-foundations-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ math-foundations --version
math-foundations-ai999x 1.0.3 (Rust build, ax-engine 0.1.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
$ math-foundations ask "Solve x^2 - 5x + 6 = 0."
Read as: Roots of a quadratic equation
x1 = 2
x2 = 3
discriminant = 1
  by: Roots of a quadratic equation  (quadratic formula in a numerically stable form (q = -(b + sign(b)*sqrt(b^2 - 4ac))/2, x = q/a and c/q))
  a = 1
  b = -5
  c = 6
  assumes: a != 0 (otherwise the equation is linear); real coefficients
```

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ math-foundations search "roots of a quadratic equation"
quadratic_roots                      Roots of a quadratic equation  (polynomial equations / quadratic equations)
quartic_roots                        Roots of a quartic equation  (polynomial equations / cubic and quartic equations)
cubic_roots                          Roots of a cubic equation  (polynomial equations / cubic and quartic equations)
quadratic_discriminant               Discriminant of a quadratic  (polynomial equations / quadratic equations)
linear_equation_root                 Root of a linear equation  (linear equations)
quadratic_inequality                 Quadratic inequality  (algebra / inequalities)
factor_quadratic_integer             Factor a quadratic over the integers  (algebra / factoring)
factor_monic_quadratic               Factoring a monic quadratic  (factoring)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ math-foundations list
abs_linear_inequality                    Absolute value inequality |ax + b| compared with c
abs_value_inequality_lower_bound         Solution set of |x - c| > d
abs_value_inequality_upper_bound         Solution interval of |x - c| < d
absolute_value_equation                  Solutions of an absolute value equation
add_rational_expressions_same_denom      Add rational expressions with same denominator
arithmetic_sequence                      Term and partial sum of an arithmetic sequence
average_rate_of_change_definition        Average rate of change of a function between two inputs
average_rate_of_change_linear            Average rate of change between two points
… 92 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
$ math-foundations info quadratic_roots
Roots of a quadratic equation
  quadratic formula in a numerically stable form (q = -(b + sign(b)*sqrt(b^2 - 4ac))/2, x = q/a and c/q)
  in   a        coefficient of x^2 [1]
  in   b        coefficient of x [1]
  in   c        constant term [1]
  out  x1       first root (the smaller real root, or the one with positive imaginary part) [1]
  out  x2       second root [1]
  out  discriminant b^2 - 4ac [1]
```

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
$ math-foundations solve quadratic_roots a=1.0 b=-5.0 c=6.0
x1 = 2
x2 = 3
discriminant = 1
  by: Roots of a quadratic equation  (quadratic formula in a numerically stable form (q = -(b + sign(b)*sqrt(b^2 - 4ac))/2, x = q/a and c/q))
  a = 1
  b = -5
  c = 6
  assumes: a != 0 (otherwise the equation is linear); real coefficients
```

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ math-foundations solve quadratic_roots a=1.0 b=-5.0 c=6.0 --json
{"outcome":"SOLVED","message":"","value":2.0,"unit":"","solved_for":"x1","si":[["a",1.0],["b",-5.0],["c",6.0]],"values":[["x1",2.0,""],["x2",3.0,""],["discriminant",1.0,""]],"target":"x1","phrase":"first root (the smaller real root, or the one with positive imaginary part)","assumptions":["a != 0 (otherwise the equation is linear)","real coefficients"],"name":"Roots of a quadratic equation","equation":"quadratic formula in a numerically stable form (q = -(b + sign(b)*sqrt(b^2 - 4ac))/2, x = q/a and c/q)"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ math-foundations solve quadratic_roots a=1.0 b=-5.0
NEEDS_INPUT: quadratic_roots needs c (constant term). Example: solve('quadratic_roots', a=1, b=-5, c=6).
```

### 2.9 Draw the figures

The live figures for your own inputs, written as SVG files into the folder given with --out (here the current folder); open them in any browser. Without --out the SVG text comes back as JSON.

```shell
$ math-foundations figures quadratic_roots a=1.0 b=-5.0 c=6.0 --out .
[
  {
    "slug": "quadratic-roots-live-1",
    "title": "First root (the smaller real root, or the one... as coefficient of x^2 varies",
    "caption": "First root (the smaller real root, or the one... as a runs from 0.25 to 1.038, every other input as you entered it (22 solutions). Marked: your a = 1 gives x1 = 2. Your inputs: a = 1.0, b = -5.0, c = 6.0.",
    "file": ".\\quadratic-roots-live-1.svg"
  },
  {
    "slug": "quadratic-roots-live-2",
    "title": "First root (the smaller real root, or the one... as coefficient of x varies",
    "caption": "First root (the smaller real root, or the one... as b runs from -8.75 to -5, every other input as you entered it (15 solutions). Marked: your b = -5 gives x1 = 2. Your inputs: a = 1.0, b = -5.0, c = 6.0.",
    "file": ".\\quadratic-roots-live-2.svg"
  }
]
```

### 2.10 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | math-foundations batch

`cases.jsonl`:

```json
{"key": "quadratic_roots", "inputs": {"a": 1.0, "b": -5.0, "c": 6.0}}
{"key": "law_of_sines_ssa", "inputs": {"a": 8.0, "b": 10.0, "A": 40.0}}
```

```shell
$ math-foundations batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":2.0,"unit":"","solved_for":"x1","si":[["a",1.0],["b",-5.0],["c",6.0]],"values":[["x1",2.0,""],["x2",3.0,""],["discriminant",1.0,""]],"target":"x1","phrase":"first root (the smaller real root, or the one with positive imaginary part)","assumptions":["a != 0 (otherwise the equation is linear)","real coefficients"],"name":"Roots of a quadratic equation","equation":"quadratic formula in a numerically stable form (q = -(b + sign(b)*sqrt(b^2 - 4ac))/2, x = q/a and c/q)"}
{"outcome":"SOLVED","message":"","value":2.0,"unit":"","solved_for":"count","si":[["a",8.0],["b",10.0],["A",40.0]],"values":[["count",2.0,""],["B1",53.464149014388475,""],["C1",86.53585098561152,""],["c1",12.42304969336998,""],["B2",126.53585098561152,""],["C2",13.464149014388482,""],["c2",2.897839169009583,""]],"target":"count","phrase":"number of triangles (1 or 2; none is refused)","assumptions":["a, b > 0","0 < A < 180","a triangle exists (else refused)"],"name":"Law of sines, the ambiguous case (two sides and a non-included angle)","equation":"sin B = b sin A / a; B and 180 - B both give a triangle when A + B < 180 (the ambiguous case)"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `math-foundations info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`math-foundations solve ... --json` prints one answer as one line of JSON. `math-foundations batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | math-foundations batch`. `math-foundations catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["math-foundations", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`math-foundations figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

```shell
$ math-foundations figures quadratic_roots a=1.0 b=-5.0 c=6.0 --out .
[
  {
    "slug": "quadratic-roots-live-1",
    "title": "First root (the smaller real root, or the one... as coefficient of x^2 varies",
    "caption": "First root (the smaller real root, or the one... as a runs from 0.25 to 1.038, every other input as you entered it (22 solutions). Marked: your a = 1 gives x1 = 2. Your inputs: a = 1.0, b = -5.0, c = 6.0.",
    "file": ".\\quadratic-roots-live-1.svg"
  },
  {
    "slug": "quadratic-roots-live-2",
    "title": "First root (the smaller real root, or the one... as coefficient of x varies",
    "caption": "First root (the smaller real root, or the one... as b runs from -8.75 to -5, every other input as you entered it (15 solutions). Marked: your b = -5 gives x1 = 2. Your inputs: a = 1.0, b = -5.0, c = 6.0.",
    "file": ".\\quadratic-roots-live-2.svg"
  }
]
```

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `math_foundations_ai999x.wasm` (https://www.ai999x.com/compute/rust/math_foundations_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("math_foundations_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `math-foundations info <id>` shows the same inputs and outputs in the terminal.

### Algebra (11)

<a id="abs_linear_inequality"></a>

#### Absolute value inequality |ax + b| compared with c — `abs_linear_inequality`

Solve |a*x + b| < c, <= c, > c or >= c (c > 0): the two boundary points and whether x lies between or outside them.

$$
|ax+b| < c \iff -c < ax+b < c,\qquad |ax+b| > c \iff ax+b < -c \ \text{or}\ ax+b > c
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x inside the bars | — |
| `b` | in | constant inside the bars | — |
| `c` | in | the bound (positive) | — |
| `rel` | in | the comparison | — |
| `lower` | out | the smaller boundary point | — |
| `upper` | out | the larger boundary point | — |
| `region` | out | between (for < and <=) or outside (for > and >=) | — |
| `solution` | out | the solution set | — |

**Applies when:** a != 0; c > 0.

```shell
$ math-foundations solve abs_linear_inequality a=2 b=3 c=7 "rel=<"
lower = -5
upper = 2
region = between
solution = -5 < x < 2
  by: Absolute value inequality |ax + b| compared with c  (|u| < c means -c < u < c; |u| > c means u < -c or u > c; solve each linear piece for x)
  a = 2
  b = 3
  c = 7
  rel = <
  assumes: a != 0; c > 0
```

In the worked example, a = 2, b = 3, c = 7, rel = < go through the equation above and give lower = -5, upper = 2, region = between, solution = -5 < x < 2. It applies only when a != 0; c > 0: outside that it says what is wrong instead of returning a number.

<a id="factor_quadratic_integer"></a>

#### Factor a quadratic over the integers — `factor_quadratic_integer`

Factor a*x^2 + b*x + c with whole-number coefficients as g(px + q)(rx + s), or say that it does not factor over the integers.

$$
ax^2+bx+c = g\,(px+q)(rx+s)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x^2 (not 0) | — |
| `b` | in | coefficient of x | — |
| `c` | in | constant term | — |
| `g` | out | common factor (sign of a) | — |
| `p` | out | x-coefficient of the first factor | — |
| `q` | out | constant of the first factor | — |
| `r` | out | x-coefficient of the second factor | — |
| `s` | out | constant of the second factor | — |
| `factored` | out | the factored form | — |

**Applies when:** whole-number coefficients; a != 0; the discriminant is a perfect square.

```shell
$ math-foundations solve factor_quadratic_integer a=6 b=5 c=-6
g = 1
p = 2
q = 3
r = 3
s = -2
factored = (2x + 3)(3x - 2)
  by: Factor a quadratic over the integers  (take out the greatest common factor; the rational roots -q/p and -s/r of the primitive quadratic give its factors (Gauss's lemma))
  assumes: whole-number coefficients; a != 0; the discriminant is a perfect square
```

**What the worked example means:** This calculation determines if the quadratic expression defined by input coefficients 6, 5, and -6 can be written as a product of two linear binomials with integer coefficients. The process first extracts any greatest common factor, resulting in the constant 1. It then identifies the remaining primitive quadratic structure to find the x-coefficients 2 and 3, and the constant terms 3 and -2. If these values are integers, the final factored form is presented as (2x + 3)(3x - 2); otherwise, the system indicates that no such integer factorization exists.

<a id="partial_fractions_linear"></a>

#### Partial fractions (two distinct linear factors) — `partial_fractions_linear`

Write (p*x + q)/((x - r1)(x - r2)) as A/(x - r1) + B/(x - r2).

$$
\frac{px+q}{(x-r_1)(x-r_2)} = \frac{A}{x-r_1} + \frac{B}{x-r_2},\quad A = \frac{pr_1+q}{r_1-r_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | coefficient of x in the numerator | — |
| `q` | in | constant of the numerator | — |
| `r1` | in | root of the first factor x - r1 | — |
| `r2` | in | root of the second factor x - r2 (r2 != r1) | — |
| `A` | out | numerator over x - r1 | — |
| `B` | out | numerator over x - r2 | — |

**Applies when:** r1 != r2.

```shell
$ math-foundations solve partial_fractions_linear p=5 q=1 r1=1 r2=-2
A = 2
B = 3
  by: Partial fractions (two distinct linear factors)  (cover-up rule: A = (p r1 + q)/(r1 - r2), B = (p r2 + q)/(r2 - r1))
  p = 5
  q = 1
  r1 = 1
  r2 = -2
  assumes: r1 != r2
```

**What the worked example means:** This calculation decomposes the rational expression with numerator coefficient 5 and constant 1, divided by factors rooted at 1 and -2, into two simpler fractions. The resulting numerators are computed as 2 for the first term and 3 for the second term, allowing the original complex fraction to be expressed as a sum of these basic components.

<a id="polynomial_long_division"></a>

#### Polynomial long division — `polynomial_long_division`

Divide one polynomial by another: quotient and remainder, with p = q*d + r and deg r < deg d.

$$
p(x) = q(x)\,d(x) + r(x),\quad \deg r < \deg d
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dividend` | in | coefficients of the dividend, highest power first | — |
| `divisor` | in | coefficients of the divisor, highest power first (degree >= 1) | — |
| `quotient` | out | coefficients of the quotient, highest power first | — |
| `remainder` | out | coefficients of the remainder, highest power first | — |

**Applies when:** divisor of degree >= 1.

```shell
$ math-foundations solve polynomial_long_division "dividend=[2, 3, -1, 5]" "divisor=[1, 0, 1]"
quotient = [2.0,3.0]
remainder = [-3.0,2.0]
  by: Polynomial long division  (long division: divide leading terms, multiply back, subtract, repeat; exact rational arithmetic)
  assumes: divisor of degree >= 1
```

**What the worked example means:** This calculation performs polynomial long division on the input polynomials. The dividend, represented by [2, 3, -1, 5], is divided by the divisor, represented by [1, 0, 1]. The result consists of two parts: the quotient [2, 3], which indicates how many times the divisor fits into the dividend, and the remainder [-3, 2], which is what is left over after subtraction. Together they satisfy the fundamental relationship where the original dividend equals the product of the divisor and quotient plus the remainder.

<a id="quadratic_inequality"></a>

#### Quadratic inequality — `quadratic_inequality`

Solve a*x^2 + b*x + c < 0 (or <=, >, >=) when the quadratic has two distinct real roots.

$$
a(x - r_1)(x - r_2) < 0 \iff r_1 < x < r_2 \ \ (a > 0)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x^2 | — |
| `b` | in | coefficient of x | — |
| `c` | in | constant term | — |
| `rel` | in | the comparison with 0 | — |
| `r1` | out | the smaller root | — |
| `r2` | out | the larger root | — |
| `region` | out | between or outside the roots | — |
| `solution` | out | the solution set | — |

**Applies when:** a != 0; b^2 - 4ac > 0.

```shell
$ math-foundations solve quadratic_inequality a=1 b=-5 c=6 "rel=<"
r1 = 2
r2 = 3
region = between
solution = 2 < x < 3
  by: Quadratic inequality  (the roots split the line into three intervals; the sign of a decides whether the quadratic is negative between or outside them)
  a = 1
  b = -5
  c = 6
  rel = <
  assumes: a != 0; b^2 - 4ac > 0
```

**What the worked example means:** This calculation solves the quadratic inequality defined by coefficients 1, -5, and constant 6 with comparison <. It first identifies the two distinct real roots, labeled as the smaller root 2 and the larger root 3. The sign of the leading coefficient 1 dictates whether the expression is negative between these roots or outside them. Consequently, the final region where the inequality holds true is described as between, yielding the complete solution set 2 < x < 3.

<a id="radical_equation"></a>

#### Radical equation sqrt(ax + b) = cx + d — `radical_equation`

Solve sqrt(a*x + b) = c*x + d: square, solve, and keep only the roots that satisfy the original equation.

$$
\sqrt{ax+b} = cx+d \;\Rightarrow\; ax+b = (cx+d)^2,\ cx+d \ge 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x under the root | — |
| `b` | in | constant under the root | — |
| `c` | in | coefficient of x on the right | — |
| `d` | in | constant on the right | — |
| `count` | out | number of solutions (0, 1 or 2) | — |
| `solutions` | out | the solutions, ascending | — |
| `extraneous` | out | roots of the squared equation that fail the original (extraneous) | — |

**Applies when:** a != 0; real numbers.

```shell
$ math-foundations solve radical_equation a=2 b=3 c=1 d=0
count = 1
solutions = [3.0]
extraneous = [-1.0]
  by: Radical equation sqrt(ax + b) = cx + d  (square both sides (a x + b = (c x + d)^2), solve the quadratic exactly, keep a root only if c x + d >= 0)
  a = 2
  b = 3
  c = 1
  d = 0
  assumes: a != 0; real numbers
```

**What the worked example means:** This calculation solves the radical equation by squaring both sides to form a quadratic, finding all algebraic roots, and then filtering them against the condition that the right side must be non-negative. The result indicates there are 1 valid solutions, specifically [3]. Any roots from the squared equation that do not satisfy the original constraint are listed as extraneous in [-1].

<a id="rational_equation"></a>

#### Rational equation a/(x + b) = c/(x + d) — `rational_equation`

Solve a/(x + b) = c/(x + d) for x, rejecting a candidate that makes a denominator 0.

$$
\frac{a}{x+b} = \frac{c}{x+d} \;\Rightarrow\; x = \frac{cb - ad}{a - c}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | numerator on the left | — |
| `b` | in | constant in the left denominator x + b | — |
| `c` | in | numerator on the right | — |
| `d` | in | constant in the right denominator x + d | — |
| `x` | out | the solution | — |

**Applies when:** a != c; the candidate is not an excluded value.

```shell
$ math-foundations solve rational_equation a=3 b=1 c=2 d=-4
x = 14
  by: Rational equation a/(x + b) = c/(x + d)  (multiply by both denominators: a(x + d) = c(x + b), solve the linear equation, check the excluded values x = -b, x = -d)
  a = 3
  b = 1
  c = 2
  d = -4
  assumes: a != c; the candidate is not an excluded value
```

**What the worked example means:** This calculation solves the rational equation by cross-multiplying to form a linear equation. The result, 14, represents the specific value of x that balances the two fractions. This outcome assumes that the coefficients a and c are distinct and that the calculated x does not equal negative b or negative d, which would invalidate the original denominators.

<a id="rational_roots"></a>

#### Rational roots of a polynomial (rational root theorem) — `rational_roots`

Every rational root of a polynomial with whole-number coefficients (degree up to 8).

$$
a_n x^n + \dots + a_0 = 0,\ x = \tfrac{p}{q} \Rightarrow p \mid a_0,\ q \mid a_n
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `coefficients` | in | whole-number coefficients, highest power first | — |
| `count` | out | number of distinct rational roots | — |
| `roots` | out | the rational roots, ascending | — |
| `roots_exact` | out | the rational roots as fractions | — |

**Applies when:** whole-number coefficients; degree 1 to 8.

```shell
$ math-foundations solve rational_roots "coefficients=[2, -3, -11, 6]"
count = 3
roots = [-2.0,0.5,3.0]
roots_exact = ["-2","1/2","3"]
  by: Rational roots of a polynomial (rational root theorem)  (rational root theorem: a root p/q in lowest terms has p dividing the constant term and q dividing the leading coefficient; test every candidate exactly)
  assumes: whole-number coefficients; degree 1 to 8
```

**What the worked example means:** This calculation finds every rational root of a polynomial with whole-number coefficients. It lists the number of distinct roots found, which is 3, and displays them in ascending order as [-2, 0.5, 3]. The exact fractional forms are also provided as [-2, 1/2, 3]. This result represents all possible values where the polynomial equals zero that can be expressed as a simple fraction.

<a id="infinite_geometric_series"></a>

#### Sum of an infinite geometric series — `infinite_geometric_series`

a + a*r + a*r^2 + ... = a/(1 - r) for |r| < 1 (a refusal when the series diverges).

$$
\sum_{k=0}^{\infty} a r^k = \frac{a}{1-r},\quad |r| < 1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | the first term | — |
| `r` | in | the common ratio, \|r\| < 1 | — |
| `sum` | out | the sum to infinity | — |

**Applies when:** |r| < 1.

```shell
$ math-foundations solve infinite_geometric_series a=6 r=0.3333333333
sum = 9
  by: Sum of an infinite geometric series  (S_n = a(1 - r^n)/(1 - r) and r^n -> 0 when |r| < 1)
  a = 6
  r = 0.3333333333
  assumes: |r| < 1
```

**What the worked example means:** This calculation determines the total accumulation of an infinite geometric sequence. It uses the first term 6 and the common ratio 0.333333 to find the limit of the partial sums. The result, 9, represents the finite value that the series approaches as more terms are added, provided the absolute value of the ratio is less than one.

<a id="synthetic_division"></a>

#### Synthetic division and the remainder theorem — `synthetic_division`

Divide a polynomial by x - r: the quotient's coefficients and the remainder, which equals p(r).

$$
p(x) = (x - r)\,q(x) + p(r)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `coefficients` | in | coefficients of the dividend, highest power first (2x^3 - 5x + 1 -> 2, 0, -5, 1) | — |
| `r` | in | the number r of the divisor x - r | — |
| `quotient` | out | coefficients of the quotient, highest power first | — |
| `remainder` | out | the remainder = p(r) | — |

**Applies when:** dividend of degree >= 1.

```shell
$ math-foundations solve synthetic_division "coefficients=[1, -6, 11, -6]" r=1
quotient = [1.0,-5.0,6.0]
remainder = 0
  by: Synthetic division and the remainder theorem  (synthetic division (Horner's scheme): bring down, multiply by r, add; the last number is the remainder p(r))
  r = 1
  assumes: dividend of degree >= 1
```

In the worked example, coefficients = [1, -6, 11, -6], r = 1 go through the equation above and give quotient = [1, -5, 6], remainder = 0. It applies only when dividend of degree >= 1: outside that it says what is wrong instead of returning a number.

<a id="linear_system_3x3"></a>

#### System of three linear equations — `linear_system_3x3`

Solve three linear equations in x, y and z (a unique solution, or a refusal when there is none).

$$
A\,\mathbf{x} = \mathbf{d},\quad \mathbf{x} = A^{-1}\mathbf{d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `augmented` | in | augmented matrix, one row a b c d per equation a*x + b*y + c*z = d | — |
| `x` | out | x | — |
| `y` | out | y | — |
| `z` | out | z | — |

**Applies when:** determinant of the coefficients != 0 (a unique solution).

```shell
$ math-foundations solve linear_system_3x3 "augmented=[[1, 1, 1, 6], [2, -1, 1, 3], [1, 2, -1, 2]]"
x = 1
y = 2
z = 3
  by: System of three linear equations  (Gaussian elimination in exact rational arithmetic)
  assumes: determinant of the coefficients != 0 (a unique solution)
```

**What the worked example means:** This calculation determines the values of three unknowns by solving a system of linear equations using Gaussian elimination. The process begins with an augmented matrix containing coefficients and constants, represented here as [1 1 1 6; 2 -1 1 3; 1 2 -1 2]. By transforming this matrix into row-echelon form and performing back-substitution, the method isolates each variable. The resulting values are 1 for the first variable, 2 for the second, and 3 for the third. These outputs represent the single point where all three planes intersect in three-dimensional space.

### Arithmetic (12)

<a id="fraction_arithmetic"></a>

#### Arithmetic with fractions — `fraction_arithmetic`

Add, subtract, multiply or divide two fractions a/b and c/d; the result in lowest terms.

$$
\frac{a}{b} \pm \frac{c}{d} = \frac{ad \pm cb}{bd},\quad \frac{a}{b}\cdot\frac{c}{d} = \frac{ac}{bd},\quad \frac{a}{b} \div \frac{c}{d} = \frac{ad}{bc}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | numerator of the first fraction | — |
| `b` | in | denominator of the first fraction (not 0) | — |
| `c` | in | numerator of the second fraction | — |
| `d` | in | denominator of the second fraction (not 0) | — |
| `op` | in | the operation | — |
| `numerator` | out | numerator of the result in lowest terms | — |
| `denominator` | out | denominator of the result (positive) | — |
| `value` | out | the result as a decimal | — |

**Applies when:** whole-number numerators and denominators; denominators not 0; no division by 0.

```shell
$ math-foundations solve fraction_arithmetic a=3 b=4 c=5 d=6 op=+
numerator = 19
denominator = 12
value = 1.583333333
  by: Arithmetic with fractions  (common denominator (a/b ± c/d = (ad ± cb)/bd), product of numerators over product of denominators, division as multiplication by the reciprocal; then divide by the greatest common divisor)
  op = +
  assumes: whole-number numerators and denominators; denominators not 0; no division by 0
```

**What the worked example means:** To compute the result, the rule for + is applied to the two fractions. For addition or subtraction, a common denominator is found by multiplying the denominators together and adjusting the numerators accordingly. For multiplication, the top numbers and the bottom numbers are multiplied separately. For division, the second fraction is flipped and multiplied. After performing the arithmetic operation to get an intermediate fraction, it is simplified by dividing both the new numerator and denominator by their greatest common divisor. This yields the final answer in lowest terms as 19 over 12, with a decimal value of 1.58333.

<a id="ratio_split"></a>

#### Divide a quantity in a given ratio — `ratio_split`

Share a total in the ratio a:b (or a:b:c).

$$
\mathrm{part}_i = \mathrm{total}\cdot\frac{r_i}{\sum_j r_j}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `total` | in | the quantity to share | — |
| `a` | in | first term of the ratio (positive) | — |
| `b` | in | second term of the ratio (positive) | — |
| `c` | in | third term of the ratio (optional, positive) (optional) | — |
| `part1` | out | first share | — |
| `part2` | out | second share | — |
| `part3` | out | third share (a three-term ratio only) | — |

**Applies when:** every term of the ratio positive.

```shell
$ math-foundations solve ratio_split total=120 a=3 b=5
part1 = 45
part2 = 75
  by: Divide a quantity in a given ratio  (each share = total · term / (sum of the terms))
  total = 120
  a = 3
  b = 5
  assumes: every term of the ratio positive
```

**What the worked example means:** This calculation distributes the given total amount, 120, into proportional parts based on the specified ratio terms. The size of each individual share is determined by multiplying the total by that share's specific ratio term and then dividing by the sum of all ratio terms. For instance, the first part equals 45 because it represents the portion of the whole corresponding to the first ratio term relative to the combined weight of all terms.

<a id="order_of_operations"></a>

#### Evaluate an arithmetic expression (order of operations) — `order_of_operations`

The exact value of an arithmetic expression with + - * / ^ and parentheses, following the order of operations.

$$
\text{(  )} \;\to\; a^{b} \;\to\; \times\,\div \;\to\; +\,-
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `expression` | in | the expression, e.g. 3 + 4 * (2 - 5)^2 / 6 | — |
| `value` | out | the value | — |
| `numerator` | out | numerator of the exact value in lowest terms | — |
| `denominator` | out | denominator of the exact value | — |

**Applies when:** numbers, + - * / ^ and parentheses only; no division by 0; integer exponents up to 64.

```shell
$ math-foundations solve order_of_operations "expression=3 + 4 * (2 - 5)^2 / 6"
value = 9
numerator = 9
denominator = 1
  by: Evaluate an arithmetic expression (order of operations)  (parentheses first, then powers (right to left), then multiplication and division, then addition and subtraction (left to right); exact rational arithmetic)
  expression = 3 + 4 * (2 - 5)^2 / 6
  assumes: numbers, + - * / ^ and parentheses only; no division by 0; integer exponents up to 64
```

**What the worked example means:** This capability computes the precise value of an arithmetic expression by strictly following the standard order of operations. Starting with parentheses, then powers, multiplication and division, and finally addition and subtraction from left to right, it processes the input 3 + 4 * (2 - 5)^2 / 6 using exact rational arithmetic. The result is presented as the simplified fraction 9 over 1, ensuring no rounding errors occur during calculation.

<a id="percent_change"></a>

#### Percent change (increase or decrease) — `percent_change`

The percent change from an original value to a new value (negative for a decrease).

$$
\Delta\% = \frac{\mathrm{new}-\mathrm{old}}{\mathrm{old}}\cdot 100
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `old` | in | the original value (positive) | — |
| `new` | in | the new value | — |
| `percent_change` | out | percent change (negative = decrease) | — |
| `difference` | out | new - old | — |
| `direction` | out | increase, decrease or no change | — |

**Applies when:** old > 0.

```shell
$ math-foundations solve percent_change old=80 new=100
percent_change = 25
difference = 20
direction = increase
  by: Percent change (increase or decrease)  (percent change = (new - old)/old · 100)
  old = 80
  new = 100
  assumes: old > 0
```

**What the worked example means:** This calculation determines the relative shift between two values by dividing the difference, which is 20, by the original positive value and multiplying by one hundred. The resulting figure of 25 indicates whether the new value represents an increase or decrease compared to the starting point, providing a standardized measure of growth or decline that accounts for the magnitude of the initial amount.

<a id="percent_of_number"></a>

#### Percent of a number — `percent_of_number`

p% of x: the part that p per cent of x amounts to.

$$
\frac{p}{100}\,x
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | the percentage p (per hundred) | — |
| `x` | in | the whole x | — |
| `result` | out | p% of x | — |

**Applies when:** finite numbers.

```shell
$ math-foundations solve percent_of_number p=15 x=80
result = 12
  by: Percent of a number  (per cent means per hundred: p% of x = (p/100)·x)
  p = 15
  x = 80
  assumes: finite numbers
```

**What the worked example means:** This calculation determines a specific portion of a quantity by converting the percentage into a decimal fraction and multiplying it by the total amount. The input 15 indicates the rate per hundred, while 80 serves as the base value or whole. By applying the formula for percent of a number, the computed output 12 reveals exactly how much of the original quantity corresponds to that specified percentage.

<a id="round_decimal_places"></a>

#### Round to decimal places — `round_decimal_places`

Round a number to n decimal places (or to the nearest tenth, hundredth, ten, hundred ...), halves away from zero.

$$
\mathrm{round}(x, n) = \operatorname{sgn}(x)\,\frac{\lfloor |x|\,10^{n} + \tfrac12 \rfloor}{10^{n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the number to round | — |
| `n` | in | decimal places (negative: tens, hundreds) | — |
| `rounded` | out | the rounded number | — |

**Applies when:** -9 <= n <= 12.

```shell
$ math-foundations solve round_decimal_places x=3.14159 n=2
rounded = 3.14
  by: Round to decimal places  (exact decimal arithmetic on the number as written; a 5 in the first dropped place rounds away from zero)
  x = 3.14159
  assumes: -9 <= n <= 12
```

**What the worked example means:** This calculation applies exact decimal arithmetic to the input value 3.14159, rounding it to the nearest position specified by 2. The method ensures that if the first dropped digit is a five, the result moves away from zero. For negative values of 2, the rounding occurs at tens or hundreds rather than decimal fractions. The final output 3.14 represents the number adjusted to this specific precision level.

<a id="round_significant_figures"></a>

#### Round to significant figures — `round_significant_figures`

Round a nonzero number to n significant figures, halves away from zero.

$$
\mathrm{round}\big(x,\ n - 1 - \lfloor \log_{10}|x| \rfloor\big)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the number to round (not 0) | — |
| `n` | in | number of significant figures | — |
| `rounded` | out | the rounded number | — |

**Applies when:** x != 0; 1 <= n <= 15.

```shell
$ math-foundations solve round_significant_figures x=0.0045678 n=3
rounded = 0.00457
  by: Round to significant figures  (find the exponent e of the leading digit (10^e <= |x| < 10^(e+1)), round to 10^(e-n+1) exactly)
  x = 0.0045678
  assumes: x != 0; 1 <= n <= 15
```

**What the worked example means:** This calculation takes the number 0.0045678 and rounds it to 3 significant figures. The result, 0.00457, preserves the relative precision of the original value by adjusting its magnitude rather than just its decimal places. This ensures that the count of meaningful digits remains constant regardless of how large or small the input is.

<a id="scientific_notation"></a>

#### Scientific notation — `scientific_notation`

Write a nonzero number as m × 10^e with 1 <= |m| < 10.

$$
x = m \times 10^{e},\quad 1 \le |m| < 10
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the number (not 0) | — |
| `mantissa` | out | m, with 1 <= \|m\| < 10 | — |
| `exponent` | out | the integer power of ten e | — |

**Applies when:** x != 0.

```shell
$ math-foundations solve scientific_notation x=45600
mantissa = 4.56
exponent = 4
  by: Scientific notation  (exact decimal: e = position of the leading digit, m = x / 10^e)
  x = 45600
  assumes: x != 0
```

**What the worked example means:** This calculation converts a nonzero number into scientific notation by separating it into a mantissa and an exponent. The input value 45600 is expressed as the product of the mantissa 4.56 and ten raised to the integer power 4. By definition, the mantissa must have an absolute value greater than or equal to one and less than ten, which ensures a standardized format for comparing magnitudes.

<a id="proportion_solve"></a>

#### Solve a proportion — `proportion_solve`

Solve n1/d1 = n2/d2 for the one unknown term x (in any of the four positions).

$$
\frac{n_1}{d_1} = \frac{n_2}{d_2} \iff n_1 d_2 = n_2 d_1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | the first known term (reading n1, d1, n2, d2 in order) | — |
| `q` | in | the second known term | — |
| `r` | in | the third known term | — |
| `position` | in | where the unknown x stands | — |
| `x` | out | the unknown term | — |

**Applies when:** denominators not 0; the proportion has a solution.

```shell
$ math-foundations solve proportion_solve p=3 q=4 r=20 "position=second numerator"
x = 15
  by: Solve a proportion  (cross-multiplication: n1·d2 = n2·d1, then divide)
  p = 3
  q = 4
  r = 20
  position = second numerator
  assumes: denominators not 0; the proportion has a solution
```

**What the worked example means:** This calculation determines the missing value in a proportion by setting the product of the means equal to the product of the extremes. Given the first three terms p, q, and r, and the position of the unknown x, the method solves the equation where two ratios are equivalent. The resulting value for x is 15. This approach ensures that the relationship between the quantities remains consistent across both sides of the ratio.

<a id="unit_rate_compare"></a>

#### Unit rates and the better buy — `unit_rate_compare`

The unit rate (amount per one unit) of an offer, and of a second offer with which is cheaper per unit.

$$
\text{unit rate} = \frac{\text{amount}}{\text{quantity}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n1` | in | quantity of the first offer (positive) | — |
| `c1` | in | amount (price, distance, ...) of the first offer | — |
| `n2` | in | quantity of the second offer (optional) (optional) | — |
| `c2` | in | amount of the second offer (optional) (optional) | — |
| `unit_rate_1` | out | amount per unit, first offer | — |
| `unit_rate_2` | out | amount per unit, second offer | — |
| `better` | out | the offer with the lower amount per unit: first, second or equal | — |

**Applies when:** quantities positive; amounts not negative.

```shell
$ math-foundations solve unit_rate_compare n1=12 c1=30 n2=20 c2=45
unit_rate_1 = 2.5
unit_rate_2 = 2.25
better = second
  by: Unit rates and the better buy  (unit rate = amount / quantity; the lower unit price is the better buy)
  n1 = 12
  c1 = 30
  n2 = 20
  c2 = 45
  assumes: quantities positive; amounts not negative
```

**What the worked example means:** This calculation determines the cost or quantity per single unit for each offer by dividing the total amount by the total quantity. For the first offer, the unit rate is computed as 2.5. If a second offer is provided, its unit rate is calculated as 2.25. Comparing these values reveals which option offers more value per unit, with the lower rate indicating the better buy, identified here as the second offer.

<a id="what_percent"></a>

#### What percent one number is of another — `what_percent`

The percentage p such that `part` is p% of `whole`.

$$
p = 100\,\frac{\mathrm{part}}{\mathrm{whole}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `part` | in | the part | — |
| `whole` | in | the whole (not 0) | — |
| `percent` | out | part as a percentage of whole | — |

**Applies when:** whole != 0.

```shell
$ math-foundations solve what_percent part=12 whole=80
percent = 15
  by: What percent one number is of another  (p = 100·part/whole)
  part = 12
  whole = 80
  assumes: whole != 0
```

**What the worked example means:** This calculation determines what portion of a total quantity is represented by a specific part, expressed as a percentage. Dividing the part by the whole and multiplying by one hundred shows that 12 constitutes 15 of 80. This standardized scale allows for easy comparison of relative sizes regardless of the absolute magnitude of the numbers involved.

<a id="percent_of_what"></a>

#### Whole from a part and its percentage — `percent_of_what`

The whole w of which `part` is p per cent (part is p% of what number?).

$$
w = \frac{100\,\mathrm{part}}{p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `part` | in | the known part | — |
| `p` | in | the percentage that the part is of the whole | — |
| `whole` | out | the whole | — |

**Applies when:** p != 0.

```shell
$ math-foundations solve percent_of_what part=18 p=15
whole = 120
  by: Whole from a part and its percentage  (part = (p/100)·whole, so whole = 100·part/p)
  part = 18
  p = 15
  assumes: p != 0
```

**What the worked example means:** This calculation finds the total quantity, known as the whole, given a specific portion and the percentage that portion represents. Applying the relationship where the part equals the percentage times the whole divided by one hundred reverses the operation to solve for the whole. The result, 120, indicates that if 18 is indeed 15 percent of the total, then the complete amount must be this value.

### Complex numbers (5)

<a id="complex_arithmetic"></a>

#### Arithmetic with complex numbers — `complex_arithmetic`

(a + bi) + - * / (c + di) in the form re + im*i.

$$
\frac{a+bi}{c+di} = \frac{(ac+bd) + (bc-ad)i}{c^2+d^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | real part of the first number | — |
| `b` | in | imaginary part of the first number | — |
| `c` | in | real part of the second number | — |
| `d` | in | imaginary part of the second number | — |
| `op` | in | the operation | — |
| `re` | out | real part of the result | — |
| `im` | out | imaginary part of the result | — |

**Applies when:** no division by 0.

```shell
$ math-foundations solve complex_arithmetic a=3 b=2 c=1 d=-1 op=/
re = 0.5
im = 2.5
  by: Arithmetic with complex numbers  (i^2 = -1; division multiplies numerator and denominator by the conjugate c - di)
  a = 3
  b = 2
  c = 1
  d = -1
  op = /
  assumes: no division by 0
```

**What the worked example means:** This calculation determines the real and imaginary components of a complex number resulting from an arithmetic operation. By applying the rule that i squared equals negative one, the system processes the input parts to yield a final value expressed as 0.5 for the real component and 2.5 for the imaginary component.

<a id="complex_modulus_argument"></a>

#### Modulus, argument and polar form of a complex number — `complex_modulus_argument`

|z| and the principal argument (-180, 180] degrees of z = a + bi, and z in polar form.

$$
z = r(\cos\theta + i\sin\theta),\quad r = \sqrt{a^2+b^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | real part | — |
| `b` | in | imaginary part | — |
| `modulus` | out | \|z\| | — |
| `argument_deg` | out | principal argument, degrees in (-180, 180] | — |
| `argument_rad` | out | principal argument, radians | — |
| `polar` | out | z = r(cos t + i sin t) | — |

**Applies when:** z != 0.

```shell
$ math-foundations solve complex_modulus_argument a=-1 b=1
modulus = 1.414213562
argument_deg = 135
argument_rad = 2.35619449
polar = 1.414213562(cos(135 deg) + i sin(135 deg))
  by: Modulus, argument and polar form of a complex number  (|z| = sqrt(a^2 + b^2); the argument in the quadrant of (a, b))
  a = -1
  b = 1
  assumes: z != 0
```

**What the worked example means:** This calculation determines the polar representation of a complex number defined by its real part -1 and imaginary part 1. The modulus, representing the distance from the origin, is computed as 1.41421. The principal argument, indicating the angle relative to the positive real axis within the range of negative one hundred eighty to one hundred eighty degrees, is found to be 135 degrees or 2.35619 radians. Consequently, the complex number can be expressed in polar form as 1.414213562(cos(135 deg) + i sin(135 deg)), which provides geometric insight into its position on the complex plane.

<a id="de_moivre_power"></a>

#### Power of a complex number (De Moivre) — `de_moivre_power`

(a + bi)^n for a whole number n (|n| <= 40): real and imaginary parts, modulus and argument.

$$
\left[r(\cos\theta + i\sin\theta)\right]^n = r^n(\cos n\theta + i\sin n\theta)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | real part | — |
| `b` | in | imaginary part | — |
| `n` | in | the power n | — |
| `re` | out | real part of z^n | — |
| `im` | out | imaginary part of z^n | — |
| `modulus` | out | \|z^n\| = \|z\|^n | — |
| `argument_deg` | out | argument of z^n, degrees | — |

**Applies when:** |n| <= 40; z != 0 when n <= 0.

```shell
$ math-foundations solve de_moivre_power a=1 b=1 n=8
re = 16
im = 0
modulus = 16
argument_deg = 0
  by: Power of a complex number (De Moivre)  (De Moivre: z^n = r^n (cos n t + i sin n t); computed exactly (binary exponentiation in rational arithmetic))
  a = 1
  b = 1
  assumes: |n| <= 40; z != 0 when n <= 0
```

**What the worked example means:** De Moivre's theorem is used to calculate the power of a complex number by converting it into polar form. Given an input with real part 1 and imaginary part 1, the modulus becomes 16 and the argument becomes 0 degrees after raising to the power 8. The final result is expressed as a complex number with real part 16 and imaginary part 0. This method ensures precision for integer exponents up to forty.

<a id="complex_product_modulus"></a>

#### Product and modulus of complex numbers — `complex_product_modulus`

The product (a + b*i)(c + d*i) = re + im*i and the modulus of the product.

$$
\operatorname{Re} = a c - b d,\quad \operatorname{Im} = a d + b c,\quad \text{modulus}^{2} = \left(a^{2} + b^{2}\right) \left(c^{2} + d^{2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | real part of the first number | — |
| `b` | in | imaginary part of the first number | — |
| `c` | in | real part of the second number | — |
| `d` | in | imaginary part of the second number | — |
| `re` | out | real part of the product | — |
| `im` | out | imaginary part of the product | — |
| `modulus` | out | modulus of the product | — |

```shell
$ math-foundations solve complex_product_modulus a=1 b=2 c=3 d=4
re = -5
im = 10
modulus = 11.18033989
  by: Product and modulus of complex numbers  (distribute and use i^2 = -1; the modulus of a product is the product of the moduli)
  a = 1
  b = 2
  c = 3
  d = 4
```

**What the worked example means:** This calculation determines the product of two complex numbers defined by their real and imaginary components. Distributing the terms and applying the rule that the square of the imaginary unit equals negative one shows that the resulting real part is -5 and the imaginary part is 10. The modulus of this product is calculated as 11.1803, which represents the distance from the origin to the point representing the result in the complex plane.

<a id="complex_nth_roots"></a>

#### The n-th roots of a complex number — `complex_nth_roots`

All n complex n-th roots of z = a + bi (n from 2 to 12), ordered by argument from 0 to 360 degrees.

$$
w_k = \sqrt[n]{r}\left(\cos\frac{\theta + 2\pi k}{n} + i\sin\frac{\theta + 2\pi k}{n}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | real part of z | — |
| `b` | in | imaginary part of z | — |
| `n` | in | which roots (2 = square roots) | — |
| `modulus` | out | the common modulus \|z\|^(1/n) | — |
| `count` | out | the number of roots, n | — |
| `roots` | out | the roots, by argument in [0, 360) | — |

**Applies when:** z != 0; 2 <= n <= 12.

```shell
$ math-foundations solve complex_nth_roots a=0 b=8 n=3
modulus = 2
count = 3
roots = [{"re":1.73205080757,"im":1.0},{"re":-1.73205080757,"im":1.0},{"re":0.0,"im":-2.0}]
  by: The n-th roots of a complex number  (w_k = |z|^(1/n) (cos((t + 2 pi k)/n) + i sin((t + 2 pi k)/n)), k = 0 .. n-1)
  a = 0
  b = 8
  assumes: z != 0; 2 <= n <= 12
```

**What the worked example means:** This calculation finds every complex root of a given number by determining its magnitude and angle. For an input with real part 0 and imaginary part 8, the result shows that there are exactly 3 distinct roots. Each of these roots lies on a circle centered at the origin with a radius equal to 2. The roots are evenly spaced around this circle, starting from the smallest positive argument and proceeding in steps of one full rotation divided by the count.

### Conic sections (1)

<a id="circle_from_general_form"></a>

#### Centre and radius of a circle from its general equation — `circle_from_general_form`

For x^2 + y^2 + D*x + E*y + F = 0, the centre (h, k) and radius rad.

$$
\left(t - h\right)^{2} + \left(w - k\right)^{2} - \text{rad}^{2} = t^{2} + w^{2} + d_{d} t + e_{e} w + f_{f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dd` | in | coefficient D of x | — |
| `ee` | in | coefficient E of y | — |
| `ff` | in | constant F | — |
| `h` | out | x-coordinate of the centre | — |
| `k` | out | y-coordinate of the centre | — |
| `rad` | out | radius | — |

**Applies when:** dd**2/4 + ee**2/4 - ff > 0.

```shell
$ math-foundations solve circle_from_general_form dd=-4 ee=6 ff=-3
h = 2
k = -3
rad = 4
  by: Centre and radius of a circle from its general equation  (complete the square in x and in y)
  dd = -4
  ee = 6
  ff = -3
  assumes: dd**2/4 + ee**2/4 - ff > 0
```

**What the worked example means:** Completing the square for the general equation shows that the center is located at 2 and the radius is 4. This geometric interpretation relies on the input coefficients where D is -4, E is 6, and F is -3. The result confirms a valid real circle exists only if the derived radius squared is positive.

### Coordinate plane (1)

<a id="distance_between_points"></a>

#### Distance between two points — `distance_between_points`

The distance between (x1, y1) and (x2, y2) in the plane.

$$
\text{dist}^{2} = \left(x_{2} - x_{1}\right)^{2} + \left(y_{2} - y_{1}\right)^{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x1` | in | x-coordinate of the first point | — |
| `y1` | in | y-coordinate of the first point | — |
| `x2` | in | x-coordinate of the second point | — |
| `y2` | in | y-coordinate of the second point | — |
| `dist` | out | distance | — |

```shell
$ math-foundations solve distance_between_points x1=1 y1=1 x2=4 y2=5
dist = 5
  by: Distance between two points  (the Pythagorean theorem on the horizontal and vertical differences)
  x1 = 1
  y1 = 1
  x2 = 4
  y2 = 5
```

**What the worked example means:** This calculation determines the straight-line separation between two locations in a flat plane using their coordinates. Applying the Pythagorean theorem to the horizontal and vertical differences shows that the distance is 5. This value represents the shortest path connecting the first point at (1, 1) and the second point at (4, 5).

### Counting and probability (1)

<a id="counting_choices"></a>

#### Combinations and permutations — `counting_choices`

The number of ways to choose k items from n without order (combinations) and with order (permutations).

$$
\binom{n}{k} = \frac{n!}{k!\,(n-k)!},\quad P(n,k) = \frac{n!}{(n-k)!}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | number of items to choose from (a whole number) | — |
| `k` | in | number of items chosen (a whole number, k <= n) | — |
| `combinations` | out | C(n, k) = n! / (k! (n - k)!) | — |
| `permutations` | out | P(n, k) = n! / (n - k)! | — |

**Applies when:** n and k whole numbers; k <= n; n <= 170.

```shell
$ math-foundations solve counting_choices k=3 n=10
combinations = 120
permutations = 720
  by: Combinations and permutations  (exact integer arithmetic (math.comb, math.perm))
  k = 3
  n = 10
  assumes: n and k whole numbers; k <= n; n <= 170
```

**What the worked example means:** This calculation determines two distinct counts based on selecting a subset of items. The combinations value represents the number of ways to select 3 items from a total of 10 where the order of selection does not matter, yielding 120. The permutations value represents the number of ways to arrange those same 3 items in a specific sequence, resulting in 720. These figures are derived using exact integer arithmetic to ensure precision without rounding errors.

### Exponential functions (1)

<a id="compound_interest"></a>

#### Compound interest — `compound_interest`

The amount after t years when a principal is compounded n times a year at an annual rate r (a decimal).

$$
\ln\left(\text{amount}\right) = \ln\left(\text{principal}\right) + n t \ln\left(1 + \frac{r}{n}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `principal` | in | starting amount | — |
| `r` | in | annual interest rate as a decimal | — |
| `n` | in | compounding periods per year | — |
| `t` | in | time in years | — |
| `amount` | out | amount after t years | — |

**Applies when:** principal > 0; n >= 1; t >= 0; 1 + r/n > 0.

```shell
$ math-foundations solve compound_interest t=2 principal=1000 r=0.05 n=1
amount = 1102.5
  by: Compound interest  (each period multiplies the amount by (1 + r/n), n*t times)
  t = 2
  principal = 1000
  r = 0.05
  n = 1
  assumes: principal > 0; n >= 1; t >= 0; 1 + r/n > 0
```

**What the worked example means:** This calculation determines the future value of an investment using compound interest. Starting with a principal of 1000, the balance grows by multiplying by one plus the periodic rate for each of the total compounding periods defined by 1 times 2. The resulting amount after this duration is 1102.5, reflecting the exponential growth where earned interest itself generates further returns over time.

### Exponential models (1)

<a id="doubling_time"></a>

#### Doubling time of exponential growth — `doubling_time`

The time for a quantity growing as N = N0*e^(k*t) to double, for a growth constant k > 0.

$$
k t_{d} = \ln\left(2\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | continuous growth constant per unit time | — |
| `td` | out | doubling time | — |

**Applies when:** k > 0.

```shell
$ math-foundations solve doubling_time k=0.1
td = 6.931471806
  by: Doubling time of exponential growth  (solve e^(k*t) = 2 for t)
  k = 0.1
  assumes: k > 0
```

**What the worked example means:** This calculation finds the doubling time for a quantity growing continuously at rate k. Using the formula where the growth constant multiplied by time equals the natural log of two shows that with a growth constant of 0.1, the system takes 6.93147 to double in size. This metric indicates how quickly populations or investments expand under exponential conditions.

### Exponentials and logarithms (7)

<a id="half_life_remaining"></a>

#### Amount remaining after radioactive decay (half-life) — `half_life_remaining`

The amount left after time t of a substance with half-life T: N0 * (1/2)^(t/T).

$$
N(t) = N_0 \left(\tfrac12\right)^{t/T}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n0` | in | initial amount (positive) | — |
| `half_life` | in | the half-life T (positive) | — |
| `t` | in | elapsed time, in the unit of T | — |
| `remaining` | out | amount remaining (in the unit of N0) | — |
| `fraction` | out | fraction remaining | — |

**Applies when:** n0 > 0; T > 0; t >= 0; t and T in the same unit.

```shell
$ math-foundations solve half_life_remaining n0=80 half_life=5 t=15
remaining = 10
fraction = 0.125
  by: Amount remaining after radioactive decay (half-life)  (each half-life halves the amount: N = N0 (1/2)^(t/T))
  n0 = 80
  half_life = 5
  t = 15
  assumes: n0 > 0; T > 0; t >= 0; t and T in the same unit
```

**What the worked example means:** This calculation determines how much of a radioactive substance remains after a specific period. By applying the half-life decay formula, it computes the final quantity based on the initial amount, the elapsed time, and the substance's characteristic half-life. The result shows that 10 of the original material persists, representing a fraction of 0.125 from the starting point.

<a id="continuous_growth"></a>

#### Continuous exponential growth or decay — `continuous_growth`

N(t) = N0 * e^(k t) for continuous growth (k > 0) or decay (k < 0), including continuous compounding.

$$
N(t) = N_0\,e^{kt}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n0` | in | initial amount N0 | — |
| `k` | in | continuous rate per unit time, as a decimal (0.03 for 3%; negative for decay) | — |
| `t` | in | elapsed time, in the rate's unit | — |
| `amount` | out | N(t) | — |

**Applies when:** |k t| <= 700.

```shell
$ math-foundations solve continuous_growth n0=500 k=0.03 t=10
amount = 674.9294038
  by: Continuous exponential growth or decay  (the solution of dN/dt = kN: N = N0 e^(kt))
  n0 = 500
  k = 0.03
  t = 10
  assumes: |k t| <= 700
```

**What the worked example means:** This calculation determines the future quantity of a resource undergoing continuous exponential change. Starting with an initial amount of 500, the model applies a constant proportional rate of 0.03 across the elapsed duration of 10. The resulting value, 674.929, represents the total accumulation or reduction after this continuous process completes.

<a id="doubling_time_percent"></a>

#### Doubling time at a percentage growth rate — `doubling_time_percent`

The time for a quantity growing by r% per period (compounded once per period) to double.

$$
T = \frac{\ln 2}{\ln\left(1 + \frac{r}{100}\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | growth rate per period, in percent (positive) | — |
| `doubling_time` | out | periods to double (exact) | — |
| `rule_of_70_estimate` | out | the rule-of-70 approximation 70/r | — |

**Applies when:** r > 0.

```shell
$ math-foundations solve doubling_time_percent r=7
doubling_time = 10.24476835
rule_of_70_estimate = 10
  by: Doubling time at a percentage growth rate  (solve (1 + r/100)^T = 2: T = ln 2 / ln(1 + r/100))
  r = 7
  assumes: r > 0
```

**What the worked example means:** This calculation determines how many periods it takes for a quantity to double when growing at a constant percentage rate. Using the input growth rate of 7, the model computes the precise mathematical solution as 10.2448 periods. It also provides the common approximation known as the rule of seventy, which yields an estimate of 10 periods. This helps quickly understand the pace of exponential growth in contexts like finance or population dynamics.

<a id="log_evaluate"></a>

#### Evaluate a logarithm — `log_evaluate`

log_b(x) for a positive base b != 1 and x > 0 (log is base 10, ln base e).

$$
\log_b x = \frac{\ln x}{\ln b}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | the base (positive, not 1) | — |
| `x` | in | the number (positive) | — |
| `value` | out | log_b(x) | — |

**Applies when:** b > 0; b != 1; x > 0.

```shell
$ math-foundations solve log_evaluate b=2 x=32
value = 5
  by: Evaluate a logarithm  (change of base: log_b(x) = ln(x)/ln(b))
  b = 2
  x = 32
  assumes: b > 0; b != 1; x > 0
```

**What the worked example means:** This calculation determines the exponent to which the base 2 must be raised to produce the number 32. By applying the change of base formula, the result is computed as the natural logarithm of 32 divided by the natural logarithm of 2, yielding a final value of 5. This method allows for the evaluation of logarithms with arbitrary positive bases using standard natural logarithm functions.

<a id="exponential_equation_general"></a>

#### Exponential equation a*b^(cx + d) = k — `exponential_equation_general`

Solve a * b^(c*x + d) = k for x by taking logarithms.

$$
x = \frac{\log_b(k/a) - d}{c}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | the coefficient a | — |
| `b` | in | the base (positive, not 1; e for the natural base) | — |
| `c` | in | coefficient of x in the exponent | — |
| `d` | in | constant in the exponent | — |
| `k` | in | the right-hand side | — |
| `x` | out | the solution | — |

**Applies when:** a != 0; c != 0; b > 0, b != 1; k/a > 0.

```shell
$ math-foundations solve exponential_equation_general a=3 b=2 c=1 d=1 k=48
x = 3
  by: Exponential equation a*b^(cx + d) = k  (divide by a, take log base b: c x + d = log_b(k/a))
  a = 3
  b = 2
  c = 1
  d = 1
  k = 48
  assumes: a != 0; c != 0; b > 0, b != 1; k/a > 0
```

**What the worked example means:** This calculation determines the value of x that satisfies the exponential equation. Isolating the exponential term and applying logarithms shows that x equals 3. This result represents the specific input value where the scaled exponential growth or decay reaches the target magnitude defined by k.

<a id="log_laws_combine"></a>

#### Laws of logarithms (product, quotient, power) — `log_laws_combine`

Given log_b(M) = u and log_b(N) = v, the value of log_b(M^p * N^q) (quotients and roots included).

$$
\log_b\!\left(M^{p} N^{q}\right) = p\log_b M + q\log_b N
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `u` | in | log_b(M) | — |
| `v` | in | log_b(N) | — |
| `p` | in | power of M | — |
| `q` | in | power of N (negative: N in the denominator) | — |
| `value` | out | log_b(M^p N^q) | — |

**Applies when:** M, N > 0.

```shell
$ math-foundations solve log_laws_combine u=2 v=3 p=2 q=1
value = 7
  by: Laws of logarithms (product, quotient, power)  (product law log(MN) = log M + log N, quotient law, power law log(M^p) = p log M)
  u = 2
  v = 3
  p = 2
  q = 1
  assumes: M, N > 0
```

**What the worked example means:** This calculation determines the logarithm of a combined expression involving powers and products. By applying the laws of logarithms, specifically the product and power rules, the result 7 is obtained using the given base values 2 and 3 along with the exponents 2 and 1. This method simplifies complex multiplicative relationships into additive ones, allowing for easier computation and interpretation of scale changes.

<a id="log_equation_linear_argument"></a>

#### Logarithmic equation log_b(cx + d) = k — `log_equation_linear_argument`

Solve log_b(c*x + d) = k for x by rewriting it in exponential form.

$$
\log_b(cx+d) = k \iff cx + d = b^{k}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | the base (positive, not 1; 10 for log, e for ln) | — |
| `c` | in | coefficient of x in the argument | — |
| `d` | in | constant in the argument | — |
| `k` | in | the right-hand side | — |
| `x` | out | the solution | — |

**Applies when:** b > 0, b != 1; c != 0.

```shell
$ math-foundations solve log_equation_linear_argument b=2 c=3 d=1 k=4
x = 5
  by: Logarithmic equation log_b(cx + d) = k  (log_b(u) = k means u = b^k; the argument b^k is positive, so the solution is always admissible)
  b = 2
  c = 3
  d = 1
  k = 4
  assumes: b > 0, b != 1; c != 0
```

**What the worked example means:** To solve the logarithmic equation, it is converted to its exponential form. The argument of the logarithm equals the base raised to the power of the right-hand side value. The variable is then isolated by subtracting the constant term and dividing by the coefficient of the variable. This yields the result 5, which represents the specific value that satisfies the original equation.

### Exponents and logarithms (1)

<a id="exponential_equation"></a>

#### Solution of an exponential equation — `exponential_equation`

Solve b^x = y for x, with b > 0, b != 1 and y > 0.

$$
x \ln\left(b\right) = \ln\left(y\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | the base | — |
| `y` | in | the value of the power | — |
| `x` | out | the exponent | — |

**Applies when:** b > 0; y > 0; b != 1.

```shell
$ math-foundations solve exponential_equation b=2 y=32
x = 5
  by: Solution of an exponential equation  (take logarithms of both sides: x = log(y)/log(b))
  b = 2
  y = 32
  assumes: b > 0; y > 0; b != 1
```

**What the worked example means:** This calculation determines the exponent 5 required for a base 2 to produce a target value 32. Taking the natural logarithm of both sides of the exponential equation isolates the unknown exponent. The result indicates how many times the base must be multiplied by itself to reach the specified power, assuming the base is positive and not equal to one.

### Factoring (1)

<a id="factor_monic_quadratic"></a>

#### Factoring a monic quadratic — `factor_monic_quadratic`

Write x^2 + b*x + c as (x + p)*(x + q) with real p and q.

$$
\left(t + p\right) \left(t + q\right) = t^{2} + b t + c
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | coefficient of x | — |
| `c` | in | constant term | — |
| `p` | out | first constant | — |
| `q` | out | second constant | — |

**Applies when:** b**2 - 4*c >= 0.

```shell
$ math-foundations solve factor_monic_quadratic b=5 c=6
p = 3
q = 2
  by: Factoring a monic quadratic  (find p and q with p + q = b and p*q = c (the roots of the quadratic are -p and -q))
  b = 5
  c = 6
  assumes: b**2 - 4*c >= 0
```

**What the worked example means:** This calculation rewrites a monic quadratic polynomial as a product of two linear binomials. By identifying two numbers, p and q, that sum to the linear coefficient b and multiply to the constant term c, the expression is factored. The resulting constants are 3 and 2, which allow for immediate identification of the roots and simplification of rational expressions.

### Functions (10)

<a id="average_rate_of_change_definition"></a>

#### Average rate of change of a function between two inputs — `average_rate_of_change_definition`

(f(x_end) - f(x_start)) / (x_end - x_start) from two values of a function.

$$
\text{arc} \left(x_{\text{end}} - x_{\text{start}}\right) = f_{x,end} - f_{x,start}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x_start` | in | first input | — |
| `f_x_start` | in | value of the function at the first input | — |
| `x_end` | in | second input | — |
| `f_x_end` | in | value of the function at the second input | — |
| `arc` | out | average rate of change | — |

**Applies when:** x_end != x_start.

```shell
$ math-foundations solve average_rate_of_change_definition x_start=0 f_x_start=1 x_end=5 f_x_end=11
arc = 2
  by: Average rate of change of a function between two inputs  (change in output divided by change in input)
  x_start = 0
  f_x_start = 1
  x_end = 5
  f_x_end = 11
  assumes: x_end != x_start
```

**What the worked example means:** This calculation determines the average rate of change for a function over an interval defined by two distinct input points. Taking the difference between the final function value and the initial function value, then dividing that result by the difference between the final input and the initial input gives a single number representing the slope of the secant line connecting these two points on the graph. The computed value, 2, indicates how much the output changes on average for each unit increase in the input between the starting point and the ending point.

<a id="compose_linear_functions"></a>

#### Composition of two linear functions — `compose_linear_functions`

For f(x) = a*x + b and g(x) = c*x + d, the composition f(g(x)) = m*x + k.

$$
m t + k = a \left(c t + d\right) + b
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | slope of f | — |
| `b` | in | intercept of f | — |
| `c` | in | slope of g | — |
| `d` | in | intercept of g | — |
| `m` | out | slope of f(g(x)) | — |
| `k` | out | intercept of f(g(x)) | — |

```shell
$ math-foundations solve compose_linear_functions a=2 b=1 c=3 d=-4
m = 6
k = -7
  by: Composition of two linear functions  (substitute g(x) for x in f)
  a = 2
  b = 1
  c = 3
  d = -4
```

**What the worked example means:** This calculation determines the single linear function that represents the composition of two separate linear functions. Substituting the second function into the first gives a new slope-intercept form where the resulting slope is 6 and the resulting intercept is -7. This simplifies sequential transformations into one direct relationship between input and output.

<a id="distance_point_to_line"></a>

#### Distance from a point to a line — `distance_point_to_line`

The perpendicular distance from (x0, y0) to the line a*x + b*y + c = 0.

$$
d = \frac{|a x_0 + b y_0 + c|}{\sqrt{a^2 + b^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x0` | in | x-coordinate of the point | — |
| `y0` | in | y-coordinate of the point | — |
| `a` | in | coefficient of x | — |
| `b` | in | coefficient of y | — |
| `c` | in | constant term | — |
| `distance` | out | the perpendicular distance | — |

**Applies when:** a and b not both 0.

```shell
$ math-foundations solve distance_point_to_line x0=1 y0=2 a=3 b=4 c=-5
distance = 1.2
  by: Distance from a point to a line  (|a x0 + b y0 + c| / sqrt(a^2 + b^2))
  x0 = 1
  y0 = 2
  a = 3
  b = 4
  c = -5
  assumes: a and b not both 0
```

**What the worked example means:** This calculation determines the shortest straight-line gap between a specific point and an infinite line on a flat plane. Taking the absolute value of the line equation evaluated at the point's coordinates and dividing it by the magnitude of the line's normal vector gives the precise separation. The result, 1.2, represents this minimal geometric distance, indicating how far the point lies from satisfying the line's condition.

<a id="domain_sqrt_over_linear"></a>

#### Domain of a square-root function — `domain_sqrt_over_linear`

The domain of f(x) = sqrt(a*x + b)/(c*x + d) (or 1/sqrt(a*x + b)): where the radicand is allowed and the denominator is not 0.

$$
ax + b \ge 0,\quad cx + d \ne 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x under the root | — |
| `b` | in | constant under the root | — |
| `c` | in | coefficient of x in the denominator (0 for none) | — |
| `d` | in | constant in the denominator (1 for none) | — |
| `root_in_denominator` | in | true when the root itself is in the denominator (the radicand must then be positive) (optional) | — |
| `boundary` | out | where the radicand is 0 | — |
| `side` | out | the inequality x must satisfy with the boundary | — |
| `excluded` | out | a point removed because the denominator is 0 there (None if none) | — |
| `domain` | out | the domain in interval notation | — |

**Applies when:** a != 0; the denominator is not identically 0.

```shell
$ math-foundations solve domain_sqrt_over_linear a=2 b=-4 c=0 d=1 root_in_denominator=False
boundary = 2
side = >=
excluded = null
domain = [2, inf)
  by: Domain of a square-root function  (radicand a*x + b >= 0 (> 0 when the root is in a denominator); remove the zero of c*x + d)
  a = 2
  b = -4
  c = 0
  d = 1
  assumes: a != 0; the denominator is not identically 0
```

In the worked example, a = 2, b = -4, c = 0, d = 1, root_in_denominator = False go through the equation above and give boundary = 2, side = >=, excluded = None, domain = [2, inf). It applies only when a != 0; the denominator is not identically 0: outside that it says what is wrong instead of returning a number.

<a id="evaluate_function"></a>

#### Evaluate a function at a number — `evaluate_function`

The value f(x0) of a function given by a formula in x (exact when it is rational).

$$
f(x_0)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `expression` | in | the formula of f in x: numbers, x, + - * / ^, parentheses, sqrt, abs, ln | — |
| `x` | in | the number x0 to put in | — |
| `value` | out | f(x0) | — |
| `numerator` | out | numerator of the exact value | — |
| `denominator` | out | denominator of the exact value | — |

**Applies when:** x0 in the domain of f (no division by 0, no root of a negative number, no logarithm of a number <= 0).

```shell
$ math-foundations solve evaluate_function "expression=2x^2 - 3x + 1" x=3
value = 10
numerator = 10
denominator = 1
  by: Evaluate a function at a number  (substitute x0 for x and evaluate with the order of operations, in exact rational arithmetic)
  x = 3
  expression = 2x^2 - 3x + 1
  assumes: x0 in the domain of f (no division by 0, no root of a negative number, no logarithm of a number <= 0)
```

**What the worked example means:** This calculation determines the specific output of a function by substituting the provided input number into its algebraic expression. The result, 10, represents the exact rational value obtained after performing all arithmetic operations in sequence according to standard precedence rules. This approach ensures precision by avoiding decimal approximations, yielding a numerator of 10 and a denominator of 1.

<a id="piecewise_evaluate"></a>

#### Evaluate a piecewise function — `piecewise_evaluate`

f(x0) for f(x) = e1 when x < c (or x <= c) and e2 otherwise.

$$
f(x) = \begin{cases} e_1(x) & x < c \\ e_2(x) & x \ge c \end{cases}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `e1` | in | formula of the left piece | — |
| `left_rel` | in | the left piece holds for x < c or x <= c | — |
| `c` | in | the break point c | — |
| `e2` | in | formula of the right piece | — |
| `x` | in | the number x0 | — |
| `value` | out | f(x0) | — |
| `piece` | out | which piece applies (1 = left, 2 = right) | — |

**Applies when:** the two conditions split the line at c (one < and one >=, or <= and >); x0 in the domain of its piece.

```shell
$ math-foundations solve piecewise_evaluate "e1=2x + 1" "left_rel=<" c=2 "e2=x^2 - 1" x=3
value = 8
piece = 2
  by: Evaluate a piecewise function  (decide which condition x0 satisfies (the break point belongs to the piece whose condition includes it), then evaluate that piece)
  c = 2
  x = 3
  e2 = x^2 - 1
  left_rel = <
  e1 = 2x + 1
  assumes: the two conditions split the line at c (one < and one >=, or <= and >); x0 in the domain of its piece
```

**What the worked example means:** To find f(x0), the condition that x0 satisfies is checked. Since x0 is 3, it falls into the region defined by < relative to the break point 2. This selects piece 2, so its formula is evaluated at x0 to get the result 8.

<a id="graph_transformation_point"></a>

#### Image of a point under a graph transformation — `graph_transformation_point`

If (x0, y0) is on y = f(x), the corresponding point on y = A*f(B*(x - h)) + k.

$$
(x_0, y_0) \mapsto \left(\frac{x_0}{B} + h,\; A y_0 + k\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x0` | in | x-coordinate of the point on y = f(x) | — |
| `y0` | in | y-coordinate of the point | — |
| `A` | in | vertical stretch (negative: reflection in the x-axis) (optional) | — |
| `B` | in | horizontal compression factor (negative: reflection in the y-axis) (optional) | — |
| `h` | in | horizontal shift (optional) | — |
| `k` | in | vertical shift (optional) | — |
| `x_new` | out | x-coordinate of the image | — |
| `y_new` | out | y-coordinate of the image | — |

**Applies when:** A != 0; B != 0.

```shell
$ math-foundations solve graph_transformation_point x0=2 y0=5 A=3 B=2 h=1 k=4
x_new = 2
y_new = 19
  by: Image of a point under a graph transformation  (B(x - h) must equal x0, so x = x0/B + h; the height becomes A*y0 + k)
  x0 = 2
  y0 = 5
  A = 3
  B = 2
  h = 1
  k = 4
  assumes: A != 0; B != 0
```

**What the worked example means:** This calculation finds the new location of a specific point on a graph after it undergoes transformations defined by vertical stretch A, horizontal compression B, and shifts h and k. Starting from the original coordinates 2, 5, the method determines that the image point is located at 2, 19. This result shows exactly where the feature moves under the combined effects of scaling and translation.

<a id="line_intercepts"></a>

#### Intercepts and slope of a line — `line_intercepts`

The x- and y-intercepts and the slope of the line a*x + b*y = c.

$$
ax + by = c:\quad x_0 = \frac{c}{a},\ y_0 = \frac{c}{b},\ m = -\frac{a}{b}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x | — |
| `b` | in | coefficient of y | — |
| `c` | in | right-hand side | — |
| `x_intercept` | out | where the line meets the x-axis (y = 0) | — |
| `y_intercept` | out | where it meets the y-axis (x = 0) | — |
| `slope` | out | the slope -a/b | — |

**Applies when:** a != 0; b != 0.

```shell
$ math-foundations solve line_intercepts a=2 b=3 c=12
x_intercept = 6
y_intercept = 4
slope = -0.6666666667
  by: Intercepts and slope of a line  (set y = 0 for the x-intercept and x = 0 for the y-intercept)
  a = 2
  b = 3
  c = 12
  assumes: a != 0; b != 0
```

**What the worked example means:** This calculation finds where the straight line defined by coefficients 2, 3, and constant 12 crosses the axes. The x-intercept is 6, indicating the horizontal position where the vertical value is zero. The y-intercept is 4, showing the vertical starting point when the horizontal value is zero. The slope, calculated as negative 2 divided by 3, results in -0.666667, which describes the steepness and direction of the line's ascent or descent.

<a id="inverse_linear_function"></a>

#### Inverse of a linear function — `inverse_linear_function`

For f(x) = a*x + b (a != 0): f^-1(x) = x/a - b/a, and f^-1(y) for a given y.

$$
f^{-1}(x) = \frac{x - b}{a}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | slope of f | — |
| `b` | in | intercept of f | — |
| `y` | in | a value to invert (optional) (optional) | — |
| `inv_slope` | out | slope of the inverse, 1/a | — |
| `inv_intercept` | out | intercept of the inverse, -b/a | — |
| `value` | out | f^-1(y) | — |

**Applies when:** a != 0.

```shell
$ math-foundations solve inverse_linear_function a=3 b=-6
inv_slope = 0.3333333333
inv_intercept = 2
  by: Inverse of a linear function  (swap x and y in y = a*x + b and solve for y)
  a = 3
  b = -6
  assumes: a != 0
```

**What the worked example means:** The inverse of the linear function with slope 3 and intercept -6 has a new slope of 0.333333 and an intercept of 2.

<a id="composite_function_value"></a>

#### Value of a composite function — `composite_function_value`

f(g(x0)) or g(f(x0)) for two functions given by formulas.

$$
(f\circ g)(x_0) = f\big(g(x_0)\big)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | the formula of f in x | — |
| `g` | in | the formula of g in x | — |
| `x` | in | the number x0 | — |
| `order` | in | which composition | — |
| `inner` | out | value of the inner function at x0 | — |
| `value` | out | value of the composite at x0 | — |

**Applies when:** each value in the domain of the function applied to it.

```shell
$ math-foundations solve composite_function_value "f=x^2 + 1" "g=2x - 3" x=2 "order=f(g(x))"
inner = 1
value = 2
  by: Value of a composite function  (evaluate the inner function at x0, then the outer function at that value)
  x = 2
  f = x^2 + 1
  g = 2x - 3
  order = f(g(x))
  assumes: each value in the domain of the function applied to it
```

**What the worked example means:** This calculation determines the output of a nested function by first evaluating the inner function at the input x, yielding 1. This intermediate value is then used as the input for the outer function to produce the final composite result of 2. The process relies on correctly identifying which function is applied first based on the specified order.

### Inequalities (1)

<a id="linear_inequality"></a>

#### Solution of a linear inequality — `linear_inequality`

Solve a*x + b < c (or >): the boundary x0 and the direction s (s = 1 means the solution of the < form is x < x0; s = -1 means it reverses to x > x0 because a < 0).

$$
a x_{0} + b = c,\quad s \sqrt{a^{2}} = a
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x | — |
| `b` | in | constant on the left | — |
| `c` | in | right-hand side | — |
| `x0` | out | boundary of the solution set | — |
| `s` | out | 1 if the inequality keeps its direction, -1 if it reverses | — |

**Applies when:** a != 0.

```shell
$ math-foundations solve linear_inequality a=2 b=1 c=7
x0 = 3
s = 1
  by: Solution of a linear inequality  (subtract b, divide by a; dividing by a negative number reverses the inequality)
  a = 2
  b = 1
  c = 7
  assumes: a != 0
```

**What the worked example means:** This calculation finds the critical value where the inequality changes from true to false, which is 3. It also determines whether the solution set extends to values less than or greater than this boundary based on the sign of the coefficient. The result depends on inputs a, b, and c, with the boundary shifting as these values change.

### Linear and absolute value inequalities (2)

<a id="abs_value_inequality_upper_bound"></a>

#### Solution interval of |x - c| < d — `abs_value_inequality_upper_bound`

For |x - c| < d with d > 0, the solutions are lower < x < upper.

$$
\text{upper} - \text{lower} = 2 d,\quad \text{upper} + \text{lower} = 2 c
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c` | in | centre c | — |
| `d` | in | bound d | — |
| `lower` | out | lower end of the solution interval (excluded) | — |
| `upper` | out | upper end of the solution interval (excluded) | — |

**Applies when:** d > 0.

```shell
$ math-foundations solve abs_value_inequality_upper_bound c=4 d=1
lower = 3
upper = 5
  by: Solution interval of |x - c| < d  (|x - c| < d means -d < x - c < d)
  c = 4
  d = 1
  assumes: d > 0
```

**What the worked example means:** This calculation determines the range of values that satisfy the condition where the distance from the center c is less than the bound d. Converting the absolute value inequality into a compound inequality shows that x must be greater than the lower limit 3 and less than the upper limit 5. These endpoints are derived directly from the given center 4 and bound 1, ensuring that all values within this open interval meet the specified proximity requirement.

<a id="abs_value_inequality_lower_bound"></a>

#### Solution set of |x - c| > d — `abs_value_inequality_lower_bound`

For |x - c| > d with d >= 0, the solutions are x < left_limit or x > right_limit.

$$
\text{right limit} - \text{left limit} = 2 d,\quad \text{right limit} + \text{left limit} = 2 c
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c` | in | centre c | — |
| `d` | in | bound d | — |
| `left_limit` | out | x must be below this | — |
| `right_limit` | out | or above this | — |

**Applies when:** d >= 0.

```shell
$ math-foundations solve abs_value_inequality_lower_bound c=1 d=3
left_limit = -2
right_limit = 4
  by: Solution set of |x - c| > d  (|x - c| > d means x - c < -d or x - c > d)
  c = 1
  d = 3
  assumes: d >= 0
```

**What the worked example means:** This calculation determines the range of values that lie outside a specific distance from a central point. By using the centre 1 and the bound 3, it identifies two critical limits: -2 and 4. Any value of x strictly below -2 or strictly above 4 satisfies the condition, meaning the variable must be sufficiently far away from the target center.

### Linear equations (1)

<a id="linear_equation_root"></a>

#### Root of a linear equation — `linear_equation_root`

Solve a*x + b = c for x.

$$
a x + b = c
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x | — |
| `b` | in | constant term on the left side | — |
| `c` | in | right-hand side | — |
| `x` | out | the solution | — |

**Applies when:** a != 0.

```shell
$ math-foundations solve linear_equation_root a=3 b=6 c=0
x = -2
  by: Root of a linear equation  (subtract b from both sides, then divide by a)
  a = 3
  b = 6
  c = 0
  assumes: a != 0
```

**What the worked example means:** To solve the equation a times x plus b equals c, the variable is isolated by subtracting the constant term from both sides and then dividing by the coefficient. Given the inputs a as 3, b as 6, and c as 0, the calculated value for the unknown variable x is -2. This result represents the specific quantity that satisfies the linear relationship defined in the problem.

### Linear functions (4)

<a id="average_rate_of_change_linear"></a>

#### Average rate of change between two points — `average_rate_of_change_linear`

(y2 - y1) / (x2 - x1) between the points (x1, y1) and (x2, y2).

$$
m \left(x_{2} - x_{1}\right) = y_{2} - y_{1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x1` | in | x-coordinate of the first point | — |
| `y1` | in | y-coordinate of the first point | — |
| `x2` | in | x-coordinate of the second point | — |
| `y2` | in | y-coordinate of the second point | — |
| `m` | out | average rate of change | — |

**Applies when:** x2 != x1.

```shell
$ math-foundations solve average_rate_of_change_linear x1=-2 y1=5 x2=3 y2=0
m = -1
  by: Average rate of change between two points  (change in y divided by change in x)
  x1 = -2
  y1 = 5
  x2 = 3
  y2 = 0
  assumes: x2 != x1
```

**What the worked example means:** This calculation determines the slope of the line connecting two specific points on a coordinate plane. Taking the difference in the vertical values and dividing it by the difference in the horizontal values shows that the average rate of change is -1. This value represents how much the y-coordinate changes for every single unit increase in the x-coordinate between the first point at -2 and the second point at 3.

<a id="line_through_two_points"></a>

#### Line through two points — `line_through_two_points`

The slope m and y-intercept k of the line through (x1, y1) and (x2, y2), in the form y = m*x + k.

$$
m x_{1} + k = y_{1},\quad m x_{2} + k = y_{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x1` | in | x-coordinate of the first point | — |
| `y1` | in | y-coordinate of the first point | — |
| `x2` | in | x-coordinate of the second point | — |
| `y2` | in | y-coordinate of the second point | — |
| `m` | out | slope | — |
| `k` | out | y-intercept | — |

**Applies when:** x2 - x1 != 0.

```shell
$ math-foundations solve line_through_two_points x1=0 y1=1 x2=2 y2=5
m = 2
k = 1
  by: Line through two points  (slope = rise over run; the intercept follows from one point)
  x1 = 0
  y1 = 1
  x2 = 2
  y2 = 5
  assumes: x2 - x1 != 0
```

**What the worked example means:** This calculation determines the equation of a straight line passing through two specific points. Using the first point's coordinates 0 and 1, and the second point's coordinates 2 and 5 gives the rate of change known as the slope, which is 2. The y-intercept, the value where the line crosses the vertical axis, is then 1. Together, these values define the linear relationship between the variables.

<a id="perpendicular_lines_slope_product"></a>

#### Slope of a perpendicular line — `perpendicular_lines_slope_product`

The slope m_perp of a line perpendicular to a line of nonzero slope m.

$$
m m_{\text{perp}} = -1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | slope of the given line | — |
| `m_perp` | out | slope of the perpendicular line | — |

**Applies when:** m != 0.

```shell
$ math-foundations solve perpendicular_lines_slope_product m=5
m_perp = -0.2
  by: Slope of a perpendicular line  (the product of the slopes of perpendicular lines is -1)
  m = 5
  assumes: m != 0
```

**What the worked example means:** This calculation finds the slope of a line that stands at a right angle to a given line. Applying the rule that the product of slopes for perpendicular lines equals negative one shows that if the original line has a slope of 5, the perpendicular line must have a slope of -0.2. This relationship ensures the two lines intersect orthogonally.

<a id="convert_to_slope_intercept"></a>

#### y-intercept of a line from its slope and a point — `convert_to_slope_intercept`

The constant b of y = m*x + b for the line with slope m through (x1, y1).

$$
y_{1} = m x_{1} + b
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | slope | — |
| `x1` | in | x-coordinate of the point | — |
| `y1` | in | y-coordinate of the point | — |
| `b` | out | y-intercept | — |

```shell
$ math-foundations solve convert_to_slope_intercept m=4 x1=3 y1=10
b = -2
  by: y-intercept of a line from its slope and a point  (substitute the point into y = m*x + b and solve for b)
  m = 4
  x1 = 3
  y1 = 10
```

**What the worked example means:** This calculation determines the constant term of a linear equation in slope-intercept form. Substituting the known slope 4 and the coordinates of a specific point, x-value 3 and y-value 10, into the general line equation isolates the intercept. The resulting value, -2, represents the height at which the line crosses the vertical axis, completing the description of the straight-line relationship.

### Logarithms (1)

<a id="logarithmic_equation"></a>

#### Solution of a basic logarithmic equation — `logarithmic_equation`

Solve log_b(x) = c for x.

$$
\ln\left(x\right) = c \ln\left(b\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | base | — |
| `c` | in | value of the logarithm | — |
| `x` | out | the solution | — |

**Applies when:** b > 0; b != 1.

```shell
$ math-foundations solve logarithmic_equation b=3 c=4
x = 81
  by: Solution of a basic logarithmic equation  (rewrite in exponential form: x = b^c)
  b = 3
  c = 4
  assumes: b > 0; b != 1
```

In the worked example, b = 3, c = 4 go through the equation above and give x = 81. It applies only when b > 0; b != 1: outside that it says what is wrong instead of returning a number.

### Matrices (1)

<a id="matrix_2x2_inverse"></a>

#### Determinant and inverse of a 2 by 2 matrix — `matrix_2x2_inverse`

For the matrix [[a, b], [c, d]], its determinant and the entries of its inverse.

$$
\text{det} = a d - b c,\quad a i_{11} + b i_{21} = 1,\quad a i_{12} + b i_{22} = 0,\quad c i_{11} + d i_{21} = 0,\quad c i_{12} + d i_{22} = 1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | top-left entry | — |
| `b` | in | top-right entry | — |
| `c` | in | bottom-left entry | — |
| `d` | in | bottom-right entry | — |
| `det` | out | determinant | — |
| `i11` | out | inverse, top-left | — |
| `i12` | out | inverse, top-right | — |
| `i21` | out | inverse, bottom-left | — |
| `i22` | out | inverse, bottom-right | — |

**Applies when:** a*d - b*c != 0.

```shell
$ math-foundations solve matrix_2x2_inverse a=2 b=1 c=5 d=3
det = 1
i11 = 3
i12 = -1
i21 = -5
i22 = 2
  by: Determinant and inverse of a 2 by 2 matrix  (swap the diagonal, negate the off-diagonal, divide by the determinant)
  a = 2
  b = 1
  c = 5
  d = 3
  assumes: a*d - b*c != 0
```

**What the worked example means:** This calculation determines the determinant and the inverse of a two by two matrix using inputs 2, 1, 5, and 3. The determinant, shown as 1, confirms the matrix is invertible. The resulting inverse matrix has top-left entry 3, top-right entry -1, bottom-left entry -5, and bottom-right entry 2. These values allow for reversing linear transformations and solving associated systems of equations.

### Matrices and linear systems (2)

<a id="determinant_2x2_formula"></a>

#### Determinant of 2x2 matrix — `determinant_2x2_formula`

Computes the determinant of a 2x2 matrix using the standard formula.

$$
\text{det} = a d - b c
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | entry in row 1 column 1 | — |
| `b` | in | entry in row 1 column 2 | — |
| `c` | in | entry in row 2 column 1 | — |
| `d` | in | entry in row 2 column 2 | — |
| `det` | out | determinant of the matrix | — |

```shell
$ math-foundations solve determinant_2x2_formula a=3 b=1 c=2 d=4
det = 10
  by: Determinant of 2x2 matrix  (closed form verified against the defining equations)
  a = 3
  b = 1
  c = 2
  d = 4
```

**What the worked example means:** This calculation determines the scalar value known as the determinant for a two by two matrix. It is derived from the four input entries using the standard formula where the product of the top left and bottom right values is subtracted from the product of the top right and bottom left values. The resulting value, 10, indicates whether the matrix can be inverted and describes how the transformation represented by the matrix scales area in a two dimensional space.

<a id="matrix_scalar_multiple_definition"></a>

#### Matrix scalar multiple definition — `matrix_scalar_multiple_definition`

Computes the entries of a 2x2 matrix scaled by a scalar.

$$
b_{11} = s a_{11},\quad b_{12} = s a_{12},\quad b_{21} = s a_{21},\quad b_{22} = s a_{22}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `s` | in | scalar multiplier | — |
| `a11` | in | entry in row 1 column 1 | — |
| `a12` | in | entry in row 1 column 2 | — |
| `a21` | in | entry in row 2 column 1 | — |
| `a22` | in | entry in row 2 column 2 | — |
| `b11` | out | entry in row 1 column 1 of result | — |
| `b12` | out | entry in row 1 column 2 of result | — |
| `b21` | out | entry in row 2 column 1 of result | — |
| `b22` | out | entry in row 2 column 2 of result | — |

```shell
$ math-foundations solve matrix_scalar_multiple_definition s=3 a11=1 a12=2 a21=3 a22=4
b11 = 3
b12 = 6
b21 = 9
b22 = 12
  by: Matrix scalar multiple definition  (closed form verified against the defining equations)
  s = 3
  a11 = 1
  a12 = 2
  a21 = 3
  a22 = 4
```

**What the worked example means:** This calculation determines the new matrix obtained by multiplying every entry of the original two by two matrix by the scalar value 3. The resulting entries, such as 3 for the top left position, represent the scaled components of the transformation. Each output entry is derived solely from its corresponding input entry multiplied by the uniform scaling factor.

### Other equations (1)

<a id="absolute_value_equation"></a>

#### Solutions of an absolute value equation — `absolute_value_equation`

Solve |a*x + b| = c for x (two solutions when c > 0, one when c = 0).

$$
a x_{1} + b = c,\quad a x_{2} + b = -c
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x | — |
| `b` | in | constant inside the absolute value | — |
| `c` | in | right-hand side | — |
| `x1` | out | solution of a*x + b = c | — |
| `x2` | out | solution of a*x + b = -c | — |

**Applies when:** a != 0; c >= 0.

```shell
$ math-foundations solve absolute_value_equation a=2 b=-1 c=5
x1 = 3
x2 = -2
  by: Solutions of an absolute value equation  (the expression inside equals c or -c; solve each linear equation)
  a = 2
  b = -1
  c = 5
  assumes: a != 0; c >= 0
```

**What the worked example means:** This calculation finds the values of x that satisfy the condition where the magnitude of the linear expression equals a constant. Setting the expression inside the absolute value bars equal to both the positive and negative versions of the right-hand side gives two linear equations. Solving these yields the specific roots 3 and -2, which represent the points where the function touches the specified magnitude level.

### Polynomial equations (4)

<a id="quadratic_discriminant"></a>

#### Discriminant of a quadratic — `quadratic_discriminant`

The discriminant b^2 - 4ac of a*x^2 + b*x + c and the nature of the roots it implies.

$$
\Delta = b^2 - 4ac
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x^2 | — |
| `b` | in | coefficient of x | — |
| `c` | in | constant term | — |
| `discriminant` | out | b^2 - 4ac | — |
| `nature` | out | two distinct real / one repeated real / two complex conjugate roots | — |

**Applies when:** a != 0; real coefficients.

```shell
$ math-foundations solve quadratic_discriminant a=1 b=4 c=4
discriminant = 0
nature = one repeated real root
  by: Discriminant of a quadratic  (direct evaluation of b^2 - 4ac and its sign)
  a = 1
  b = 4
  c = 4
  assumes: a != 0; real coefficients
```

**What the worked example means:** The discriminant is calculated using the formula b squared minus four times a times c. With coefficients a, b, and c provided as inputs, the resulting value of 0 determines the nature of the roots. A positive result means two distinct real roots, zero means one repeated real root, and a negative result implies two complex conjugate roots.

<a id="cubic_roots"></a>

#### Roots of a cubic equation — `cubic_roots`

Solve a*x^3 + b*x^2 + c*x + d = 0 for x: all three roots, real or complex.

$$
a x^3 + b x^2 + c x + d = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x^3 | — |
| `b` | in | coefficient of x^2 | — |
| `c` | in | coefficient of x | — |
| `d` | in | constant term | — |
| `x1` | out | first root (real roots ascending, then complex roots) | — |
| `x2` | out | second root | — |
| `x3` | out | third root | — |

**Applies when:** a != 0 (otherwise the equation is not cubic); real coefficients.

```shell
$ math-foundations solve cubic_roots a=1 b=-6 c=11 d=-6
x1 = 1
x2 = 2
x3 = 3
  by: Roots of a cubic equation  (high-precision polynomial root finding (Durand-Kerner, 30 digits), each root substituted back)
  a = 1
  b = -6
  c = 11
  d = -6
  assumes: a != 0 (otherwise the equation is not cubic); real coefficients
```

**What the worked example means:** This calculation solves the cubic equation defined by coefficients 1 for x cubed, -6 for x squared, 11 for x, and -6 as the constant. The method finds all three roots, which are reported as 1, 2, and 3. These values represent the points where the polynomial equals zero, encompassing both real and complex solutions.

<a id="quadratic_roots"></a>

#### Roots of a quadratic equation — `quadratic_roots`

Solve a*x^2 + b*x + c = 0 for x: both roots, real or complex.

$$
x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x^2 | — |
| `b` | in | coefficient of x | — |
| `c` | in | constant term | — |
| `x1` | out | first root (the smaller real root, or the one with positive imaginary part) | — |
| `x2` | out | second root | — |
| `discriminant` | out | b^2 - 4ac | — |

**Applies when:** a != 0 (otherwise the equation is linear); real coefficients.

```shell
$ math-foundations solve quadratic_roots a=1 b=-5 c=6
x1 = 2
x2 = 3
discriminant = 1
  by: Roots of a quadratic equation  (quadratic formula in a numerically stable form (q = -(b + sign(b)*sqrt(b^2 - 4ac))/2, x = q/a and c/q))
  a = 1
  b = -5
  c = 6
  assumes: a != 0 (otherwise the equation is linear); real coefficients
```

**What the worked example means:** This calculation finds the values of x that satisfy the quadratic equation using coefficients 1, -5, and 6. The result yields two roots, 2 and 3, determined by the discriminant value of 1. A positive discriminant indicates two distinct real solutions, while a negative one suggests complex conjugate roots. These values represent the points where the parabolic curve intersects the horizontal axis.

<a id="quartic_roots"></a>

#### Roots of a quartic equation — `quartic_roots`

Solve a*x^4 + b*x^3 + c*x^2 + d*x + e = 0 for x: all four roots, real or complex.

$$
a x^4 + b x^3 + c x^2 + d x + e = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x^4 | — |
| `b` | in | coefficient of x^3 | — |
| `c` | in | coefficient of x^2 | — |
| `d` | in | coefficient of x | — |
| `e` | in | constant term | — |
| `x1` | out | first root (real roots ascending, then complex roots) | — |
| `x2` | out | second root | — |
| `x3` | out | third root | — |
| `x4` | out | fourth root | — |

**Applies when:** a != 0 (otherwise the equation is not quartic); real coefficients.

```shell
$ math-foundations solve quartic_roots a=1 b=0 c=-5 d=0 e=4
x1 = -2
x2 = -1
x3 = 1
x4 = 2
  by: Roots of a quartic equation  (high-precision polynomial root finding (Durand-Kerner, 30 digits), each root substituted back)
  a = 1
  b = 0
  c = -5
  d = 0
  e = 4
  assumes: a != 0 (otherwise the equation is not quartic); real coefficients
```

**What the worked example means:** This calculation determines the four solutions to the polynomial equation defined by coefficients 1, 0, -5, 0, and 4. The method locates all real and complex roots, returning them as -2, -1, 1, and 2. Each root represents a value of x that satisfies the equation, verified through high-precision substitution.

### Polynomials (1)

<a id="binomial_product"></a>

#### Product of two binomials — `binomial_product`

Expand (a*x + b)*(c*x + d) into p2*x^2 + p1*x + p0.

$$
p_{2} t^{2} + p_{1} t + p_{0} = \left(a t + b\right) \left(c t + d\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x in the first binomial | — |
| `b` | in | constant of the first binomial | — |
| `c` | in | coefficient of x in the second binomial | — |
| `d` | in | constant of the second binomial | — |
| `p2` | out | coefficient of x^2 | — |
| `p1` | out | coefficient of x | — |
| `p0` | out | constant term | — |

```shell
$ math-foundations solve binomial_product a=1 b=2 c=1 d=3
p2 = 1
p1 = 5
p0 = 6
  by: Product of two binomials  (distribute each term of the first binomial over the second (FOIL) and collect like terms)
  a = 1
  b = 2
  c = 1
  d = 3
```

**What the worked example means:** This calculation distributes each term of the first binomial over the second and collects like terms. It transforms two linear expressions into a standard quadratic form. The resulting coefficient for the squared term is 1, the coefficient for the linear term is 5, and the constant term is 6. This expansion simplifies complex algebraic products into a single polynomial expression.

### Precalculus (8)

<a id="binomial_coefficient_term"></a>

#### Coefficient of a term in a binomial expansion — `binomial_coefficient_term`

The coefficient of x^k in (a*x + b)^n, by the binomial theorem.

$$
(ax + b)^n = \sum_{k=0}^{n} \binom{n}{k} a^k b^{n-k} x^k
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x in the binomial | — |
| `b` | in | constant of the binomial | — |
| `n` | in | the power n | — |
| `k` | in | the power of x whose coefficient is wanted | — |
| `coefficient` | out | coefficient of x^k | — |
| `binomial` | out | the binomial coefficient C(n, k) | — |

**Applies when:** 0 <= n <= 60; k >= 0.

```shell
$ math-foundations solve binomial_coefficient_term a=2 b=-1 n=5 k=3
coefficient = 80
binomial = 10
  by: Coefficient of a term in a binomial expansion  (binomial theorem: the x^k term is C(n, k) (ax)^k b^(n-k))
  a = 2
  b = -1
  assumes: 0 <= n <= 60; k >= 0
```

**What the worked example means:** This calculation determines the specific numerical factor multiplying the term with x raised to the power 3 within the binomial expansion. It applies the binomial theorem, which states that each term is formed by the product of a binomial coefficient and powers of the individual terms. The result 80 represents this combined value, derived from the input parameters where 2 is the coefficient of x, -1 is the constant term, and 5 is the total exponent. The intermediate binomial coefficient 10 reflects the number of ways to choose 3 items from 5, serving as a scaling factor for the variable parts.

<a id="vector_dot_angle"></a>

#### Dot product and angle between two vectors — `vector_dot_angle`

The dot product, the magnitudes and the angle between two vectors in the plane or in space.

$$
\mathbf u\cdot\mathbf v = |\mathbf u||\mathbf v|\cos\theta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `u` | in | the first vector, e.g. 3, 4 | — |
| `v` | in | the second vector | — |
| `dot` | out | u . v | — |
| `mag_u` | out | \|u\| | — |
| `mag_v` | out | \|v\| | — |
| `angle_deg` | out | the angle between them, degrees (0 to 180) | — |

**Applies when:** same dimension (2 or 3); neither vector is zero.

```shell
$ math-foundations solve vector_dot_angle "u=[3, 4]" "v=[4, -3]"
dot = 0
mag_u = 5
mag_v = 5
angle_deg = 90
  by: Dot product and angle between two vectors  (u . v = sum of products of components; the angle from atan2(|u x v|, u . v) (stable for nearly parallel vectors))
  assumes: same dimension (2 or 3); neither vector is zero
```

**What the worked example means:** This calculation determines the dot product, which measures how much two vectors align in direction. It also computes the magnitude of each vector, representing their length, and the angle between them. The result shows that for input vectors with values [3, 4] and [4, -3], the alignment yields a dot product of 0. The lengths are 5 and 5, resulting in an angular separation of 90. This information helps understand geometric relationships and projections.

<a id="ellipse_parameters"></a>

#### Foci, axes and eccentricity of an ellipse — `ellipse_parameters`

For (x - h)^2/A + (y - k)^2/B = 1: semi-axes a and b, focal distance c, eccentricity and the two foci.

$$
\frac{(x-h)^2}{A} + \frac{(y-k)^2}{B} = 1,\quad c^2 = a^2 - b^2,\quad e = \frac{c}{a}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | x of the centre | — |
| `k` | in | y of the centre | — |
| `A` | in | denominator under the x term (positive) | — |
| `B` | in | denominator under the y term (positive) | — |
| `a` | out | semi-major axis | — |
| `b` | out | semi-minor axis | — |
| `c` | out | centre-to-focus distance | — |
| `eccentricity` | out | e = c/a | — |
| `f1x` | out | first focus x | — |
| `f1y` | out | first focus y | — |
| `f2x` | out | second focus x | — |
| `f2y` | out | second focus y | — |
| `orientation` | out | the major axis is horizontal or vertical | — |

**Applies when:** A, B > 0; A != B (otherwise a circle).

```shell
$ math-foundations solve ellipse_parameters h=0 k=0 A=25 B=9
a = 5
b = 3
c = 4
eccentricity = 0.8
f1x = -4
f1y = 0
f2x = 4
f2y = 0
orientation = horizontal
  by: Foci, axes and eccentricity of an ellipse  (a^2 = the larger denominator, b^2 = the smaller, c^2 = a^2 - b^2; foci on the major axis)
  h = 0
  k = 0
  A = 25
  B = 9
  assumes: A, B > 0; A != B (otherwise a circle)
```

**What the worked example means:** This calculation determines the geometric properties of an ellipse from its standard equation. By comparing the denominators A and B, it identifies the larger value as the square of the semi-major axis a and the smaller as the square of the semi-minor axis b. The focal distance c is derived from the difference between these squares, yielding an eccentricity e of 0.8. The resulting foci are located at (-4, 0) and (4, 0), defining the shape's orientation and stretch relative to the center.

<a id="hyperbola_parameters"></a>

#### Foci, eccentricity and asymptotes of a hyperbola — `hyperbola_parameters`

For (x - h)^2/A - (y - k)^2/B = 1 (or the vertical form): a, b, c, eccentricity, the asymptotes' slope and the foci.

$$
\frac{(x-h)^2}{a^2} - \frac{(y-k)^2}{b^2} = 1,\quad c^2 = a^2 + b^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | x of the centre | — |
| `k` | in | y of the centre | — |
| `A` | in | denominator of the positive term | — |
| `B` | in | denominator of the negative term | — |
| `orientation` | in | horizontal (x term positive) or vertical | — |
| `a` | out | a = sqrt(A) | — |
| `b` | out | b = sqrt(B) | — |
| `c` | out | centre-to-focus distance | — |
| `eccentricity` | out | e = c/a | — |
| `asymptote_slope` | out | slope of the asymptotes (plus or minus this) | — |
| `f1x` | out | first focus x | — |
| `f1y` | out | first focus y | — |
| `f2x` | out | second focus x | — |
| `f2y` | out | second focus y | — |

**Applies when:** A, B > 0.

```shell
$ math-foundations solve hyperbola_parameters h=0 k=0 A=16 B=9 orientation=horizontal
a = 4
b = 3
c = 5
eccentricity = 1.25
asymptote_slope = 0.75
f1x = -5
f1y = 0
f2x = 5
f2y = 0
  by: Foci, eccentricity and asymptotes of a hyperbola  (c^2 = a^2 + b^2; asymptotes y - k = ±(b/a)(x - h) (horizontal) or ±(a/b) (vertical))
  h = 0
  k = 0
  A = 16
  B = 9
  orientation = horizontal
  assumes: A, B > 0
```

**What the worked example means:** This calculation extracts the geometric properties of a hyperbola from its standard equation. Taking the square root of the denominator of the positive term gives the semi-transverse axis length 4. Similarly, the semi-conjugate axis is 3. The distance from the center to each focus, known as c, is computed using the relationship that c squared equals a squared plus b squared, yielding 5. The eccentricity, which measures how stretched the curve is, is the ratio of c to a, resulting in 1.25. Finally, the slopes of the asymptotes are determined by the ratio of the axes, giving a value of 0.75 for the lines passing through the center.

<a id="parabola_focus_directrix"></a>

#### Focus and directrix of a parabola — `parabola_focus_directrix`

Vertex, focal distance p, focus and directrix of (x - h)^2 = 4p(y - k) or (y - k)^2 = 4p(x - h).

$$
(x-h)^2 = 4p(y-k):\ F = (h, k+p),\ y = k - p
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `orientation` | in | vertical axis ((x-h)^2 = m(y-k)) or horizontal | — |
| `m` | in | the coefficient m = 4p | — |
| `h` | in | x of the vertex (optional) | — |
| `k` | in | y of the vertex (optional) | — |
| `p` | out | focal distance p = m/4 (signed) | — |
| `focus_x` | out | x of the focus | — |
| `focus_y` | out | y of the focus | — |
| `directrix` | out | the directrix: y = this (vertical axis) or x = this (horizontal axis) | — |
| `vertex_x` | out | x of the vertex | — |
| `vertex_y` | out | y of the vertex | — |

**Applies when:** m != 0.

```shell
$ math-foundations solve parabola_focus_directrix orientation=vertical m=12 h=0 k=0
p = 3
focus_x = 0
focus_y = 3
directrix = -3
vertex_x = 0
vertex_y = 0
  by: Focus and directrix of a parabola  (compare with (x - h)^2 = 4p(y - k): p = m/4, focus at distance p from the vertex along the axis)
  m = 12
  h = 0
  k = 0
  orientation = vertical
  assumes: m != 0
```

**What the worked example means:** This calculation identifies the key geometric properties of a parabola given its vertex form equation. Using the coefficient 12 gives the signed focal distance p as 3. This value dictates the location of the focus, which lies at coordinates (0, 3), and defines the directrix line located at -3. The vertex remains fixed at (0, 0). These elements describe how the curve opens and reflects signals, with the orientation determined by whether the squared term involves x or y.

<a id="vector_magnitude_direction"></a>

#### Magnitude and direction of a vector — `vector_magnitude_direction`

The length of the plane vector <x, y> and its direction angle from the positive x-axis (0 to 360 degrees).

$$
|\mathbf v| = \sqrt{x^2 + y^2},\quad \theta = \operatorname{atan2}(y, x)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the x-component | — |
| `y` | in | the y-component | — |
| `magnitude` | out | \|v\| | — |
| `direction_deg` | out | the direction angle, degrees in [0, 360) | — |

**Applies when:** not the zero vector.

```shell
$ math-foundations solve vector_magnitude_direction x=-3 y=4
magnitude = 5
direction_deg = 126.8698976
  by: Magnitude and direction of a vector  (|v| = sqrt(x^2 + y^2); the angle in the quadrant of (x, y))
  x = -3
  y = 4
  assumes: not the zero vector
```

**What the worked example means:** This calculation finds the length and angle of a vector from its horizontal component -3 and vertical component 4. The magnitude 5 represents the straight-line distance from the origin, while the direction 126.87 indicates the angle measured counter-clockwise from the positive x-axis.

<a id="polar_to_rectangular"></a>

#### Polar to rectangular coordinates — `polar_to_rectangular`

The point (x, y) with polar coordinates (r, theta).

$$
x = r\cos\theta,\quad y = r\sin\theta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | the distance r from the pole | — |
| `theta` | in | the angle theta | — |
| `unit` | in | degrees or radians | — |
| `x` | out | x = r cos(theta) | — |
| `y` | out | y = r sin(theta) | — |

**Applies when:** the angle's unit stated.

```shell
$ math-foundations solve polar_to_rectangular r=4 theta=60 unit=degrees
x = 2
y = 3.464101615
  by: Polar to rectangular coordinates  (x = r cos(theta), y = r sin(theta))
  r = 4
  theta = 60
  unit = degrees
  assumes: the angle's unit stated
```

**What the worked example means:** This calculation converts polar coordinates into rectangular coordinates by projecting the radial distance onto the horizontal and vertical axes. Using the input distance 4 and the angle 60, the system computes the x-coordinate as 2 and the y-coordinate as 3.4641. These values represent the position of the point in a Cartesian grid, allowing for easier visualization and further geometric analysis.

<a id="rectangular_to_polar"></a>

#### Rectangular to polar coordinates — `rectangular_to_polar`

Polar coordinates (r, theta) of the point (x, y), with r > 0 and theta in [0, 360) degrees / [0, 2 pi).

$$
r = \sqrt{x^2+y^2},\quad \tan\theta = \frac{y}{x}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | x-coordinate | — |
| `y` | in | y-coordinate | — |
| `r` | out | r = sqrt(x^2 + y^2) | — |
| `theta_deg` | out | theta in degrees, [0, 360) | — |
| `theta_rad` | out | theta in radians, [0, 2 pi) | — |

**Applies when:** not the origin.

```shell
$ math-foundations solve rectangular_to_polar x=3 y=-3
r = 4.242640687
theta_deg = 315
theta_rad = 5.497787144
  by: Rectangular to polar coordinates  (r = sqrt(x^2 + y^2); theta in the quadrant of (x, y))
  x = 3
  y = -3
  assumes: not the origin
```

**What the worked example means:** This calculation determines the polar representation of a rectangular point. Using the input coordinates 3 and -3, it computes the radial distance from the origin as 4.24264. The angular position is found by identifying the quadrant containing the point and calculating the reference angle, resulting in 315 degrees or 5.49779 radians. This transformation provides an alternative view of location that emphasizes distance and direction rather than horizontal and vertical offsets.

### Quadratic functions (1)

<a id="vertex_form"></a>

#### Vertex form of a quadratic function — `vertex_form`

Write a*x^2 + b*x + c as a*(x - h)^2 + k, giving the vertex (h, k).

$$
a \left(t - h\right)^{2} + k = a t^{2} + b t + c
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x^2 | — |
| `b` | in | coefficient of x | — |
| `c` | in | constant term | — |
| `h` | out | x-coordinate of the vertex | — |
| `k` | out | y-coordinate of the vertex | — |

**Applies when:** a != 0.

```shell
$ math-foundations solve vertex_form a=1 b=-6 c=5
h = 3
k = -4
  by: Vertex form of a quadratic function  (completing the square)
  a = 1
  b = -6
  c = 5
  assumes: a != 0
```

**What the worked example means:** This calculation converts the quadratic function from standard form to vertex form using completing the square. The resulting vertex coordinates are 3 for the horizontal position and -4 for the vertical position, identifying the parabola's turning point.

### Rational expressions (2)

<a id="add_rational_expressions_same_denom"></a>

#### Add rational expressions with same denominator — `add_rational_expressions_same_denom`

Add two rational expressions that share a common denominator.

$$
\text{sum num} = \text{num}_{1} + \text{num}_{2},\quad \text{sum den} = \text{den}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `num1` | in | numerator of first expression | — |
| `num2` | in | numerator of second expression | — |
| `den` | in | common denominator | — |
| `sum_num` | out | numerator of the sum | — |
| `sum_den` | out | denominator of the sum | — |

**Applies when:** den != 0.

```shell
$ math-foundations solve add_rational_expressions_same_denom num1=1 den=6 num2=2
sum_num = 3
sum_den = 6
  by: Add rational expressions with same denominator  (closed form verified against the defining equations)
  num1 = 1
  den = 6
  num2 = 2
  assumes: den != 0
```

**What the worked example means:** This calculation combines two rational expressions that share a common denominator by adding their numerators together while keeping the denominator unchanged. The resulting numerator is 3, derived from the input numerators, and the resulting denominator remains 6. This process simplifies the addition of fractions in algebraic contexts, ensuring the final expression is in a single term form.

<a id="multiply_rational_expressions"></a>

#### Multiply rational expressions — `multiply_rational_expressions`

Compute the product of two rational expressions.

$$
\text{prod num} = \text{num}_{1} \text{num}_{2},\quad \text{prod den} = \text{den}_{1} \text{den}_{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `num1` | in | numerator of first expression | — |
| `den1` | in | denominator of first expression | — |
| `num2` | in | numerator of second expression | — |
| `den2` | in | denominator of second expression | — |
| `prod_num` | out | numerator of the product | — |
| `prod_den` | out | denominator of the product | — |

**Applies when:** den1 != 0; den2 != 0.

```shell
$ math-foundations solve multiply_rational_expressions num1=2 den1=3 num2=5 den2=7
prod_num = 10
prod_den = 21
  by: Multiply rational expressions  (closed form verified against the defining equations)
  num1 = 2
  den1 = 3
  num2 = 5
  den2 = 7
  assumes: den1 != 0; den2 != 0
```

**What the worked example means:** To find the product of two rational expressions, multiply their numerators together to get the new numerator, which is 10, and multiply their denominators together to get the new denominator, which is 21. This method combines the fractional parts into a single rational expression while preserving the mathematical relationship between the original terms.

### Rational functions (1)

<a id="rational_function_asymptotes"></a>

#### Asymptotes of a linear rational function — `rational_function_asymptotes`

The vertical asymptote x = va and horizontal asymptote y = ha of f(x) = (a*x + b)/(c*x + d).

$$
c v_{a} + d = 0,\quad c h_{a} = a
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x in the numerator | — |
| `b` | in | constant of the numerator | — |
| `c` | in | coefficient of x in the denominator | — |
| `d` | in | constant of the denominator | — |
| `va` | out | vertical asymptote | — |
| `ha` | out | horizontal asymptote | — |

**Applies when:** c != 0; a*d - b*c != 0.

```shell
$ math-foundations solve rational_function_asymptotes a=2 b=1 c=1 d=-3
va = 3
ha = 2
  by: Asymptotes of a linear rational function  (the denominator vanishes at x = -d/c; for large x the ratio tends to a/c (if a*d = b*c the function is constant and has no vertical asymptote))
  a = 2
  b = 1
  c = 1
  d = -3
  assumes: c != 0; a*d - b*c != 0
```

**What the worked example means:** This calculation determines the asymptotes of a linear rational function defined by coefficients for the numerator and denominator. The vertical asymptote, located at 3, represents the x-value where the denominator becomes zero and the function is undefined. The horizontal asymptote, found at 2, indicates the value that the function approaches as x grows infinitely large in either direction.

### Real numbers and algebraic expressions (1)

<a id="perimeter_of_rectangle"></a>

#### Perimeter of a rectangle — `perimeter_of_rectangle`

The perimeter p of a rectangle with width w and height h.

$$
p - 2 h = 2 w
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `w` | in | width | — |
| `h` | in | height | — |
| `p` | out | perimeter | — |

**Applies when:** w >= 0; h >= 0.

```shell
$ math-foundations solve perimeter_of_rectangle w=6 h=2
p = 16
  by: Perimeter of a rectangle  (add the four sides)
  w = 6
  h = 2
  assumes: w >= 0; h >= 0
```

**What the worked example means:** This calculation determines the total distance around a rectangle by adding all four sides together. Using the provided width of 6 and height of 2, the resulting perimeter is 16. This value represents the complete boundary length, which is essential for tasks such as determining the amount of fencing needed to enclose a rectangular area or calculating the frame size required for a picture.

### Sequences (2)

<a id="geometric_sequence"></a>

#### Term and partial sum of a geometric sequence — `geometric_sequence`

The n-th term and the sum of the first n terms of the geometric sequence with first term g1 and common ratio q (q != 1).

$$
g_{n} = g_{1} q^{n - 1},\quad s_{n} \left(1 - q\right) = g_{1} \left(1 - q^{n}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g1` | in | first term | — |
| `q` | in | common ratio | — |
| `n` | in | position of the term | — |
| `gn` | out | the n-th term | — |
| `sn` | out | the sum of the first n terms | — |

**Applies when:** q != 1; n >= 1.

```shell
$ math-foundations solve geometric_sequence n=4 g1=2 q=3
gn = 54
sn = 80
  by: Term and partial sum of a geometric sequence  (each term multiplies the previous one by q; the sum telescopes when multiplied by (1 - q))
  n = 4
  g1 = 2
  q = 3
  assumes: q != 1; n >= 1
```

**What the worked example means:** This calculation determines the specific value of the geometric sequence term located at position 4, denoted as 54, based on the starting value 2 and the common ratio 3. It also computes the cumulative total, or partial sum, of all terms from the beginning up to this position, resulting in 80. The method relies on multiplying the initial term by the ratio raised to the power of one less than the position index for the single term, while the sum utilizes a telescoping property that simplifies when multiplied by one minus the ratio.

<a id="arithmetic_sequence"></a>

#### Term and partial sum of an arithmetic sequence — `arithmetic_sequence`

The n-th term and the sum of the first n terms of the arithmetic sequence with first term a1 and common difference d.

$$
a_{n} = a_{1} + \left(n - 1\right) d,\quad 2 s_{n} = n \left(a_{1} + a_{n}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a1` | in | first term | — |
| `d` | in | common difference | — |
| `n` | in | position of the term (a positive whole number) | — |
| `an` | out | the n-th term | — |
| `sn` | out | the sum of the first n terms | — |

**Applies when:** n >= 1.

```shell
$ math-foundations solve arithmetic_sequence n=10 a1=3 d=4
an = 39
sn = 210
  by: Term and partial sum of an arithmetic sequence  (the n-th term adds d to the first term n - 1 times; the sum pairs first and last terms)
  n = 10
  a1 = 3
  d = 4
  assumes: n >= 1
```

**What the worked example means:** This calculation determines the value of a specific term in an arithmetic sequence and the total of all terms up to that point. Given a starting value of 3 and a constant step size of 4, the term located at position 10 is found by adding the step size repeatedly. The result shows that this specific term equals 39. Furthermore, by pairing the first and last terms of the sequence, the cumulative total for the first 10 positions is calculated as 210. This method effectively models scenarios involving steady linear growth or fixed increments.

### Systems of equations (1)

<a id="linear_system_2x2"></a>

#### Solution of two linear equations in two unknowns — `linear_system_2x2`

Solve a*x + b*y = c and d*x + e*y = f for x and y.

$$
a x + b y = c,\quad d x + e y = f
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | coefficient of x in the first equation | — |
| `b` | in | coefficient of y in the first equation | — |
| `c` | in | right-hand side of the first equation | — |
| `d` | in | coefficient of x in the second equation | — |
| `e` | in | coefficient of y in the second equation | — |
| `f` | in | right-hand side of the second equation | — |
| `x` | out | the value of x | — |
| `y` | out | the value of y | — |

**Applies when:** a*e - b*d != 0.

```shell
$ math-foundations solve linear_system_2x2 a=1 b=1 c=10 d=1 e=-1 f=2
x = 6
y = 4
  by: Solution of two linear equations in two unknowns  (elimination (Cramer's rule); the determinant a*e - b*d must not be zero)
  a = 1
  b = 1
  c = 10
  d = 1
  e = -1
  f = 2
  assumes: a*e - b*d != 0
```

**What the worked example means:** This calculation determines the unique values for two unknowns by solving a system of two linear equations. Using the method of elimination, it finds the intersection point where both equations are satisfied simultaneously. The resulting value for x is 6, representing the first unknown, while the value for y is 4, representing the second unknown.

### The coordinate plane and graphs (1)

<a id="midpoint_of_segment"></a>

#### Midpoint of a line segment — `midpoint_of_segment`

The midpoint (mx, my) of the segment from (x1, y1) to (x2, y2).

$$
2 m_{x} = x_{1} + x_{2},\quad 2 m_{y} = y_{1} + y_{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x1` | in | x-coordinate of the first endpoint | — |
| `y1` | in | y-coordinate of the first endpoint | — |
| `x2` | in | x-coordinate of the second endpoint | — |
| `y2` | in | y-coordinate of the second endpoint | — |
| `mx` | out | x-coordinate of the midpoint | — |
| `my` | out | y-coordinate of the midpoint | — |

```shell
$ math-foundations solve midpoint_of_segment x1=-4 y1=2 x2=0 y2=6
mx = -2
my = 4
  by: Midpoint of a line segment  (average the x-coordinates and the y-coordinates)
  x1 = -4
  y1 = 2
  x2 = 0
  y2 = 6
```

**What the worked example means:** This calculation determines the exact center of a straight line segment by averaging the coordinates of its two endpoints. The horizontal position, -2, is derived from the first endpoint's x-coordinate, -4, and the second endpoint's x-coordinate, 0. Similarly, the vertical position, 4, comes from the first endpoint's y-coordinate, 2, and the second endpoint's y-coordinate, 6. This result represents the point that divides the segment into two equal halves in a standard Cartesian plane.

### Trigonometry (13)

<a id="heron_area"></a>

#### Area of a triangle from its three sides (Heron) — `heron_area`

The area of a triangle with sides a, b, c, and its semi-perimeter s.

$$
K = \sqrt{s(s-a)(s-b)(s-c)},\quad s = \frac{a+b+c}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | side a | — |
| `b` | in | side b | — |
| `c` | in | side c | — |
| `area` | out | the area | — |
| `s` | out | the semi-perimeter (a + b + c)/2 | — |

**Applies when:** positive sides; triangle inequality.

```shell
$ math-foundations solve heron_area a=13 b=14 c=15
area = 84
s = 21
  by: Area of a triangle from its three sides (Heron)  (Heron's formula, in its numerically stable arrangement)
  a = 13
  b = 14
  c = 15
  assumes: positive sides; triangle inequality
```

**What the worked example means:** This calculation determines the area of a triangle using Heron's formula, which relies solely on the lengths of its three sides. First, the semi-perimeter s is computed as half the sum of the side lengths, yielding 21. Then, this value is used to find the area, resulting in 84. This method is particularly useful when the height or angles of the triangle are not known, allowing for direct area determination from side measurements.

<a id="deg_to_rad"></a>

#### Degrees to radians — `deg_to_rad`

Convert an angle in degrees to radians (also as a multiple of pi).

$$
\theta_{\mathrm{rad}} = \theta_{\deg}\cdot\frac{\pi}{180}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `deg` | in | the angle in degrees | — |
| `radians` | out | the angle in radians | — |
| `pi_multiple` | out | the angle as a multiple of pi | — |
| `exact` | out | exact form, e.g. 3pi/4 | — |

**Applies when:** a finite angle.

```shell
$ math-foundations solve deg_to_rad deg=135
radians = 2.35619449
pi_multiple = 0.75
exact = 3pi/4
  by: Degrees to radians  (multiply by pi/180)
  deg = 135
  assumes: a finite angle
```

**What the worked example means:** This calculation transforms an angular measurement from degrees into radians by multiplying the input value by the constant ratio of pi over one hundred and eighty. The result shows that the given angle corresponds to 2.35619 in radian measure, while also expressing it as a multiple of pi, specifically 0.75. This conversion is essential for performing trigonometric operations in calculus and physics contexts where radians are the standard unit.

<a id="unit_circle_exact"></a>

#### Exact trigonometric values on the unit circle — `unit_circle_exact`

The exact sin, cos and tan of an angle that is a multiple of 30 or 45 degrees (pi/6, pi/4), with the reference angle.

$$
\sin 30^\circ = \tfrac12,\ \sin 45^\circ = \tfrac{\sqrt2}{2},\ \sin 60^\circ = \tfrac{\sqrt3}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `deg` | in | the angle in degrees (a multiple of 30 or 45) | — |
| `sin` | out | sin of the angle | — |
| `cos` | out | cos of the angle | — |
| `tan` | out | tan of the angle (None where undefined) | — |
| `sin_exact` | out | exact sin | — |
| `cos_exact` | out | exact cos | — |
| `tan_exact` | out | exact tan | — |
| `reference_angle` | out | the reference angle, degrees | — |

**Applies when:** a whole multiple of 30 or 45 degrees.

```shell
$ math-foundations solve unit_circle_exact deg=150
sin = 0.5
cos = -0.8660254038
tan = -0.5773502692
sin_exact = 1/2
cos_exact = -sqrt(3)/2
tan_exact = -sqrt(3)/3
reference_angle = 30
  by: Exact trigonometric values on the unit circle  (the reference angle's exact values from the 30-60-90 and 45-45-90 triangles, with the signs of the quadrant (ASTC))
  deg = 150
  assumes: a whole multiple of 30 or 45 degrees
```

**What the worked example means:** This calculation determines the precise trigonometric ratios for an angle measured in degrees. It identifies the reference angle as 30 to locate the magnitude of the values using standard special right triangles. The signs of the results are then adjusted based on the quadrant in which the original angle lies, ensuring that the reported sine, cosine, and tangent values reflect both the correct magnitude and the proper algebraic sign for the given position on the unit circle.

<a id="law_of_cosines_sss"></a>

#### Law of cosines (three sides) — `law_of_cosines_sss`

Given the three sides a, b, c: the angles A, B, C.

$$
\cos A = \frac{b^2 + c^2 - a^2}{2bc}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | side a | — |
| `b` | in | side b | — |
| `c` | in | side c | — |
| `A` | out | angle A, degrees | — |
| `B` | out | angle B, degrees | — |
| `C` | out | angle C, degrees | — |

**Applies when:** positive sides; triangle inequality (each side shorter than the sum of the other two).

```shell
$ math-foundations solve law_of_cosines_sss a=7 b=8 c=9
A = 48.1896851
B = 58.41186449
C = 73.3984504
  by: Law of cosines (three sides)  (cos A = (b^2 + c^2 - a^2)/(2bc), likewise B; C = 180 - A - B)
  a = 7
  b = 8
  c = 9
  assumes: positive sides; triangle inequality (each side shorter than the sum of the other two)
```

**What the worked example means:** This calculation determines the three interior angles of a triangle based on the lengths of its three sides. Using the law of cosines, angle A is derived from side a opposite it and the adjacent sides b and c, while angle B follows a similar pattern for side b. The third angle C is then found by subtracting the sum of angles A and B from one hundred eighty degrees. For the input sides 7, 8, and 9, the resulting angles are 48.1897, 58.4119, and 73.3985 respectively, confirming the geometric shape defined by these measurements.

<a id="law_of_cosines_sas"></a>

#### Law of cosines (two sides and the included angle) — `law_of_cosines_sas`

Given sides b and c and the included angle A: the third side a and the angles B and C.

$$
a^2 = b^2 + c^2 - 2bc\cos A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | side b | — |
| `c` | in | side c | — |
| `A` | in | the included angle A, degrees | — |
| `a` | out | side a | — |
| `B` | out | angle B, degrees | — |
| `C` | out | angle C, degrees | — |

**Applies when:** b, c > 0; 0 < A < 180.

```shell
$ math-foundations solve law_of_cosines_sas b=5 c=7 A=49
a = 5.298666622
B = 45.41169387
C = 85.58830613
  by: Law of cosines (two sides and the included angle)  (a^2 = b^2 + c^2 - 2bc cos A; B from the law of cosines (no ambiguity))
  b = 5
  c = 7
  A = 49
  assumes: b, c > 0; 0 < A < 180
```

**What the worked example means:** The third side a is calculated as 5.29867 using the law of cosines from sides b (5) and c (7) with included angle A (49). The remaining angles are found to be B (45.4117) and C (85.5883), completing the triangle solution without ambiguity.

<a id="law_of_sines_aas"></a>

#### Law of sines (two angles and a side) — `law_of_sines_aas`

Given angles A and B and the side a opposite A: the third angle C and the sides b and c.

$$
\frac{a}{\sin A} = \frac{b}{\sin B} = \frac{c}{\sin C}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | angle A, degrees | — |
| `B` | in | angle B, degrees | — |
| `a` | in | side a, opposite A | — |
| `C` | out | angle C, degrees | — |
| `b` | out | side b | — |
| `c` | out | side c | — |

**Applies when:** A, B > 0; A + B < 180; a > 0.

```shell
$ math-foundations solve law_of_sines_aas A=40 B=60 a=10
C = 80
b = 13.47296355
c = 15.32088886
  by: Law of sines (two angles and a side)  (C = 180 - A - B; law of sines a/sin A = b/sin B = c/sin C)
  A = 40
  B = 60
  a = 10
  assumes: A, B > 0; A + B < 180; a > 0
```

**What the worked example means:** This calculation determines the missing parts of a triangle using the Law of Sines. Given angle A as 40 degrees, angle B as 60 degrees, and side a as 10, angle C is found first by subtracting the known angles from one hundred eighty degrees, resulting in 80 degrees. Then, using the ratio of side to sine of its opposite angle, side b is 13.473 and side c is 15.3209. These values complete the description of the triangle.

<a id="law_of_sines_ssa"></a>

#### Law of sines, the ambiguous case (two sides and a non-included angle) — `law_of_sines_ssa`

Given sides a and b and the angle A opposite a: how many triangles exist (0, 1 or 2) and every one of them.

$$
\sin B = \frac{b\sin A}{a},\quad B_2 = 180^\circ - B_1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | side a, opposite A | — |
| `b` | in | side b | — |
| `A` | in | angle A, degrees | — |
| `count` | out | number of triangles (1 or 2; none is refused) | — |
| `B1` | out | angle B of the first triangle (the acute one) | — |
| `C1` | out | angle C of the first triangle | — |
| `c1` | out | side c of the first triangle | — |
| `B2` | out | angle B of the second triangle (obtuse), when there are two | — |
| `C2` | out | angle C of the second triangle | — |
| `c2` | out | side c of the second triangle | — |

**Applies when:** a, b > 0; 0 < A < 180; a triangle exists (else refused).

```shell
$ math-foundations solve law_of_sines_ssa a=8 b=10 A=40
count = 2
B1 = 53.46414901
C1 = 86.53585099
c1 = 12.42304969
B2 = 126.535851
C2 = 13.46414901
c2 = 2.897839169
  by: Law of sines, the ambiguous case (two sides and a non-included angle)  (sin B = b sin A / a; B and 180 - B both give a triangle when A + B < 180 (the ambiguous case))
  a = 8
  b = 10
  A = 40
  assumes: a, b > 0; 0 < A < 180; a triangle exists (else refused)
```

**What the worked example means:** This routine resolves the ambiguous case for two sides and a non-included angle. Given side a as 8, side b as 10, and angle A as 40, it determines if zero, one, or two triangles exist. The primary solution produces angle B of 53.4641 and side c of 12.423. If a second valid configuration exists, it yields an obtuse angle B of 126.536 and corresponding side c of 2.89784. The number of valid triangles is reported as 2.

<a id="rad_to_deg"></a>

#### Radians to degrees — `rad_to_deg`

Convert an angle in radians (a number, or a multiple of pi such as 3pi/4) to degrees.

$$
\theta_{\deg} = \theta_{\mathrm{rad}}\cdot\frac{180}{\pi}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rad` | in | the angle in radians | — |
| `degrees` | out | the angle in degrees | — |

**Applies when:** a finite angle.

```shell
$ math-foundations solve rad_to_deg rad=2.35619449
degrees = 135
  by: Radians to degrees  (multiply by 180/pi)
  rad = 2.35619449
  assumes: a finite angle
```

**What the worked example means:** This calculation transforms an angle measured in radians into its equivalent value in degrees. By multiplying the input radian measure by the conversion factor of one hundred eighty divided by pi, the result expresses the same angular magnitude using the degree system. The output of 135 represents how many full turns would be fractionally completed if the circle were divided into three hundred sixty equal parts instead of two pi equal parts.

<a id="right_triangle_leg_hypotenuse"></a>

#### Right triangle from a leg and the hypotenuse — `right_triangle_leg_hypotenuse`

Given a leg a and the hypotenuse c of a right triangle: the other leg b and the acute angles A (opposite a) and B.

$$
b = \sqrt{c^2 - a^2},\quad \sin A = \frac{a}{c}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | the known leg (positive) | — |
| `c` | in | the hypotenuse (longer than the leg) | — |
| `b` | out | the other leg | — |
| `A` | out | angle opposite a, degrees | — |
| `B` | out | angle opposite b, degrees | — |

**Applies when:** 0 < a < c.

```shell
$ math-foundations solve right_triangle_leg_hypotenuse a=5 c=13
b = 12
A = 22.61986495
B = 67.38013505
  by: Right triangle from a leg and the hypotenuse  (b = sqrt(c^2 - a^2); sin A = a/c; B = 90 - A)
  a = 5
  c = 13
  assumes: 0 < a < c
```

**What the worked example means:** This calculation finds the missing side and angles of a right triangle using one known leg, 5, and the hypotenuse, 13. The remaining leg, 12, is derived from the Pythagorean relationship. Angle A, opposite the known leg, is found via sine, while angle B, opposite the calculated leg, completes the acute angles. The result shows that for these inputs, the triangle has sides 5 and 12 with hypotenuse 13, and angles 22.6199 and 67.3801.

<a id="right_triangle_angle_side"></a>

#### Right triangle from an acute angle and a side — `right_triangle_angle_side`

Given an acute angle A and one side (the hypotenuse, the side opposite A or the side adjacent to A): every side and the other angle.

$$
\sin A = \frac{\mathrm{opp}}{\mathrm{hyp}},\ \cos A = \frac{\mathrm{adj}}{\mathrm{hyp}},\ \tan A = \frac{\mathrm{opp}}{\mathrm{adj}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | the acute angle A in degrees | — |
| `side` | in | the length of the known side | — |
| `which` | in | which side is known | — |
| `opposite` | out | side opposite A | — |
| `adjacent` | out | side adjacent to A | — |
| `hypotenuse` | out | the hypotenuse | — |
| `B` | out | the other acute angle, degrees | — |

**Applies when:** 0 < A < 90; side > 0.

```shell
$ math-foundations solve right_triangle_angle_side A=30 side=10 which=hypotenuse
opposite = 5
adjacent = 8.660254038
hypotenuse = 10
B = 60
  by: Right triangle from an acute angle and a side  (SOH-CAH-TOA: sin A = opp/hyp, cos A = adj/hyp, tan A = opp/adj)
  A = 30
  side = 10
  which = hypotenuse
  assumes: 0 < A < 90; side > 0
```

**What the worked example means:** This calculation determines all remaining sides and angles of a right triangle using the known acute angle 30 and one specific side length 10. By applying trigonometric ratios, it computes the side opposite the given angle as 5, the side adjacent to it as 8.66025, and the hypotenuse as 10. The second acute angle is found to be 60. This process expands partial geometric information into a complete description of the triangle's dimensions.

<a id="right_triangle_legs"></a>

#### Right triangle from its two legs — `right_triangle_legs`

Given the legs a and b of a right triangle: the hypotenuse c and the acute angles A (opposite a) and B.

$$
c = \sqrt{a^2 + b^2},\quad \tan A = \frac{a}{b}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | the leg opposite angle A (positive) | — |
| `b` | in | the leg opposite angle B (positive) | — |
| `c` | out | the hypotenuse | — |
| `A` | out | angle opposite a, degrees | — |
| `B` | out | angle opposite b, degrees | — |

**Applies when:** a > 0; b > 0.

```shell
$ math-foundations solve right_triangle_legs a=3 b=4
c = 5
A = 36.86989765
B = 53.13010235
  by: Right triangle from its two legs  (Pythagoras c = sqrt(a^2 + b^2); tan A = a/b; B = 90 - A)
  a = 3
  b = 4
  assumes: a > 0; b > 0
```

**What the worked example means:** Given the leg lengths 3 and 4, this calculation finds the hypotenuse 5 using the Pythagorean theorem. It also determines the angle opposite the first leg, 36.8699, and the remaining acute angle, 53.1301. This fully defines the geometry of the right triangle from its two perpendicular sides.

<a id="trig_equation_interval"></a>

#### Solve a trigonometric equation over one full turn — `trig_equation_interval`

Every solution of a*sin(n x) = k (or cos, tan) with 0 <= x < 360 degrees or 0 <= x < 2 pi.

$$
\sin u = k \Rightarrow u = \arcsin k,\ 180^\circ - \arcsin k \pmod{360^\circ},\quad u = n x
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `func` | in | sin, cos or tan | — |
| `k` | in | the value the function must take (after dividing by its coefficient) | — |
| `n` | in | the multiple n in f(n x) (default 1) (optional) | — |
| `unit` | in | degrees ([0, 360)) or radians ([0, 2pi)) (optional) | — |
| `count` | out | number of solutions in the interval | — |
| `solutions` | out | the solutions, ascending | — |

**Applies when:** n a whole number from 1 to 12.

```shell
$ math-foundations solve trig_equation_interval func=sin k=0.5 n=1 unit=degrees
count = 2
solutions = [30.000000000000004,150.0]
  by: Solve a trigonometric equation over one full turn  (principal value (asin, acos or atan), the second solution of the period (180 - a, -a, a + 180), then every period: x = (u + 360 m)/n inside the interval)
  k = 0.5
  func = sin
  unit = degrees
  assumes: n a whole number from 1 to 12
```

**What the worked example means:** This calculation determines all angles that satisfy the trigonometric relationship for the specified function and value. By applying the principal value method and accounting for the period of the function, it identifies every solution between zero and the full turn limit. The result shows that there are 2 distinct valid angles in this range, listed in ascending order as [30, 150]. This approach ensures that no periodic solutions are missed while respecting the constraints of the input parameters.

<a id="trig_values_from_one"></a>

#### Trigonometric values from one value and the quadrant — `trig_values_from_one`

Given sin, cos or tan of an angle and its quadrant: sin, cos, tan, sin 2t and cos 2t (Pythagorean and double-angle identities).

$$
\cos t = \pm\sqrt{1 - \sin^2 t},\quad \sin 2t = 2\sin t\cos t,\quad \cos 2t = \cos^2 t - \sin^2 t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `func` | in | the function whose value is known | — |
| `value` | in | its value | — |
| `quadrant` | in | the quadrant of the angle (1 to 4) | — |
| `sin` | out | sin t | — |
| `cos` | out | cos t | — |
| `tan` | out | tan t | — |
| `sin2` | out | sin 2t | — |
| `cos2` | out | cos 2t | — |

**Applies when:** the value's sign agrees with the quadrant; 0 < |sin|, |cos| < 1.

```shell
$ math-foundations solve trig_values_from_one func=sin value=0.6 quadrant=2
sin = 0.6
cos = -0.8
tan = -0.75
sin2 = -0.96
cos2 = 0.28
  by: Trigonometric values from one value and the quadrant  (sin^2 + cos^2 = 1 with the quadrant's signs (ASTC); sin 2t = 2 sin t cos t; cos 2t = cos^2 t - sin^2 t)
  value = 0.6
  func = sin
  assumes: the value's sign agrees with the quadrant; 0 < |sin|, |cos| < 1
```

**What the worked example means:** This calculation derives the sine, cosine, tangent, and double-angle values for an angle based on a single known trigonometric function value and its quadrant. By applying the Pythagorean identity and sign conventions specific to the quadrant, the system computes the remaining fundamental ratios. The resulting value of -0.8 reflects the horizontal component consistent with the given information, while -0.75 represents the ratio of vertical to horizontal displacement. Double-angle results such as -0.96 are then generated using standard identities, ensuring all outputs align with the initial constraint that the magnitude of sine and cosine lies strictly between zero and one.

Questions and reports: support@ai999x.com. AI999X — The Future Is Self-Improving.
