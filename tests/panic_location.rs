//! Tests that panicking functions are annotated with `#[track_caller]`.
//! This is a separate integration test so that we can replace the (process-wide)
//! panic hook without influencing other tests.

use arbitrary_int::{i3, i4, u3, u4, Number, SignedNumber};
use std::{cell::Cell, panic};

thread_local! {
    /// Stores the value of [`std::panic::Location::file()`]. This is a global
    /// so that [`std::panic::set_hook()`] can write to it, and a thread-local
    /// because tests may run in parallel.
    static PANIC_FILE: Cell<Option<String>> = const { Cell::new(None) };
}

#[track_caller]
fn panic_loc_is_current_file<T>(f: impl FnOnce() -> T + panic::UnwindSafe) {
    // Register our panic hook to capture the location
    panic::set_hook(Box::new(|info| {
        let file = info.location().map(|loc| loc.file().to_string());
        PANIC_FILE.with(move |b| b.set(file));
    }));

    // Run the panicking code, capturing the result
    let result = panic::catch_unwind(f);

    // Restore the standard library's panic hook so that assertions function properly
    let _ = panic::take_hook();

    match result {
        Ok(_) => panic!("should have panicked"),
        Err(_) => {
            let loc = PANIC_FILE.with(|b| b.take().expect("panic location is set"));
            assert_eq!(file!(), loc, "`#[track_caller]` attribute missing");
        }
    }
}

macro_rules! test_operator {
    ($op:tt, $op_assign:tt; $lhs:expr, $rhs:expr) => {
        #[cfg(debug_assertions)]
        {
            panic_loc_is_current_file(|| $lhs $op $rhs);
            panic_loc_is_current_file(|| {
                let mut x = $lhs;
                x $op_assign $rhs;
            });
        }
    };
}

fn panic_location_unsigned() {
    // Const constructors
    panic_loc_is_current_file(|| u3::new(u3::MAX.value() + 1));
    panic_loc_is_current_file(|| u3::from_u8(u3::MAX.value() + 1));
    panic_loc_is_current_file(|| u3::from_u16(u3::MAX.value() as u16 + 1));
    panic_loc_is_current_file(|| u3::from_u32(u3::MAX.value() as u32 + 1));
    panic_loc_is_current_file(|| u3::from_u64(u3::MAX.value() as u64 + 1));
    panic_loc_is_current_file(|| u3::from_u128(u3::MAX.value() as u128 + 1));

    // Trait constructors
    panic_loc_is_current_file(|| <u3 as Number>::new(u3::MAX.value() + 1));
    #[cfg(not(feature = "const_convert_and_const_trait_impl"))]
    {
        panic_loc_is_current_file(|| u3::from_(u3::MAX.value() + 1));
        panic_loc_is_current_file(|| u3::from_(u3::MAX.value() as u16 + 1));
        panic_loc_is_current_file(|| u3::from_(u3::MAX.value() as u32 + 1));
        panic_loc_is_current_file(|| u3::from_(u3::MAX.value() as u64 + 1));
        panic_loc_is_current_file(|| u3::from_(u3::MAX.value() as u128 + 1));
        panic_loc_is_current_file(|| u3::from_(u4::new(u3::MAX.value() + 1)));
    }

    // Extract functions
    #[allow(deprecated)]
    panic_loc_is_current_file(|| u3::extract(0, 6));
    panic_loc_is_current_file(|| u3::extract_u8(0, 6));
    panic_loc_is_current_file(|| u3::extract_i8(0, 6));
    panic_loc_is_current_file(|| u3::extract_u16(0, 14));
    panic_loc_is_current_file(|| u3::extract_i16(0, 14));
    panic_loc_is_current_file(|| u3::extract_u32(0, 30));
    panic_loc_is_current_file(|| u3::extract_i32(0, 30));
    panic_loc_is_current_file(|| u3::extract_u64(0, 62));
    panic_loc_is_current_file(|| u3::extract_i64(0, 62));
    panic_loc_is_current_file(|| u3::extract_u128(0, 126));
    panic_loc_is_current_file(|| u3::extract_i128(0, 126));

    // Arithmetic functions
    panic_loc_is_current_file(|| u3::new(0).wrapping_div(u3::new(0)));
    panic_loc_is_current_file(|| u3::new(0).saturating_div(u3::new(0)));
    panic_loc_is_current_file(|| u3::new(0).overflowing_div(u3::new(0)));

    // Operators
    test_operator!(+, +=; u3::MAX, u3::new(1));
    test_operator!(-, -=; u3::MIN, u3::new(1));
    test_operator!(*, *=; u3::MAX, u3::new(2));
    test_operator!(/, /=; u3::MAX, u3::new(0));
    test_operator!(<<, <<=; u3::MAX, 4);
    test_operator!(>>, >>=; u3::MAX, 4);
}

fn panic_location_signed() {
    // Const constructors
    panic_loc_is_current_file(|| i3::new(i3::MAX.value() + 1));
    panic_loc_is_current_file(|| i3::from_i8(i3::MAX.value() + 1));
    panic_loc_is_current_file(|| i3::from_i16(i3::MAX.value() as i16 + 1));
    panic_loc_is_current_file(|| i3::from_i32(i3::MAX.value() as i32 + 1));
    panic_loc_is_current_file(|| i3::from_i64(i3::MAX.value() as i64 + 1));
    panic_loc_is_current_file(|| i3::from_i128(i3::MAX.value() as i128 + 1));

    // Trait constructors
    panic_loc_is_current_file(|| <i3 as SignedNumber>::new(i3::MAX.value() + 1));
    #[cfg(not(feature = "const_convert_and_const_trait_impl"))]
    {
        panic_loc_is_current_file(|| i3::from_(i3::MAX.value() + 1));
        panic_loc_is_current_file(|| i3::from_(i3::MAX.value() as i16 + 1));
        panic_loc_is_current_file(|| i3::from_(i3::MAX.value() as i32 + 1));
        panic_loc_is_current_file(|| i3::from_(i3::MAX.value() as i64 + 1));
        panic_loc_is_current_file(|| i3::from_(i3::MAX.value() as i128 + 1));
        panic_loc_is_current_file(|| i3::from_(i4::new(i3::MAX.value() + 1)));
    }

    // Extract functions
    #[allow(deprecated)]
    panic_loc_is_current_file(|| i3::extract_u8(0, 6));
    panic_loc_is_current_file(|| i3::extract_i8(0, 6));
    panic_loc_is_current_file(|| i3::extract_u16(0, 14));
    panic_loc_is_current_file(|| i3::extract_i16(0, 14));
    panic_loc_is_current_file(|| i3::extract_i32(0, 30));
    panic_loc_is_current_file(|| i3::extract_i32(0, 30));
    panic_loc_is_current_file(|| i3::extract_u64(0, 62));
    panic_loc_is_current_file(|| i3::extract_i64(0, 62));
    panic_loc_is_current_file(|| i3::extract_u128(0, 126));
    panic_loc_is_current_file(|| i3::extract_i128(0, 126));

    // Arithmetic functions

    // TODO: We use the primitive's equivalent methods internally, which currently aren't
    // annotated with `#[track_caller]`: https://github.com/rust-lang/rust/issues/139672
    // panic_loc_is_current_file(|| i3::new(0).wrapping_div(i3::new(0)));
    // panic_loc_is_current_file(|| i3::new(0).saturating_div(i3::new(0)));

    // Operators
    test_operator!(+, +=; i3::MAX, i3::new(1));
    test_operator!(-, -=; i3::MIN, i3::new(1));
    test_operator!(*, *=; i3::MAX, i3::new(2));
    test_operator!(/, /=; i3::MAX, i3::new(0));
    test_operator!(<<, <<=; i3::MAX, 4);
    test_operator!(>>, >>=; i3::MAX, 4);
    panic_loc_is_current_file(|| -i3::MIN);
}

// This is done sequentially in a single test so that we do not have to worry
// about multiple tests concurrently modifying the (process-wide) panic hook.
#[test]
#[cfg(panic = "unwind")]
pub fn panic_location() {
    panic_location_unsigned();
    panic_location_signed();
}
