use borsh::{BorshDeserialize, BorshSerialize};
use spin_primitives::{Block, SignedTransaction};

pub mod context;
pub mod executor;
pub mod syscalls;

pub struct SpinNode {
    db: sled::Db,
    txs_pool: std::collections::VecDeque<SignedTransaction>,
}

impl SpinNode {
    pub fn new(db_path: String) -> Self {
        Self {
            db: sled::open(db_path).unwrap(),
            txs_pool: std::collections::VecDeque::new(),
        }
    }

    pub fn init_genesis(&mut self) {
        let genesis_block = Block {
            height: 1,
            hash: [0; 32],
            parent_hash: [0; 32],
            timestamp: 0,
            txs: vec![],
        };

        self.insert_block(genesis_block);
    }

    fn insert_block(&mut self, block: Block) {
        self.db
            .insert(
                format!("block_{}", block.height),
                block.try_to_vec().unwrap(),
            )
            .unwrap();

        self.db
            .insert(b"latest_block", block.try_to_vec().unwrap())
            .unwrap();
    }

    pub fn block_by_height(&self, height: u64) -> Option<Block> {
        let block = self
            .db
            .get(format!("block_{}", height))
            .unwrap()
            .map(|bytes| BorshDeserialize::try_from_slice(&mut bytes.to_vec()).unwrap());

        block
    }

    pub fn latest_block(&self) -> Block {
        let mut latest_block_bytes = self.db.get(b"latest_block").unwrap().unwrap().to_vec();
        let latest_block: Block =
            BorshDeserialize::try_from_slice(&mut latest_block_bytes).unwrap();

        latest_block
    }

    pub fn add_tx(&mut self, tx: SignedTransaction) {
        self.txs_pool.push_back(tx);
    }

    pub fn produce_block(&mut self) -> Block {
        let txs = self.txs_pool.drain(0..3).collect::<Vec<_>>();

        unimplemented!()
    }
}

#[cfg(test)]
mod ests {
    use super::*;

    #[test]
    fn test_init_genesis() {
        let mut node = SpinNode::new("temp_spin_db".to_string());
        node.init_genesis();

        let latest_block = node.latest_block();
        assert_eq!(latest_block.height, 1);
    }
}
