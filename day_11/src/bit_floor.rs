
pub use ::Material;

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
        let paired_gens = ((chip_bits >> 1) & gen_bits).count_ones();
        paired_gens & (paired_gens << 1)
    }

    // All bits that are *not* part of a [chip,gen] pair.
    pub fn unpaired_bits(&self) -> u32 {
        self.bits & !self.paired_bits()
    }

    pub fn unpaired_indices(&self) -> Vec<usize> {
        let mut indices = Vec::with_capacity(14);
        let unpaired = self.unpaired_bits();
        for i in 0..14 {
            if unpaired & (1<<i) > 0 {
                indices.push(i)
            }
        }
        indices
    }

    pub fn first_paired_indices(&self) -> Vec<usize> {
        let unpaired = self.paired_bits();
        for i in 0..14 {
            if unpaired & (1<<i) > 0 {
                return vec!(i, i+1)
            }
        }
        Vec::new()
    }
}

impl PartialEq for BitFloor {
    fn eq(&self, other: &BitFloor) -> bool {
        // Note that {Chip,Gen} pairs are equivalent, and can be treated as identical units.
        self.num_pairs() == other.num_pairs()
        && self.unpaired_bits() == other.unpaired_bits()
    }
}
impl Eq for BitFloor {}