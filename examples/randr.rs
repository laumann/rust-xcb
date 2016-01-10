extern crate xcb;

use xcb::base::*;
use xcb::randr;

const RANDR_VERSION_MAJOR: u32 = 1;
const RANDR_VERSION_MINOR: u32 = 3;

fn main() {
    let (conn, screen_num) = Connection::connect();

    match randr::QueryVersion(&conn,
                              RANDR_VERSION_MAJOR,
                              RANDR_VERSION_MINOR).get_reply() {
        Ok(mut ver_reply) => {
            if ver_reply.major_version() != RANDR_VERSION_MAJOR || ver_reply.minor_version() < RANDR_VERSION_MINOR {
                println!("Unsupported RandR version: {}.{}", ver_reply.major_version(), ver_reply.minor_version());
                return;
            }
        }
        Err(_e) => panic!("Generic error")
    }

    let mut setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let window = conn.generate_id();

    match randr::GetScreenResourcesCurrent(&conn, window).get_reply() {
        Ok(mut res_reply) => {
            println!("Timestamp: {}", res_reply.timestamp());
        }
        Err(_e) => panic!("Generic error again!")
    }
}
