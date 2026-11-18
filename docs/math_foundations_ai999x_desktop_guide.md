# Math Foundations: the desktop app, user guide

**Math Foundations**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Math Foundations covers arithmetic, fractions, percentages, ratios, algebraic equations and inequalities, functions, exponentials and logarithms, trigonometry, precalculus and complex numbers. Its desktop app holds all 106 calculations in 34 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-math-foundations-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-math-foundations-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-math-foundations-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-math-foundations.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-math-foundations-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-math-foundations-1.6.5-linux-x86_64.tar.gz
./ai999x-math-foundations-1.6.5-linux-x86_64/ai999x-math-foundations
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Math Foundations app, opened on Roots of a quadratic equation with its worked example.](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/hero.webp)

*The Math Foundations app, opened on Roots of a quadratic equation with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Polynomial equations › Roots of a quadratic equation (or type part of its name in the search box). It opens with a worked example filled in: “Solve x^2 - 5x + 6 = 0.” The answer: x1 = 2, x2 = 3, discriminant = 1.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Roots of a quadratic equation: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/scene-1.webp)

*Roots of a quadratic equation: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Solve x^2 - 5x + 6 = 0.” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Roots of a quadratic equation
x1 = 2
x2 = 3
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Law of sines, the ambiguous case (two sides and a non-included angle)](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/scene-2.webp)

*Law of sines, the ambiguous case (two sides and a non-included angle)*

![Arithmetic with complex numbers](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/scene-3.webp)

*Arithmetic with complex numbers*

![Coefficient of a term in a binomial expansion](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/scene-4.webp)

*Coefficient of a term in a binomial expansion*

![Absolute value inequality |ax + b| compared with c](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/scene-5.webp)

*Absolute value inequality |ax + b| compared with c*

![Arithmetic with fractions](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/scene-6.webp)

*Arithmetic with fractions*

![Centre and radius of a circle from its general equation](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/scene-7.webp)

*Centre and radius of a circle from its general equation*

![Distance between two points](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/scene-8.webp)

*Distance between two points*

![Roots of a quadratic equation: a chart, enlarged.](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/chart-1.webp)

*Roots of a quadratic equation: a chart, enlarged.*

![Law of sines, the ambiguous case (two sides and a non-included angle): a chart, enlarged.](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/chart-2.webp)

*Law of sines, the ambiguous case (two sides and a non-included angle): a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Math Foundations app in the dark theme.](https://www.ai999x.com/?ax_shot=math-foundations-ai999x/hero-dark.webp)

*The Math Foundations app in the dark theme.*

## 7. Every calculation

106 calculations in 34 topics; each has its own entry on the model's page (https://www.ai999x.com/models/math-foundations-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Algebra | Absolute value inequality \|ax + b\| compared with c, Factor a quadratic over the integers, Partial fractions (two distinct linear factors), Polynomial long division, Quadratic inequality, Radical equation sqrt(ax + b) = cx + d, Rational equation a/(x + b) = c/(x + d), Rational roots of a polynomial (rational root theorem), Sum of an infinite geometric series, Synthetic division and the remainder theorem, System of three linear equations |
| Arithmetic | Arithmetic with fractions, Divide a quantity in a given ratio, Evaluate an arithmetic expression (order of operations), Percent change (increase or decrease), Percent of a number, Round to decimal places, Round to significant figures, Scientific notation, Solve a proportion, Unit rates and the better buy, What percent one number is of another, Whole from a part and its percentage |
| Complex numbers | Arithmetic with complex numbers, Modulus, argument and polar form of a complex number, Power of a complex number (De Moivre), Product and modulus of complex numbers, The n-th roots of a complex number |
| Conic sections | Centre and radius of a circle from its general equation |
| Coordinate plane | Distance between two points |
| Counting and probability | Combinations and permutations |
| Exponential functions | Compound interest |
| Exponential models | Doubling time of exponential growth |
| Exponentials and logarithms | Amount remaining after radioactive decay (half-life), Continuous exponential growth or decay, Doubling time at a percentage growth rate, Evaluate a logarithm, Exponential equation a*b^(cx + d) = k, Laws of logarithms (product, quotient, power), Logarithmic equation log_b(cx + d) = k |
| Exponents and logarithms | Solution of an exponential equation |
| Factoring | Factoring a monic quadratic |
| Functions | Average rate of change of a function between two inputs, Composition of two linear functions, Direct, inverse, joint and combined variation, Distance from a point to a line, Domain of a square-root function, Evaluate a function at a number, Evaluate a piecewise function, Image of a point under a graph transformation, Intercepts and slope of a line, Inverse of a linear function, Value of a composite function |
| Inequalities | Solution of a linear inequality |
| Linear and absolute value inequalities | Solution interval of \|x - c\| < d, Solution set of \|x - c\| > d |
| Linear equations | Root of a linear equation |
| Linear functions | Average rate of change between two points, Line through two points, Slope of a perpendicular line, y-intercept of a line from its slope and a point |
| Logarithms | Solution of a basic logarithmic equation |
| Matrices | Determinant and inverse of a 2 by 2 matrix |
| Matrices and linear systems | Determinant of 2x2 matrix, Matrix scalar multiple definition |
| Measurement | Unit conversion (metric prefixes, customary units, temperature scales) |
| Other equations | Solutions of an absolute value equation |
| Polynomial and rational functions | Analysis of a rational function (domain, intercepts, holes, asymptotes) |
| Polynomial equations | Discriminant of a quadratic, Roots of a cubic equation, Roots of a quadratic equation, Roots of a quartic equation |
| Polynomials | Product of two binomials |
| Precalculus | Coefficient of a term in a binomial expansion, Dot product and angle between two vectors, Foci, axes and eccentricity of an ellipse, Foci, eccentricity and asymptotes of a hyperbola, Focus and directrix of a parabola, Magnitude and direction of a vector, Polar to rectangular coordinates, Rectangular to polar coordinates |
| Quadratic functions | Vertex form of a quadratic function |
| Rational expressions | Add rational expressions with same denominator, Multiply rational expressions |
| Rational functions | Asymptotes of a linear rational function |
| Real numbers and algebraic expressions | Perimeter of a rectangle |
| Sequences | Term and partial sum of a geometric sequence, Term and partial sum of an arithmetic sequence |
| Sequences and series | Binomial series for any real power |
| Systems of equations | Intersection of a line and a conic (nonlinear system), Intersection of two conics (nonlinear system), Solution of two linear equations in two unknowns |
| The coordinate plane and graphs | Midpoint of a line segment |
| Trigonometry | Area of a triangle from its three sides (Heron), Degrees to radians, Exact trigonometric values on the unit circle, Law of cosines (three sides), Law of cosines (two sides and the included angle), Law of sines (two angles and a side), Law of sines, the ambiguous case (two sides and a non-included angle), Radians to degrees, Right triangle from a leg and the hypotenuse, Right triangle from an acute angle and a side, Right triangle from its two legs, Solve a trigonometric equation over one full turn, Trigonometric values from one value and the quadrant |

## 8. Other ways to use it

The same engine runs on the command line (`math-foundations`, its own guide), in Python (`pip install math-foundations-ai999x`, `import math_foundations_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/math-foundations-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/math-foundations-ai999x/
