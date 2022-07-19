mod store_error;

pub use store_error::StoreError;
pub type Result<A> = std::result::Result<A, StoreError>;
