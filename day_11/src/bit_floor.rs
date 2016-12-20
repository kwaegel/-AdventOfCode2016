
pub use ::Material;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// Materials stored as a bit sequence, in [generator, chip] pairs, with low-bit = generator;
// 0xA = 1010
// 0x5 = 0101
const CHIP_MASK: u32 = 0x_AA_AA_AA_AA_u32; // alternating, with low bit = false.
const GEN_MASK: u32 = 0x_55_55_55_55_u32; // alternating, with low bit = true.


#[derive(Debug,Clone,Copy)]
pub struct BitFloor {
    bits: u32
}
impl BitFloor {
    pub fn new() -> BitFloor {
        BitFloor { bits: 0u32 }
    }

    pub fn is_safe(&self) -> bool {
        // safe if:
        // * No chips
        // * No generators
        // * No chip without it's associated generator

        let gen_bits = self.bits & GEN_MASK;
        let chip_bits = self.bits & CHIP_MASK;

        // Shifting the chip bits down by one lets us check the corresponding gens.
        let unmatched_chips = (chip_bits >> 1) & !gen_bits;

        chip_bits == 0 || gen_bits == 0 || unmatched_chips == 0
    }

    pub fn add_gen(&mut self, item: Material) {
        //println!("Adding gen in cell {}", item as usize * 2);
        let bit_shift = item as usize * 2;
        self.bits |= 0x1 << bit_shift;
    }

    pub fn add_chip(&mut self, item: Material) {
        //println!("Adding chip in cell {}", item as usize * 2+1);
        let bit_shift = item as usize * 2 + 1;
        self.bits |= 0x1 << bit_shift;
    }

    pub fn is_set(&self, index: usize) -> bool {
        (self.bits & 0x1 << index) > 0
    }
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn set(&mut self, index: usize) {
        self.bits |= 0x1 << index;
    }

    pub fn clear(&mut self, index: usize) {
        self.bits &= !(0x1 << index);
    }

    pub fn num_items(&self) -> u32 {
        self.bits.count_ones()
    }

    pub fn num_pairs(&self) -> u32 {
        // Shifting the chip bits down by one lets us check the corresponding gens.
        let gen_bits = self.bits & GEN_MASK;
        let chip_bits = self.bits & CHIP_MASK;
        ((chip_bits >> 1) & gen_bits).count_ones()
    }

    // All bits that are part of a [chip,gen] pair.
    pub fn paired_bits(&self) -> u32 {
        let gen_bits = self.bits & GEN_MASK;
        let chip_bits = self.bits & CHIP_MASK;

        // This gives us a high bit in the low-order part of each matched pair.
        let paired_gens = (chip_bits >> 1) & gen_bits;
        let paired_bits = paired_gens | (paired_gens << 1);

//        println!("Inside paired_bits()");
//        println!("Gen bits   : {:016b}", gen_bits);
//        println!("Chip bits  : {:016b}", chip_bits);
//        println!("Paired gens: {:016b}", paired_gens);
//        println!("Paired bits: {:016b}", paired_bits);

        paired_bits
    }

    // All bits that are *not* part of a [chip,gen] pair.
    pub fn unpaired_bits(&self) -> u32 {
        self.bits & !self.paired_bits()
    }
}

// -----------------------------------------------------------------------------

impl PartialEq for BitFloor {
    fn eq(&self, other: &BitFloor) -> bool {
        // Note that {Chip,Gen} pairs are equivalent, and can be treated as identical units.
        self.num_pairs() == other.num_pairs()
        && self.unpaired_bits() == other.unpaired_bits()
    }
}
impl Eq for BitFloor {}

// -----------------------------------------------------------------------------

impl Hash for BitFloor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        //println!("Hashing {:16b} with {} pairs and {:?} remaining bits",
        //         self.bits, self.num_pairs(), self.unpaired_bits().count_ones());
        self.num_pairs().hash(state);
        self.unpaired_bits().hash(state);
    }
}

// -----------------------------------------------------------------------------

//#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn floor_test_paired() {
        let mut floor = BitFloor::new();
        assert!(floor.num_pairs() == 0);

        floor.add_gen(Material::Hydrogen);
        floor.add_chip(Material::Hydrogen);
        assert!(floor.num_pairs() == 1);

        floor.add_gen(Material::Lithium);
        assert!(floor.num_pairs() == 1);

        floor.add_chip(Material::Lithium);
        assert!(floor.num_pairs() == 2);
    }

    #[test]
    fn floor_test_unpaired() {
        let mut floor = BitFloor::new();
        assert!(floor.num_pairs() == 0);

        floor.add_gen(Material::Hydrogen);
        println!("\nUnpaired bits: {:016b}, {} ones", floor.unpaired_bits(), floor.unpaired_bits().count_ones());
        assert!(floor.unpaired_bits().count_ones() == 1);
        assert!(floor.paired_bits().count_ones() == 0);


        floor.add_chip(Material::Hydrogen);
        println!("Added second item to make one pair...");
        println!("Paired bits:   {:016b}, {} bits set", floor.paired_bits(), floor.paired_bits().count_ones());
        println!("Unpaired bits: {:016b}, {} bits set", floor.unpaired_bits(), floor.unpaired_bits().count_ones());
        assert!(floor.num_pairs() == 1);
        assert!(floor.paired_bits().count_ones() == 2);
        assert!(floor.unpaired_bits().count_ones() == 0);

        floor.add_chip(Material::Cobalt);
        floor.add_chip(Material::Polonium);
        assert!(floor.unpaired_bits().count_ones() == 2);
    }

    #[test]
    fn floor_test_unpaired2() {
        let mut floor = BitFloor::new();
        assert!(floor.num_pairs() == 0);

        floor.add_chip(Material::Lithium);
        println!("\nUnpaired bits: {:016b}, {} ones", floor.unpaired_bits(), floor.unpaired_bits().count_ones());
        assert!(floor.unpaired_bits().count_ones() == 1);
        assert!(floor.paired_bits().count_ones() == 0);

        println!("Hash: {:?}", hasher(&floor));
    }

    fn hasher<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}