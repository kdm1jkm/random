pub trait Generator {
    fn next_value(&mut self) -> u32;
}

pub struct LGC {
    a: u32,
    c: u32,
    m: u32,
    value: u32,
}

impl LGC {
    pub fn new(seed: u32) -> Self {
        LGC {
            a: 1103515245,
            c: 12345,
            m: 0x80000000,
            value: seed,
        }
    }
}

impl Generator for LGC {
    fn next_value(&mut self) -> u32 {
        self.value = (self.value.wrapping_mul(self.a).wrapping_add(self.c)) % self.m;
        self.value
    }
}

pub struct MSM {
    value: u32,
    original_length: isize,
}

impl MSM {
    pub fn new(seed: u32) -> Self {
        let original_length = seed.to_string().len() as isize;
        MSM {
            value: seed,
            original_length,
        }
    }
}

impl Generator for MSM {
    fn next_value(&mut self) -> u32 {
        self.value = self.value.wrapping_mul(self.value);
        let remain_length = self.value.to_string().len() as isize - self.original_length;
        if remain_length < 0 {
            return self.value;
        }
        self.value /= 10u32.pow((remain_length / 2) as u32);
        self.value %= 10u32.pow(self.original_length as u32);

        self.value
    }
}

pub struct XorShift {
    value: u32,
}

impl XorShift {
    pub fn new(seed: u32) -> Self {
        XorShift { value: seed }
    }
}

impl Generator for XorShift {
    fn next_value(&mut self) -> u32 {
        self.value ^= self.value << 13;
        self.value ^= self.value >> 17;
        self.value ^= self.value << 5;

        self.value
    }
}
