
pub use ::Material;

// Materials stored as a bit sequence, in [generator, chip] pairs, with low-bit = generator;
// 0xA = 1010
// 0x5 = 0101
const CHIP_MASK: u32 = 0x_AA_AA_AA_AA_u32; // alternating, with low bit = false.
const GEN_MASK: u32 = 0x_55_55_55_55_u32; // alternating, with low bit = true.


#[derive(Debug,PartialEq,Eq,Clone,Copy)]
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

//        println!("chip bits:   {:032b}", chip_bits);
//        println!("shift chips: {:032b}", chip_bits >> 1);
//        println!("gen bits:    {:032b}", gen_bits);
//        println!("unmatched    {:032b}", unmatched_chips);

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
}