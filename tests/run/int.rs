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

            check!($ty, $var.wrapping_shl(argc as u32 - 1));
            check!($ty, $var.wrapping_shl(argc as u32));
            check!($ty, $var.wrapping_shl((argc + 32) as u32));
            check!($ty, $var.wrapping_shl((argc + 48) as u32));
            check!($ty, $var.wrapping_shl((argc + 60) as u32));
            check!($ty, $var.wrapping_shl((argc + 62) as u32));
            check!($ty, $var.wrapping_shl((argc + 63) as u32));
            check!($ty, $var.wrapping_shl((argc + 80) as u32));

            check!(Option<$ty>, $var.checked_shl(argc as u32 - 1));
            check!(Option<$ty>, $var.checked_shl(argc as u32));
            check!(Option<$ty>, $var.checked_shl((argc + 32) as u32));
            check!(Option<$ty>, $var.checked_shl((argc + 48) as u32));
            check!(Option<$ty>, $var.checked_shl((argc + 60) as u32));
            check!(Option<$ty>, $var.checked_shl((argc + 62) as u32));
            check!(Option<$ty>, $var.checked_shl((argc + 63) as u32));
            check!(Option<$ty>, $var.checked_shl((argc + 80) as u32));

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

            check!($ty, $var.wrapping_shr(argc as u32 - 1));
            check!($ty, $var.wrapping_shr(argc as u32));
            check!($ty, $var.wrapping_shr((argc + 32) as u32));
            check!($ty, $var.wrapping_shr((argc + 48) as u32));
            check!($ty, $var.wrapping_shr((argc + 60) as u32));
            check!($ty, $var.wrapping_shr((argc + 62) as u32));
            check!($ty, $var.wrapping_shr((argc + 63) as u32));
            check!($ty, $var.wrapping_shr((argc + 80) as u32));

            check!(Option<$ty>, $var.checked_shr(argc as u32 - 1));
            check!(Option<$ty>, $var.checked_shr(argc as u32));
            check!(Option<$ty>, $var.checked_shr((argc + 32) as u32));
            check!(Option<$ty>, $var.checked_shr((argc + 48) as u32));
            check!(Option<$ty>, $var.checked_shr((argc + 60) as u32));
            check!(Option<$ty>, $var.checked_shr((argc + 62) as u32));
            check!(Option<$ty>, $var.checked_shr((argc + 63) as u32));
            check!(Option<$ty>, $var.checked_shr((argc + 80) as u32));

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

            check!(Option<$ty>, $var.checked_add(argc as $ty));
            check!(Option<$ty>, $var2.checked_add(argc as $ty));
            check!(Option<$ty>, $var2.checked_add(($var2 + argc as $ty) as $ty));
            check!(Option<$ty>, $var3.checked_add($ty::MAX));
            check!(Option<$ty>, $var3.checked_add($ty::MIN));

            check!($ty, $var.wrapping_add(argc as $ty));
            check!($ty, $var2.wrapping_add(argc as $ty));
            check!($ty, $var2.wrapping_add(($var2 + argc as $ty) as $ty));
            check!($ty, $var3.wrapping_add($ty::MAX));
            check!($ty, $var3.wrapping_add($ty::MIN));

            check!(($ty, bool), $var.overflowing_add(argc as $ty));
            check!(($ty, bool), $var2.overflowing_add(argc as $ty));
            check!(($ty, bool), $var2.overflowing_add(($var2 + argc as $ty) as $ty));
            check!(($ty, bool), $var3.overflowing_add($ty::MAX));
            check!(($ty, bool), $var3.overflowing_add($ty::MIN));

            check!($ty, $var.saturating_add(argc as $ty));
            check!($ty, $var2.saturating_add(argc as $ty));
            check!($ty, $var2.saturating_add(($var2 + argc as $ty) as $ty));
            check!($ty, $var3.saturating_add($ty::MAX));
            check!($ty, $var3.saturating_add($ty::MIN));

            // Subtraction
            check!($ty, $var - argc as $ty);
            check!($ty, $var2 - argc as $ty);
            check!($ty, $var3 - argc as $ty);

            check!(Option<$ty>, $var.checked_sub(argc as $ty));
            check!(Option<$ty>, $var2.checked_sub(argc as $ty));
            check!(Option<$ty>, $var2.checked_sub(($var2 + argc as $ty) as $ty));
            check!(Option<$ty>, $var3.checked_sub($ty::MAX));
            check!(Option<$ty>, $var3.checked_sub($ty::MIN));

            check!($ty, $var.wrapping_sub(argc as $ty));
            check!($ty, $var2.wrapping_sub(argc as $ty));
            check!($ty, $var2.wrapping_sub(($var2 + argc as $ty) as $ty));
            check!($ty, $var3.wrapping_sub($ty::MAX));
            check!($ty, $var3.wrapping_sub($ty::MIN));

            check!(($ty, bool), $var.overflowing_sub(argc as $ty));
            check!(($ty, bool), $var2.overflowing_sub(argc as $ty));
            check!(($ty, bool), $var2.overflowing_sub(($var2 + argc as $ty) as $ty));
            check!(($ty, bool), $var3.overflowing_sub($ty::MAX));
            check!(($ty, bool), $var3.overflowing_sub($ty::MIN));

            check!($ty, $var.saturating_sub(argc as $ty));
            check!($ty, $var2.saturating_sub(argc as $ty));
            check!($ty, $var2.saturating_sub(($var2 + argc as $ty) as $ty));
            check!($ty, $var3.saturating_sub($ty::MAX));
            check!($ty, $var3.saturating_sub($ty::MIN));

            // Multiplication
            check!($ty, $var * (argc + 1) as $ty);
            check!($ty, $var * (argc as $ty + $var2));
            check!($ty, $var2 * (argc + 1) as $ty);
            check!($ty, $var2 * (argc as $ty + $var2));
            check!($ty, $var3 * argc as $ty);
            check!($ty, $var4 * (argc + 1) as $ty);
            check!($ty, $var5 * (argc + 1) as $ty);

            check!(Option<$ty>, $var.checked_mul((argc + 1) as $ty));
            check!(Option<$ty>, $var.checked_mul((argc as $ty + $var2)));
            check!(Option<$ty>, $var3.checked_mul($var3));
            check!(Option<$ty>, $var4.checked_mul((argc + 1) as $ty));
            check!(Option<$ty>, $var5.checked_mul((argc + 1) as $ty));

            check!($ty, $var.wrapping_mul((argc + 1) as $ty));
            check!($ty, $var.wrapping_mul((argc as $ty + $var2)));
            check!($ty, $var3.wrapping_mul($var3));
            check!($ty, $var4.wrapping_mul((argc + 1) as $ty));
            check!($ty, $var5.wrapping_mul((argc + 1) as $ty));

            check!(($ty, bool), $var.overflowing_mul((argc + 1) as $ty));
            check!(($ty, bool), $var.overflowing_mul((argc as $ty + $var2)));
            check!(($ty, bool), $var3.overflowing_mul($var3));
            check!(($ty, bool), $var4.overflowing_mul((argc + 1) as $ty));
            check!(($ty, bool), $var5.overflowing_mul((argc + 1) as $ty));

            check!($ty, $var.saturating_mul((argc + 1) as $ty));
            check!($ty, $var.saturating_mul((argc as $ty + $var2)));
            check!($ty, $var3.saturating_mul($var3));
            check!($ty, $var4.saturating_mul((argc + 1) as $ty));
            check!($ty, $var5.saturating_mul((argc + 1) as $ty));

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

            check!(Option<$ty>, $var.checked_div((argc + 1) as $ty));
            check!(Option<$ty>, $var.checked_div((argc as $ty + $var2)));
            check!(Option<$ty>, $var3.checked_div($var3));
            check!(Option<$ty>, $var4.checked_div((argc + 1) as $ty));
            check!(Option<$ty>, $var5.checked_div((argc + 1) as $ty));
            check!(Option<$ty>, ($ty::MIN).checked_div((0 as $ty).wrapping_sub(argc as $ty)));
            check!(Option<$ty>, $var5.checked_div((argc - 1) as $ty)); // var5 / 0

            check!($ty, $var.wrapping_div((argc + 1) as $ty));
            check!($ty, $var.wrapping_div((argc as $ty + $var2)));
            check!($ty, $var3.wrapping_div($var3));
            check!($ty, $var4.wrapping_div((argc + 1) as $ty));
            check!($ty, $var5.wrapping_div((argc + 1) as $ty));
            check!($ty, ($ty::MIN).wrapping_div((0 as $ty).wrapping_sub(argc as $ty)));

            check!(($ty, bool), $var.overflowing_div((argc + 1) as $ty));
            check!(($ty, bool), $var.overflowing_div((argc as $ty + $var2)));
            check!(($ty, bool), $var3.overflowing_div($var3));
            check!(($ty, bool), $var4.overflowing_div((argc + 1) as $ty));
            check!(($ty, bool), $var5.overflowing_div((argc + 1) as $ty));
            check!(($ty, bool), ($ty::MIN).overflowing_div((0 as $ty).wrapping_sub(argc as $ty)));

            check!($ty, $var.saturating_div((argc + 1) as $ty));
            check!($ty, $var.saturating_div((argc as $ty + $var2)));
            check!($ty, $var3.saturating_div($var3));
            check!($ty, $var4.saturating_div((argc + 1) as $ty));
            check!($ty, $var5.saturating_div((argc + 1) as $ty));
            check!($ty, ($ty::MIN).saturating_div((0 as $ty).wrapping_sub(argc as $ty)));
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
