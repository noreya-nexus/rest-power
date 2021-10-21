pub mod empty;
pub(crate) mod fan;
pub mod rpm;
pub mod info;
pub mod limits;


use rocket::http::Status;

pub trait Validate {
    fn validate(&self) -> Result<(),Status>;
}