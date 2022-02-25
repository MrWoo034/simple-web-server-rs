use crate::prelude::*;
use anyhow::anyhow;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct Foo {
    pub name: Option<String>,
    pub id: Option<String>,
}

impl Foo {
    pub fn new(name: String) -> Self {
        let id: String = uuid::Uuid::new_v4().to_string();
        Self {
            id: Some(id),
            name: Some(name),
        }
    }

    pub fn foo_from_store(id: String, name: String) -> Self {
        Self {
            id: Some(id),
            name: Some(name),
        }
    }
}

//////////////////////
// Simple FooStorage
// Will place Foos into a HashMap
// To recall later.
// Uses a Mutex to ensure
// updates / inserts / deletes are thread safe.
pub struct FooStore {
    map: Mutex<HashMap<String, String>>,
}

impl Clone for FooStore {
    fn clone(&self) -> Self {
        let original = self.map.lock().unwrap();

        Self {
            map: Mutex::new(original.clone()),
        }
    }
}

impl FooStore {
    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }

    ///
    /// Takes a `Foo` object and places into the cache.
    /// If the `Foo` didn't exist in the `Map` already
    /// it will be added.  If it did exist, the `Foo.name`
    /// tied to `Foo.id` will be updated.
    ///
    /// [param] `foo` A `Foo` object to be added to the cache.
    ///
    /// [return] `Result<Foo>` returns `Ok(Foo)` if the object
    /// was successfully added / updated in the cache, and an
    /// `Err(_)` otherwise.

    pub fn put_foo(&self, foo: Foo) -> anyhow::Result<Foo> {
        let id = match &foo.id {
            Some(string) => string,
            None => return Err(anyhow!("No id present for foo: {:#?}", foo)),
        };

        let name = match &foo.name {
            Some(string) => string,
            None => return Err(anyhow!("No name present for foo: {:#?}", foo)),
        };

        let mut map = self.map.lock().unwrap();
        map.insert(id.clone(), name.clone());
        Ok(foo)
    }

    ///
    /// Retrieves a `Foo` object from the cache based on its id.
    ///
    /// [param] `id` An `id` of a `Foo` to retrive..
    ///
    /// [return] `Result<Foo>` returns `Ok(Foo)` if the object
    /// was found in the cache, and an `Err(anyhow::Error)`
    /// otherwise.

    pub fn get_foo(&self, id: String) -> anyhow::Result<Foo, anyhow::Error> {
        let map = self.map.lock().unwrap();

        match map.get(&id) {
            Some(string) => Ok(Foo::foo_from_store(id, string.clone())),
            None => Err(anyhow!("No record found for id: {}", id)),
        }
    }

    ///
    /// Takes an `id` of a `Foo` object to be removed from the
    /// cache.
    ///
    /// [param] `id` An `id` as a String of a `Foo` object to
    /// be removed from the cache.
    ///
    /// [return] `Result<String, anyhow::Error>` returns the
    /// `Ok(Foo.name)` of the removed `Foo` if the remove was
    /// successful, and `Err(anyhow::Err)` otherwise.

    pub fn delete_foo(&self, id: String) -> anyhow::Result<String, anyhow::Error> {
        let mut map = self.map.lock().unwrap();

        match map.remove(&id) {
            Some(name) => Ok(name),
            None => Err(anyhow!("No record found for id: {}", id)),
        }
    }
}
