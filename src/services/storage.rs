use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Debug, Clone, PartialEq, Properties, Default)]
pub struct Storage;

#[derive(Default, Debug)]
pub enum Mode {
    #[default]
    Local,
    Session,
}

#[derive(Debug)]
pub enum Error<'k, Item> {
    Parse(Item),
    Loading(Mode),
    NotFounded(Mode),
    Key(&'k str),
    Empty,
}

type StorageResult<'a, IN = (), OUT = ()> = Result<OUT, Error<'a, IN>>;

impl Storage {
    pub fn save<T: Serialize>(&self, key: &str, item: T, mode: Mode) -> StorageResult<T> {
        let Ok(item) = serde_json::to_string(&item) else {
            return Err(Error::Parse(item));
        };
        let document = web_sys::window().unwrap();

        let Ok(option) = (match mode { Mode::Local => document.local_storage(), Mode::Session => document.session_storage() }) else {
            return Err(Error::Loading(mode));
        };

        let Some(storage) = option else {
            return Err(Error::NotFounded(mode));
        };

        let _ = storage.set_item(key, &item);

        Ok(())
    }

    pub fn get_as<'k, T: for<'de> Deserialize<'de>>(
        &'k self,
        key: &'k str,
        mode: Mode,
    ) -> StorageResult<&'k str, T> {
        let document = web_sys::window().unwrap();

        let Ok(option) = (match mode { Mode::Local => document.local_storage(), Mode::Session => document.session_storage() }) else {
            return Err(Error::Loading(mode));
        };
        let Some(storage) = option else {
                return Err(Error::NotFounded(mode));
        };
        let Ok(possible) = storage.get_item(key) else {
            return Err(Error::Key(key));
        };

        match possible {
            None => Err(Error::Empty),
            Some(item) => Ok(serde_json::from_str(&item).unwrap()),
        }
    }
}
