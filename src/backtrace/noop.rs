#[inline(always)]
pub fn trace(_cb: &mut FnMut(&super::Frame) -> bool) {}

pub struct Frame;

impl Frame {
    pub fn ip(&self) -> *mut u8 {
        0 as *mut _
    }

    pub fn symbol_address(&self) -> *mut u8 {
        0 as *mut _
    }
}
