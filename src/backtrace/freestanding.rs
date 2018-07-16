use x86::current::registers;

#[derive(Debug, Clone)]
pub struct Frame {
    rbp: u64,
    rsp: u64,
    rip: u64,
}

impl Frame {
    pub fn ip(&self) -> *mut u8 {
        (self.rip - 1) as *mut u8
    }

    pub fn symbol_address(&self) -> *mut u8 {
        0 as *mut u8
    }
}

#[inline(always)]
pub fn trace(cb: &mut FnMut(&super::Frame) -> bool) {
    let mut curframe = Frame {
        rbp: registers::rbp(),
        rsp: registers::rsp(),
        rip: registers::rip(),
    };

    loop {
        let mut bomb = ::Bomb { enabled: true };
        let ctxt = super::Frame {
            inner: curframe.clone(),
        };
        let keep_going = cb(&ctxt);
        bomb.enabled = false;
        let reached_end = curframe.rbp == 0;

        if keep_going && !reached_end {
            unsafe {
                curframe.rip = *((curframe.rbp + 8) as *mut u64);
                curframe.rsp = curframe.rbp;
                curframe.rbp = *(curframe.rbp as *mut u64);
            }
        } else {
            break;
        }
    }
}
