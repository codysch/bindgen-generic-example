#![deny(unsafe_op_in_unsafe_fn)]
// struct A;
// void a_create(A**);
// void a_free(A*);
// void a_print(A*)
//

#[no_mangle]
pub extern "C" fn a_create() -> *mut a1::A<usize> {
    Box::into_raw(Box::new(a1::A::new(10)))
}

#[no_mangle]
pub extern "C" fn a_print(v: &a1::A<usize>) {
    println!("a = {:?}", v);
}

#[no_mangle]
pub unsafe extern "C" fn a_free(v: *mut a1::A<usize>) {
    unsafe { Box::from_raw(v) };
}
