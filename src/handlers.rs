use structs::Message;
// pub mod structs;
use rocket_contrib::JSON;
use ::{VERSION, SimpleMap, ID, send_message};

fn message_str(message: T) -> String {
    message.map(|s| format!("{}", s))
        .unwrap_or("".to_string())

}


#[post("/<id>", format = "application/json", data = "<message>")]
pub fn new(id: ID, message: JSON<Message>) -> JSON<SimpleMap> {
    //    let api = get_telegram_api().unwrap();
    let mut evals = "".to_string();
    for x in &message.0.eval_matches {
        evals.push_str(format!("{}: {}\n", x.metric, x.value).as_str());
    }
    let msg = format!("{}\n{}\n{}{}",
                      message.0.title,
                      message_str(message),
                      //                      message.message_str(),
                      evals,
                      message.0.rule_url);
    send_message(id, msg);
    JSON(map!{ "status" => "ok" })
}

#[get("/version")]
pub fn version() -> &'static str {
    VERSION
}
