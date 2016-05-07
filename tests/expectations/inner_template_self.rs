/* automatically generated by rust-bindgen */


#![feature(const_fn)]
#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Struct_LinkedList<T> {
    pub next: *mut Struct_LinkedList<T>,
    pub prev: *mut Struct_LinkedList<T>,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_InstantiateIt {
    pub m_list: Struct_LinkedList<::std::os::raw::c_int>,
}
impl ::std::clone::Clone for Struct_InstantiateIt {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_InstantiateIt() {
    assert_eq!(::std::mem::size_of::<Struct_InstantiateIt>() , 16usize);
    assert_eq!(::std::mem::align_of::<Struct_InstantiateIt>() , 8usize);
}
