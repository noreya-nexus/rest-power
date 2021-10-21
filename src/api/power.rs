use crate::settings::Settings;
use crate::api_version::ApiVersion;
use rocket::serde::json::Json;

use crate::input::*;
use rocket::{State};
use crate::api::helper::Helper;
use nexus_unity_sdbp::sdbp::response::custom::power::powercmd::*;
use crate::response;
use crate::response::CResponse;
use nexus_unity_sdbp::sdbp::request::custom::power::Power;

#[get("/power/<version>/<slot>/protectionlog")]
pub fn protection(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err
    };

    let result: Result<ProtectionStatus, std::io::Error> = com_manager.device_command(Power::power_builder().protection_status().unwrap());
    response::ok(serde_json::to_string_pretty(&result.unwrap()).unwrap())
}

#[get("/power/<version>/<slot>/stats")]
pub fn voltage(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err
    };

    let result: Result<VoltageStatus, std::io::Error> = com_manager.device_command(Power::power_builder().voltage_current_status().unwrap());
    response::ok(serde_json::to_string_pretty(&result.unwrap()).unwrap())
}

#[post("/power/<version>/<slot>/limits", data = "<param>")]
pub fn limits(settings: &State<Settings>, version: ApiVersion, slot: u16, param: Json<json::limits::Param>) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err
    };

    let command = match Power::power_builder().current_limit(param.limit_3v3, param.limit_5v0, param.limit_12v) {
        Ok(value) => value,
        Err(err) => return response::unprocessable_entity(err.to_string()),
    };

    let result: Result<Limit, std::io::Error> = com_manager.device_command(command);
    return match result {
        Ok(status) => {
            if status.status == "success" {
                response::ok("{ \"status\": \"success\" }".to_string())
            } else {
                response::bad_request("Failed setting current limit".to_string())
            }
        }
        Err(_) => response::internal_server_error("Internal driver communication failed".to_string())
    }
}

#[get("/power/<version>/<slot>/source")]
pub fn source(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err
    };
    let result: Result<Source, std::io::Error> = com_manager.device_command(Power::power_builder().source().unwrap());
    response::ok(serde_json::to_string_pretty(&result.unwrap()).unwrap())
}