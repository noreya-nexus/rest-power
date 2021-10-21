use crate::json::Validate;
use rocket::http::Status;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Param {
    pub limit_3v3: u32,
    pub limit_5v0: u32,
    pub limit_12v: u32,
}

impl Validate for Param {
    fn validate(&self) -> Result<(), Status> {
        // Limit check is already done in nexus_unity_sdbp::sdbp::request::custom::power
        Ok(())
    }
}