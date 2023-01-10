use actix::Addr;use actix::Actor;
use actix::ActorContext;
use actix::AsyncContext;
use actix::ContextFutureSpawner;
use actix::fut;
use actix::fut::future::ActorFutureExt;
use actix::Handler;
use actix::Message as MessageTrait;
use actix::StreamHandler;
use actix::SystemService;
use actix::WrapFuture;
use actix_web_actors::ws::Message;
use actix_web_actors::ws::ProtocolError;
use actix_web_actors::ws::WebsocketContext;
use once_cell::sync::Lazy;
use query::DataFrame;
use query::doc::Doc;
use query::doc::Operation;
use rand::distributions::Alphanumeric;
use rand::prelude::Distribution;
use rand::thread_rng;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

use crate::broker::Broker;
use crate::broker::OpenDocumentMsg;
use crate::document::Document;
use crate::document::GetDocMsg;
use crate::document::QueryMsg;
use crate::document::ReadFileMsg;
use crate::document::SubscribeMsg;
use crate::document::UnsubscribeMsg;
use crate::document::UpdateDocMsg;
use crate::executor::CancelJobMsg;
use crate::executor::Executor;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
static CLIENT_IDS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| {
    Mutex::new(HashSet::new())
});

pub struct Client {
    id: String,
    document: Option<Addr<Document>>,
    hb: Instant
}

impl Client {
    pub fn new() -> Client {
        Client {
            id: new_client_id(),
            document: None,
            hb: Instant::now()
        }
    }

    fn open_doc(&mut self, id: usize, path: String, ctx: &mut WebsocketContext<Client>) {
        self.unsubscribe();
        let msg = OpenDocumentMsg { path };
        let client_id = self.id.clone();
        <Broker as SystemService>::from_registry()
            .send(msg)
            .into_actor(self)
            .then(move |doc, act, ctx| {
                match doc {
                    Ok(Ok(doc)) => {
                        let msg = SubscribeMsg {
                            id: client_id,
                            client: ctx.address()
                        };
                        doc
                            .send(msg)
                            .into_actor(act)
                            .then(move |res, act, ctx| {
                                if let Ok(_) = res {
                                    // Get the latest doc.
                                    let msg = GetDocMsg {
                                        client: ctx.address(),
                                        req_id: id
                                    };
                                    doc.do_send(msg);
                                    act.document = Some(doc);
                                } else {
                                    let msg = RpcResponseMsg::Error {
                                        id: Some(id),
                                        code: RpcErrorCode::InternalError,
                                        msg: String::from("Failed to subscribe to doc changes")
                                    };
                                    ctx.address().do_send(msg);
                                }
                                fut::ready(())
                            })
                            .wait(ctx)
                    }
                    Ok(Err(e)) => {
                        let msg = RpcResponseMsg::Error {
                            id: Some(id),
                            code: RpcErrorCode::InternalError,
                            msg: e.to_string()
                        };
                        ctx.address().do_send(msg);
                    }
                    _ => {
                        let msg = RpcResponseMsg::Error {
                            id: Some(id),
                            code: RpcErrorCode::InternalError,
                            msg: String::from("Something went wrong")
                        };
                        ctx.address().do_send(msg);
                    }
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn close_doc(&mut self, id: usize, ctx: &mut WebsocketContext<Client>) {
        self.unsubscribe();
        let res = RpcResponseMsg::DocClosed { id };
        Self::response(&res, ctx);
    }

    fn response(res: &RpcResponseMsg, ctx: &mut WebsocketContext<Client>) {
        if let Ok(msg) = serde_json::to_string(&res) {
            ctx.text(msg);
        }
    }

    fn unsubscribe(&mut self) {
        if let Some(doc) = self.document.take() {
            let msg = UnsubscribeMsg {
                id: self.id.clone()
            };
            doc.do_send(msg);
        }
    }
}

impl Actor for Client {
    type Context = WebsocketContext<Client>;

    fn started(&mut self, ctx: &mut WebsocketContext<Client>) {
        // Check heart beat.
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
        let msg = RpcResponseMsg::ClientId { client_id: self.id.clone() };
        ctx.address().do_send(msg);

        let dir = Path::new(".");
        let sources = get_sources(dir);
        let msg = RpcResponseMsg::Sources { sources };
        ctx.address().do_send(msg);
    }

    fn stopped(&mut self, _ctx: &mut WebsocketContext<Client>) {
        self.unsubscribe();
        let mut ids = CLIENT_IDS.lock().unwrap();
        ids.remove(&self.id);
    }
}

impl StreamHandler<Result<Message, ProtocolError>> for Client {
    fn handle(
        &mut self,
        msg: Result<Message, ProtocolError>,
        ctx: &mut WebsocketContext<Client>
    ) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        use Message::*;
        match msg {
            Ping(msg) => {
                ctx.pong(&msg);
            }
            Pong(_) => {
                self.hb = Instant::now();
            }
            Text(msg) => {
                let msg = msg.to_string();
                if let Ok(rpc) = serde_json::from_str::<RpcRequest>(&*msg) {
                    // Handle request.
                    use RpcRequest::*;
                    match rpc {
                        OpenDoc { id, path } => {
                            self.open_doc(id, path, ctx);
                        }
                        UpdateDoc { id, version, operations } => {
                            if let Some(addr) = &self.document {
                                let msg = UpdateDocMsg {
                                    req_id: id,
                                    client_id: self.id.clone(),
                                    version,
                                    operations
                                };
                                addr.do_send(msg);
                            } else {
                                let msg = RpcResponseMsg::Error {
                                    id: Some(id),
                                    code: RpcErrorCode::InvalidRequest,
                                    msg: String::from("Open doc before updating doc!")
                                };
                                ctx.address().do_send(msg);
                            }
                        }
                        GetDoc { id } => {
                            if let Some(addr) = &self.document {
                                let msg = GetDocMsg {
                                    client: ctx.address(),
                                    req_id: id
                                };
                                addr.do_send(msg);
                            } else {
                                let msg = RpcResponseMsg::Error {
                                    id: Some(id),
                                    code: RpcErrorCode::InvalidRequest,
                                    msg: String::from("Open doc before requesting doc!")
                                };
                                ctx.address().do_send(msg);
                            }
                        }
                        Query { id, node_id } => {
                            if let Some(addr) = &self.document {
                                let msg = QueryMsg {
                                    client: ctx.address(),
                                    client_id: self.id.clone(),
                                    req_id: id,
                                    node_id
                                };
                                addr.do_send(msg);
                            } else {
                                let msg = RpcResponseMsg::Error {
                                    id: Some(id),
                                    code: RpcErrorCode::InvalidRequest,
                                    msg: String::from("Open doc before querying!")
                                };
                                ctx.address().do_send(msg);
                            }
                        }
                        ReadFile { id, filename } => {
                            if let Some(addr) = &self.document {
                                let msg = ReadFileMsg {
                                    client: ctx.address(),
                                    client_id: self.id.clone(),
                                    req_id: id,
                                    filename
                                };
                                addr.do_send(msg);
                            } else {
                                let msg = RpcResponseMsg::Error {
                                    id: Some(id),
                                    code: RpcErrorCode::InvalidRequest,
                                    msg: String::from("Open doc before querying!")
                                };
                                ctx.address().do_send(msg);
                            }
                        }
                        CancelJob { id } => {
                            let msg = CancelJobMsg {
                                client: ctx.address(),
                                client_id: self.id.clone(),
                                job_id: id,
                            };
                            <Executor as SystemService>::from_registry()
                                .do_send(msg);
                        }
                        CloseDoc { id } => {
                            self.close_doc(id, ctx);
                        }
                    }
                } else {
                    let res = RpcResponseMsg::Error {
                        id: None,
                        code: RpcErrorCode::ParseError,
                        msg: String::from("Failed to parse message.")
                    };
                    if let Ok(msg) = serde_json::to_string(&res) {
                        ctx.text(msg);
                    }
                }
            }
            Binary(_) => log::info!("Unexpected binary"),
            Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            Continuation(_) => {
                ctx.stop();
            }
            Nop => (),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RpcRequest {
    OpenDoc {
        id: usize,
        path: String
    },
    UpdateDoc {
        id: usize,
        version: usize,
        operations: Vec<Operation>
    },
    GetDoc {
        id: usize
    },
    Query {
        id: usize,
        node_id: String
    },
    ReadFile {
        id: usize,
        filename: String
    },
    CancelJob {
        id: usize
    },
    CloseDoc {
        id: usize
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RpcErrorCode {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize, MessageTrait)]
#[rtype(result = "()")]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RpcResponseMsg {
    ClientId {
        client_id: String
    },
    Sources {
        sources: Vec<String>
    },
    Doc {
        id: usize,
        version: usize,
        doc: Doc
    },
    DocClosed {
        id: usize
    },
    UpdateDoc {
        /// The client that make the update gets a response with an id, others
        /// don't.
        id: Option<usize>,
        version: usize,
        operations: Vec<Operation>
    },
    QueryResult {
        id: usize,
        data: DataFrame
    },
    FileData {
        id: usize,
        data: DataFrame
    },
    JobCanceled {
        id: usize
    },
    Error {
        id: Option<usize>,
        code: RpcErrorCode,
        msg: String
    }
}

impl Handler<RpcResponseMsg> for Client {
    type Result = ();

    fn handle(
        &mut self,
        msg: RpcResponseMsg,
        ctx: &mut WebsocketContext<Client>
    ) {
        if let Ok(msg) = serde_json::to_string(&msg) {
            ctx.text(msg);
        }
    }
}

fn new_client_id() -> String {
    let mut id = random_string(5);
    let mut ids = CLIENT_IDS.lock().unwrap();
    let mut i = 0;
    loop {
        if !ids.contains(&id) {
            ids.insert(id.clone());
            break;
        }
        let len = i.max(5);
        id = random_string(len);
        i += 1;
    }
    id
}

fn random_string(len: usize) -> String {
    Alphanumeric
        .sample_iter(thread_rng())
        .take(len)
        .map(char::from)
        .collect()
}

fn get_sources<T: AsRef<Path>>(dir: T) -> Vec<String> {
    let mut sources = vec![];
    if let Ok(entries) = dir.as_ref().read_dir() {
        entries.for_each(|entry_res| {
            if let Ok(entry) = entry_res {
                if let Ok(ftype) = entry.file_type(){
                    if ftype.is_file() {
                        let path = entry.path();
                        if let Some(filename) = path.file_name() {
                            let filename = filename
                                .to_string_lossy()
                                .to_string();
                            if filename.ends_with(".csv") {
                                sources.push(filename);
                            }
                        }
                    }
                }
            }
        });
    }
    sources
}
