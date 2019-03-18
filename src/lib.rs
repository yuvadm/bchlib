extern crate bchlib_sys as ffi;

pub fn init(m:i32, t:i32, poly: u32) {
    unsafe {
        let bch = ffi::init_bch(m, t, poly);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        init();
    }
}
