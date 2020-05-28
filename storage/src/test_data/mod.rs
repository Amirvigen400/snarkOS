use crate::Ledger;

use snarkos_algorithms::merkle_tree::MerkleParameters;
use snarkos_models::objects::{LedgerScheme, Transaction};
use snarkos_objects::Block;

use rand::{thread_rng, Rng};
use std::{path::PathBuf, sync::Arc};

pub fn random_storage_path() -> String {
    let random_path: usize = thread_rng().gen();
    format!("./test_db-{}", random_path)
}

// Initialize a test blockchain given genesis attributes
pub fn initialize_test_blockchain<T: Transaction, P: MerkleParameters>(
    parameters: P,
    genesis_block: Block<T>,
) -> Ledger<T, P> {
    let mut path = std::env::temp_dir();
    path.push(random_storage_path());

    Ledger::<T, P>::destroy_storage(path.clone()).unwrap();

    let storage = Ledger::<T, P>::new(&path, parameters, genesis_block).unwrap();

    storage
}

// Open a test blockchain from stored genesis attributes
pub fn open_test_blockchain<T: Transaction, P: MerkleParameters>() -> (Arc<Ledger<T, P>>, PathBuf) {
    let mut path = std::env::temp_dir();
    path.push(random_storage_path());

    Ledger::<T, P>::destroy_storage(path.clone()).unwrap();

    let storage = Arc::new(Ledger::<T, P>::open_at_path(path.clone()).unwrap());

    (storage, path)
}

pub fn kill_storage<T: Transaction, P: MerkleParameters>(ledger: Ledger<T, P>) {
    let path = ledger.storage.db.path().to_owned();

    drop(ledger);
    Ledger::<T, P>::destroy_storage(path).unwrap();
}

pub fn kill_storage_async<T: Transaction, P: MerkleParameters>(path: PathBuf) {
    Ledger::<T, P>::destroy_storage(path).unwrap();
}

pub fn kill_storage_sync<T: Transaction, P: MerkleParameters>(ledger: Arc<Ledger<T, P>>) {
    let path = ledger.storage.db.path().to_owned();

    drop(ledger);
    Ledger::<T, P>::destroy_storage(path).unwrap();
}