#![crate_name="grafaman"]
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

use std::collections::HashMap;
use std::{env, thread};
use std::sync::Mutex;
use rocket_contrib::JSON;
use telegram_bot::{Api, Error, Result};

pub use self::structs::Message;
pub use self::handlers::{new, version};
use handlers::static_rocket_route_info_for_version;
pub mod structs;
pub mod handlers;


// The type to represent the ID of a message.
type ID = usize;
// The type to represent the JSON response.
type SimpleMap = HashMap<&'static str, &'static str>;
// The type to represent the Chat mapping.
type ChatMap = HashMap<usize, i64>;

// The global map for Chat mapping.
lazy_static! {
        static ref CHATS: Mutex<ChatMap> = {
            let mut m = ChatMap::new();
		    m.insert(100, -187394921); //TestAlerts
                    // m.insert(id_for_webhook, id_telegramm_channel)
            Mutex::new(m)
        };
    }
const VERSION: &'static str = env!("CARGO_PKG_VERSION");


fn get_chat(chat_id: usize) -> i64 {
    return *CHATS.lock().unwrap().get(&chat_id).unwrap();
}

fn send_message(chat_id: usize, msg: String) {
    let chat = get_chat(chat_id);
    println!("{:?}", chat);
    thread::spawn(move || {
        let api = get_telegram_api().unwrap();
        let _ = api.send_message(chat, msg, None, None, None, None);
    });
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
    println!("Grafaman version {}", VERSION);
    let api = match get_telegram_api() {
        Ok(api) => api,
        Err(_) => panic!("Telegram Bot Token wrong"),
    };
    println!("getMe: {:?}", api.get_me().unwrap().username.unwrap());
    rocket::ignite()
        .mount("/telegram", routes![handlers::new])
        .mount("/", routes![version])
        .catch(errors![not_found, bad_request])
        .launch();
}
