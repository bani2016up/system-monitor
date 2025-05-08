#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Mutex;
use sysinfo::System;

mod core;

struct SystemState {
    system: Mutex<System>,
}

#[get("/system/os")]
fn get_os() -> Json<core::SystemOSData> {
    Json(core::get_os_data())
}

#[get("/system/monitor")]
fn get_monitor(state: &State<SystemState>) -> Json<core::SystemMonitor> {
    let mut system = state.system.lock().unwrap();
    Json(core::get_system_monitor(&mut system))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_os, get_monitor])
        .manage(SystemState {
            system: Mutex::new(System::new_all()),
        })
}
