use SymbolName;

pub fn resolve(_addr: *mut u8, _cb: &mut FnMut(&super::Symbol)) {
}

pub struct Symbol;

impl Symbol {
    pub fn name(&self) -> Option<SymbolName> {
        None
    }

    pub fn addr(&self) -> Option<*mut u8> {
        None
    }

    pub fn filename(&self) -> Option<&str> {
        None
    }

    pub fn lineno(&self) -> Option<u32> {
        None
    }
}
