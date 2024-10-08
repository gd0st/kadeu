pub mod engine;
pub trait Kadeu {
    type Front;
    type Back;
    fn front(&self) -> &Self::Front;
    fn back(&self) -> &Self::Back;
    fn display_front(&self) -> String;
    fn display_back(&self) -> String;
}

pub enum Score {
    Hit,
    Miss,
}

impl Score {
    pub fn to_string(&self) -> String {
        String::from(match self {
            Self::Hit => "hit",
            Self::Miss => "miss",
        })
    }
}

pub struct Progress<T> {
    item: T,
    score: Option<Score>,
}

impl<T> Progress<T> {
    fn has_score(&self) -> bool {
        self.score.is_some()
    }

    fn score(&self) -> Option<&Score> {
        if let Some(score) = &self.score {
            Some(score)
        } else {
            None
        }
    }
}
