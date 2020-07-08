//////////////////////////////////////////////////////////////////////
/* Signal and Slots begin */

// Purpose of this source file is to define the global queue of slot transactions
// as well as provide helper functions for handling signal/slot state transitions.

// Global address that stores the trie containing all the future slot transactions
// that are not currently available for execution. This trie is pruned every epoch.
// When a new epoch starts, all the slot transactions that become available are
// pushed into the queues of individual accounts.

use cfx_types::{Address};
use std::str::FromStr;

lazy_static! {
    // Last 20 digits of keccak256(Boundless!!!)
    pub static ref GLOBAL_SLOT_TX_QUEUE_ADDRESS: Address =
        Address::from_str("db73c9d8eeaac3e5de3f83b71fb7aa4e41764d09").unwrap();

    pub static ref GLOBAL_SLOT_TX_ACCOUNT_LIST_ADDRESS: Address =
        Address::from_str("bab69eae9ea958e501ea40b8c6dc27a9614a5b9b").unwrap();
}

/* Signal and Slots end */
//////////////////////////////////////////////////////////////////////
