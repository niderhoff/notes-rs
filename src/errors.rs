use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatastoreError {
    #[error("Invalid Id `{given:?}` not found in datastore.")]
    InvalidId { given: usize },
}
