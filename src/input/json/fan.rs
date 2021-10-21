use crate::json::Validate;
use rocket::http::Status;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Param {
    pub fan_forced : bool,
    pub fan_mode : Option<u8>,
}

impl Validate for Param {
    fn validate(&self) -> Result<(), Status> {


        if self.fan_forced == true && None == self.fan_mode {
            return Err(Status::UnprocessableEntity);
        }
        Ok(())
    }
}