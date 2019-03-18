
pub struct BCH {
    m: u32,
}

impl BCH {
    pub fn new(m: u32, _t: u32, _prim_poly: u32) -> Option<BCH> {
        let _err: u32 = 0;
        let _i: u32;
        let _words: u32;
        let _genpoly: u32;

        const MIN_M: u32 = 5;
        const MAX_M: u32 = 15;

        if (m < MIN_M) || (m > MAX_M) {
            // `m` values must be between5 and 15 in this implementation
            return None
        }

        Some(BCH {
            m
        })
    }

    pub fn decode(self) -> u32{
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
