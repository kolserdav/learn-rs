mod prelude;
use std::str::FromStr;
use std::ffi::OsStr;
extern crate iron;
use iron::prelude::*;
use iron::status;
use std::fs;
use urlencoded::UrlEncodedBody;
use std::path::Path;
use std::io::Read;

pub fn post_gcd(request: &mut Request) -> IronResult<Response> {
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
        d = prelude::gcd(d, *m);
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

pub fn get_form(_request: &mut Request) -> IronResult<Response> {
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

pub fn get_image(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Image/Png));
    let str_t = prelude::get_image_path();
    let file_path = Path::new(OsStr::new(&str_t));
    println!("{:?}", file_path);
    let mut file_content = Vec::new();
    let mut file = fs::File::open(&file_path).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");
    response.set_mut(file_content);
    Ok(response)
}
