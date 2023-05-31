use crate::settings::Settings;
use crate::api_version::ApiVersion;
use rocket::serde::json::Json;

use crate::input::*;
use rocket::{State};
use noreya_sdbp::sdbp::response::custom::power::fan::*;
use noreya_sdbp::sdbp::response::custom::power::temperature::*;
use crate::api::helper::Helper;
use crate::response;
use crate::response::CResponse;
use noreya_sdbp::sdbp::request::custom::power::Power;

#[post("/power/<version>/<slot>/fan/rpm", data="<param>")]
pub fn rpm_control(settings : &State<Settings>, version : ApiVersion, slot : u16, param : Json<json::rpm::Param>) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param){
        Ok(value) => value,
        Err(err) => return err
    };

    let result: Result<RpmControl,std::io::Error> = com_manager.device_command(Power::temperature_builder().fan_rpm_control(param.measurement_enabled).unwrap());
    return match result {
        Ok(status) => {
            if status.status == "success" {
                response::ok("{ \"status\": \"success\" }".to_string())
            } else {
                response::bad_request("Failed changing fan rpm control".to_string())
            }
        }
        Err(_) => response::internal_server_error("Internal driver communication failed".to_string())
    }
}

#[get("/power/<version>/<slot>/fan/rpm")]
pub fn rpm(settings : &State<Settings>, version : ApiVersion, slot : u16) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()){
        Ok(value) => value,
        Err(err) => return err
    };

    let result: Result<RpmStatus,std::io::Error> = com_manager.device_command(Power::temperature_builder().fan_rpm().unwrap());
    response::ok(serde_json::to_string_pretty(&result.unwrap()).unwrap())
}


#[post("/power/<version>/<slot>/fan", data="<param>")]
pub fn fan_control(settings : &State<Settings>, version : ApiVersion, slot : u16, param : Json<json::fan::Param>) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param){
        Ok(value) => value,
        Err(err) => return err
    };
    let command = match Power::temperature_builder().fan_control(param.fan_forced, param.fan_mode){
        Ok(value) => value,
        Err(_err) => return response::unprocessable_entity("Fan mode must be 0, 20, 40, 60, 80, 100".to_string()),
    };

    let result: Result<FanControl,std::io::Error> = com_manager.device_command(command);
    return match result {
        Ok(status) => {
            if status.status == "success" {
                response::ok("{ \"status\": \"success\" }".to_string())
            } else {
                response::bad_request("Failed setting fan mode".to_string())
            }
        }
        Err(_) => response::internal_server_error("Internal driver communication failed".to_string())
    }
}

#[get("/power/<version>/<slot>/fan")]
pub fn fan(settings : &State<Settings>, version : ApiVersion, slot : u16) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()){
        Ok(value) => value,
        Err(err) => return err
    };

    let result: Result<FanStatus,std::io::Error> = com_manager.device_command(Power::temperature_builder().fan_status().unwrap());
    response::ok(serde_json::to_string_pretty(&result.unwrap()).unwrap())
}

#[get("/power/<version>/<slot>/temperature")]
pub fn temperature(settings : &State<Settings>, version : ApiVersion, slot : u16) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()){
        Ok(value) => value,
        Err(err) => return err
    };

    let result: Result<ResponseTemperature,std::io::Error> = com_manager.device_command(Power::temperature_builder().temperature_sensor().unwrap());
    response::ok(serde_json::to_string_pretty(&result.unwrap()).unwrap())
}