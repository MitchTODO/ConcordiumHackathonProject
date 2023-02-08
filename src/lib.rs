//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

/// Your smart contract state.
#[derive(Serialize, SchemaType, Clone)]
pub struct State {
    is_active: bool,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum ContractError {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
    AlreadyActive,
    OnlyAccount,

}

/// Init function that creates a new smart contract.
#[init(contract = "activation_contract")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext, // Who init , time 
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {

    // Init contract state
    Ok(State {is_active: true})
}

/// Receive function. The input parameter is the boolean variable `throw_error`.
///  If `throw_error == true`, the receive function will throw a custom error.
///  If `throw_error == false`, the receive function executes successfully.
#[receive(
    contract = "activation_contract",
    name = "activate",
    parameter = "bool",
    error = "ContractError",
    mutable
)]
fn activate<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), ContractError> {
    // Your code
    //let params :InitParameter = ctx.parameter_cursor().get()?;
    // ensure that the sender is an account
    let acc = match ctx.sender() {
        Address::Account(acc) => acc,
        Address::Contract(_) => return Err(ContractError::OnlyAccount),
    };

    let activation: bool = ctx.parameter_cursor().get()?;

    // ensure that state is non active
    if host.state().is_active == activation  {
        return Err(ContractError :: AlreadyActive);
    }
    host.state_mut().is_active = activation;
    Ok(())
}

/// View function that returns the content of the state.
#[receive(
    contract = "activation_contract", 
    name = "view", 
    return_value = "State")
]
fn view<'a, 'b, S: HasStateApi>(
    _ctx: &'a impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<bool> {
    Ok(host.state().is_active)
}

#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    const ACC:Address = AccountAddress([0u8;32]);

    #[test]
    fn activation_works() {
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(Address::Account(ACC));
        ctx.set_metadata_slot_time(Timestamp::from_timestamp_millis(0));
        let set_activation = true;
        let parameter = to_bytes(&set_activation);
        ctx.set_parameter(&parameter);
        let state = State{is_active : false};

        let mut host = TestHost::new(state,TestStateBuilder::new());
        let result = activate(&ctx,&mut host);

        assert!(result.is_ok());
        assert_eq!(host.state().is_active, true);
    }
}
