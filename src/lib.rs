use std::ops::{Add, Sub, Mul};

pub struct BCH {
    m: i16,
    t: i16,
    prim_poly: u16,

    n:i16,
}

fn div_round_up(a: i16, b: i16) -> i16 {
    (a + b - 1) / b
}

impl BCH {

    pub fn build_gf_tables(self) {
        let i: u16 = 1;
        let x: u16 = 1;
        //const k: u16 = 1 << deg(self.po);
    }

    /// Create a new BCH object initialized with
    /// `m` - the Galois field order, must be in the 5-15 range
    /// `t` - maximum error correction capability in bits
    /// `prim_poly` - user-provided primitive polynomial
    pub fn new(m: i16, t: i16, prim_poly: u16) -> Option<BCH> {
        let err: u16 = 0;
        let i: u16;
        let genpoly: u32;

        const MIN_M: i16 = 5;
        const MAX_M: i16 = 15;

        if (m < MIN_M) || (m > MAX_M) {
            // `m` values must be between5 and 15 in this implementation
            return None
        }

        let n = (1 << m) - 1;
        let words = div_round_up(m * t, 32);
        let ecc_bytes = div_round_up(m * t, 8);

        let bch = BCH {
            m,
            t,
            prim_poly,
            n
        };

        Some(bch)
    }

    pub fn decode(self) -> i16{
        return self.m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bch_init() {
        let bch = BCH::new(6, 0, 0).unwrap();
        assert_eq!(bch.decode(), 6);
    }

    #[test]
    fn fail_m_values() {
        let bch = BCH::new(4, 0, 0);
        assert_eq!(bch.is_none(), true);
        let bch = BCH::new(16, 0, 0);
        assert_eq!(bch.is_none(), true);
    }

}
