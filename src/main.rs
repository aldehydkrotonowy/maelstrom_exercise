use std::io::StdoutLock;

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String, // A string identifying the node this message came from
    #[serde(rename = "dest")]
    dst: String, //A string identifying the node this message is to
    body: Body,  //An object: the payload of the message
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
    // #[serde(rename = "type")]
    // tpe: String, // (mandatory) A string identifying the type of message this is
    #[serde(rename = "msg_id")]
    id: Option<usize>, //(optional)  A unique integer identifier
    in_reply_to: Option<usize>, //(optional)  For req/response, the msg_id of the request
    // rest: HashMap<String, serde_json::Value>,
    #[serde(flatten)]
    paylad: Payload,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

struct EchoNode {
    id: usize,
}

impl EchoNode {
    pub fn step(
        &mut self,
        input: Message,
        output: &mut serde_json::Serializer<StdoutLock, serde_json::ser::PrettyFormatter>,
    ) -> anyhow::Result<()> {
        match input.body.paylad {
            Payload::Init { .. } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        paylad: Payload::InitOk,
                    },
                };
                reply
                    .serialize(output)
                    .context("serialize response to init")?;
                self.id += 1;
            }
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        paylad: Payload::EchoOk { echo },
                    },
                };
                reply
                    .serialize(output)
                    .context("serialize response to echo")?;
                self.id += 1;
            }
            Payload::InitOk { .. } => bail!("InitOk should never happen"),
            Payload::EchoOk { .. } => {}
        }

        Ok(())
    }
}
fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let stdout = std::io::stdout().lock();
    let mut output = serde_json::Serializer::pretty(stdout);

    let mut state = EchoNode { id: 0 };

    for input in inputs {
        let input = input.context("melstrome input form STDIN could not be deserialized")?;
        state
            .step(input, &mut output)
            .context("step function failed")?;
    }
    Ok(())
}
