use sled::Config;

mod open_tree;

pub struct SledDb(pub(crate) sled::Db);

impl SledDb {
    pub fn new_from_config(cfg: Config) -> Result<Self, sled::Error> {
        Ok(Self(cfg.open()?))
    }
}
