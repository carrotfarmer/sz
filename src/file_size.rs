use std::fmt::Display;

pub struct FileSize {
    value: f64,
    units: String,
}

impl FileSize {
    pub fn new(value: f64) -> Self {
        let (mut val, mut un) = (value, String::from(""));

        match value {
            v if v == 0.0 => {
                val = value;
                un = String::from("B");
            }

            v if v > 1.0 && v <= 1000.0 => {
                val = value;
                un = String::from("B");
            }

            v if v > 1001.0 && v <= 1000000.0 => {
                val = v / 1000.0;
                un = String::from("KB");
            }

            v if v > 1000001.0 && v <= 1000000000.0 => {
                val = v / 1000000.0;
                un = String::from("MB");
            }

            v if v > 1000000001.0 && v <= 1000000000000.0 => {
                val = v / 1000000000.0;
                un = String::from("GB");
            }

            _ => {
                val = val / 1000000000000.0;
                un = String::from("TB");
            }
        }

        Self {
            value: val,
            units: un,
        }
    }
}

impl Display for FileSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.units)
    }
}
