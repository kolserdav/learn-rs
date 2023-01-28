extern crate iron;
#[macro_use]
extern crate mime;
use iron::prelude::*;
use iron::status;
extern crate router;
use router::Router;
// use std::io::Write;
use std::str::FromStr;
extern crate urlencoded;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use urlencoded::UrlEncodedBody;
use users::get_current_username;
use std::path::Path;
use std::io::Read;

const ADDRRESS: &str = "localhost:3020";

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };
    println!("Form data: {:?}", form_data);
    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums,
    };
    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!(
                    "Value for 'n' parameter not a number: {:?}\n",
                    unparsed
                ));
                return Ok(response);
            }
            Ok(n) => {
                numbers.push(n);
            }
        }
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!(
        r#"
        The greatest common divisor of the numbers {:?} is <b>{}</b> 
        <br><br> <a href="/">Home</a>
        "#,
        numbers, d
    ));
    Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
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

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
        </form>
    "#,
    );
    Ok(response)
}

fn get_image(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Image/Png));
    let str_t = get_image_path();
    let file_path = Path::new(OsStr::new(&str_t));
    println!("{:?}", file_path);
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut file_content = Vec::new();
    let mut file = fs::File::open(&file_path).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");
    response.set_mut(file_content);
    Ok(response)
}

fn get_image_path() -> String {
    let user;
    match get_current_username() {
        Some(uname) => user = uname,
        None => user = OsString::from(""),
    };
    let str_t = format!("/home/{:?}/1png.png", user);
    // println!("{:?}",  str_t.replace("\"",""));
    str_t.replace("\"", "")
}

fn web_server() {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");
    router.get("/img", get_image, "img");
    println!("Serving on http://{}...", ADDRRESS);
    Iron::new(router).http(ADDRRESS).unwrap();
}

fn main() {
    web_server();
}
