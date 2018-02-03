use std::result::Result;
use reqwest::{Client, Method, Response, Url};
extern crate serde;
extern crate serde_json;
use MyError;

// ref. https://github.com/rust-on-slack/rust-slack-inviter/blob/master/src/slack.rs

#[derive(Debug, Deserialize)]
pub struct MeetupResult {
    id: String,
    member_id: String,
    message_id: Option<String>,
    is_reply: Option<String>, // "False",
    updated: Option<String>,  // "Sun Nov 19 17:19:59 EST 2017"
    discussion_body: Option<String>,
    discussion_title: Option<String>,
    photo_url: Option<String>, // "https://secure.meetupstatic.com/photos/member/c/8/7/3/thumb_271311315.jpeg",
    group_name: Option<String>,
    item_type: Option<String>,
    link: Option<String>,
    pub published: Option<String>, // "Sun Nov 19 17:19:59 EST 2017",
    pub title: Option<String>,
    pub member_name: Option<String>,
    thread_id: Option<String>,
    group_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MeetupActivityResponseSerializer {
    pub results: Vec<MeetupResult>,
}

pub struct MeetupClient {
    url: Url,
}

// https://www.reddit.com/r/rust/comments/69i105/the_grass_is_always_greener_my_struggles_with_rust/dh71fzh/
// https://www.reddit.com/r/rust/comments/69i105/the_grass_is_always_greener_my_struggles_with_rust/dh6ryh0/
type ClientResult<T> = Result<T, MyError>;

impl MeetupClient {
    pub fn new(base_url: &str, token: &str) -> MeetupClient {
        let mut url = Url::parse(base_url).unwrap();
        url.query_pairs_mut().append_pair("key", token);
        MeetupClient { url }
    }

    pub fn get_activity(&self, member_id: &str) -> ClientResult<MeetupActivityResponseSerializer> {
        let mut _url = self.url.clone();
        _url.query_pairs_mut().append_pair("member_id", member_id);
        _url.set_path("activity");
        let mut resp = self._make_request(Method::Get, _url);
        // debug serialize and print (need to cast)
        // println!("{:?}", resp);
        // let resp_data: MeetupActivityResponseSerializer = resp.json().unwrap();
        // println!("{:?}", resp_data);
        resp.json().map_err(MyError::ReqwestError)
    }

    fn _make_request(&self, method: Method, url: Url) -> Response {
        let client = Client::new().unwrap();
        let resp = client.request(method, url).unwrap().send().unwrap();
        resp
    }

    // fn _make_request_2(&self, method: Method, url: Url, body: &mut String) {
    //     let mut client = Client::new().unwrap();
    //     let mut res = client.request(method, url).unwrap().send().unwrap();
    //     res.read_to_string(body).unwrap();
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use MEETUP_URL;
    static TOKEN: &str = "YOUR_MEETUP_TOKEN";

    // setup() and teardown() are arbitrary names
    fn setup() -> MeetupClient {
        MeetupClient::new(MEETUP_URL, TOKEN)
    }
    fn teardown() {
        // undo what you've done in setup()
    }

    #[test]
    fn test_client() {
        let client = setup();
        let member_id = "YOUR_MEMBER_ID";
        let url = Url::parse(&format!(
            "{}/activity?&key={}&member_id={}",
            MEETUP_URL, TOKEN, member_id
        )).unwrap();
        let resp_data = client._make_request(Method::Get, url);
        assert_eq!(200, resp_data.status().as_u16());
        teardown();
    }

}
