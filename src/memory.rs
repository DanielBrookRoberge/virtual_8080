use std::fmt;

pub struct Memory {
    m: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { m: vec![0; 65536] }
    }

    pub fn get(&self, addr: u16) -> u8 {
        self.m[addr as usize]
    }

    pub fn set(&mut self, addr: u16, data: u8) {
        self.m[addr as usize] = data;
    }

    pub fn load(&mut self, base: u16, data: Vec<u8>) {
        let mut addr = base;
        for byte in data.iter() {
            self.set(addr, *byte);
            addr += 1;
        }
    }

    pub fn view(&self, start: u16, end: u16) -> &[u8] {
        &self.m[(start as usize)..=(end as usize)]
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        assert_eq!(Memory::new().m.len(), 65536);
        assert!(Memory::new().m.iter().all(|n| *n == 0))
    }

    #[test]
    fn get_test() {
        let mut mem = Memory::new();

        mem.m[0x1020] = 0x93;
        assert_eq!(mem.get(0x1020), 0x93);
    }

    #[test]
    fn set_test() {
        let mut mem = Memory::new();

        mem.set(0x8080, 0xbc);
        assert_eq!(mem.m[0x8080], 0xbc);
    }

    #[test]
    fn load_test() {
        let mut mem = Memory::new();

        let test = vec![0x02, 0x08, 0x22, 0x8a];
        mem.load(0xabcd, test);
        assert_eq!(mem.m[0xabcd], 0x02);
        assert_eq!(mem.m[0xabce], 0x08);
        assert_eq!(mem.m[0xabcf], 0x22);
        assert_eq!(mem.m[0xabd0], 0x8a);
    }

    #[test]
    fn view_test() {
        let mut mem = Memory::new();

        mem.m[0x1010] = 0x11;
        mem.m[0x1011] = 0x12;
        mem.m[0x1012] = 0x13;
        mem.m[0x1013] = 0x14;

        let buffer = mem.view(0x1010, 0x1013);
        assert_eq!(buffer[0], 0x11);
        assert_eq!(buffer[1], 0x12);
        assert_eq!(buffer[2], 0x13);
        assert_eq!(buffer[3], 0x14);
        assert_eq!(buffer.len(), 4);
    }
}
