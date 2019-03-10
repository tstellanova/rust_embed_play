#[no_mangle]
pub extern fn remote_add(lhs: u32, rhs: u32) -> u32 {
    lhs + rhs
}

#[no_mangle]
pub extern fn happy_testo(first: u32, second: u32) -> u32 {
    let foo = Happy {
        first, second
    };

    foo.testo()
}

pub struct Happy {
    first: u32,
    second: u32
}

impl Happy {
    pub  fn testo(&self) -> u32 {
        self.first + self.second
    }
}
