use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatastoreError {
    #[error("Invalid Id `{given:?}` not found in datastore.")]
    UnknownId { given: usize },
    #[error("Nothing to update, datastore is empty.")]
    NotUpdateable,
    #[error("Empty Datastore, nothing to remove.")]
    Empty,
}
