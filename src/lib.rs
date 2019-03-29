extern crate bchlib_sys as ffi;

use std::ptr;

#[derive(Debug)]
pub struct BCH(ffi::bch_control);

impl BCH {
    pub fn init(m: i32, t: i32) -> Result<BCH, &'static str> {
        BCH::init_with_poly(m, t, 0)
    }

    pub fn init_with_poly(m: i32, t: i32, poly: u32) -> Result<BCH, &'static str> {
        unsafe {
            let bch = ffi::init_bch(m, t, poly);
            if bch == ptr::null_mut() {
                Err("Invalid BCH params")
            }
            else {
                Ok(BCH(*bch))
            }
        }
    }

    pub fn decode(&mut self, msg: &[u8], ecc: &[u8], errloc: &mut[u32]) -> i32 {
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
        let mut bch = BCH::init(5, 2).unwrap();
        let msg: [u8; 21] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let ecc: [u8; 10] = [1, 1, 1, 0, 1, 1, 0, 1, 0, 0];
        let mut errloc: [u32; 2] = [0, 0];
        bch.decode(&msg, &ecc, &mut errloc);
        assert_eq!(errloc[0], 0);
        assert_eq!(errloc[1], 0);
    }

    #[test]
    fn test_decode_err() {
        let mut bch = BCH::init(5, 2).unwrap();
        let msg: [u8; 21] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let ecc: [u8; 10] = [1, 1, 1, 0, 1, 1, 0, 1, 0, 0];
        let mut errloc: [u32; 2] = [0, 0];
        bch.decode(&msg, &ecc, &mut errloc);
        assert_eq!(errloc[0], 9);
        assert_eq!(errloc[1], 0);
    }

    #[test]
    fn test_sync_codeword() {
        let mut bch = BCH::init(5, 2).unwrap();
        let msg: [u8; 21] = [0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0];
        let ecc: [u8; 10] = [1, 0, 1, 1, 1, 0, 1, 1, 0, 0];
        let mut errloc: [u32; 2] = [0, 0];
        bch.decode(&msg, &ecc, &mut errloc);
        assert_eq!(errloc[0], 0);
        assert_eq!(errloc[1], 0);
    }

    #[test]
    fn test_init_fail() {
        let bch = BCH::init_with_poly(5, 2, 1897);
        assert_eq!(bch.is_err(), true);
    }
}
