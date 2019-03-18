
pub struct BCH {
    n: u32,
}

impl BCH {
    pub fn new(n: u32) -> BCH {
        BCH {
            n
        }
    }

    pub fn decode(self) -> u32{
        return self.n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bch = BCH::new(3);
        assert_eq!(bch.decode(), 3);
    }
}
