// FIXME: Sometimes clippy incorrectly calculates lifetimes.
#![cfg_attr(feature = "cargo-clippy", allow(let_and_return))]

#[macro_use]
extern crate display_derive;
#[macro_use]
extern crate enum_primitive_derive;
#[macro_use]
extern crate exonum;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

extern crate bodyparser;
extern crate byteorder;
extern crate chrono;
extern crate exonum_time;
extern crate iron;
extern crate num_traits;
extern crate rand;
extern crate router;
extern crate serde;
extern crate pnet;

pub mod api;
pub mod data_layout;
pub mod schema;
pub mod transactions;

use iron::Handler;
use router::Router;

use exonum::api::Api;
use exonum::blockchain::{ApiContext, Service, ServiceContext, Transaction, TransactionSet};
use exonum::crypto::Hash;
use exonum::encoding;
use exonum::helpers::fabric::{Context, ServiceFactory};
use exonum::messages::RawTransaction;
use exonum::storage::Snapshot;
use pnet::datalink;

use api::ExoneumCoreApi;
use schema::ExoneumCoreSchema;
use transactions::Transactions;

/// Unique service identifier
pub const CORE_SERVICE_ID: u16 = 9999;
/// Unique service name which will be used in API and configuration
pub const CORE_SERVICE_NAME: &str = "exoneumcore";

/// Sum to be issued each time
pub const ISSUE_AMOUNT: u64 = 100;

/// Timeout (seconds) before user will be able to issue funds again
pub const ISSUE_TIMEOUT: i64 = 60;

#[derive(Debug, Default)]
pub struct ExoneumCoreService;

#[derive(Debug, Default)]
pub struct ExoneumCoreServiceFactory;

impl ServiceFactory for ExoneumCoreServiceFactory {
    fn make_service(&mut self, _: &Context) -> Box<Service> {
        Box::new(ExoneumCoreService)
    }
}

impl Service for ExoneumCoreService {
    fn service_name(&self) -> &'static str {
        "exoneum_core"
    }

    fn service_id(&self) -> u16 {
        CORE_SERVICE_ID
    }

    // Method to deserialize transacitons
    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        let tx = Transactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    // Tables hashes to be included into blockchain state hash
    fn state_hash(&self, snapshot: &Snapshot) -> Vec<Hash> {
        let schema = ExoneumCoreSchema::new(snapshot);
        schema.state_hash()
    }

    // Check open auctions state after each block's commit
    fn handle_commit(&self, ctx: &ServiceContext) {
        let current_time = if let Some(time) = transactions::current_time(ctx.snapshot()) {
            time
        } else {
            return;
        };

        let schema = ExoneumCoreSchema::new(ctx.snapshot());
    }

    // Handling requests to a node
    fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let api = ExoneumCoreApi {
            channel: ctx.node_channel().clone(),
            blockchain: ctx.blockchain().clone(),
        };
        api.wire(&mut router);
        Some(Box::new(router))
    }
}
