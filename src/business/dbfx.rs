use super::error::RepoError;
use std::result;
use entities::*;

type Result<T> = result::Result<T, RepoError>;

pub trait DbFX {
    fn create_effect(&mut self, &Effect) -> Result<()>;
    fn get_effect(&self, &str) -> Result<Effect>;
    fn all_effects(&self) -> Result<Vec<Effect>>;
    fn update_effect(&mut self, &Effect) -> Result<()>;

    //DbFX needs its own versions of these too
    fn create_effect_triple(&mut self, &Triple) -> Result<()>;
    fn all_triples(&self) -> Result<Vec<Triple>>;
    fn delete_effect_triple(&mut self, &Triple) -> Result <()>;
}

