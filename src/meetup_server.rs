extern crate hyper;
extern crate futures;

use self::futures::future::Future;
use hyper::server::{Request, Response, Service};
use meetup_client::MeetupClient;

pub struct MeetupServer;

impl Service for MeetupServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        let mut response = Response::new();
        match _req.query() {
            Some(query) => {
                let mut url = "";
                let mut member_id = "";
                let mut api_key = "";

                let parts = query.split('&');
                for part in parts {
                    let items: Vec<&str> = part.split('=').collect();
                    if items[0] == "url" {
                        url = items[1];
                    } else if items[0] == "member_id" {
                        member_id = items[1];
                    } else if items[0] == "api_key" {
                        api_key = items[1];
                    }
                }

                let mut body = String::new();
                if url != "" && api_key != "" && member_id != "" {
                    let client = MeetupClient::new(url, api_key);
                    body = format!("{:?}", client.get_activity(member_id));
                }

                response.set_body(body);
            },
            None => response.set_body("No query")
        };
        Box::new(futures::future::ok(response))
    }
}