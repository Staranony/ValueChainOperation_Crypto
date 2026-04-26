// This is for understanding Rust code and blockchain architecture
// it does not provide any answer to problems
// My memo on understanding logic of #[contract] and pub struct contracgt

#[contract]
pub struct  Contract;

// 1. Your original empty anchor remains untouched
pub struct Contract;

// 2. The plugin generates this massive block of behavior
impl Contract {
    pub fn execute_tranfer(env: BlockchainEnvironment, amount: u64) {
        // ... hundreds of lines of complex 1s and 0s to move money ...
        env.storage.save_balance(amount);
    }
}

impl Contract {
    // Look very closely at the parameters inside the parentheses here:
    pub fn execute_tranfer(env: BlockchainEnvironment, amount: u64) {
        env.storage.save_balance(amount);
    }
}
// ###################################################################
struct Wallet {
    balance: u64
}

impl Wallet {
    // Notice the &self parameters here!
    fn check_balance(&self) {
        // ... logic ...
    }
}

fn main() {
    // Step 1: We MUST allocate memory to create instance of the struct
    let my_wallet = Wallet{ balance: 100};
    
    // Step 2: We use that specific instance in memory to call the method
    my_wallet.check_balance();
}


pub struct Contract;

impl Constract {
    // Notice there is NO self here!
    pub fn execute_transfer(amount: u64) {
        // ... logic ...
    }
}

fn main() {
    // Step 1: Notice what we do NOT do. We do not write 'let my_contract = Contract;'
    
    // Step 2: We just use the struct's name as a pathway to the function!
    Contract::execute_transfer(50);
}


// ###########################################################

// Notice: There is NO #[contract] plugin here at all!

// "Folder A"
pub struct MyContract;
impl MyContract {
    pub fn execute_transfer() {
        // Moves money from my wallet
    }
}

// "Folder B"
pub struct TokenLibrary;
impl TokenLibrary {
    pub fn execute_transfer() {
        // Moves money inside the external library
    }
}

fn main () {
    // We tell the computer exactly which one we want:
    MyContract::execute_transfer();
    TokenLibrary::execute_tranfer();
}


// #####

#[contract] // Line 1: The Macro

pub struct Contract; // Line 2: The Struct
