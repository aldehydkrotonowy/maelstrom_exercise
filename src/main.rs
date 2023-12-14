use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    // {
    //     "src":  A string identifying the node this message came from
    //     "dest": A string identifying the node this message is to
    //     "body": An object: the payload of the message
    //   }
    src: String,
    #[serde(rename = 'dest)]
    dst: String,
    body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
    //     "type":        (mandatory) A string identifying the type of message this is
    //     "msg_id":      (optional)  A unique integer identifier
    //     "in_reply_to": (optional)  For req/response, the msg_id of the request
    #[serde(rename = "type")]
    tpe: String,
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,
}

fn main() {}
