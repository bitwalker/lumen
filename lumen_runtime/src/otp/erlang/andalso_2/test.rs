use proptest::prop_assert_eq;

use crate::otp::erlang::andalso_2::native;
use crate::test::strategy;

#[test]
fn without_boolean_left_errors_badarg() {
    run!(
        |arc_process| {
            (
                strategy::term::is_not_boolean(arc_process.clone()),
                strategy::term::is_boolean(),
            )
        },
        |(left, right)| {
            prop_assert_badarg!(native(left, right), "left must be a bool");

            Ok(())
        },
    );
}

#[test]
fn with_false_left_returns_false() {
    run!(|arc_process| strategy::term(arc_process.clone()), |right| {
        prop_assert_eq!(native(false.into(), right), Ok(false.into()));

        Ok(())
    },);
}

#[test]
fn with_true_left_returns_right() {
    run!(|arc_process| strategy::term(arc_process.clone()), |right| {
        prop_assert_eq!(native(true.into(), right), Ok(right));

        Ok(())
    },);
}
