/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct Foo<T> {
    pub t_member: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl <T> Default for Foo<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash)]
pub struct Bar {
    pub member: ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct Quux<V> {
    pub v_member: V,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<V>>,
}
impl <V> Default for Quux<V> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash)]
pub struct Lobo {
    pub also_member: ::std::os::raw::c_char,
}
pub type AliasWithAnonType = ::std::os::raw::c_char;
