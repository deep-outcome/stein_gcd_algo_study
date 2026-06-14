#![feature(test)]

use stein_gcd_algo_study::{
    gcd_disingenuous, gcd_disingenuous_2, gcd_naive, gcd_naive_2, gcd_nonnaive_extended,
    gcd_nonnaive_extended_2, gcd_nonnaive_extended_b, gcd_nonnaive_extended_c,
};

extern crate test;
use std::hint::black_box;
use test::Bencher;

#[bench]
fn gcd_naive_test(b: &mut Bencher) {
    let num_1 = 2_559_031_471u64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ    

    b.iter(|| _ = gcd_naive(black_box(num_1), black_box(num_2)));
}

#[bench]
fn gcd_naive_2_test(b: &mut Bencher) {
    let num_1 = 2_559_031_471u64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ

    b.iter(|| _ = gcd_naive_2(black_box(num_1), black_box(num_2)));
}

#[bench]
fn gcd_disingenuous_test(b: &mut Bencher) {
    let num_1 = 2_559_031_471u64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ

    b.iter(|| _ = gcd_disingenuous(num_1, num_2));
}

#[bench]
fn gcd_disingenuous_2_test(b: &mut Bencher) {
    let num_1 = 2_559_031_471u64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ

    b.iter(|| _ = gcd_disingenuous_2(num_1, num_2));
}

#[bench]
fn gcd_e_test(b: &mut Bencher) {
    let num_1 = 2_559_031_471u64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ

    b.iter(|| _ = gcd_e(num_1, num_2));
}

#[bench]
fn gcd_nonnaive_extended_test(b: &mut Bencher) {
    let num_1 = 2_559_031_471i64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ

    b.iter(|| _ = gcd_nonnaive_extended(num_1, num_2));
}

#[bench]
fn gcd_nonnaive_extended_b_test(b: &mut Bencher) {
    let num_1 = 2_559_031_471i64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ

    b.iter(|| _ = gcd_nonnaive_extended_b(num_1, num_2));
}

#[bench]
fn gcd_nonnaive_extended_c_test(b: &mut Bencher) {
    let num_1 = 2_559_031_471i64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ

    b.iter(|| _ = gcd_nonnaive_extended_c(num_1, num_2));
}

#[bench]
fn gcd_nonnaive_extended_2_test(b: &mut Bencher) {
    let num_1 = 2_559_031_471i64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ

    b.iter(|| _ = gcd_nonnaive_extended_2(num_1, num_2));
}

#[bench]
fn gcd_ee_test(b: &mut Bencher) {
    let num_1 = 2_559_031_471i64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ

    b.iter(|| _ = gcd_ee(num_1, num_2));
}

fn gcd_e(mut gcd: u64, mut rem: u64) -> u64 {
    loop {
        let ratio = gcd / rem;
        let old_rem = rem;
        rem = gcd - ratio * rem;
        gcd = old_rem;

        if rem == 0 {
            break;
        }
    }

    gcd
}

fn gcd_ee(mut gcd: i64, mut rem: i64) -> (i64, i128, i128) {
    let mut x = 0;
    let mut y = 1;

    let mut u = 1;
    let mut v = 0;

    loop {
        let ratio = gcd / rem;
        let old_rem = rem;
        rem = gcd - ratio * rem;
        gcd = old_rem;

        if rem == 0 {
            break;
        }

        let mut swap = x;
        x = u - x * ratio;
        u = swap;

        swap = y;
        y = v - y * ratio;
        v = swap;
    }

    (gcd, x as i128, y as i128)
}

mod tests_of_units {

    mod gcd_e {
        use super::super::gcd_e;

        #[test]
        fn coprime_primes_test() {
            // both prime numbers
            let l_hand = 1_299_709;
            let r_hand = 56_999;

            let proof = 1;

            let res = gcd_e(l_hand, r_hand);
            assert_eq!(proof, res);
        }

        #[test]
        fn coprime_odd_test() {
            let l_hand = 2_559_031_471u64; // 150531263ᵖ ⋅17ᵖ
            let r_hand = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ

            let proof = 1;

            let res = gcd_e(l_hand, r_hand);

            assert_eq!(proof, res);
        }

        #[test]
        fn coprime_mixed_a_test() {
            let l_hand = 52_685_751_650_u64; // 150530719ᵖ ⋅350ᶜ
            let r_hand = 52_535_230_703_u64; // 150530747ᵖ ⋅349ᵖ

            let proof = 1;

            let res = gcd_e(l_hand, r_hand);

            assert_eq!(proof, res);
        }

        #[test]
        fn coprime_mixed_b_test() {
            let l_hand = 19_209_934_347_u64; // 56666473ᵖ ⋅113ᵖ ⋅3ᵖ
            let r_hand = 10_993_312_058_u64; // 56666557ᵖ ⋅2ᵖ ⋅97ᵖ

            let proof = 1;

            let res = gcd_e(l_hand, r_hand);

            assert_eq!(proof, res);
        }

        #[test]
        fn not_coprime_mixed_a_test() {
            let l_hand = 37_683_426; // 570961ᵖ ⋅66ᶜ
            let r_hand = 18_804_423; // 569831ᵖ ⋅33ᶜ

            let proof = 33;

            let res = gcd_e(l_hand, r_hand);

            assert_eq!(proof, res);
        }

        #[test]
        fn not_coprime_mixed_b_test() {
            let l_hand = 1_822_623; // 5021ᵖ ⋅33ᶜ ⋅11ᵖ
            let r_hand = 1_650_990; // 5003ᵖ ⋅10ᶜ ⋅33ᶜ

            let proof = 33;

            let res = gcd_e(l_hand, r_hand);

            assert_eq!(proof, res);
        }

        #[test]
        fn not_coprime_extra_test() {
            let l_hand = 55_286_231; // 5021ᵖ ⋅77ᶜ ⋅11ᵖ ⋅13ᵖ
            let r_hand = 7_704_620; // 5003ᵖ ⋅10ᶜ ⋅154ᶜ

            let proof = 77;

            let res = gcd_e(l_hand, r_hand);

            assert_eq!(proof, res);
        }

        #[test]
        fn not_coprime_divisor_is_gcd_a() {
            let l_hand = 777_777_777;
            let r_hand = 111_111_111;

            let res = gcd_e(l_hand, r_hand);

            assert_eq!(r_hand, res);
        }

        #[test]
        fn not_coprime_divisor_is_gcd_b() {
            let num = 777_777_777;

            let res = gcd_e(num, num);
            assert_eq!(num, res);
        }

        #[test]
        fn not_coprime_odd_a_test() {
            let l_hand = 3_150_055_839u64; // 150002659ᵖ ⋅7ᵖ ⋅3ᵖ
            let r_hand = 76_604_397; // 1502047ᵖ ⋅17ᵖ ⋅3ᵖ

            let proof = 3;

            let res = gcd_e(l_hand, r_hand);

            assert_eq!(proof, res);
        }

        #[test]
        fn not_coprime_odd_b_test() {
            let l_hand = 56_991; // 157ᵖ ⋅33ᶜ ⋅11ᵖ
            let r_hand = 49_599; // 167ᵖ ⋅9ᶜ ⋅33ᶜ

            let proof = 33;

            let res = gcd_e(l_hand, r_hand);

            assert_eq!(proof, res);
        }

        #[test]
        fn not_coprime_even_a_test() {
            let l_hand = 549_755_813_888u64; // 2³⁹
            let r_hand = 300_005_318; // 150002659ᵖ ⋅2ᵖ

            let proof = 2;

            let res = gcd_e(l_hand, r_hand);

            assert_eq!(proof, res);
        }

        #[test]
        fn not_coprime_even_b_test() {
            let l_hand = 549_755_813_888u64; // 2³⁹
            let r_hand = 33_554_432; // 2²⁵            

            let res = gcd_e(l_hand, r_hand);
            assert_eq!(r_hand, res);
        }
    }

    mod gcd_ee {
        use super::super::gcd_ee;

        fn run_test(l_hand: i64, r_hand: i64, proof: i64) {
            let res = gcd_ee(l_hand, r_hand);
            assert_eq!(proof, res.0);

            let (x, y) = (res.1, res.2);

            let test = x * l_hand as i128 + y * r_hand as i128;
            assert_eq!(proof as i128, test);
        }

        #[test]
        fn coprime_primes_test() {
            // both prime numbers
            let l_hand = 1_299_709;
            let r_hand = 56_999;

            let proof = 1;

            run_test(l_hand, r_hand, proof)
        }

        #[test]
        fn coprime_odd_test() {
            let l_hand = 2_559_031_471i64; // 150531263ᵖ ⋅17ᵖ
            let r_hand = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ

            let proof = 1;

            run_test(l_hand, r_hand, proof)
        }

        #[test]
        fn coprime_mixed_a_test() {
            let l_hand = 52_685_751_650_i64; // 150530719ᵖ ⋅350ᶜ
            let r_hand = 52_535_230_703_i64; // 150530747ᵖ ⋅349ᵖ

            let proof = 1;

            run_test(l_hand, r_hand, proof)
        }

        #[test]
        fn coprime_mixed_b_test() {
            let l_hand = 19_209_934_347_i64; // 56666473ᵖ ⋅113ᵖ ⋅3ᵖ
            let r_hand = 10_993_312_058_i64; // 56666557ᵖ ⋅2ᵖ ⋅97ᵖ

            let proof = 1;

            run_test(l_hand, r_hand, proof)
        }

        #[test]
        fn not_coprime_mixed_a_test() {
            let l_hand = 37_683_426; // 570961ᵖ ⋅66ᶜ
            let r_hand = 18_804_423; // 569831ᵖ ⋅33ᶜ

            let proof = 33;

            run_test(l_hand, r_hand, proof)
        }

        #[test]
        fn not_coprime_mixed_b_test() {
            let l_hand = 1_822_623; // 5021ᵖ ⋅33ᶜ ⋅11ᵖ
            let r_hand = 1_650_990; // 5003ᵖ ⋅10ᶜ ⋅33ᶜ

            let proof = 33;

            run_test(l_hand, r_hand, proof)
        }

        #[test]
        fn not_coprime_extra_test() {
            let l_hand = 55_286_231; // 5021ᵖ ⋅77ᶜ ⋅11ᵖ ⋅13ᵖ
            let r_hand = 7_704_620; // 5003ᵖ ⋅10ᶜ ⋅154ᶜ

            let proof = 77;

            run_test(l_hand, r_hand, proof)
        }

        #[test]
        fn not_coprime_divisor_is_gcd_a() {
            let l_hand = 777_777_777;
            let r_hand = 111_111_111;

            run_test(l_hand, r_hand, r_hand)
        }

        #[test]
        fn not_coprime_divisor_is_gcd_b() {
            let num = 777_777_777;

            run_test(num, num, num)
        }

        #[test]
        fn not_coprime_odd_a_test() {
            let l_hand = 3_150_055_839i64; // 150002659ᵖ ⋅7ᵖ ⋅3ᵖ
            let r_hand = 76_604_397; // 1502047ᵖ ⋅17ᵖ ⋅3ᵖ

            let proof = 3;

            run_test(l_hand, r_hand, proof)
        }

        #[test]
        fn not_coprime_odd_b_test() {
            let l_hand = 56_991; // 157ᵖ ⋅33ᶜ ⋅11ᵖ
            let r_hand = 49_599; // 167ᵖ ⋅9ᶜ ⋅33ᶜ

            let proof = 33;

            run_test(l_hand, r_hand, proof)
        }

        #[test]
        fn not_coprime_even_a_test() {
            let l_hand = 549_755_813_888i64; // 2³⁹
            let r_hand = 300_005_318; // 150002659ᵖ ⋅2ᵖ

            let proof = 2;

            run_test(l_hand, r_hand, proof)
        }

        #[test]
        fn not_coprime_even_b_test() {
            let l_hand = 549_755_813_888i64; // 2³⁹
            let r_hand = 33_554_432; // 2²⁵            

            run_test(l_hand, r_hand, r_hand)
        }
    }
}

// rustup default nightly-x86_64-unknown-linux-gnu
// rustup default stable-x86_64-unknown-linux-gnu
//
// cargo fmt && cargo bench --test bench --profile bench
// cargo fmt && cargo test --release
