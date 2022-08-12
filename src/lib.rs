#![deny(unsafe_op_in_unsafe_fn)]
// struct A;
// void a_create(A**);
// void a_free(A*);
// void a_print(A*)
pub use a1::A;

#[no_mangle]
pub extern "C" fn a_create() -> *mut A {
    Box::into_raw(Box::new(A::new(10)))
}

#[no_mangle]
pub extern "C" fn a_print(v: &A) {
    println!("a = {:?}", v);
}

#[no_mangle]
pub unsafe extern "C" fn a_free(v: *mut A) {
    unsafe { Box::from_raw(v) };
}
