# rust-meetup-client
A minimal client interfacing to the Meetup.com REST APIs

This is a toy/learning project for the [Rust FVG Meetup](https://www.meetup.com/Rust-FVG-Meetup) group.

Run with:

`$ MEMBER_ID=<a_member_id> API_KEY=<your_api_key> cargo run`


## Server

You can fire up a Web application running:
`$ cargo run -- -s`

then visit `localhost:8080` on your browser to use the application. You can retrieve the same information you were
getting from the command line application appending the following query string to the URL:
`?api_key=<your_api_key>&member_id=<a_member_id>`

Beware that the code is still very unstable, the application will crash if you pass wrong values.
