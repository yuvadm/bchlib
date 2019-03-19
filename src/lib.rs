extern crate bchlib_sys as ffi;

struct BCH(ffi::bch_control);

impl BCH {
    fn init(m: i32, t: i32, poly: u32) -> BCH {
        let bch = unsafe { *ffi::init_bch(m, t, poly) };
        BCH(bch)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bch = BCH::init(5, 2, 0);
    }
}
