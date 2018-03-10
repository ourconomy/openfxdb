use std::result;
use entities::*;

pub trait Db {
    fn create_effect(&mut self, &Effect) -> Result<()>;

    fn get_effect(&self, &str) -> Result<Effect>;

    fn all_effects(&self) -> Result<Vec<Effect>>;

    fn update_effect(&mut self, &Effect) -> Result<()>;
}

