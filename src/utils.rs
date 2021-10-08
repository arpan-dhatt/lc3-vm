pub fn sext(val: i16, bits: i32) -> i16 {
    let uval = val as u16;
    match (uval >> (bits - 1)) & 0b1 {
        0b1 => (0xFFFF << bits | uval) as i16,
        _ => val,
    }
}

#[derive(Debug, PartialEq)]
pub struct Condition {
    pub n: bool,
    pub z: bool,
    pub p: bool,
}

impl Condition {
    pub fn is_satisfied_by(&self, other: &Condition) -> bool {
        (self.n && other.n) || (self.z && other.z) || (self.p && other.p)
    }
}

impl Default for Condition {
    fn default() -> Self {
        Condition {
            n: false,
            z: false,
            p: false,
        }
    }
}
