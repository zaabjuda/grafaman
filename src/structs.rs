

#[derive(Serialize, Deserialize, Debug)]
pub struct Matches {
    pub metric: String,
    pub value: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub title: String,
    pub state: String,
    #[serde(rename="ruleName")]
    pub rule_url: String,
    pub message: Option<String>,
    #[serde(rename="evalMatches")]
    pub eval_matches: Vec<Matches>,
}

//impl Message {
//    pub fn message_str(&self) -> String {
//        self.message
//            .as_ref()
//            .map(|s| format!("{}", s))
//            .unwrap_or("".to_string())
//
//    }
//}
