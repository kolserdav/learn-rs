use std::ffi::OsString;
use users::get_current_username;

const IMAGE_NAME: &str = "mandel1.png";

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}


pub fn get_image_path() -> String {
    /*
    let user;
    match get_current_username() {
        Some(uname) => user = uname,
        None => user = OsString::from(""),
    };
    */
    let str_t = format!("/usr/local/share/applications/first-rs/images/{}", IMAGE_NAME);
    str_t.replace("\"", "")
}


/*
fn parse_from_args() {
    let mut numbers = Vec::new();
    let dd = std::env::args();
    println!("dasd {:?}", dd);
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
*/
