# Stein's greatest common divisor algorithm study
Simple dive into Stein's greatest common divisor (GCD) algorithm, also known as Binary GCD algorithm, created by Josef Stein in 1967.

- See [Wikipedia export](./stein-gcd-analysis.pdf) for analysis.
- Stein's GCD extension with Bezout's coefficients is taken from [Joris Barkema's Bachelor Thesis](./extending-stein-gcd-by-joris-barkema.pdf).


```rust
let gcd = gcd_naive(a,b);
let gcd = gcd_naive_2(a,b);
let gcd = gcd_disingenuous(a,b);
let gcd = gcd_disingenuous_2(a,b);
let gcd = gcd_nonnaive_extended(a, b);
let gcd = gcd_nonnaive_extended_2(a, b);
```

## Simple Performance Comparison

Orientative benchmark sample, no extended statistical measurement applied. 

<small>
Configuration:
<ul>
<li>Operating System: openSUSE Leap 16.0</li>
<li>Kernel Version: 6.12.0-160000.28-default (64-bit)</li>
<li>Processors: 16 × AMD Ryzen 7 3800X 8-Core Processor</li>
</ul>
</small>

It was observed that _gcd_nonnaive_extended*_ tests reach usually worst times when
run both in one benchmark run. As seen in _deviation_ column these tests are subject
to high change, their high deviation can be seen prevalently in all benchmark runs. Under
some conditions these tests can reach times similar to Euclidean GCD but never better.

```rust
#[bench]
fn extended_test(b: &mut Bencher) {
    let num_1 = 2_559_031_471; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ
    let proof = 1;

    b.iter(|| {
        let gcd = method(num_1, num_2);
        assert_eq!(proof, gcd);
    });
}
```

It can be assumed that high devition is due failing branch predicting in methods having complicated
conditional branching which all `gcd_naive_2`, `gcd_nonnaive_extended_2`, `gcd_nonnaive_extended` have.


|         Method          |                Description                |   Mean   | Deviation |
|-------------------------|-------------------------------------------|----------|-----------|
| gcd_disingenuous_2      | Optimized iterative Stein's GCD           | 19.24 ns | ± 0.10    |
| gcd_disingenuous        | Iterative Stein's GCD                     | 35.67 ns | ± 0.24    |
| gcd_e                   | Euclidean GCD                             | 54.95 ns | ± 0.49    |
| gcd_ee                  | Extended Euclidean GCD                    | 55.15 ns | ± 0.93    |
| gcd_naive_2             | Still recursive but optmized              | 47.37 ns | ± 4.61    |
| gcd_naive               | Very naive implementation                 | 66.11 ns | ± 0.83    |
| gcd_nonnaive_extended_2 | Extended optimized performant Stein's GCD | 72.58 ns | ± 5.40    |
| gcd_nonnaive_extended   | Extended optimized Stein's GCD            | 75.18 ns | ± 6.26    |
