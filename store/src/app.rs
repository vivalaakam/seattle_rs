use collection::{Collections, Storage};

#[derive(Clone)]
pub struct App<T> {
    collections: Collections<T>,
    secret_code: String,
}

impl<T> App<T>
where
    T: Storage,
{
    pub fn new(collections: Collections<T>, secret_code: String) -> Self {
        Self {
            collections,
            secret_code,
        }
    }

    pub fn get_collections(&self) -> &Collections<T> {
        &self.collections
    }

    pub fn is_valid(&self, code: &str) -> bool {
        self.secret_code.eq(code)
    }
}
