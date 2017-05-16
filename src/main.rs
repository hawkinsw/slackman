extern crate iron;
extern crate hyper_native_tls;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate params;

use iron::prelude::*;
use iron::status;
use hyper_native_tls::NativeTlsServer;
use std::process::Command;
use params::Params;

#[derive(Serialize, Debug)]
struct SlackResponse {
	response_type : String,
	text : String
}

fn respond(req: &mut Request) -> IronResult<Response> {
	let ps = req.get_ref::<Params>().unwrap();
	let response_body : String;

	if let Some(&params::Value::String(ref value)) = ps.find(&["text"]) {
		let man_param = value.clone().replace(";", "");
		let man = Command::new("/bin/bash")
			.arg("-c")
			.arg(format!("man {}", man_param))
			.output()
			.expect("failed!");

		if let Some(0) = man.status.code() {
			println!("man() succeeded.");
			response_body = serde_json::to_string(
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


fn main() {
	let ssl = NativeTlsServer::new("<path to pkcs12 file>","<password>").unwrap();
	let _server = Iron::new(respond).https("0.0.0.0:5001", ssl).unwrap();
	println!("Running ...");
}
