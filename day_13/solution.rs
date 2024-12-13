// Import the necessary modules

#[derive(Debug)]
pub enum ParseError {
    // 1. Add variants here (read description)
    NoName,
    NoGoodDeeds,
    NoBadDeeds,
    InvalidGoodDeeds,
    InvalidBadDeeds,
}

// 2. Implement the Error trait for ParseError

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::NoName => write!(f, "Name field is missing"),
            ParseError::NoGoodDeeds => write!(f, "Good deeds field is missing"),
            ParseError::NoBadDeeds => write!(f, "Bad deeds field is missing"),
            ParseError::InvalidGoodDeeds => write!(f, "Good deeds value is invalid"),
            ParseError::InvalidBadDeeds => write!(f, "Bad deeds value is invalid"),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug)]
pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Result<Kid, ParseError> {
        if name == "" {
            return Err(ParseError::NoName);
        }
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Ok(Kid { name, niceness })
    }

    pub fn parse_row(csv_row: &str) -> Result<Kid, ParseError> {
        // 3. Update the code to return meaningful errors
        let mut fields = csv_row.split(',');
        let name = fields.next().ok_or(ParseError::NoName)?.to_string();
        let good_deeds_str = fields.next().map(|s| if s == "" {None} else {Some(s)})
            .ok_or(ParseError::NoGoodDeeds)?;
        if good_deeds_str.is_none() {
            return Err(ParseError::NoGoodDeeds);
        }
        let good_deeds = good_deeds_str.unwrap().parse::<u32>()
            .map_err(|_| ParseError::InvalidGoodDeeds)?;
        let bad_deeds_str = fields.next().map(|s| if s == "" {None} else {Some(s)})
            .ok_or(ParseError::NoBadDeeds)?;
        if bad_deeds_str.is_none() {
            return Err(ParseError::NoBadDeeds);
        }
        let bad_deeds = bad_deeds_str.unwrap().parse::<u32>()
            .map_err(|_| ParseError::InvalidBadDeeds)?;

        Kid::new(name, good_deeds, bad_deeds)
    }

    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = good_deeds / (good_deeds + bad_deeds);

        ratio >= 0.75
    }
}

pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}

