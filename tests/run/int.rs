// Compiler:
//
// Run-time:
//   status: 0

#![feature(core_intrinsics, start)]

#![no_std]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort();
}

/*
 * Code
 */

#[start]
fn main(argc: isize, _argv: *const *const u8) -> isize {
    macro_rules! check {
        ($ty:ty, $expr:expr) => {
            {
                const EXPECTED: $ty = {
                    #[allow(non_upper_case_globals)]
                    #[allow(dead_code)]
                    const argc: isize = 1;
                    $expr
                };
                assert_eq!($expr, EXPECTED);
            }
        };
    }

    check!(u32, (2220326408_u32 + argc as u32) >> (32 - 6));

    macro_rules! check_ops {
        (
            $S:tt
            $ty:ident,
            (
                $var:expr,
                $var2:expr,
                $var3:expr,
                $var4:expr,
                $var5:expr,
            )
        ) => {
            macro_rules! if_ty {
                ($ty $S(| $rest:ident)* => $S($tt:tt)*) => {
                    // Found a match
                    $S($tt)*
                };
                ($head:ident $S(| $rest:ident)* => $S($tt:tt)*) => {
                    if_ty!($S($rest)|* => $S($tt)*)
                };
                ( => $S($tt:tt)*) => {};
            }

            // Shifts.
            check!($ty, $var << (argc as $ty - 1));
            check!($ty, $var << argc as $ty);
            if_ty! {
                u64 | i64 | u128 | i128 =>
                check!($ty, $var << (argc + 32) as $ty);
                check!($ty, $var << (argc + 48) as $ty);
                check!($ty, $var << (argc + 60) as $ty);
                check!($ty, $var << (argc + 62) as $ty);
            }
            if_ty! {
                u128 | i128 =>
                check!($ty, $var << (argc + 63) as $ty);
                check!($ty, $var << (argc + 80) as $ty);
            }

            check!($ty, $var2 << argc as $ty);
            check!($ty, $var2 << (argc as $ty - 1));
            if_ty! {
                u64 | i64 | u128 | i128 =>
                check!($ty, $var2 << (argc + 32) as $ty);
                check!($ty, $var2 << (argc + 48) as $ty);
                check!($ty, $var2 << (argc + 60) as $ty);
                check!($ty, $var2 << (argc + 62) as $ty);
            }
            if_ty! {
                u128 | i128 =>
                check!($ty, $var2 << (argc + 63) as $ty);
            }

            check!($ty, $var3 << argc as $ty);
            check!($ty, $var3 << (argc as $ty - 1));
            if_ty! {
                u64 | i64 | u128 | i128 =>
                check!($ty, $var3 << (argc + 32) as $ty);
                check!($ty, $var3 << (argc + 48) as $ty);
                check!($ty, $var3 << (argc + 60) as $ty);
            }
            if_ty! {
                u128 | i128 =>
                check!($ty, $var3 << (argc + 62) as $ty);
                check!($ty, $var3 << (argc + 63) as $ty);
            }

            check!($ty, $var >> (argc as $ty - 1));
            check!($ty, $var >> argc as $ty);
            if_ty! {
                u64 | i64 | u128 | i128 =>
                check!($ty, $var >> (argc + 32) as $ty);
                check!($ty, $var >> (argc + 48) as $ty);
                check!($ty, $var >> (argc + 60) as $ty);
                check!($ty, $var >> (argc + 62) as $ty);
            }
            if_ty! {
                u128 | i128 =>
                check!($ty, $var >> (argc + 63) as $ty);
            }

            check!($ty, $var2 >> argc as $ty);
            check!($ty, $var2 >> (argc as $ty - 1));
            if_ty! {
                u64 | i64 | u128 | i128 =>
                check!($ty, $var2 >> (argc + 32) as $ty);
                check!($ty, $var2 >> (argc + 48) as $ty);
                check!($ty, $var2 >> (argc + 60) as $ty);
                check!($ty, $var2 >> (argc + 62) as $ty);
            }
            if_ty! {
                u128 | i128 =>
                check!($ty, $var2 >> (argc + 63) as $ty);
            }

            check!($ty, $var3 >> (argc as $ty - 1));
            check!($ty, $var3 >> argc as $ty);
            if_ty! {
                u64 | i64 | u128 | i128 =>
                check!($ty, $var3 >> (argc + 32) as $ty);
                check!($ty, $var3 >> (argc + 48) as $ty);
                check!($ty, $var3 >> (argc + 60) as $ty);
                check!($ty, $var3 >> (argc + 62) as $ty);
            }
            if_ty! {
                u128 | i128 =>
                check!($ty, $var3 >> (argc + 63) as $ty);
                check!($ty, $var3 >> (argc + 80) as $ty);
            }

            // Casts
            if_ty! {
                u64 | i64 | u128 | i128 =>
                check!(u64, ($var >> (argc + 32) as $ty) as u64);
            }
            check!(u64, ($var >> argc as $ty) as u64);

            // Addition.
            check!($ty, $var + argc as $ty);

            check!($ty, $var2 + argc as $ty);
            check!($ty, $var2 + ($var2 + argc as $ty) as $ty);

            check!($ty, $var3 + argc as $ty);

            // Subtraction
            check!($ty, $var - argc as $ty);

            check!($ty, $var2 - argc as $ty);

            check!($ty, $var3 - argc as $ty);

            // Multiplication
            check!($ty, $var * (argc + 1) as $ty);
            check!($ty, $var * (argc as $ty + $var2));

            check!($ty, $var2 * (argc + 1) as $ty);
            check!($ty, $var2 * (argc as $ty + $var2));

            check!($ty, $var3 * argc as $ty);

            check!($ty, $var4 * (argc + 1) as $ty);

            check!($ty, $var5 * (argc + 1) as $ty);

            // Division.
            check!($ty, $var / (argc + 1) as $ty);
            check!($ty, $var / (argc + 2) as $ty);

            check!($ty, $var2 / (argc + 1) as $ty);
            check!($ty, $var2 / (argc + 2) as $ty);

            check!($ty, $var3 / (argc + 1) as $ty);
            check!($ty, $var3 / (argc + 2) as $ty);
            check!($ty, $var3 / (argc as $ty + $var4));
            check!($ty, $var3 / (argc as $ty + $var2));

            check!($ty, $var4 / (argc + 1) as $ty);
            check!($ty, $var4 / (argc + 2) as $ty);
        };
    }

    check_ops!(
        $ u32,
        (
            14162_u32,
            14556_u32,
            323656954_u32,
            2023651954_u32,
            1323651954_u32,
        )
    );
    check_ops!(
        $ i32,
        (
            13456_i32,
            10475_i32,
            923653954_i32,
            993198738_i32,
            1023653954_i32,
        )
    );

    check_ops!(
        $ u64,
        (
            134217856_u64,
            104753732_u64,
            12323651988970863954_u64,
            7323651988970863954_u64,
            8323651988970863954_u64,
        )
    );
    check_ops!(
        $ i64,
        (
            134217856_i64,
            104753732_i64,
            6323651988970863954_i64,
            2323651988970863954_i64,
            3323651988970863954_i64,
        )
    );

    check_ops!(
        $ u128,
        (
            134217856_u128,
            10475372733397991552_u128,
            193236519889708027473620326106273939584_u128,
            123236519889708027473620326106273939584_u128,
            153236519889708027473620326106273939584_u128,
        )
    );
    check_ops!(
        $ i128,
        (
            134217856_i128,
            10475372733397991552_i128,
            83236519889708027473620326106273939584_i128,
            63236519889708027473620326106273939584_i128,
            73236519889708027473620326106273939584_i128,
        )
    );

    0
}
