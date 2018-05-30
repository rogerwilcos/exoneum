/// Database schema
use exonum::crypto::{Hash, PublicKey};
use exonum::storage::{
    Fork, ListIndex, MapIndex, ProofListIndex, ProofMapIndex, Snapshot, ValueSetIndex,
};

use data_layout::User;

pub struct ExoneumCoreSchema<T> {
    view: T,
}

/// Read-only tables
impl<T> ExoneumCoreSchema<T>
where
    T: AsRef<Snapshot>,
{
    pub fn new(view: T) -> Self {
        ExoneumCoreSchema { view }
    }

    /// Users
    pub fn users(&self) -> ProofMapIndex<&T, PublicKey, User> {
        ProofMapIndex::new("exoneum_core.users", &self.view)
    }

    /// Method to get state hash. Depends on `users` table.
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![
            self.users().merkle_root(),
        ]
    }
}

/// Mutable accessors for all our tables
impl<'a> ExoneumCoreSchema<&'a mut Fork> {
    pub fn users_mut(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, User> {
        ProofMapIndex::new("exoneum_core.users", self.view)
    }
}
