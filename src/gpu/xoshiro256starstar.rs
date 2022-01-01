const LONG_JUMP: [u64; 4] = [0x76e15d3efefdcbbf, 0xc5004e441c522fb3, 0x77710069854ee241, 0x39109bb02acbe635];

#[derive(Clone,Copy)]
pub struct Xoshiro256StarStar {
    pub(crate) state: [u64; 4]
}

pub struct Xoshiro256StarStarStateIter {
    current: Xoshiro256StarStar
}

fn rotl(x: u64, k: i32) -> u64 {
    return (x << k) | (x >> (64 - k));
}

impl Xoshiro256StarStar {
    pub fn new(seed: &[u64;4]) -> Self{
        let mut state = [0u64;4];
        state.copy_from_slice(seed);
        return Self{state};
    }

    pub fn next(&mut self) -> u64 {
        let result = rotl(self.state[1] * 5, 7) * 9;
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;

        self.state[3] = rotl(self.state[3], 45);

        return result;
    }

    pub fn long_jump(&mut self)  {
        let mut s0 = 0u64;
        let mut s1 = 0u64;
        let mut s2 = 0u64;
        let mut s3 = 0u64;
        for i in 0..LONG_JUMP.len() {
            for b in 0..64 {
                if LONG_JUMP[i] & 1u64 << b != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                    s2 ^= self.state[2];
                    s3 ^= self.state[3];
                }
                self.next();
            }

            self.state[0] = s0;
            self.state[1] = s1;
            self.state[2] = s2;
            self.state[3] = s3;
        }
    }

    pub fn iter_jump_state(&self) -> impl Iterator<Item=[u64; 4]> {
        let current = Xoshiro256StarStar::new(&self.state);
        return Xoshiro256StarStarStateIter{ current }
    }
}

impl Iterator for Xoshiro256StarStarStateIter {
    type Item = [u64; 4];

    fn next(&mut self) -> Option<[u64;4]> {
        self.current.long_jump();
        return Some(self.current.state)
    }
}