use crate::conf::config::AppConfig;

#[derive(Clone, PartialEq)]
pub enum Strategy {
    Contains,
    Startswith,
    Trailing,
}

pub trait Score {
    fn score(&self, config: &AppConfig, address: &String) -> u64;
}

impl Score for Strategy {
    fn score(&self, config: &AppConfig, address: &String) -> u64 {
        match self {
            Strategy::Startswith => {
                let mut _s = 0;
                for (i, c) in config.pattern.chars().enumerate() {
                    if address.chars().nth(i).unwrap() == c {
                        _s += 1;
                    }
                }
                _s
            }

            Strategy::Contains => {
                if address.contains(&config.pattern) {
                    return 1;
                }
                0
            }
            Strategy::Trailing => {
                let mut _s = 0;
                for c in address.chars() {
                    if c == config.pattern.chars().next().unwrap() {
                        _s += 1;
                    } else {
                        break;
                    }
                }
                _s
            }
        }
    }
}
