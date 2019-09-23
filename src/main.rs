use structopt::StructOpt;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use std::str::FromStr;

#[derive(StructOpt, Debug)]
#[structopt(name = "minicurl")]
struct Options {
    #[structopt(
        short="H",
        long="headers",
        default_value = "",
    )]
    headers: String,

    #[structopt()]
    url: String,
}

///
/// Parse a string into a vector of header_name and header_value pairs,
/// with very small amount of error handling (using expect).
/// 
/// The strings will be of the form
///     "Header1: value1, Header2: value2"
/// 
fn parse_headers(s: String) -> Vec<(HeaderName, HeaderValue)> {
    let raw_headers = s.split(',');
    let res = raw_headers
        .map(|header| {
            let mut parts = header.split(':');
            let name = HeaderName::from_str(parts
                .next()
                .expect("There was no header!")
            ).expect("Error parsing header name");

            let value = HeaderValue::from_str(parts
                .next()
                .expect("There was no header!")
            ).expect("Error parsing header value.");
            (name, value)
        })
        .collect();
    res
}


///
/// Create a new mutable empty HeaderMap.
/// Iterate over the headers, filling the map.
/// Return the map.
///
fn construct_header_map(headers: Vec<(HeaderName, HeaderValue)>) -> HeaderMap {
    let mut map = HeaderMap::new();
    for (name, val) in headers {
        map.insert(name, val);
    };
    map
}

fn main() {    
    let opt = Options::from_args();
    let client = Client::new();
    println!("I have {} cookies, which I'm fetching from {}", &opt.headers, opt.url);
    let headers: HeaderMap = construct_header_map(parse_headers(opt.headers));
    let response = client.get(&opt.url).headers(headers).send();
    match response {
        Ok(mut resp) => println!("Got successful response \n{:?}", resp.text().unwrap()),
        Err(e) => println!("Error! {}", e)
    }
}
