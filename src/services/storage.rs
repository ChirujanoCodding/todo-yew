use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Storage;

type StorageResult<T = ()> = Result<T, String>;

impl Storage {
    fn new() -> Self {
        Self {}
    }

    pub fn save<T: Serialize>(&self, key: &str, item: T) -> StorageResult {
        let Ok(item) = serde_json::to_string(&item) else {
            return Err("error parsing item".into());
        };

        let document = web_sys::window().unwrap();
        let Ok(option) = document.local_storage() else {
            return Err("cannot load local storage".into());
        };

        let Some(storage) = option else {
            return Err("no storage founded".into());
        };

        let _ = storage.set_item(key, &item);

        Ok(())
    }

    pub fn get_as<T: for<'a> Deserialize<'a>>(&self, key: &str) -> StorageResult<T> {
        let document = web_sys::window().unwrap();
        let Ok(option) = document.local_storage() else {
            return Err("cannot load local storage".into());
        };
        let Some(storage) = option else {
                return Err("no storage founded".into());
        };
        let Ok(possible) = storage.get_item(key) else {
            return Err("key not found".into());
        };

        match possible {
            None => Err("key not founded".into()),
            Some(item) => Ok(serde_json::from_str(&item).unwrap()),
        }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}
