#[derive(PartialEq, Debug, Default)]
pub struct Flags {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Default::default()
    }

    pub fn serialize(&self) -> u8 {
        (self.z as u8)
            | (self.s as u8) << 1
            | (self.p as u8) << 2
            | (self.cy as u8) << 3
            | (self.ac as u8) << 4
    }

    pub fn deserialize(&mut self, flags: u8) {
        self.z = (flags & 0x01) != 0;
        self.s = (flags & 0x02) != 0;
        self.p = (flags & 0x04) != 0;
        self.cy = (flags & 0x08) != 0;
        self.ac = (flags & 0x10) != 0;
    }

    pub fn set_z(&mut self, n: u8) {
        self.z = n == 0;
    }

    pub fn set_s(&mut self, n: u8) {
        self.s = (n & 0x80) != 0;
    }

    pub fn set_p(&mut self, n: u8) {
        self.p = (n.count_ones() & 0x01) == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_test() {
        let mut flags = Flags::new();

        assert_eq!(flags.serialize(), 0x00);

        flags.z = true;
        assert_eq!(flags.serialize(), 0x01);

        flags.s = true;
        assert_eq!(flags.serialize(), 0x03);
    }

    #[test]
    fn deserialize_test() {
        let mut flags = Flags::new();

        flags.deserialize(0x00);
        assert_eq!(flags, Flags::new());

        flags.deserialize(0x01);
        assert_eq!(
            flags,
            Flags {
                z: true,
                ..Default::default()
            }
        );

        flags.deserialize(0x05);
        assert_eq!(
            flags,
            Flags {
                z: true,
                p: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn set_z_test() {
        let mut flags = Flags::new();

        flags.set_z(20);
        assert_eq!(flags.z, false);

        flags.set_z(0);
        assert_eq!(flags.z, true);
    }

    #[test]
    fn set_s_test() {
        let mut flags = Flags::new();

        flags.set_s(20);
        assert_eq!(flags.s, false);

        flags.set_s(0);
        assert_eq!(flags.s, false);

        flags.set_s(0x80);
        assert_eq!(flags.s, true);

        flags.set_s(0xff);
        assert_eq!(flags.s, true);
    }

    #[test]
    fn set_p_test() {
        let mut flags = Flags::new();

        flags.set_p(0b11110000);
        assert_eq!(flags.p, true);

        flags.set_p(0b10101011);
        assert_eq!(flags.p, false);

        flags.set_p(0);
        assert_eq!(flags.p, true);

        flags.set_p(0xff);
        assert_eq!(flags.p, true);
    }
}
