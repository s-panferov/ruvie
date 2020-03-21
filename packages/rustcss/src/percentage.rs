use std::fmt::Display;

#[derive(Hash)]
pub struct Percentage(u64);

impl Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

pub trait ToPercentage {
    fn percent(self) -> Percentage;
}

impl ToPercentage for u64 {
    fn percent(self) -> Percentage {
        Percentage(self)
    }
}
