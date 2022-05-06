use std::str::FromStr;

use approx::AbsDiffEq;

#[derive(Debug, PartialEq)]
pub struct Fc(pub usize, pub usize, pub usize, pub usize, pub f64);

impl FromStr for Fc {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_whitespace().collect::<Vec<_>>();
        let e = Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to parse Fc from string",
        ));
        if s.len() != 5 {
            return e;
        } else {
            let us = s[..4].iter().map(|s| s.parse::<usize>());
            if us.clone().any(|s| s.is_err()) {
                return e;
            }
            let us = us.flatten().collect::<Vec<_>>();
            let f = match s[4].parse::<f64>() {
                Ok(f) => f,
                Err(_) => return e,
            };
            Ok(Fc(us[0], us[1], us[2], us[3], f))
        }
    }
}

impl AbsDiffEq for Fc {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1e-12
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0 == other.0
            && self.1 == other.1
            && self.2 == other.2
            && self.3 == other.3
            && self.4.abs_diff_eq(&other.4, epsilon)
    }
}