extern crate env_logger;
extern crate hyper;
#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
mod meetup_client;
mod meetup_server;

use std::env;
use meetup_client::MeetupClient;
use meetup_server::MeetupServer;
use hyper::server::Http;

quick_error! {
    #[derive(Debug)]
    pub enum MyError {
        ReqwestError(err: reqwest::Error){
            description(err.description())
        }
    }
}

static MEETUP_URL: &str = "https://api.meetup.com";

fn main() {
    match env_logger::init() {
        Ok(_) => debug!("logging started."),
        Err(err) => error!("error starting logger: {}", err),
    };

    match env::args().nth(1) {
        Some(arg) => process_arguments(&arg),
        None => get_activity(),
    }
}

fn process_arguments(arg: &str) {
    if arg != "-s" {
        error!("Unrecognized argument: {}", arg);
    } else {
        let addr = "127.0.0.1:8080".parse().unwrap();
        let server = Http::new().bind(&addr, || Ok(MeetupServer)).unwrap();
        println!("Server listening on: http://{}", addr);
        // Synchronous blocking call
        server.run().unwrap();
    }
}

fn get_activity() {
    let member_id = env::var("MEMBER_ID").expect("MEMBER_ID was not found.");
    let token = env::var("MEETUP_API_KEY").expect("MEETUP_API_KEY was not found.");
    let client = MeetupClient::new(MEETUP_URL, token.as_str());
    // https://www.meetup.com/meetup_api/docs/activity/
    let resp_data = client.get_activity(member_id.as_str()).unwrap();
    for item in resp_data.results {
        println!(
            "[{}] {}: {}",
            item.published.unwrap(),
            item.member_name.unwrap(),
            item.title.unwrap()
        );
    }
}
