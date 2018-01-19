use std::io::Error;

use db::entity::Entity;
use db::file_storage;
use db::storage::Storage;

pub struct Database<S: Storage> {
    storage: S,
}

impl<S: Storage> Database<S> {
    pub fn new(storage: S) -> Database<S> {
        Database {storage: storage}
    }

    pub fn get_entity<E: Entity>(&self, name: &str) -> Result<E, Error> {
        let mut key = name.to_owned();
        key.push_str(".entity");
        let value: Vec<u8> = self.storage.get_byte_vec(&key)?;
        let entity = Entity::from_byte_vec(value)?;
        Ok(entity)
    }

    pub fn set_entity<E: Entity>(&self, name: &str, entity: E) -> Result<(), Error> {
        let mut key = name.to_owned();
        key.push_str(".entity");
        let value = entity.as_byte_vec()?;
        self.storage.set_byte_vec(&key, value)?;
        Ok(())
    }
}

impl Database<file_storage::FileStorage> {
    pub fn new_with_file_storage(dir: &str) -> Result<Database<file_storage::FileStorage>, Error> {
        let storage = file_storage::FileStorage::new(dir)?;
        Ok(Database {storage: storage})
    }
}
