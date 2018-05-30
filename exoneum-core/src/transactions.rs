/// Module with description of all transactions
use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, Duration, Utc};
use data_layout::User;
use exonum::blockchain::{ExecutionError, ExecutionResult, Schema, Transaction};
use exonum::crypto::{CryptoHash, Hash, PublicKey};
use exonum::messages::Message;
use exonum::storage::{Fork, Snapshot};
use exonum_time::TimeSchema;
use num_traits::ToPrimitive;
use rand::distributions::{Sample, Weighted, WeightedChoice};
use rand::{IsaacRng, Rng, SeedableRng};
use std::io::Cursor;

use schema;
use schema::ExoneumCoreSchema;
use {CORE_SERVICE_ID, ISSUE_AMOUNT, ISSUE_TIMEOUT};

transactions! {
    pub Transactions {
        const SERVICE_ID = CORE_SERVICE_ID;

        /// Transaction to create a new user
        struct CreateUser {
            /// Public user identifier
            public_key: &PublicKey,
            /// Name
            name: &str,
        }
    }
}

impl Transaction for CreateUser {
    fn verify(&self) -> bool {
        self.verify_signature(self.public_key())
    }

    fn execute(&self, fork: &mut Fork) -> ExecutionResult {
        let ts = current_time(fork).unwrap();

        let state_hash = {
            let info_schema = Schema::new(&fork);
            info_schema.state_hash_aggregator().merkle_root()
        };

        let key = self.public_key();
        let mut schema = schema::ExoneumCoreSchema::new(fork);

        // Reject tx if the user with the same public identifier is already exists
        if schema.users().get(key).is_some() {
            Err(ErrorKind::UserAlreadyRegistered)?;
        }

        let user = User::new(key, self.name(), ISSUE_AMOUNT);
        schema.users_mut().put(key, user);

        Ok(())
    }
}

/// Read-only tables
impl<T> ExoneumCoreSchema<T>
where
    T: AsRef<Snapshot>,
{

}

/// Mutable accessors for all our tables
impl<'a> ExoneumCoreSchema<&'a mut Fork> {

    /// Helper method to increase user balance
    pub fn increase_user_balance(
        &mut self,
        user_id: &PublicKey,
        balance: u64,
    ) {
        let user = self.users().get(user_id).expect("User should be exist.");
        self.users_mut().put(
            user.public_key(),
            User::new(
                user.public_key(),
                user.name(),
                user.balance() + balance,
            ),
        );
    }

    /// Helper method to decrease user balance
    pub fn decrease_user_balance(&mut self, user_id: &PublicKey, balance: u64) {
        let user = self.users().get(user_id).expect("User should be exist.");
        self.users_mut().put(
            user.public_key(),
            User::new(
                user.public_key(),
                user.name(),
                user.balance() - balance,
            ),
        );
    }
}

// A helper function to get current time from the time oracle
pub fn current_time(snapshot: &Snapshot) -> Option<DateTime<Utc>> {
    let time_schema = TimeSchema::new(snapshot);
    time_schema.time().get()
}

#[derive(Display, Primitive)]
pub enum ErrorKind {
    #[display(fmt = "Too early for breeding.")]
    EarlyBreeding = 1,
    //
    #[display(fmt = "Too early for balance refill.")]
    EarlyIssue = 2,
    //
    #[display(fmt = "Insufficient funds.")]
    InsufficientFunds = 3,
    //
    #[display(fmt = "Not your property.")]
    AccessViolation = 4,
    //
    #[display(fmt = "You need two different owls.")]
    SelfBreeding = 5,
    //
    #[display(fmt = "User is already registered")]
    UserAlreadyRegistered = 6,
    //
    #[display(fmt = "Participant is not registered")]
    UserIsNotRegistered = 7,
    //
    #[display(fmt = "Owl does not exist")]
    OwlNotFound = 8,
    //
    #[display(fmt = "You do not own of the item")]
    OwlNotOwned = 9,
    //
    #[display(fmt = "Owl is already auctioned")]
    OwlAlreadyAuctioned = 10,
    //
    #[display(fmt = "Auction does not exist")]
    AuctionNotFound = 11,
    //
    #[display(fmt = "Auction is closed")]
    AuctionClosed = 12,
    //
    #[display(fmt = "Bid is below the current highest bid")]
    BidTooLow = 13,
    // CloseAuction may only be performed by the validator nodes.
    #[display(fmt = "Transaction is not authorized.")]
    UnauthorizedTransaction = 14,
    //
    #[display(fmt = "You may not bid on your own item.")]
    NoSelfBidding = 15,
}

impl ErrorKind {
    /// Converts error to the raw code
    pub fn as_code(&self) -> u8 {
        self.to_u8().unwrap()
    }
}

impl From<ErrorKind> for ExecutionError {
    fn from(e: ErrorKind) -> ExecutionError {
        let err_txt = format!("{}", e);
        ExecutionError::with_description(e.as_code(), err_txt)
    }
}
