const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;

pub struct SnowKg(pub f64);

impl SnowKg {
    pub fn new(kg: f64) -> Self {
        SnowKg(kg)
    }
}

pub struct SnowLb(pub f64);

impl SnowLb {
    pub fn new(lb: f64) -> Self {
        SnowLb(lb)
    }
}

pub struct Snowball(pub i64);

impl Snowball {
    pub fn new(snowballs: i64) -> Self {
        Snowball(snowballs)
    }
}

impl From<SnowKg> for Snowball {
    // 1. Implement the conversion from SnowKg to Snowball
    fn from(weight: SnowKg) -> Self {
        let value = weight.0 / SNOWBALL_WEIGHT_KG;
        Snowball(value.round() as i64)
    }
}

impl From<SnowLb> for Snowball {
    // 2. Implement the same for SnowLb
    fn from(weight: SnowLb) -> Self {
        let value = weight.0 / SNOWBALL_WEIGHT_LB;
        Snowball(value.round() as i64)
    }
}

