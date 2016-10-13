#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BitSet(pub u32);

impl BitSet {
    pub const fn zero() -> BitSet {
        BitSet(0)
    }
    pub const fn new(val: u32) -> BitSet {
        BitSet(val)
    }
    pub const fn bits4(b0: usize, b1: usize, b2: usize, b3: usize) -> BitSet {
        BitSet(1 << b0 | 1 << b1 | 1 << b2 | 1 << b3)
    }
    pub fn set(&mut self, index: usize) {
        self.0 |= 1 << index;
    }
    pub fn merge(&self, other: BitSet) -> BitSet {
        return BitSet(self.0 | other.0);
    }
    pub fn get(&self, index: usize) -> bool {
        (self.0 & (1 << index)) != 0
    }
    pub fn empty(&self) -> bool {
        self.0 == 0
    }
    pub fn size(&self) -> usize {
        let mut num = 0;
        let mut b = self.0 as usize;
        while b != 0 {
            num += 1;
            b &= b - 1;
        }
        num
    }
    pub fn neg(&self) -> BitSet {
        BitSet(!self.0)
    }
    pub fn lowest(&self) -> Option<usize> {
        if self.0 == 0 {
            None
        } else {
            let mut num = 0;
            let mut b = self.0 as usize;
            while b & 1 == 0 {
                num += 1;
                b >>= 1;
            }
            Some(num)
        }
    }
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl Iterator for BitSet {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.0 == 0 {
            None
        } else {
            let old = self.0;
            self.0 &= self.0 - 1;
            Some(match (!self.0) & old {
                0x00000001 => 0,
                0x00000002 => 1,
                0x00000004 => 2,
                0x00000008 => 3,
                0x00000010 => 4,
                0x00000020 => 5,
                0x00000040 => 6,
                0x00000080 => 7,
                0x00000100 => 8,
                0x00000200 => 9,
                0x00000400 => 10,
                0x00000800 => 11,
                0x00001000 => 12,
                0x00002000 => 13,
                0x00004000 => 14,
                0x00008000 => 15,
                0x00010000 => 16,
                0x00020000 => 17,
                0x00040000 => 18,
                0x00080000 => 19,
                0x00100000 => 20,
                0x00200000 => 21,
                0x00400000 => 22,
                0x00800000 => 23,
                0x01000000 => 24,
                0x02000000 => 25,
                0x04000000 => 26,
                0x08000000 => 27,
                0x10000000 => 28,
                0x20000000 => 29,
                0x40000000 => 30,
                0x80000000 => 31,
                x => panic!("not a single bit: {:?}", x),
            })
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn merge() {
        assert_eq!(super::BitSet::zero().merge(super::BitSet::zero()),
                   super::BitSet::zero());
        assert_eq!(super::BitSet::new(0b01).merge(super::BitSet::new(0b00)),
                   super::BitSet::new(0b01));
        assert_eq!(super::BitSet::new(0b00).merge(super::BitSet::new(0b10)),
                   super::BitSet::new(0b10));
        assert_eq!(super::BitSet::new(0b01).merge(super::BitSet::new(0b10)),
                   super::BitSet::new(0b11));
        assert_eq!(super::BitSet::new(0b11).merge(super::BitSet::new(0b11)),
                   super::BitSet::new(0b11));
    }

    #[test]
    fn empty() {
        assert_eq!(super::BitSet::new(0b00000000).empty(), true);
        assert_eq!(super::BitSet::new(0b10000000).empty(), false);
        assert_eq!(super::BitSet::new(0b01001100).empty(), false);
        assert_eq!(super::BitSet::new(0b11001101).empty(), false);
        assert_eq!(super::BitSet::new(0b11111111).empty(), false);
    }

    #[test]
    fn size() {
        assert_eq!(super::BitSet::new(0b00000000).size(), 0);
        assert_eq!(super::BitSet::new(0b10000000).size(), 1);
        assert_eq!(super::BitSet::new(0b01001100).size(), 3);
        assert_eq!(super::BitSet::new(0b11001101).size(), 5);
        assert_eq!(super::BitSet::new(0b11111111).size(), 8);
    }

    #[test]
    fn neg() {
        assert_eq!(super::BitSet::new(0x12345678).neg(),
                   super::BitSet::new(!0x12345678));
    }
    #[test]
    fn lowest() {
        assert_eq!(super::BitSet::new(0b00000000).lowest(), None);
        assert_eq!(super::BitSet::new(0b10000000).lowest(), Some(7));
        assert_eq!(super::BitSet::new(0b01001100).lowest(), Some(2));
        assert_eq!(super::BitSet::new(0b11001101).lowest(), Some(0));
        assert_eq!(super::BitSet::new(0b11111111).lowest(), Some(0));
    }
    #[test]
    fn iterate() {
        let mut b = super::BitSet::new(0b01001010);
        assert_eq!(b.next(), Some(1));
        assert_eq!(b.next(), Some(3));
        assert_eq!(b.next(), Some(6));
        assert_eq!(b.next(), None);
    }
}
