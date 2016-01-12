
extern crate xcb;
extern crate libc;

use xcb::base::*;
use xcb::xproto::*;
use xcb::randr::*;

fn main() {

    let (conn, screen_num) = Connection::connect();
    let mut screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();
    let window_dummy = conn.generate_id();

    CreateWindow(&conn, 0, window_dummy, screen.root(), 0, 0, 1, 1, 0, 0, 0, &[]);

    conn.flush();

    let screen_res_cookie = GetScreenResources(&conn, window_dummy);
    let mut screen_res_reply = screen_res_cookie.get_reply().unwrap();
    let crtcs = screen_res_reply.crtcs();

    let mut crtc_cookies = Vec::with_capacity(crtcs.len());
    for crtc in crtcs {
        crtc_cookies.push(GetCrtcInfo(&conn, crtc, 0));
    }

    for (i, crtc_cookie) in crtc_cookies.iter().enumerate() {
        if let Ok(mut reply) = crtc_cookie.get_reply() {
            if i != 0 { println!(""); }
            println!("CRTC[{}] INFO:", i);
            println!(" x-off\t: {}", reply.x());
            println!(" y-off\t: {}", reply.y());
            println!(" width\t: {}", reply.width());
            println!(" height\t: {}", reply.height());
        }
    }
}
