use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Pair {
    pub lprice: String,
    pub curr1: String,
    pub curr2: String, 
}

pub trait PairImpl {
    fn pairImpl(&self) -> Pair;
}

impl PairImpl for Pair {
    fn pairImpl(&self) -> Pair {
        return Pair{
            lprice: self.lprice.to_string(),
            curr1: self.curr1.to_string(),
            curr2: self.curr2.to_string(),
        };
    }
}
