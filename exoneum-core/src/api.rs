    /// Module with API implementation
    use bodyparser;
    use iron::prelude::*;

    use router::Router;

    use exonum::api::{Api, ApiError};
    use exonum::crypto::{Hash, PublicKey};

    use exonum::blockchain::{Blockchain, Transaction};
    use exonum::node::{ApiSender, TransactionSend};

    use data_layout::User;
    use schema;
    use transactions::Transactions;

    #[derive(Clone)]
    pub struct ExoneumCoreApi {
        pub channel: ApiSender,
        pub blockchain: Blockchain,
    }

    impl Api for ExoneumCoreApi {
        fn wire(&self, router: &mut Router) {
            let self_ = self.clone();
            let get_user = move |req: &mut Request| {
                let public_key: PublicKey = self_.url_fragment(req, "public_key")?;
                if let Some(user) = self_.get_user(&public_key) {
                    self_.ok_response(&json!(user))
                } else {
                    self_.not_found_response(&json!("User not found"))
                }
            };

            let self_ = self.clone();
            let get_users = move |_: &mut Request| {
                let users = self_.get_users();
                self_.ok_response(&json!(&users))
            };

            let self_ = self.clone();
            let transaction =
                move |req: &mut Request| match req.get::<bodyparser::Struct<Transactions>>() {
                    Ok(Some(transaction)) => {
                        let tx_hash = self_.post_transaction(transaction).map_err(ApiError::from)?;
                        let json = json!({ "tx_hash": tx_hash });
                        self_.ok_response(&json)
                    }
                    Ok(None) => Err(ApiError::BadRequest("Empty request body".into()))?,
                    Err(e) => Err(ApiError::BadRequest(e.to_string()))?,
                };

            // View-only handlers
            router.get("/v1/users", get_users, "get_users");
            router.get("/v1/user/:public_key", get_user, "get_user");

            // Transactions
            router.post("/v1/transaction", transaction, "post_transaction");
        }
    }

    impl ExoneumCoreApi {
        /// User profile
        fn get_user(&self, public_key: &PublicKey) -> Option<User> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::ExoneumCoreSchema::new(snapshot);
            schema.users().get(public_key)
        }

        /// All users
        fn get_users(&self) -> Vec<User> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::ExoneumCoreSchema::new(snapshot);
            let idx = schema.users();
            let users: Vec<User> = idx.values().collect();
            users
        }

        fn post_transaction(&self, transaction: Transactions) -> Result<Hash, ApiError> {
            let transaction: Box<Transaction> = transaction.into();
            let tx_hash = transaction.hash();
            self.channel.send(transaction)?;
            Ok(tx_hash)
        }
    }
