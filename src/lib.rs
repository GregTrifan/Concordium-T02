//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

/// Your smart contract state.
#[derive(Serialize, SchemaType, Clone)]
pub struct State {
    // Your state
    count:i64,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
}

/// Init function that creates a new smart contract.
#[init(contract = "t02_counter")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    Ok(State {
        count:0
    })
}

/// View function that returns the content of the state.
#[receive(contract = "t02_counter", name = "view", return_value = "State")]
fn view<'a, 'b, S: HasStateApi>(
    _ctx: &'a impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<&'b State> {
    Ok(host.state())
}

// Increment function that increments the counter from the state
#[receive(
    contract = "t02_counter",
    name = "increment",
    mutable
)]
fn increment<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    _host.state_mut().count += 1;
    Ok(())
}

// Decrement function that decrements the counter from the state
#[receive(
    contract = "t02_counter",
    name = "decrement",
    mutable
)]
fn decrement<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    _host.state_mut().count -= 1;
    Ok(())
}


#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    type ContractResult<A> = Result<A, Error>;

    #[concordium_test]
    /// Test that initializing the contract succeeds with some state.
    fn test_init() {
        let ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        let state_result = init(&ctx, &mut state_builder);
        state_result.expect_report("Contract initialization results in error");
    }

    
    #[concordium_test]
    fn test_increment() {
        let ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        // Initializing state
        let initial_state = init(&ctx, &mut state_builder).expect("Initialization should pass");

        let ctx = TestReceiveContext::empty();

        let mut host = TestHost::new(initial_state, state_builder);


        let result: ContractResult<()> = increment(&ctx, &mut host);
        claim!(result.is_ok(), "Results in rejection");
        
        claim_eq!(host.state().count,1,"Didn't increment count");
    }

    #[concordium_test]
    fn test_decrement() {
        let ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        // Initializing state
        let initial_state = init(&ctx, &mut state_builder).expect("Initialization should pass");

        let ctx = TestReceiveContext::empty();

        let mut host = TestHost::new(initial_state, state_builder);


        let result: ContractResult<()> = decrement(&ctx, &mut host);
        claim!(result.is_ok(), "Results in rejection");
        claim_eq!(host.state().count,-1,"Didn't decrement count");
    }
}