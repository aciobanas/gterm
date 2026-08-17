use crate::ratio::Ratio;

/// prefixes for units
pub struct Prefix;

impl Prefix {
    pub const FEMTO: Ratio = Ratio::new(1, 1_000_000_000_000_000);
    pub const PICO: Ratio = Ratio::new(1, 1_000_000_000_000);
    pub const NANO: Ratio = Ratio::new(1, 1_000_000_000);
    pub const MICRO: Ratio = Ratio::new(1, 1_000_000);
    pub const MILLI: Ratio = Ratio::new(1, 1_000);
    pub const CENTI: Ratio = Ratio::new(1, 100);
    pub const DECI: Ratio = Ratio::new(1, 10);
    pub const DECA: Ratio = Ratio::new(10, 1);
    pub const HECTO: Ratio = Ratio::new(100, 1);
    pub const KILO: Ratio = Ratio::new(1_000, 1);
    pub const MEGA: Ratio = Ratio::new(1_000_000, 1);
    pub const GIGA: Ratio = Ratio::new(1_000_000_000, 1);
    pub const TERA: Ratio = Ratio::new(1_000_000_000_000, 1);
    pub const PETA: Ratio = Ratio::new(1_000_000_000_000_000, 1);
}
