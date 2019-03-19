extern crate bchlib_sys as ffi;

use std::ptr::null;

struct BCH(ffi::bch_control);

impl BCH {
    fn init(m: i32, t: i32, poly: u32) -> BCH {
        let bch = unsafe { *ffi::init_bch(m, t, poly) };
        BCH(bch)
    }

    fn decode(&mut self, msg: &[u8], ecc: &[u8], errloc: &mut[u32]) -> i32 {
        let err = unsafe {
            ffi::decodebits_bch(&mut self.0, msg.as_ptr(), ecc.as_ptr(), errloc.as_mut_ptr())
        };
        err
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let mut bch = BCH::init(5, 2, 37);
        let msg: [u8; 21] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let ecc: [u8; 10] = [1, 1, 1, 0, 1, 1, 0, 1, 0, 0];
        let mut errloc: [u32; 2] = [0, 0];
        bch.decode(&msg, &ecc, &mut errloc);
        assert_eq!(errloc[0], 0);
        assert_eq!(errloc[1], 0);
    }

    #[test]
    fn test_decode_err() {
        let mut bch = BCH::init(5, 2, 37);
        let msg: [u8; 21] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let ecc: [u8; 10] = [1, 1, 1, 0, 1, 1, 0, 1, 0, 1];
        let mut errloc: [u32; 2] = [0, 0];
        bch.decode(&msg, &ecc, &mut errloc);
        assert_eq!(errloc[0], 30);
        assert_eq!(errloc[1], 0);
    }
}
