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
            $ty:ty,
            (
                $var:expr,
                $var2:expr,
                $var3:expr,
                $var4:expr,
                $var5:expr,
            )
        ) => {
            // Shifts.
            check!($ty, $var << (argc as $ty - 1));
            check!($ty, $var << argc as $ty);
            check!($ty, $var << (argc + 32) as $ty);
            check!($ty, $var << (argc + 48) as $ty);
            check!($ty, $var << (argc + 60) as $ty);
            check!($ty, $var << (argc + 62) as $ty);
            check!($ty, $var << (argc + 63) as $ty);
            check!($ty, $var << (argc + 80) as $ty);

            check!($ty, $var2 << argc as $ty);
            check!($ty, $var2 << (argc as $ty - 1));
            check!($ty, $var2 << (argc + 32) as $ty);
            check!($ty, $var2 << (argc + 48) as $ty);
            check!($ty, $var2 << (argc + 60) as $ty);
            check!($ty, $var2 << (argc + 62) as $ty);
            check!($ty, $var2 << (argc + 63) as $ty);

            check!($ty, $var3 << argc as $ty);
            check!($ty, $var3 << (argc as $ty - 1));
            check!($ty, $var3 << (argc + 32) as $ty);
            check!($ty, $var3 << (argc + 48) as $ty);
            check!($ty, $var3 << (argc + 60) as $ty);
            check!($ty, $var3 << (argc + 62) as $ty);
            check!($ty, $var3 << (argc + 63) as $ty);

            check!($ty, $var >> (argc as $ty - 1));
            check!($ty, $var >> argc as $ty);
            check!($ty, $var >> (argc + 32) as $ty);
            check!($ty, $var >> (argc + 48) as $ty);
            check!($ty, $var >> (argc + 60) as $ty);
            check!($ty, $var >> (argc + 62) as $ty);
            check!($ty, $var >> (argc + 63) as $ty);

            check!($ty, $var2 >> argc as $ty);
            check!($ty, $var2 >> (argc as $ty - 1));
            check!($ty, $var2 >> (argc + 32) as $ty);
            check!($ty, $var2 >> (argc + 48) as $ty);
            check!($ty, $var2 >> (argc + 60) as $ty);
            check!($ty, $var2 >> (argc + 62) as $ty);
            check!($ty, $var2 >> (argc + 63) as $ty);

            check!($ty, $var3 >> (argc as $ty - 1));
            check!($ty, $var3 >> argc as $ty);
            check!($ty, $var3 >> (argc + 32) as $ty);
            check!($ty, $var3 >> (argc + 48) as $ty);
            check!($ty, $var3 >> (argc + 60) as $ty);
            check!($ty, $var3 >> (argc + 62) as $ty);
            check!($ty, $var3 >> (argc + 63) as $ty);
            check!($ty, $var3 >> (argc + 80) as $ty);

            // Casts
            check!(u64, ($var >> (argc + 32) as $ty) as u64);
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
        u128,
        (
            134217856_u128,
            10475372733397991552_u128,
            193236519889708027473620326106273939584_u128,
            123236519889708027473620326106273939584_u128,
            153236519889708027473620326106273939584_u128,
        )
    );

    0
}
