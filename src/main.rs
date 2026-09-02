mod balances;
mod support;
mod system;
mod proof_of_existence;

// These are the cincrete types we will use in our simple state machine.
// Modules are configurated for these types directly,
// and they satisfy all of our trait requirements.
mod types {
    use crate::support;

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
}

use crate::support::Dispatch;

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl Runtime {
    // Create a new instance of the main Runtime,
    // by creating a new instance of each pallet.
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
    // Execute a block of extrinsics. Increments the block number.
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();

        if self.system.block_number() != block.header.block_number {
            return Err("Block number mismatch");
        }

        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);

            let _res = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            });
        }

        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    // Dispatch a call on behalf of a caller.
    // Increments the caller's nonce.
    // Dispatch allows us to identify which underlying module
    // call we want to execute.
    // Note that we extract the `caller` from the extrinsic,
    // and use that information to determine
    // who are executing the call on behalf of.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}
fn main() {
    // Create a new instance of the Runtime.
    // It will instantiate with it all the modules it uses.
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    // Initilize the system with some initial balance.
    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer { to: bob.clone(), amount: 30 })
            },
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie, amount: 20 })
            },
        ],
    };

    runtime
        .execute_block(block_1)
        .expect("wrong block execution");

    println!("{:#?}", runtime);
}
