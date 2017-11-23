use std::io::Read;
use reqwest::{Client, Method, Url, Response};
extern crate serde_json;
extern crate serde;

// ref. https://github.com/rust-on-slack/rust-slack-inviter/blob/master/src/slack.rs

#[derive(Debug)]
#[derive(Deserialize)]
struct MeetupResult {
    id: String,
    member_id: String,
    message_id: Option<String>,
    is_reply: Option<String>, // "False",
    updated: Option<String>, // "Sun Nov 19 17:19:59 EST 2017"
    discussion_body: Option<String>,
    discussion_title: Option<String>,
    photo_url: Option<String>, // "https://secure.meetupstatic.com/photos/member/c/8/7/3/thumb_271311315.jpeg",
    group_name: Option<String>,
    item_type: Option<String>,
    link: Option<String>,
    published: Option<String>, // "Sun Nov 19 17:19:59 EST 2017",
    title: Option<String>,
    member_name: Option<String>,
    thread_id: Option<String>,
    group_id: Option<String>,
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct MeetupActivityResponseSerializer {
    results: Vec<MeetupResult>,
}

pub struct MeetupClient {
    url: String,
    token: String,
}

impl MeetupClient {
    pub fn new(url: String, token: String) -> MeetupClient {
        MeetupClient {
            url: url,
            token: token,
        }
    }

    pub fn get_activity(&self, member_id: String) -> MeetupActivityResponseSerializer {
        let mut resp = self._make_request(Method::Get,
                                          format!("{}/activity?key={}&member_id={}",
                                                  self.url,
                                                  self.token,
                                                  member_id));
        // debug serialize and print (need to cast)
        //println!("{:?}", resp);
        //let resp_data: MeetupActivityResponseSerializer = resp.json().unwrap();
        //println!("{:?}", resp_data);
        resp.json().unwrap()

    }

    fn _make_request(&self, method: Method, url: String) -> Response {
        let client = Client::new().unwrap();
        let mut _url = Url::parse(&url).unwrap();
        let resp = client.request(method, _url).unwrap().send().unwrap();
        // debug serialize and print (need to cast)
        // let resp_data: MeetupActivityResponseSerializer = resp.json().unwrap();
        // println!("{:?}", resp_data);
        //resp.json().unwrap()
        resp
    }

    // fn _make_request_2(&self, method: Method, url: Url, body: &mut String) {
    //     let mut client = Client::new().unwrap();
    //     let mut res = client.request(method, url).unwrap().send().unwrap();
    //     res.read_to_string(body).unwrap();
    // }
}
