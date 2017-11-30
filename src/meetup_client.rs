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
    url: Url,
}

impl MeetupClient {
    pub fn new(base_url: &str, token: &str) -> MeetupClient {
        let mut url = Url::parse(base_url).unwrap();
        url.query_pairs_mut().append_pair("key", token);
        MeetupClient { url }
    }

    pub fn get_activity(&self, member_id: &str) -> MeetupActivityResponseSerializer {
        let mut _url = self.url.clone();
        _url.query_pairs_mut().append_pair("member_id", member_id);
        _url.set_path("activity");
        let mut resp = self._make_request(Method::Get, _url);
        // debug serialize and print (need to cast)
        //println!("{:?}", resp);
        //let resp_data: MeetupActivityResponseSerializer = resp.json().unwrap();
        //println!("{:?}", resp_data);
        resp.json().unwrap()
    }

    fn _make_request(&self, method: Method, url: Url) -> Response {
        let client = Client::new().unwrap();
        let resp = client.request(method, url).unwrap().send().unwrap();
        // debug serialize and print (need to cast)
        // let resp_data: MeetupActivityResponseSerializer = resp.json().unwrap();
        // println!("{:?}", resp_data);
        // resp.json().unwrap();
        resp
    }

    // fn _make_request_2(&self, method: Method, url: Url, body: &mut String) {
    //     let mut client = Client::new().unwrap();
    //     let mut res = client.request(method, url).unwrap().send().unwrap();
    //     res.read_to_string(body).unwrap();
    // }
}
