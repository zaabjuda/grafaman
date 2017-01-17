#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
extern crate telegram_bot;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::env;
use rocket_contrib::JSON;
use std::collections::HashMap;
use telegram_bot::*;

// The type to represent the ID of a message.
type ID = usize;
type SimpleMap = HashMap<&'static str, &'static str>;

#[derive(Serialize, Deserialize, Debug)]
struct Matches {
    metric: String,
    value: Integer,
}

#[derive(Serialize, Deserialize)]
struct Message {
    title: String,
    state: String,
    ruleUrl: String,
    message: String,
    evalMatches: Vec<Matches>,
}



// TODO: This example can be improved by using `route` with muliple HTTP verbs.
#[post("/<id>", format = "application/json", data = "<message>")]
fn new(id: ID, message: JSON<Message>) -> JSON<SimpleMap> {
    let api = get_telegram_api().unwrap();
    let mut evals = "".to_string();
    for x in &message.0.evalMatches {
        evals.push_str(format!("{}: {}\n", x.metric, x.value).as_str());
    }
    let msg = format!("{}\n{}\n{}{}",
                      message.0.title,
                      message.0.message,
                      evals,
                      message.0.ruleUrl);
    let _ = api.send_message(-182028917, msg, None, None, None, None);

    JSON(map!{ "status" => "ok" })
}

#[error(404)]
fn not_found() -> JSON<SimpleMap> {
    JSON(map! {
        "status" => "error",
        "reason" => "Resource was not found."
    })
}

#[error(400)]
fn bad_request() -> JSON<SimpleMap> {
    JSON(map! {
        "status" => "error",
        "reason" => "The request could not be understood by the server due
                to malformed syntax."
    })
}

fn get_telegram_api() -> Result<Api> {
    let token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(tok) => tok,
        Err(e) => return Err(Error::InvalidEnvironmentVar(e)),
    };
    Api::from_token(&token)
}

fn main() {
    let api = match get_telegram_api() {
        Ok(api) => api,
        Err(_) => panic!("Telegram Bot Token wrong"),
    };
    println!("getMe: {:?}", api.get_me());

    rocket::ignite()
        .mount("/telegram", routes![new])
        .catch(errors![not_found])
        .launch();
}
