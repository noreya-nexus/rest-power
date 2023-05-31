pub mod empty;
pub(crate) mod fan;
pub mod rpm;
pub mod info;
pub mod limits;
pub mod reset;


use rocket::http::Status;

pub trait Validate {
    fn validate(&self) -> Result<(),Status>;
}