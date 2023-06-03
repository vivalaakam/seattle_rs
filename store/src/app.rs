use collection::{Collections, Storage};

#[derive(Clone)]
pub struct App<T> {
    collections: Collections<T>,
}

impl<T> App<T>
where
    T: Storage,
{
    pub fn new(collections: Collections<T>) -> Self {
        Self { collections }
    }

    pub fn get_collections(&self) -> &Collections<T> {
        &self.collections
    }
}
