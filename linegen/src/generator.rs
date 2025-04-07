use rand::{Rng, distr::Alphanumeric};

/// LineGenerator is an iterator that generates a random line (String)
/// that includes some substrings "incr" and "decr" (always in lowercase).
/// It's infinite so it always returns some value, never None.
///
/// Example:
/// let mut lg = LineGenerator::default();
/// println!("{}", lg.next().unwrap());
///
/// By the default, the generator generates approximately the
/// same number of "incr" and "decr" strings in a line,
/// but if it's created with LineGenerator::new(bias),
/// it can generate different ratio (higher bias means more "incr" values)
pub struct LineGenerator {
    bias: f64,
}

impl LineGenerator {
    pub fn new(bias: f64) -> Self {
        Self { bias }
    }
}

impl Default for LineGenerator {
    fn default() -> Self {
        Self { bias: 0.5 }
    }
}

impl Iterator for LineGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::rng();
        let mut line = String::default();
        let n_operations = rng.random::<u16>() % 100;
        for _ in 0..n_operations {
            let salt_len = (rng.random::<u16>() % 10) as usize;
            let salt = (&mut rng)
                .sample_iter(&Alphanumeric)
                .take(salt_len)
                .map(char::from)
                .collect::<String>();
            line += salt.as_str();
            line += if rng.random::<f64>() <= self.bias {
                "incr"
            } else {
                "decr"
            };
        }
        let salt_len = (rng.random::<u16>() % 10) as usize;
        let salt = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(salt_len)
            .map(char::from)
            .collect::<String>();
        line += salt.as_str();
        Some(line)
    }
}

#[cfg(test)]
mod tests {
    use super::LineGenerator;

    #[test]
    fn test_generate() {
        let mut lg = LineGenerator::default();
        assert!(lg.next().unwrap().contains("incr"));
        assert!(lg.next().unwrap().contains("decr"));
    }

    #[test]
    fn test_bias1() {
        let mut lg = LineGenerator::new(1.0);
        assert!(!lg.next().unwrap().contains("decr"));
    }

    #[test]
    fn test_bias0() {
        let mut lg = LineGenerator::new(0.0);
        assert!(!lg.next().unwrap().contains("incr"));
    }
}
