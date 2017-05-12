extern crate iron;
extern crate hyper_native_tls;
extern crate rustc_serialize;
extern crate params;

use iron::prelude::*;
use iron::status;
use hyper_native_tls::NativeTlsServer;
use std::process::Command;
use rustc_serialize::json;
use params::Params;

#[derive(RustcDecodable, RustcEncodable)]
struct SlackResponse {
	response_type : String,
	text : String
}

fn main() {
	fn respond(req: &mut Request) -> IronResult<Response> {
		let ps = req.get_ref::<Params>().unwrap();
		let man_param: String;
		let mut had_param: bool = false;
		let response_body : String;

		match ps.find(&["text"]) {
			Some(&params::Value::String(ref value)) => 
				{
					had_param = true;
					man_param = value.clone().replace(";", "");
				},
			_ => 
				{
					man_param = "".to_string()
				}
		};

		println!("man_param: {}", man_param);

		if had_param == true {
			let man = Command::new("/bin/bash")
				.arg("-c")
				.arg(format!("man {}", man_param))
				.output()
				.expect("failed!");

			if man.status.code().unwrap() == 0 {
				println!("man() succeeded.");
				response_body = json::encode(
					&SlackResponse{
						response_type: "ephemeral".to_string(),
						text: String::from_utf8_lossy(&man.stdout).into_owned(),
					}
				).unwrap();
			} else {
				println!("man() failed.");
				response_body = "".to_string();
			}
		} else {
			println!("No parameter.");
			response_body = "".to_string();
		}

		println!("response_body: {}", response_body);
		let mut response = Response::with((status::Ok, response_body));
		response.headers.set(iron::headers::ContentType::json());
		Ok(response)
	}
	let ssl = NativeTlsServer::new("<path to pkcs12 file>","<password>").unwrap();
	let _server = Iron::new(respond).https("0.0.0.0:5001", ssl).unwrap();
	println!("Running ...");
}
