mod routes;
extern crate iron;
#[macro_use]
extern crate mime;
use iron::prelude::*;
extern crate router;
extern crate urlencoded;
use router::Router;

const ADDRRESS: &str = "localhost:3020";


fn web_server() {
    let mut router = Router::new();
    router.get("/", routes::get_form, "root");
    router.post("/gcd", routes::post_gcd, "gcd");
    router.get("/img", routes::get_image, "img");
    println!("Serving on http://{}...", ADDRRESS);
    Iron::new(router).http(ADDRRESS).unwrap();
}

fn main() {
    web_server();
}
