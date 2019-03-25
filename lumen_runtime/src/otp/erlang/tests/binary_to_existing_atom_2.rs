use super::*;

use std::sync::{Arc, RwLock};

use num_traits::Num;

use crate::environment::{self, Environment};
use crate::process::IntoProcess;

#[test]
fn with_atom_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let atom_term = Term::str_to_atom("atom", DoNotCare, &mut process).unwrap();
    let encoding_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(atom_term, encoding_term, &mut process),
        &mut process
    );
}

#[test]
fn with_local_reference_errors_badarg() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let binary = Term::local_reference(&mut process);
    let encoding_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(binary, encoding_term, &mut process),
        &mut process
    );
}

#[test]
fn with_empty_list_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let encoding_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(Term::EMPTY_LIST, encoding_term, &mut process),
        &mut process
    );
}

#[test]
fn with_list_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let list_term = list_term(&mut process);
    let encoding_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(list_term, encoding_term, &mut process),
        &mut process
    );
}

#[test]
fn with_small_integer_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let small_integer_term = 0usize.into_process(&mut process);
    let encoding_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(small_integer_term, encoding_term, &mut process),
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
    let encoding_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(big_integer_term, encoding_term, &mut process),
        &mut process
    );
}

#[test]
fn with_float_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let float_term = 1.0.into_process(&mut process);
    let encoding_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(float_term, encoding_term, &mut process),
        &mut process
    );
}

#[test]
fn with_tuple_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let tuple_term = Term::slice_to_tuple(
        &[0.into_process(&mut process), 1.into_process(&mut process)],
        &mut process,
    );
    let encoding_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(tuple_term, encoding_term, &mut process),
        &mut process
    );
}

#[test]
fn with_map_is_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let map_term = Term::slice_to_map(&[], &mut process);
    let encoding_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(map_term, encoding_term, &mut process),
        &mut process
    );
}

#[test]
fn with_heap_binary_without_encoding_atom_returns_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let heap_binary_term = Term::slice_to_binary(&[], &mut process);

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(
            heap_binary_term,
            0.into_process(&mut process),
            &mut process
        ),
        &mut process
    );
}

#[test]
fn with_heap_binary_with_invalid_encoding_atom_returns_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let heap_binary_term = Term::slice_to_binary(&[], &mut process);
    let invalid_encoding_term =
        Term::str_to_atom("invalid_encoding", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(heap_binary_term, invalid_encoding_term, &mut process),
        &mut process
    );
}

#[test]
fn with_heap_binary_with_valid_encoding_without_existing_atom_returns_atom() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let heap_binary_term = Term::slice_to_binary("😈".as_bytes(), &mut process);
    let latin1_atom_term = Term::str_to_atom("latin1", DoNotCare, &mut process).unwrap();
    let unicode_atom_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();
    let utf8_atom_term = Term::str_to_atom("utf8", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(heap_binary_term, latin1_atom_term, &mut process),
        &mut process
    );
    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(heap_binary_term, unicode_atom_term, &mut process),
        &mut process
    );
    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(heap_binary_term, utf8_atom_term, &mut process),
        &mut process
    );
}

#[test]
fn with_heap_binary_with_valid_encoding_with_existing_atom_returns_atom() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let heap_binary_term = Term::slice_to_binary("😈".as_bytes(), &mut process);
    let latin1_atom_term = Term::str_to_atom("latin1", DoNotCare, &mut process).unwrap();
    let unicode_atom_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();
    let utf8_atom_term = Term::str_to_atom("utf8", DoNotCare, &mut process).unwrap();
    let atom_term = Term::str_to_atom("😈", DoNotCare, &mut process).unwrap();

    assert_eq_in_process!(
        erlang::binary_to_existing_atom_2(heap_binary_term, latin1_atom_term, &mut process),
        Ok(atom_term),
        process
    );
    assert_eq_in_process!(
        erlang::binary_to_existing_atom_2(heap_binary_term, unicode_atom_term, &mut process),
        Ok(atom_term),
        process
    );
    assert_eq_in_process!(
        erlang::binary_to_existing_atom_2(heap_binary_term, utf8_atom_term, &mut process),
        Ok(atom_term),
        process
    );
}

#[test]
fn with_subbinary_with_bit_count_returns_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let binary_term =
        Term::slice_to_binary(&[0b0000_00001, 0b1111_1110, 0b1010_1011], &mut process);
    let subbinary_term = Term::subbinary(binary_term, 0, 7, 2, 1, &mut process);
    let unicode_atom_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(subbinary_term, unicode_atom_term, &mut process),
        &mut process
    )
}

#[test]
fn with_subbinary_without_bit_count_without_existing_atom_returns_bad_argument() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let binary_term = Term::slice_to_binary("😈🤘".as_bytes(), &mut process);
    let subbinary_term = Term::subbinary(binary_term, 4, 0, 4, 0, &mut process);
    let unicode_atom_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();

    assert_bad_argument!(
        erlang::binary_to_existing_atom_2(subbinary_term, unicode_atom_term, &mut process),
        &mut process
    )
}

#[test]
fn with_subbinary_without_bit_count_with_existing_atom_returns_atom_with_bytes() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let binary_term = Term::slice_to_binary("😈🤘".as_bytes(), &mut process);
    let subbinary_term = Term::subbinary(binary_term, 4, 0, 4, 0, &mut process);
    let unicode_atom_term = Term::str_to_atom("unicode", DoNotCare, &mut process).unwrap();
    let atom_term = Term::str_to_atom("🤘", DoNotCare, &mut process);

    assert_eq_in_process!(
        erlang::binary_to_existing_atom_2(subbinary_term, unicode_atom_term, &mut process),
        atom_term,
        process
    )
}
