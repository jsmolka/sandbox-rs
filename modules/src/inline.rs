pub mod one {
    pub fn func() {
        super::func();
    }
}

pub mod two {
    pub fn func() {
        crate::inline::func();
    }
}

pub fn func() {}
