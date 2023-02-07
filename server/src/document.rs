use actix::Addr;
use actix::Actor;
use actix::ActorContext;
use actix::AsyncContext;
use actix::Context;
use actix::Message as MessageTrait;
use actix::Handler;
use actix::Running;
use actix::SystemService;
use query::doc::Aggregate;
use query::doc::AggregateComputation;
use query::doc::Doc;
use query::doc::InputName;
use query::doc::Node;
use query::doc::Operation;
use query::doc::Position;
use query::doc::SortDirection;
use query::doc::Sorter;
use query::doc::transform_batch;
use query::doc::validate_sequence;
use query::error::PoldaError;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use crate::broker::Broker;
use crate::broker::CloseDocumentMsg;
use crate::client::Client;
use crate::client::RpcErrorCode;
use crate::client::RpcResponseMsg;
use crate::executor::Executor;
use crate::executor::Job;
use crate::executor::JobMsg;
use crate::executor::JobKind;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const TIMEOUT: Duration = Duration::from_secs(60);

pub struct Document {
    path: String,
    doc: Doc,
    operations: Vec<Operation>,
    deleted_ops: usize,
    clients: HashMap<String, Addr<Client>>,
    hb: Instant
}

impl Document {
    pub fn open(path: String) -> Result<Document, PoldaError> {
        Ok(Document {
            path,
            doc: demo_doc(),
            operations: vec![],
            deleted_ops: 0,
            clients: HashMap::new(),
            hb: Instant::now()
        })
    }
}

impl Actor for Document {
    type Context = Context<Document>;

    fn started(&mut self, ctx: &mut Context<Document>) {
        // Check heart beat.
        AsyncContext::run_interval(ctx, HEARTBEAT_INTERVAL, |act, ctx| {
            if act.clients.len() > 0 {
                act.hb = Instant::now();
            } else if Instant::now().duration_since(act.hb) > TIMEOUT {
                ActorContext::stop(ctx);
                return;
            }
        });
    }

    fn stopping(&mut self, _ctx: &mut Context<Document>) -> Running {
        let msg = CloseDocumentMsg {
            path: self.path.clone()
        };
        <Broker as SystemService>::from_registry()
            .do_send(msg);

        Running::Stop
    }
}

#[derive(MessageTrait)]
#[rtype(result = "()")]
pub struct SubscribeMsg {
    pub id: String,
    pub client: Addr<Client>
}

impl Handler<SubscribeMsg> for Document {
    type Result = ();

    fn handle(
        &mut self,
        msg: SubscribeMsg,
        _ctx: &mut Context<Document>
    ) {
        self.clients.insert(msg.id, msg.client);
    }
}

#[derive(MessageTrait)]
#[rtype(result = "()")]
pub struct UnsubscribeMsg {
    pub id: String
}

impl Handler<UnsubscribeMsg> for Document {
    type Result = ();

    fn handle(
        &mut self,
        msg: UnsubscribeMsg,
        _ctx: &mut Context<Document>
    ) {
        self.clients.remove(&msg.id);
    }
}

#[derive(MessageTrait)]
#[rtype(result = "()")]
pub struct GetDocMsg {
    pub client: Addr<Client>,
    pub req_id: usize
}

impl Handler<GetDocMsg> for Document {
    type Result = ();

    fn handle(
        &mut self,
        msg: GetDocMsg,
        _ctx: &mut Context<Document>
    ) {
        let version = self.deleted_ops + self.operations.len();
        let GetDocMsg { client, req_id } = msg;
        let msg = RpcResponseMsg::Doc {
            id: req_id,
            version,
            doc: self.doc.clone()
        };
        client.do_send(msg);
    }
}

#[derive(MessageTrait)]
#[rtype(result = "()")]
pub struct UpdateDocMsg {
    pub client_id: String,
    pub req_id: usize,
    pub version: usize,
    pub operations: Vec<Operation>
}

impl Handler<UpdateDocMsg> for Document {
    type Result = ();

    fn handle(
        &mut self,
        msg: UpdateDocMsg,
        _ctx: &mut Context<Document>
    ) {
        let UpdateDocMsg {
            client_id,
            req_id,
            version,
            operations
        } = msg;
        if version < self.deleted_ops {
            if let Some(client) = self.clients.get(&client_id) {
                let msg = RpcResponseMsg::Error {
                    id: Some(req_id),
                    code: RpcErrorCode::InvalidRequest,
                    msg: String::from("Unsyncable")
                };
                client.do_send(msg);
            }
            return;
        }
        if version > self.deleted_ops + self.operations.len() {
            if let Some(client) = self.clients.get(&client_id) {
                let msg = RpcResponseMsg::Error {
                    id: Some(req_id),
                    code: RpcErrorCode::InvalidRequest,
                    msg: String::from("Unsyncable")
                };
                client.do_send(msg);
            }
            return;
        }
        if let Err(e) = validate_sequence(&operations) {
            if let Some(client) = self.clients.get(&client_id) {
                let msg = RpcResponseMsg::Error {
                    id: Some(req_id),
                    code: RpcErrorCode::InvalidRequest,
                    msg: e.to_string()
                };
                client.do_send(msg);
            }
            return;
        }
        let preceding_ops = &self.operations[version-self.deleted_ops..];
        let mut transformed_ops = transform_batch(operations, preceding_ops);
        match self.doc.execute_operations(transformed_ops.clone()) {
            Ok(_undo_ops) => {
                self.clients
                    .iter()
                    .for_each(|(id, client)| {
                        let res_id = if id == &client_id {
                            Some(req_id.clone())
                        } else {
                            None
                        };
                        let msg = RpcResponseMsg::UpdateDoc {
                            id: res_id,
                            version: self.deleted_ops
                                + self.operations.len()
                                + transformed_ops.len(),
                            operations: transformed_ops.clone()
                        };
                        client.do_send(msg);
                    });
                self.operations.append(&mut transformed_ops);
            }
            Err(e) => {
                if let Some(client) = self.clients.get(&client_id) {
                    let msg = RpcResponseMsg::Error {
                        id: Some(req_id),
                        code: RpcErrorCode::InvalidRequest,
                        msg: format!("{:?}", e)
                    };
                    client.do_send(msg);
                }
            }
        }
    }
}

#[derive(MessageTrait)]
#[rtype(result = "()")]
pub struct QueryMsg {
    pub client: Addr<Client>,
    pub client_id: String,
    pub req_id: usize,
    pub node_id: String
}

impl Handler<QueryMsg> for Document {
    type Result = ();

    fn handle(
        &mut self,
        msg: QueryMsg,
        _ctx: &mut Context<Document>
    ) {
        let QueryMsg { client, client_id, req_id, node_id } = msg;
        match self.doc.extract_nodes(&node_id) {
            Ok(nodes) => {
                let msg = JobMsg(Arc::new(Job {
                    client,
                    client_id,
                    job_id: req_id,
                    job_kind: JobKind::Query {
                        nodes,
                        node_id
                    },
                }));
                <Executor as SystemService>::from_registry()
                    .do_send(msg);
            }
            Err(e) => {
                let msg = RpcResponseMsg::Error {
                    id: Some(req_id),
                    code: RpcErrorCode::InvalidRequest,
                    msg: e.to_string()
                };
                client.do_send(msg);
            }
        }
    }
}

#[derive(MessageTrait)]
#[rtype(result = "()")]
pub struct ReadFileMsg {
    pub client: Addr<Client>,
    pub client_id: String,
    pub req_id: usize,
    pub filename: String
}

impl Handler<ReadFileMsg> for Document {
    type Result = ();

    fn handle(
        &mut self,
        msg: ReadFileMsg,
        _ctx: &mut Context<Document>
    ) {
        let ReadFileMsg { client, client_id, req_id, filename } = msg;
        let msg = JobMsg(Arc::new(Job {
            client,
            client_id,
            job_id: req_id,
            job_kind: JobKind::ReadFile { filename }
        }));
        <Executor as SystemService>::from_registry()
            .do_send(msg);
    }
}

fn demo_doc() -> Doc {
    let mut doc = Doc::new();
    let ops = vec![
        Operation::InsertNode {
            node: Node::LoadCsv {
                id: "demo_1".to_string(),
                position: Position {
                    x: 0.0,
                    y: 0.0
                },
                filename: "Forbes 2000.csv".to_string(),
                outputs: HashSet::new()
            }
        },
        Operation::InsertIndex {
            id: "demo_1".to_string(),
            index: 0
        },
        Operation::InsertNode {
            node: Node::Aggregate {
                id: "demo_2".to_string(),
                position: Position {
                    x: 256.0 + 64.0,
                    y: 0.0
                },
                input: None,
                aggregates: vec![
                    Aggregate {
                        column: "Industry".to_string(),
                        computation: AggregateComputation::Group,
                        alias: "".to_string()
                    },
                    Aggregate {
                        column: "Market Value (Billions)".to_string(),
                        computation: AggregateComputation::Sum,
                        alias: "".to_string()
                    }
                ],
                outputs: HashSet::new()
            }
        },
        Operation::InsertIndex {
            id: "demo_2".to_string(),
            index: 0
        },
        Operation::SetInput {
            id: "demo_2".to_string(),
            name: InputName::Primary,
            input: Some("demo_1".to_string())
        },
        Operation::InsertNode {
            node: Node::Sort {
                id: "demo_3".to_string(),
                position: Position {
                    x: 2.0 * (256.0 + 64.0),
                    y: 0.0
                },
                input: None,
                sorters: vec![
                    Sorter {
                        column: "Market Value (Billions)".to_string(),
                        direction: SortDirection::Desc
                    }
                ],
                outputs: HashSet::new()
            }
        },
        Operation::InsertIndex {
            id: "demo_3".to_string(),
            index: 0
        },
        Operation::SetInput {
            id: "demo_3".to_string(),
            name: InputName::Primary,
            input: Some("demo_2".to_string())
        }
    ];
    doc.execute_operations(ops).unwrap();
    doc
}
