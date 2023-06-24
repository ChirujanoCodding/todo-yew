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
        let item = match serde_json::to_string(&item) {
            Err(err) => return Err(err.to_string()),
            Ok(json) => json,
        };

        let document = web_sys::window().unwrap();
        match document.local_storage() {
            Err(error) => Err(error.as_string().unwrap()),
            Ok(option) => match option {
                None => Err("no storage founded".into()),
                Some(storage) => {
                    let _ = storage.set_item(key, &item);
                    Ok(())
                }
            },
        }
    }

    pub fn get_as<T: for<'a> Deserialize<'a>>(&self, key: &str) -> StorageResult<T> {
        let document = web_sys::window().unwrap();
        match document.local_storage() {
            Err(error) => Err(error.as_string().unwrap()),
            Ok(option) => match option {
                None => Err("no storage founded".into()),
                Some(storage) => {
                    let item = storage.get_item(key);
                    match item {
                        Err(err) => Err(err.as_string().unwrap()),
                        Ok(possible) => match possible {
                            None => Err("key not founded".into()),
                            Some(item) => Ok(serde_json::from_str(&item).unwrap()),
                        },
                    }
                }
            },
        }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}
