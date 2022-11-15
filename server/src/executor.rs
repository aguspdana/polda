use actix::Addr;
use actix::Actor;
use actix::AsyncContext;
use actix::Context;
use actix::fut::wrap_future;
use actix::Message as MessageTrait;
use actix::Handler;
use actix::Supervised;
use actix::SystemService;
use query::doc::collect;
use query::doc::Node;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use crate::Client;
use crate::client::RpcErrorCode;
use crate::client::RpcResponseMsg;

const ROW_LIMIT: usize = 100;

#[derive(Debug)]
pub enum JobType {
    Query
}

pub struct Job {
    pub client: Addr<Client>,
    pub client_id: String,
    pub job_id: usize,
    pub job_type: JobType,
    pub nodes: HashMap<String, Node>,
    pub node_id: String
}

/// The currently running job is kept in the queue until the next job is
/// retrieved.  This way there's no job queued in the mpsc channel.
#[derive(Default)]
struct Queue {
    jobs: HashMap<(String, usize), Arc<Job>>,
    index: VecDeque<(String, usize)>
}

impl Queue {
    fn len(&self) -> usize {
        self.index.len()
    }

    fn next(&mut self) -> Option<Arc<Job>> {
        // Remove the finished job.
        if let Some(id) = self.index.pop_front() {
            if let Some(job) = self.jobs.remove(&id) {
                return Some(job);
            }
        }
        while let Some(id) = self.index.front() {
            if let Some(job) = self.jobs.get(id) {
                return Some(job.clone());
            } else {
                self.index.pop_front();
            }
        }
        None
    }

    fn push(&mut self, job: Arc<Job>) {
        self.index.push_back((job.client_id.clone(), job.job_id));
        self.jobs.insert((job.client_id.clone(), job.job_id), job);
    }

    fn remove(&mut self, client_id: String, job_id: usize) -> Option<Arc<Job>> {
        self.jobs.remove(&(client_id, job_id))
    }
}


#[derive(Default)]
pub struct Executor {
    jobs: Queue,
    sender: Option<Sender<Arc<Job>>>
}

impl Actor for Executor {
    type Context = Context<Executor>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let (sender, mut receiver) = mpsc::channel(100);
        self.sender = Some(sender);
        let executor = ctx.address();
        ctx.spawn(wrap_future(async move {
            while let Some(job) = receiver.recv().await {
                handle_job(job.as_ref());
                executor.do_send(NextJobMsg);
            }
        }));
    }
}

#[derive(MessageTrait)]
#[rtype(result = "()")]
pub struct JobMsg(pub Arc<Job>);

impl Handler<JobMsg> for Executor {
    type Result = ();

    fn handle(
        &mut self,
        msg: JobMsg,
        ctx: &mut Context<Executor>
    ) {
        if self.jobs.len() == 0 {
            if let Some(sender) = &self.sender {
                let sender = sender.clone();
                ctx.spawn(wrap_future(async move {
                    sender.send(msg.0).await.ok();
                }));
            }
        } else {
            self.jobs.push(msg.0);
        }
    }
}

#[derive(MessageTrait)]
#[rtype(result = "()")]
pub struct CancelJobMsg {
    pub client: Addr<Client>,
    pub client_id: String,
    pub job_id: usize
}

impl Handler<CancelJobMsg> for Executor {
    type Result = ();

    fn handle(
        &mut self,
        msg: CancelJobMsg,
        _ctx: &mut Context<Executor>
    ) {
        let CancelJobMsg { client, client_id, job_id } = msg;
        if let Some(job) = self.jobs.remove(client_id, job_id) {
            match &job.as_ref().job_type {
                JobType::Query => {
                    let msg = RpcResponseMsg::QueryCanceled { id: msg.job_id };
                    client.do_send(msg);
                }
            }
        }
    }
}

#[derive(MessageTrait)]
#[rtype(result = "()")]
pub struct NextJobMsg;

impl Handler<NextJobMsg> for Executor {
    type Result = ();

    fn handle(
        &mut self,
        _msg: NextJobMsg,
        ctx: &mut Context<Executor>
    ) {
        if let Some(job) = self.jobs.next() {
            if let Some(sender) = &self.sender {
                let sender = sender.clone();
                ctx.spawn(wrap_future(async move {
                    sender.send(job).await.ok();
                }));
            }
        }
    }
}

impl Supervised for Executor {}
impl SystemService for Executor {}

fn handle_job(job: &Job) {
    let Job {
        client,
        client_id: _,
        job_id,
        job_type,
        nodes,
        node_id
    } = job;
    let msg = match job_type {
        JobType::Query => {
            let res = collect(&nodes, &node_id, Some(ROW_LIMIT));
            match res {
                Ok(df) => {
                    RpcResponseMsg::QueryResult {
                        id: job_id.clone(),
                        data: df
                    }
                }
                Err(e) => {
                    RpcResponseMsg::Error {
                        id: Some(job_id.clone()),
                        // TODO: Use a more appropriate error code.
                        code: RpcErrorCode::InternalError,
                        msg: e.to_string()
                    }
                }
            }
        }
    };
    client.do_send(msg);
}

