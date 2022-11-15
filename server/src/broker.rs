use actix::Addr;
use actix::Actor;
use actix::Context;
use actix::Message as MessageTrait;
use actix::Handler;
use actix::Supervised;
use actix::SystemService;
use query::error::PoldaError;
use std::collections::HashMap;

use crate::document::Document;

#[derive(Default)]
pub struct Broker {
    documents: HashMap<String, Addr<Document>>,
}

impl Broker {
    pub fn new() -> Broker {
        Broker {
            documents: HashMap::new()
        }
    }
}

impl Actor for Broker {
    type Context = Context<Broker>;
}

#[derive(MessageTrait)]
#[rtype(result = "Result<Addr<Document>, PoldaError>")]
pub struct OpenDocumentMsg {
    pub path: String
}

impl Handler<OpenDocumentMsg> for Broker {
    type Result = Result<Addr<Document>, PoldaError>;

    fn handle(
        &mut self,
        msg: OpenDocumentMsg,
        _ctx: &mut Context<Broker>
    ) -> Result<Addr<Document>, PoldaError> {
        let OpenDocumentMsg { path } = msg;
        if let Some(doc) = self.documents.get(&path) {
            Ok(doc.clone())
        } else {
            match Document::open(path.clone()) {
                Ok(doc) => {
                    let doc = doc.start();
                    self.documents.insert(path, doc.clone());
                    Ok(doc)
                }
                Err(e) => Err(e)
            }
        }
    }
}

#[derive(MessageTrait)]
#[rtype(result = "()")]
pub struct CloseDocumentMsg {
    pub path: String
}

impl Handler<CloseDocumentMsg> for Broker {
    type Result = ();

    fn handle(
        &mut self,
        msg: CloseDocumentMsg,
        _ctx: &mut Context<Broker>
    ) {
        self.documents.remove(&msg.path);
    }
}

impl Supervised for Broker {}
impl SystemService for Broker {}
