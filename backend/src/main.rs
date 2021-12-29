#[macro_use]
extern crate rocket;

use enigo::{Enigo, Key, KeyboardControllable};
use rocket::http::Status;

#[get("/flight-assist")]
fn flight_assist() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('z'));

    Status::Ok
}

#[get("/frameshift-drive")]
fn frameshift_drive() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('j'));

    Status::Ok
}

#[get("/orbital-lines")]
fn orbital_lines() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('¡'));

    Status::Ok
}

#[get("/front-target")]
fn front_target() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('t'));

    Status::Ok
}

#[get("/next-target")]
fn next_target() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('g'));

    Status::Ok
}

#[get("/most-dangerous-target")]
fn most_dangerous_target() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('h'));

    Status::Ok
}

#[get("/next-subsystem")]
fn next_subsystem() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('y'));

    Status::Ok
}

#[get("/next-action-group")]
fn next_action_group() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('n'));

    Status::Ok
}

#[get("/change-mode")]
fn change_mode() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('m'));

    Status::Ok
}

#[get("/hardpoints")]
fn hardpoints() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('u'));

    Status::Ok
}

#[get("/silent-running")]
fn silent_running() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Delete);

    Status::Ok
}

#[get("/heatsink-launcher")]
fn heatsink_launcher() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('v'));

    Status::Ok
}

#[get("/lights")]
fn lights() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('f'));

    Status::Ok
}

#[get("/zoom-in-sensor")]
fn zoom_in_sensor() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::PageUp);

    Status::Ok
}

#[get("/zoom-out-sensor")]
fn zoom_out_sensor() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::PageDown);

    Status::Ok
}

#[get("/cargo-scoop")]
fn cargo_scoop() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Home);

    Status::Ok
}

#[get("/night-vision")]
fn night_vision() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('b'));

    Status::Ok
}

#[get("/landing-gear")]
fn landing_gear() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('l'));

    Status::Ok
}

#[get("/chaffs")]
fn chaffs() -> Status {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('s'));

    Status::Ok
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(("address", "0.0.0.0"));

    rocket::custom(figment).mount("/", routes![
        flight_assist, frameshift_drive, orbital_lines, front_target, next_target,
        most_dangerous_target, next_subsystem, change_mode, next_action_group, hardpoints,
        silent_running, heatsink_launcher, lights, zoom_in_sensor, zoom_out_sensor, cargo_scoop,
        night_vision, landing_gear, chaffs
    ])
}