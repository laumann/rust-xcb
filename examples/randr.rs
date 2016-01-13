extern crate xcb;

use xcb::base::*;
use xcb::xproto;
use xcb::randr;

const RANDR_VERSION_MAJOR: u32 = 1;
const RANDR_VERSION_MINOR: u32 = 3;

fn get_gamma_sizes(conn: &Connection, crtcs: &[u32]) {
    //println!("crtcs={:?}", crtcs);
    let mut crtc_gamma_cookies = Vec::with_capacity(crtcs.len());
    for crtc in crtcs {
        match randr::GetCrtcGammaSize(&conn, *crtc).get_reply() {
            Ok(mut gamma_size_reply) => {
                println!("GetCrtcGammaSize({})={}", crtc, gamma_size_reply.size());
                crtc_gamma_cookies.push(gamma_size_reply); //
            }
            Err(_e) => panic!("RandR Get CRTC Gamma Size")
        }
    }
}

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
            println!("Using RandR {}.{}", ver_reply.major_version(), ver_reply.minor_version());
        }
        Err(_e) => panic!("Generic error")
    }

    let mut screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();
    let window = conn.generate_id();
    xproto::CreateWindow(&conn, 0, window, screen.root(), 0, 0, 1, 1, 0, 0, 0, &[]);

    //conn.flush();

    match randr::GetScreenResourcesCurrent(&conn, window).get_reply() {
        Ok(mut res_reply) => {
            println!("Timestamp:        {}", res_reply.timestamp());
            get_gamma_sizes(&conn, res_reply.crtcs());
        }
        Err(_e) => panic!("Generic error again!")
    }
}
