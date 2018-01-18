extern crate futures;
extern crate hyper;

use self::futures::future::Future;
use hyper::server::{Request, Response, Service};
use meetup_client::MeetupClient;
use MEETUP_URL;

pub struct MeetupServer;

impl MeetupServer {
    fn create_body(&self, query: &str) -> String {
        let mut member_id = "";
        let mut api_key = "";

        for part in query.split('&') {
            let items: Vec<&str> = part.split('=').collect();
            match items[0] {
                "member_id" => member_id = items[1],
                "api_key" => api_key = items[1],
                _ => {}
            }
        }

        if api_key == "" || member_id == "" {
            "Please specify all of api_key and member_id".to_string()
        } else {
            let client = MeetupClient::new(MEETUP_URL, api_key);
            format!("{:?}", client.get_activity(member_id))
        }
    }
}

impl Service for MeetupServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        let mut response = Response::new();
        match _req.query() {
            Some(query) => response.set_body(self.create_body(query)),
            None => response.set_body("No query"),
        };
        Box::new(futures::future::ok(response))
    }
}
