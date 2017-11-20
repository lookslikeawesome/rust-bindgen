/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Copy, Clone)]
pub union foo {
    pub a: ::std::os::raw::c_int,
    pub __bindgen_anon_1: foo__bindgen_ty_1,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1 {
    pub _bitfield_1: u32,
    pub __bindgen_align: [u32; 0usize],
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1))
    );
}
impl foo__bindgen_ty_1 {
    #[inline]
    pub fn b(&self) -> ::std::os::raw::c_int {
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                4usize,
            )
        };
        let mask = 0x7f as u32;
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_b(&mut self, val: ::std::os::raw::c_int) {
        let mask = 0x7f as u32;
        let val = val as u32 as u32;
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                4usize,
            )
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &unit_field_val as *const _ as *const u8,
                &mut self._bitfield_1 as *mut _ as *mut u8,
                4usize,
            );
        }
    }
    #[inline]
    pub fn c(&self) -> ::std::os::raw::c_int {
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                4usize,
            )
        };
        let mask = 0xffffff80 as u32;
        let val = (unit_field_val & mask) >> 7usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_c(&mut self, val: ::std::os::raw::c_int) {
        let mask = 0xffffff80 as u32;
        let val = val as u32 as u32;
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                4usize,
            )
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 7usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &unit_field_val as *const _ as *const u8,
                &mut self._bitfield_1 as *mut _ as *mut u8,
                4usize,
            );
        }
    }
    #[inline]
    pub fn new_bitfield_1(b: ::std::os::raw::c_int, c: ::std::os::raw::c_int) -> u32 {
        ((0 | ((b as u32 as u32) << 0usize) & (0x7f as u32))
            | ((c as u32 as u32) << 7usize) & (0xffffff80 as u32))
    }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        4usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<foo>())).a as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(a))
    );
}
impl Default for foo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
