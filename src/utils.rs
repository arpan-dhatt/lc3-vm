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

#[cfg(test)]
mod tests {
    use super::Condition;

    #[test]
    fn test_condition_is_satisfied() {
        let cond = Condition { n: true, z: false, p: false };
        let test = Condition { n: true, z: true, p: false };
        assert_eq!(cond.is_satisfied_by(&test), true);
        assert_eq!(test.is_satisfied_by(&cond), true);

        let cond = Condition { n: false, z: true, p: false };
        let test = Condition { n: true, z: false, p: false };
        assert_eq!(cond.is_satisfied_by(&test), false);
        assert_eq!(test.is_satisfied_by(&cond), false);

    }
}
