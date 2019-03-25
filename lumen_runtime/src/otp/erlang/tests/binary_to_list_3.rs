use super::*;

use std::sync::{Arc, RwLock};

use num_traits::Num;

use crate::environment::{self, Environment};
use crate::process::IntoProcess;

#[test]
fn with_atom_returns_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let atom_term = Term::str_to_atom("atom", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_list_3(
            atom_term,
            2.into_process(&mut process),
            3.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}

#[test]
fn with_local_referene_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let binary = Term::local_reference(&mut process);

    assert_bad_argument!(
        erlang::binary_to_list_3(
            binary,
            2.into_process(&mut process),
            3.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}

#[test]
fn with_empty_list_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();

    assert_bad_argument!(
        erlang::binary_to_list_3(
            Term::EMPTY_LIST,
            2.into_process(&mut process),
            3.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}

#[test]
fn with_list_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let list_term = list_term(&mut process);

    assert_bad_argument!(
        erlang::binary_to_list_3(
            list_term,
            2.into_process(&mut process),
            3.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}

#[test]
fn with_small_integer_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let small_integer_term = 0usize.into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_to_list_3(
            small_integer_term,
            2.into_process(&mut process),
            3.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}

#[test]
fn with_big_integer_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let big_integer_term = <BigInt as Num>::from_str_radix("576460752303423489", 10)
        .unwrap()
        .into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_to_list_3(
            big_integer_term,
            2.into_process(&mut process),
            3.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}

#[test]
fn with_float_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let float_term = 1.0.into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_to_list_3(
            float_term,
            2.into_process(&mut process),
            3.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}

#[test]
fn with_local_pid_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let local_pid_term = Term::local_pid(0, 0, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_list_3(
            local_pid_term,
            2.into_process(&mut process),
            3.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}

#[test]
fn with_external_pid_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let external_pid_term = Term::external_pid(1, 0, 0, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_list_3(
            external_pid_term,
            2.into_process(&mut process),
            3.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}

#[test]
fn with_tuple_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let tuple_term = Term::slice_to_tuple(&[], &mut process);

    assert_bad_argument!(
        erlang::binary_to_list_3(
            tuple_term,
            2.into_process(&mut process),
            3.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}

#[test]
fn with_map_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let map_term = Term::slice_to_map(&[], &mut process);

    assert_bad_argument!(
        erlang::binary_to_list_3(
            map_term,
            2.into_process(&mut process),
            3.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}

#[test]
fn with_heap_binary_with_start_less_than_stop_returns_list_of_bytes() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let heap_binary_term = Term::slice_to_binary(&[0, 1, 2], &mut process);

    assert_eq_in_process!(
        erlang::binary_to_list_3(
            heap_binary_term,
            2.into_process(&mut process),
            3.into_process(&mut process),
            &mut process
        ),
        Ok(Term::cons(
            1.into_process(&mut process),
            Term::EMPTY_LIST,
            &mut process
        )),
        &mut process
    );
}

#[test]
fn with_heap_binary_with_start_equal_to_stop_returns_list_of_single_byte() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let heap_binary_term = Term::slice_to_binary(&[0, 1, 2], &mut process);

    assert_eq_in_process!(
        erlang::binary_to_list_3(
            heap_binary_term,
            2.into_process(&mut process),
            2.into_process(&mut process),
            &mut process
        ),
        Ok(Term::cons(
            1.into_process(&mut process),
            Term::EMPTY_LIST,
            &mut process
        )),
        &mut process
    );
}

#[test]
fn with_heap_binary_with_start_greater_than_stop_returns_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let heap_binary_term = Term::slice_to_binary(&[0, 1, 2], &mut process);

    assert_bad_argument!(
        erlang::binary_to_list_3(
            heap_binary_term,
            3.into_process(&mut process),
            2.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}

#[test]
fn with_subbinary_with_start_less_than_stop_returns_list_of_bytes() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    // <<1::1, 0, 1, 2>>
    let binary_term = Term::slice_to_binary(&[128, 0, 129, 0b0000_0000], &mut process);
    let subbinary_term = Term::subbinary(binary_term, 0, 1, 3, 0, &mut process);

    assert_eq_in_process!(
        erlang::binary_to_list_3(
            subbinary_term,
            2.into_process(&mut process),
            3.into_process(&mut process),
            &mut process
        ),
        Ok(Term::cons(
            1.into_process(&mut process),
            Term::EMPTY_LIST,
            &mut process
        )),
        process
    );
}

#[test]
fn with_subbinary_with_start_equal_to_stop_returns_list_of_single_byte() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    // <<1::1, 0, 1, 2>>
    let binary_term = Term::slice_to_binary(&[128, 0, 129, 0b0000_0000], &mut process);
    let subbinary_term = Term::subbinary(binary_term, 0, 1, 3, 0, &mut process);

    assert_eq_in_process!(
        erlang::binary_to_list_3(
            subbinary_term,
            2.into_process(&mut process),
            2.into_process(&mut process),
            &mut process
        ),
        Ok(Term::cons(
            1.into_process(&mut process),
            Term::EMPTY_LIST,
            &mut process
        )),
        process
    );
}

#[test]
fn with_subbinary_with_start_greater_than_stop_returns_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    // <<1::1, 0, 1, 2>>
    let binary_term = Term::slice_to_binary(&[128, 0, 129, 0b0000_0000], &mut process);
    let subbinary_term = Term::subbinary(binary_term, 0, 1, 3, 0, &mut process);

    assert_bad_argument!(
        erlang::binary_to_list_3(
            subbinary_term,
            3.into_process(&mut process),
            2.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}
