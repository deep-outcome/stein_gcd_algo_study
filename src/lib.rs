#![allow(dead_code)]

//! Algorithms are based on these 4 identities:
//! 1. gcd(u,0) =u
//! 2. gcd(2u,2v) =2 ⋅gcd(u,v)
//! 3. gcd(u,2v) =gcd(u,v), u ∤2
//! 4. gcd(u,v) =gcd(u,v-u), u ∤2, v ∤2, u ≤v
//!
//! Note: GCD is commutative, gcd(u,v) =gcd(v,u).
//!
//! Extended algorithm uses <a href="../../../extending-stein-gcd-by-joris-barkema.pdf">Joris Barkema's Bachelor Thesis</a> proofs and analyses.

/// naive recursive implementation with overhead
/// due one-step-at-a-time recursion
pub fn gcd_naive(mut l_hand: u64, mut r_hand: u64) -> u64 {
    // ident #1
    if l_hand == 0 {
        return r_hand;
    }

    // ident #1
    if r_hand == 0 {
        return l_hand;
    }

    // ident #2 and ident #3
    let lhand_even = l_hand & 1 == 0;
    if lhand_even {
        l_hand >>= 1;
    }

    // ident #2 and ident #3
    let rhand_even = r_hand & 1 == 0;
    if rhand_even {
        r_hand >>= 1;
    }

    return if lhand_even || rhand_even {
        // ident #2 and ident #3
        let gcd = gcd_naive(l_hand, r_hand);
        if lhand_even & rhand_even {
            gcd << 1
        } else {
            gcd
        }
    } else {
        // ident #4
        if l_hand < r_hand {
            let swap = l_hand;
            l_hand = r_hand;
            r_hand = swap;
        }

        l_hand -= r_hand;
        l_hand >>= 1;

        gcd_naive(r_hand, l_hand)
    };
}

/// naive recursive implementation with overhead
/// due one-step-at-a-time recursion
pub fn gcd_naive_2(mut l_hand: u64, mut r_hand: u64) -> u64 {
    // ident #1
    if l_hand == 0 {
        return r_hand;
    }

    // ident #1
    if r_hand == 0 {
        return l_hand;
    }

    // ident #3
    if l_hand & 1 == 0 {
        l_hand >>= 1;

        // ident #2
        return if r_hand & 1 == 0 {
            r_hand >>= 1;
            gcd_naive_2(l_hand, r_hand) << 1
        } else {
            // ident #3
            gcd_naive_2(l_hand, r_hand)
        };
    }

    // ident #3
    if r_hand & 1 == 0 {
        r_hand >>= 1;

        return gcd_naive_2(l_hand, r_hand);
    }

    // ident #4
    return if l_hand > r_hand {
        l_hand -= r_hand;
        l_hand >>= 1;
        gcd_naive_2(l_hand, r_hand)
    } else {
        r_hand -= l_hand;
        r_hand >>= 1;
        gcd_naive_2(l_hand, r_hand)
    };
}

/// optimized iterative implementation which uses
/// exhaustive-like precomputations
pub fn gcd_disingenuous(mut l_hand: u64, mut r_hand: u64) -> u64 {
    // ident #1
    if l_hand == 0 {
        return r_hand;
    }

    // ident #1
    if r_hand == 0 {
        return l_hand;
    }

    // ident #2 and ident #3
    // gcd(2ⁱ ⋅u,2ʲ ⋅v) = 2ᵏ ⋅gcd(u,v), k = min(i, j)
    let lhand_pow = l_hand.trailing_zeros();
    let rhand_pow = r_hand.trailing_zeros();

    l_hand >>= lhand_pow;
    r_hand >>= rhand_pow;
    let mpler = lhand_pow.min(rhand_pow);

    // both hands odd here
    loop {
        if l_hand < r_hand {
            let swap = l_hand;
            l_hand = r_hand;
            r_hand = swap;
        }

        // ident #4
        l_hand -= r_hand;
        // l_hand non-odd here

        // ident #1
        if l_hand == 0 {
            // ident #2
            return r_hand << mpler;
        }

        // ident #3
        // l_hand even here
        l_hand >>= l_hand.trailing_zeros();
    }
}

/// optimized iterative implementation which uses
/// exhaustive-like precomputations
pub fn gcd_disingenuous_2(mut l_hand: u64, mut r_hand: u64) -> u64 {
    // ident #1
    if l_hand == 0 {
        return r_hand;
    }

    // ident #1
    if r_hand == 0 {
        return l_hand;
    }

    // ident #2 and ident #3
    // gcd(2ⁱ ⋅u,2ʲ ⋅v) = 2ᵏ ⋅gcd(u,v), k = min(i, j)
    let lhand_pow = l_hand.trailing_zeros();
    let rhand_pow = r_hand.trailing_zeros();

    l_hand >>= lhand_pow;
    r_hand >>= rhand_pow;
    let mpler = lhand_pow.min(rhand_pow);

    // both hands odd here
    loop {
        if l_hand > r_hand {
            // ident #4
            l_hand -= r_hand;
            // l_hand non-odd here

            // ident #1
            if l_hand == 0 {
                // ident #2
                return r_hand << mpler;
            }

            // ident #3
            // l_hand even here
            l_hand >>= l_hand.trailing_zeros();
        } else {
            // ident #4
            r_hand -= l_hand;
            // r_hand non-odd here

            // ident #1
            if r_hand == 0 {
                // ident #2
                return l_hand << mpler;
            }

            // ident #3
            // r_hand even here
            r_hand >>= r_hand.trailing_zeros();
        }
    }
}

/// extended algorithm result
pub struct ExtRes(pub i64, i64, i64);

impl ExtRes {
    pub fn coeffs(&self) -> (i128, i128) {
        let x = self.1 as i128;
        let y = self.2 as i128;

        (x, y)
    }
}

/// optimized extended gcd algorithm
pub fn gcd_nonnaive_extended(mut l_hand: i64, mut r_hand: i64) -> ExtRes {
    // ident #1
    if l_hand == 0 {
        return ExtRes(r_hand, 0, 1);
    }

    // ident #1
    if r_hand == 0 {
        return ExtRes(l_hand, 1, 0);
    }

    // ident #2 and ident #3
    // gcd(2ⁱ ⋅u,2ʲ ⋅v) = 2ᵏ ⋅gcd(u,v), k = min(i, j)
    let lhand_pow = l_hand.trailing_zeros();
    let rhand_pow = r_hand.trailing_zeros();

    let mpler = lhand_pow.min(rhand_pow);
    l_hand >>= mpler;
    r_hand >>= mpler;

    // lx ·l +ly ·r =l_hand
    // (lx −r) ·l +(ly +l) ·r =lx ·l −l ·r +ly ·r + l ·r = lx ·l + ly ·r
    let l = l_hand as i64;
    // rx ·l +ry ·r =r_hand
    // (rx −r) ·l +(ry +l) ·r =rx ·l −l ·r +ry ·r + l ·r = rx ·l + ry ·r
    let r = r_hand as i64;

    let mut lx: i64 = 1;
    let mut ly: i64 = 0;

    let mut rx: i64 = 0;
    let mut ry: i64 = 1;

    loop {
        // #ident 3
        while l_hand & 1 == 0 {
            if lx & 1 == 1 || ly & 1 == 1 {
                lx -= r;
                ly += l;
            }

            // lx ·l +ly ·r =l_hand
            lx >>= 1;
            ly >>= 1;
            l_hand >>= 1;
        }

        // #ident 3
        while r_hand & 1 == 0 {
            if rx & 1 == 1 || ry & 1 == 1 {
                rx -= r;
                ry += l;
            }

            // rx ·l +ry ·r =r_hand
            rx >>= 1;
            ry >>= 1;
            r_hand >>= 1;
        }

        if l_hand > r_hand {
            // ident #4
            l_hand -= r_hand;
            // l_hand non-odd here

            // ident #1
            if l_hand == 0 {
                // ident #2
                return ExtRes(r_hand << mpler, rx, ry);
            }

            // (lx ·l + ly ·r) −(rx ·l +ry ·r) = l_hand −r_hand
            // (lx -rx) ·l +(ly -ry) ·r − = l_hand −r_hand
            lx -= rx;
            ly -= ry;
        } else {
            // ident #4
            r_hand -= l_hand;
            // r_hand non-odd here

            // ident #1
            if r_hand == 0 {
                // ident #2
                return ExtRes(l_hand << mpler, lx, ly);
            }

            // (rx ·l +ry ·r) -(lx ·l +ly ·r) = r_hand −l_hand
            // (rx -lx) ·l +(ry -ly) ·r − = r_hand −l_hand
            rx -= lx;
            ry -= ly;
        }
    }
}

/// optimized extended gcd algorithm
pub fn gcd_nonnaive_extended_2(mut l_hand: i64, mut r_hand: i64) -> ExtRes {
    // ident #1
    if l_hand == 0 {
        return ExtRes(r_hand, 0, 1);
    }

    // ident #1
    if r_hand == 0 {
        return ExtRes(l_hand, 1, 0);
    }

    // ident #2 and ident #3
    // gcd(2ⁱ ⋅u,2ʲ ⋅v) = 2ᵏ ⋅gcd(u,v), k = min(i, j)
    let lhand_pow = l_hand.trailing_zeros();
    let rhand_pow = r_hand.trailing_zeros();

    let mpler = lhand_pow.min(rhand_pow);
    l_hand >>= mpler;
    r_hand >>= mpler;

    // lx ·l +ly ·r =l_hand
    // (lx −r) ·l +(ly +l) ·r =lx ·l −l ·r +ly ·r + l ·r = lx ·l + ly ·r
    let l = l_hand as i64;
    // rx ·l +ry ·r =r_hand
    // (rx −r) ·l +(ry +l) ·r =rx ·l −l ·r +ry ·r + l ·r = rx ·l + ry ·r
    let r = r_hand as i64;

    let mut lx: i64 = 1;
    let mut ly: i64 = 0;

    let mut rx: i64 = 0;
    let mut ry: i64 = 1;

    while l_hand & 1 == 0 {
        if lx & 1 == 1 || ly & 1 == 1 {
            lx -= r;
            ly += l;
        }

        // lx ·l +ly ·r =l_hand
        lx >>= 1;
        ly >>= 1;
        l_hand >>= 1;
    }

    // #ident 1
    while r_hand != 0 {
        // #ident 3
        while r_hand & 1 == 0 {
            if rx & 1 == 1 || ry & 1 == 1 {
                rx -= r;
                ry += l;
            }

            // rx ·l +ry ·r =r_hand
            rx >>= 1;
            ry >>= 1;
            r_hand >>= 1;
        }

        if l_hand > r_hand {
            let mut swap = l_hand;
            l_hand = r_hand;
            r_hand = swap;

            swap = lx;
            lx = rx;
            rx = swap;

            swap = ly;
            ly = ry;
            ry = swap;
        }

        // ident #4
        r_hand -= l_hand;
        // l_hand non-odd here

        // (rx ·l +ry ·r) -(lx ·l +ly ·r) = r_hand −l_hand
        // (rx -lx) ·l +(ry -ly) ·r − = r_hand −l_hand
        rx -= lx;
        ry -= ly;
    }

    // #ident 2
    return ExtRes(l_hand << mpler, lx, ly);
}

#[cfg(test)]
mod tests_of_units {
    use super::{gcd_disingenuous, gcd_disingenuous_2, gcd_naive, gcd_naive_2};

    #[test]
    fn both_zero() {
        let zero = 0;
        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            assert_eq!(zero, f(zero, zero));
        }
    }

    #[test]
    fn left_hand_zero() {
        let l_hand = 0;
        let r_hand = 15;

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            assert_eq!(r_hand, f(l_hand, r_hand));
        }
    }

    #[test]
    fn right_hand_zero() {
        let l_hand = 15;
        let r_hand = 0;

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            assert_eq!(l_hand, f(l_hand, r_hand));
        }
    }

    #[test]
    fn coprime_primes_test() {
        // both prime numbers
        let num_1 = 1_299_709;
        let num_2 = 56_999;

        let proof = 1;

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            for duo in [(num_1, num_2), (num_2, num_1)] {
                let gcd = f(duo.0, duo.1);

                assert_eq!(proof, gcd);
            }
        }
    }

    #[test]
    fn coprime_odd_test() {
        let num_1 = 2_559_031_471u64; // 150531263ᵖ ⋅17ᵖ
        let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ
        let proof = 1;

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            for duo in [(num_1, num_2), (num_2, num_1)] {
                let gcd = f(duo.0, duo.1);

                assert_eq!(proof, gcd);
            }
        }
    }

    #[test]
    fn coprime_mixed_a_test() {
        let num_1 = 52_685_751_650_u64; // 150530719ᵖ ⋅350ᶜ
        let num_2 = 52_535_230_703_u64; // 150530747ᵖ ⋅349ᵖ
        let proof = 1;

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            for duo in [(num_1, num_2), (num_2, num_1)] {
                let gcd = f(duo.0, duo.1);

                assert_eq!(proof, gcd);
            }
        }
    }

    #[test]
    fn coprime_mixed_b_test() {
        let num_1 = 19_209_934_347_u64; // 56666473ᵖ ⋅113ᵖ ⋅3ᵖ
        let num_2 = 10_993_312_058_u64; // 56666557ᵖ ⋅2ᵖ ⋅97ᵖ
        let proof = 1;

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            for duo in [(num_1, num_2), (num_2, num_1)] {
                let gcd = f(duo.0, duo.1);

                assert_eq!(proof, gcd);
            }
        }
    }

    #[test]
    fn not_coprime_mixed_a_test() {
        let num_1 = 37_683_426; // 570961ᵖ ⋅66ᶜ
        let num_2 = 18_804_423; // 569831ᵖ ⋅33ᶜ
        let proof = 33;

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            for duo in [(num_1, num_2), (num_2, num_1)] {
                let gcd = f(duo.0, duo.1);

                assert_eq!(proof, gcd);
            }
        }
    }

    #[test]
    fn not_coprime_mixed_b_test() {
        let num_1 = 1_822_623; // 5021ᵖ ⋅33ᶜ ⋅11ᵖ
        let num_2 = 1_650_990; // 5003ᵖ ⋅10ᶜ ⋅33ᶜ
        let proof = 33;

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            for duo in [(num_1, num_2), (num_2, num_1)] {
                let gcd = f(duo.0, duo.1);

                assert_eq!(proof, gcd);
            }
        }
    }

    #[test]
    fn not_coprime_extra_test() {
        let num_1 = 55_286_231; // 5021ᵖ ⋅77ᶜ ⋅11ᵖ ⋅13ᵖ
        let num_2 = 7_704_620; // 5003ᵖ ⋅10ᶜ ⋅154ᶜ
        let proof = 77;

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            for duo in [(num_1, num_2), (num_2, num_1)] {
                let gcd = f(duo.0, duo.1);

                assert_eq!(proof, gcd);
            }
        }
    }

    #[test]
    fn not_coprime_divisor_is_gcd_a() {
        let num_1 = 777_777_777;
        let num_2 = 111_111_111;

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            for duo in [(num_1, num_2), (num_2, num_1)] {
                let gcd = f(duo.0, duo.1);

                assert_eq!(num_2, gcd);
            }
        }
    }

    #[test]
    fn not_coprime_divisor_is_gcd_b() {
        let hand = 777_777_777;
        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            let gcd = f(hand, hand);
            assert_eq!(hand, gcd);
        }
    }

    #[test]
    fn not_coprime_odd_a_test() {
        let num_1 = 3_150_055_839u64; // 150002659ᵖ ⋅7ᵖ ⋅3ᵖ
        let num_2 = 76_604_397; // 1502047ᵖ ⋅17ᵖ ⋅3ᵖ
        let proof = 3;

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            for duo in [(num_1, num_2), (num_2, num_1)] {
                let gcd = f(duo.0, duo.1);

                assert_eq!(proof, gcd);
            }
        }
    }

    #[test]
    fn not_coprime_odd_b_test() {
        let num_1 = 56_991; // 157ᵖ ⋅33ᶜ ⋅11ᵖ
        let num_2 = 49_599; // 167ᵖ ⋅9ᶜ ⋅33ᶜ
        let proof = 33;

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            for duo in [(num_1, num_2), (num_2, num_1)] {
                let gcd = f(duo.0, duo.1);

                assert_eq!(proof, gcd);
            }
        }
    }

    #[test]
    fn not_coprime_even_a_test() {
        let num_1 = 549_755_813_888u64; // 2³⁹
        let num_2 = 300_005_318; // 150002659ᵖ ⋅2ᵖ
        let proof = 2;

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            for duo in [(num_1, num_2), (num_2, num_1)] {
                let gcd = f(duo.0, duo.1);

                assert_eq!(proof, gcd);
            }
        }
    }

    #[test]
    fn not_coprime_even_b_test() {
        let num_1 = 549_755_813_888u64; // 2³⁹
        let num_2 = 33_554_432; // 2²⁵        

        for f in [gcd_naive, gcd_disingenuous, gcd_naive_2, gcd_disingenuous_2] {
            for duo in [(num_1, num_2), (num_2, num_1)] {
                let gcd = f(duo.0, duo.1);

                assert_eq!(num_2, gcd);
            }
        }
    }

    mod gcd_nonnaive_extended {
        use super::super::{gcd_nonnaive_extended, gcd_nonnaive_extended_2};

        #[test]
        fn both_zero() {
            // can be updated to return both coefficient zero
            let zero = 0;
            for f in [gcd_nonnaive_extended, gcd_nonnaive_extended_2] {
                let res = f(zero, zero);
                assert_eq!(zero, res.0);
                assert_eq!(0, res.1);
                assert_eq!(1, res.2);
            }
        }
        #[test]
        fn left_hand_zero() {
            let l_hand = 0;
            let r_hand = 15;

            for f in [gcd_nonnaive_extended, gcd_nonnaive_extended_2] {
                let res = f(l_hand, r_hand);
                assert_eq!(r_hand, res.0);
                assert_eq!(0, res.1);
                assert_eq!(1, res.2);
            }
        }

        #[test]
        fn right_hand_zero() {
            let l_hand = 15;
            let r_hand = 0;

            for f in [gcd_nonnaive_extended, gcd_nonnaive_extended_2] {
                let res = f(l_hand, r_hand);
                assert_eq!(l_hand, res.0);
                assert_eq!(1, res.1);
                assert_eq!(0, res.2);
            }
        }

        fn run_test(num_1: i64, num_2: i64, proof: i64) {
            for (l_hand, r_hand) in [(num_1, num_2), (num_2, num_1)] {
                for f in [gcd_nonnaive_extended, gcd_nonnaive_extended_2] {
                    let res = f(l_hand, r_hand);
                    assert_eq!(proof, res.0);

                    let (x, y) = res.coeffs();

                    let test = x * l_hand as i128 + y * r_hand as i128;
                    assert_eq!(proof as i128, test);
                }
            }
        }

        #[test]
        fn coprime_primes_test() {
            // both prime numbers
            let num_1 = 1_299_709;
            let num_2 = 56_999;

            let proof = 1;

            run_test(num_1, num_2, proof);
        }

        #[test]
        fn coprime_odd_test() {
            let num_1 = 2_559_031_471i64; // 150531263ᵖ ⋅17ᵖ
            let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ

            let proof = 1;
            run_test(num_1, num_2, proof);
        }

        #[test]
        fn coprime_mixed_a_test() {
            let num_1 = 52_685_751_650_i64; // 150530719ᵖ ⋅350ᶜ
            let num_2 = 52_535_230_703_i64; // 150530747ᵖ ⋅349ᵖ

            let proof = 1;
            run_test(num_1, num_2, proof);
        }

        #[test]
        fn coprime_mixed_b_test() {
            let num_1 = 19_209_934_347_i64; // 56666473ᵖ ⋅113ᵖ ⋅3ᵖ
            let num_2 = 10_993_312_058_i64; // 56666557ᵖ ⋅2ᵖ ⋅97ᵖ

            let proof = 1;
            run_test(num_1, num_2, proof);
        }

        #[test]
        fn not_coprime_mixed_a_test() {
            let num_1 = 37_683_426; // 570961ᵖ ⋅66ᶜ
            let num_2 = 18_804_423; // 569831ᵖ ⋅33ᶜ

            let proof = 33;
            run_test(num_1, num_2, proof);
        }

        #[test]
        fn not_coprime_mixed_b_test() {
            let num_1 = 1_822_623; // 5021ᵖ ⋅33ᶜ ⋅11ᵖ
            let num_2 = 1_650_990; // 5003ᵖ ⋅10ᶜ ⋅33ᶜ

            let proof = 33;
            run_test(num_1, num_2, proof);
        }

        #[test]
        fn not_coprime_extra_test() {
            let num_1 = 55_286_231; // 5021ᵖ ⋅77ᶜ ⋅11ᵖ ⋅13ᵖ
            let num_2 = 7_704_620; // 5003ᵖ ⋅10ᶜ ⋅154ᶜ

            let proof = 77;
            run_test(num_1, num_2, proof);
        }

        #[test]
        fn not_coprime_divisor_is_gcd_a() {
            let num_1 = 777_777_777;
            let num_2 = 111_111_111;

            run_test(num_1, num_2, num_2);
        }

        #[test]
        fn not_coprime_divisor_is_gcd_b() {
            let num = 777_777_777;

            run_test(num, num, num);
        }

        #[test]
        fn not_coprime_odd_a_test() {
            let num_1 = 3_150_055_839i64; // 150002659ᵖ ⋅7ᵖ ⋅3ᵖ
            let num_2 = 76_604_397; // 1502047ᵖ ⋅17ᵖ ⋅3ᵖ

            let proof = 3;
            run_test(num_1, num_2, proof);
        }

        #[test]
        fn not_coprime_odd_b_test() {
            let num_1 = 56_991; // 157ᵖ ⋅33ᶜ ⋅11ᵖ
            let num_2 = 49_599; // 167ᵖ ⋅9ᶜ ⋅33ᶜ

            let proof = 33;
            run_test(num_1, num_2, proof);
        }

        #[test]
        fn not_coprime_even_a_test() {
            let num_1 = 549_755_813_888i64; // 2³⁹
            let num_2 = 300_005_318; // 150002659ᵖ ⋅2ᵖ

            let proof = 2;
            run_test(num_1, num_2, proof);
        }

        #[test]
        fn not_coprime_even_b_test() {
            let num_1 = 549_755_813_888i64; // 2³⁹
            let num_2 = 33_554_432; // 2²⁵            

            run_test(num_1, num_2, num_2);
        }
    }
}

// cargo fmt && cargo test --release
