pub type BitType = u64;
pub trait UintGet {
    fn get(&self) -> BitType;
}

pub struct BitMask {
    pub value: BitType,
}

pub enum BitmaskSetup {
    Value(BitType),
    None,
    All,
}

impl BitMask {
    pub const fn new(value: BitmaskSetup) -> BitMask {
        let value = match value {
            BitmaskSetup::Value(value) => value,
            BitmaskSetup::None => 0,
            BitmaskSetup::All => !0,
        };
        BitMask { value }
    }

    pub const fn set_bit(mut self, bit: BitType, state: bool) -> Self {
        let shift = 1 << bit;
        if state {
            self.value |= shift;
        } else {
            self.value &= !shift;
        }
        self
    }

    pub fn allow(mut self, bit: impl UintGet) -> Self {
        self.value |= 1 << bit.get();
        //printout debug in binary format
        //bit and value
        // trace!("bit: {:b}", bit.get());
        // trace!("value: {:b}", self.value);
        self
    }

    pub fn deny(mut self, bit: impl UintGet) -> Self {
        self.value &= !(1 << bit.get());
        // debug!("bit: {:b}", 1 << bit.get());
        // debug!("value: {:b}", self.value);
        self
    }

    pub fn check(&self, bit: &impl UintGet) -> bool {
        let answer = self.value & (1 << bit.get());
        answer != 0
    }

    pub const fn default() -> BitMask {
        BitMask { value: 0 }
    }
}
