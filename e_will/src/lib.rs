//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

// params for view methods
#[derive(Serialize, SchemaType, Clone)]
pub struct WillIdParams {
    will_id: AccountAddress,
}

// params for notarization
#[derive(Serialize, SchemaType, Clone)]
pub struct NotaryParams {
    will_hash:HashSha2256,
    will_id: AccountAddress,
    witness: AccountAddress,
}

// params for minting 
#[derive(Serialize, SchemaType, Clone)]
pub struct WillParam {
    will_file:String,
    will_hash:HashSha2256,
    notary: AccountAddress,
}


// Contract Will Structs
#[derive(Serialize, SchemaType, Clone)]
pub struct Will {
    will_file:String,
    will_hash:HashSha2256,
    notary: AccountAddress,
    is_notarized: bool,
    e_seal: FileState,
}

// Will file state for esealing
#[derive(Serial, Deserial, Clone, Copy, SchemaType)]
struct FileState {
    // The timestamp when this file hash was registered.
    timestamp: Option<Timestamp>,
    // The witness (sender_account) that witness file notary.
    witness:   Option<AccountAddress>,
}

// Note file hash is contained within will struct
impl FileState {
    fn new() -> Self {
        Self {
            timestamp: None,
            witness: None,
        }
    }
}

// contract state.
#[derive(Serial, DeserialWithState, StateClone)]
#[concordium(state_parameter = "S")]
struct State<S> {
    wills: StateMap<AccountAddress,Will,S>,
}


/********************** Contract Errors **********************/

// contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum ContractError {
    // Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
    // Failed logging: Log is full.
    LogFull,
    // Failed logging: Log is malformed.
    LogMalformed,
    // Only accounts can register a file hash.
    OnlyAccount,
    // Minting errors
    NotaryCantBeTestator,
    // Will errors
    WillAlreadyOwnedError,
    NoWillToBurn,
    NoWill,

    // Notary errors
    WillAlreadyNotarized,
    IncorrectNotary,
}

// Mapping the logging errors to ContractError.
impl From<LogError> for ContractError {
    fn from(le: LogError) -> Self {
        match le {
            LogError::Full => Self::LogFull,
            LogError::Malformed => Self::LogMalformed,
        }
    }
}

/********************** State Functions  **********************/

impl<S: HasStateApi> State<S> {
    // create a new state with no files registered.
    fn new(state_builder: &mut StateBuilder<S>) -> Self {
        State {
            wills: state_builder.new_map(),
        }
    }
    
    // Check if a file exists.
    fn will_exists(&self, will_id: &AccountAddress) -> bool {
        let will = self.wills.get(will_id);
        will.is_some()
    }

    // Get Will recorded from wills by will_id
    fn get_will(&self, will_id: AccountAddress) -> Option<Will> {
        self.wills.get(&will_id).map(|v| v.clone())
    }

    // Add a new file hash (replaces existing file if present).
    fn update(&mut self,will_id: AccountAddress, will:Will) {
        
        self.wills
            .entry(will_id)
            .and_modify(|old_will| *old_will = will);
         
    }
}

/*********************** Contract Events  ***********************/

// Tagged events to be serialized for the event log.
#[derive(Debug, Serial, SchemaType)]
enum Event {
    Notarization(NotarizationEvent),
    Mint(MintEvent),
    Burn(BurnEvent),
}

// The NotarizationEvent is logged when a new file hash is registered.
#[derive(Debug, Serialize, SchemaType)]
pub struct NotarizationEvent {
    will_id : Address,
    notary:    AccountAddress,
    // Hash of the file to be registered by the witness (sender_account).
    file_hash: HashSha2256,
    // Witness (sender_account) that registered the above file hash.
    witness:   AccountAddress,
    // Timestamp when this file hash was registered in the smart contract.
    timestamp: Timestamp,
}

#[derive(Debug, Serialize, SchemaType)]
pub struct MintEvent {
    will_id : Address,
    // Notary address of minted will
    notary:  AccountAddress,
}

#[derive(Debug, Serialize, SchemaType)]
pub struct BurnEvent {
    // Address of Will that was burned
    will_id : AccountAddress,
}



/********************** Contract Functions  **********************/

// Init creates a new smart contract instance.
#[init(contract = "ewills146")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
    // Build new contract state
    Ok(State::new(state_builder))
}

// Mints a will struct owned by senders address
#[receive(
    contract = "ewills146",
    name = "mint",
    parameter = "WillParam",
    error = "ContractError",
    mutable,
    enable_logger
)]
fn mint<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> Result<(), ContractError> {
    // bail if sender is contract address
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(ContractError::OnlyAccount.into()),
        Address::Account(account_address) => account_address,
    };

    // ensure that sender dosn't already own a `will` 
    ensure!(!host.state().will_exists(&sender_account) , ContractError::WillAlreadyOwnedError);

    // parse method parameters
    let param: WillParam = ctx.parameter_cursor().get()?;

    // ensure `param.notary` address is not equal to `sender_account` address
    ensure_ne!(param.notary, sender_account, ContractError::NotaryCantBeTestator);

    // create new `will` struct
    let will =  Will{
        will_file:param.will_file,
        will_hash:param.will_hash,
        notary: param.notary,
        is_notarized: false,
        e_seal:FileState::new(),
    };

    // log new mint event
    logger.log(&Event::Mint(MintEvent {
        will_id:Address::Account(sender_account),
        notary:param.notary,
    }))?;

    // insert `will` struct into sender address key within contract state `wills` StateMap
    host.state_mut()
        .wills
        .entry(sender_account) 
        .or_insert(will);

    Ok(())
}

// Burns `will` owned by sender address
#[receive(
    contract = "ewills146",
    name = "burn",
    error = "ContractError",
    mutable,
    enable_logger
)]
fn burn<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> Result<(), ContractError> {
    // bail if sender is contract address
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(ContractError::OnlyAccount.into()),
        Address::Account(account_address) => account_address,
    };

    // ensure that address owns a `will`
    ensure!(host.state().will_exists(&sender_account), ContractError::NoWillToBurn);

    // delet `will` struct from senders key 
    host.state_mut()
        .wills
        .remove(&sender_account);
        
    // log burn event
    logger.log(&Event::Burn(BurnEvent {
        will_id:sender_account,
    }))?;

    Ok(())
}

// Allows a will to be notarized 
#[receive(
    contract = "ewills146",
    name = "notarize",
    parameter = "NotaryParams",
    error = "ContractError",
    mutable,
    enable_logger
)]
fn notarize<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> Result<(), ContractError> {

    // parse notary parameters
    let param: NotaryParams = ctx.parameter_cursor().get()?;

    // get `will` struct if exist
    let will = match host.state_mut().wills.get(&param.will_id) {
        Some(will_object) => will_object,
        None => return Err(ContractError::NoWill),
    };

    // clone `will` struct
    let mut c_will = will.clone();

    // get minted file hash 
    let file_hash:HashSha2256 =  will.will_hash;

    // match sender to address account / only accounts can notarize a will.
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(ContractError::OnlyAccount),
        Address::Account(account_address) => account_address,
    };

    // check if `will` has been already notarized
    ensure!(!will.is_notarized,ContractError::WillAlreadyNotarized);

    // only callable by notry within the `will` struct
    ensure_eq!(will.notary, sender_account, ContractError::IncorrectNotary);

    // get block time stamp
    let timestamp = ctx.metadata().slot_time();
    
    // set `is_notarized` to true
    c_will.is_notarized = true;
    // set timestamp and witness parameter
    c_will.e_seal =  FileState {
        timestamp : Some(timestamp),
        witness : Some(param.witness),
    };
    
    // update `will` struct 
    host.state_mut().update(param.will_id,c_will);

    // log notarization event
    logger.log(&Event::Notarization(NotarizationEvent {
        will_id:Address::Account(param.will_id),
        notary:sender_account,
        file_hash:file_hash,
        witness: sender_account,
        timestamp:timestamp,
    }))?;
    
    Ok(())
}

/********************** Contract Views  **********************/

#[receive(
    contract = "ewills146",
    name = "willExists",
    parameter = "AccountAddress",
    return_value = "bool"
)]
fn will_exists<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<bool> {

    // get input parameters
    let param: AccountAddress = ctx.parameter_cursor().get()?;

    // output bool true/false if `will` exist
    Ok(host.state().will_exists(&param))
}

#[receive(
    contract = "ewills146",
    name = "willExistsFromSender",
    return_value = "bool"
)]
fn will_exist_sender<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<bool> {
    // match sender to address account (bail on Contract addresses)
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(ContractError::OnlyAccount.into()),
        Address::Account(account_address) => account_address,
    };

    // use `sender_account` as key
    // output bool true/false if `will_exist` from state
    Ok(host.state().will_exists(&sender_account))
}


#[receive(
    contract = "ewills146",
    name = "getWill",
    parameter = "AccountAddress",
    return_value = "Option<Will>"
)]
fn get_will<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Option<Will>> {
    // get AccountAddress input parameter
    let param: AccountAddress = ctx.parameter_cursor().get()?;
   
    // return optional `will`  mapped to input Account Address
    Ok(host.state().get_will(param))
}

#[receive(
    contract = "ewills146",
    name = "getWillFromSender",
    return_value = "Option<Will>"
)]
fn get_will_sender<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Option<Will>> {
    // match sender to address account (bail on Contract addresses)
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(ContractError::OnlyAccount.into()),
        Address::Account(account_address) => account_address,
    };
    // return optional `will` mapped to sender
    Ok(host.state().get_will(sender_account))
}

#[receive(
    contract = "ewills146",
    name = "isNotarized",
    parameter = "AccountAddress",
    return_value = "bool",
    error = "ContractError",
)]
fn is_notarized<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<bool> {
    // get AccountAddress input parameter
    let param: AccountAddress = ctx.parameter_cursor().get()?;
    // match input parameter to `will` within 
    let will = match host.state().wills.get(&param) {
        Some(will_object) => will_object,
        None => return Err(ContractError::NoWill.into()),
    };
    // return bool true/false if will has be notarized
    Ok(will.is_notarized)
}




/********************** Contract Testing  **********************/

#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    const TESTATOR: AccountAddress = AccountAddress([0u8; 32]);
    const NOTARY: AccountAddress = AccountAddress([2u8; 32]);
    const WITNESS: AccountAddress = AccountAddress([4u8; 32]);
    const WILL_HASH :HashSha2256 = HashSha2256([5u8; 32]);


    #[concordium_test]
    // Test that initializing the contract succeeds with some state.
    fn test_init() {
        let ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        let state_result = init(&ctx, &mut state_builder);
        state_result.expect_report("Contract initialization results in error");
    }

    #[concordium_test]
    // Test minting of will.
    fn testing_mint() {
        // set up test case
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(Address::Account(TESTATOR));
        
        // construct input parameters
        let will_url:String = "file_url".to_string();
        let mut logger = TestLogger::init();
        let param_object = WillParam {
            will_file: will_url.clone(),
            will_hash: WILL_HASH,
            notary: NOTARY,
        };
        let params = to_bytes(&param_object);
        // set raw parameters to test case
        ctx.set_parameter(&params);

        // construct state & host
        let mut state_builder = TestStateBuilder::new();
        let state = State::new(&mut state_builder);
        let mut host = TestHost::new(state,state_builder);

        // test contract method 
        let result = mint(&ctx,&mut host,&mut logger);
        claim!(result.is_ok(),"Mint failed.");

        // get & check will struct from contract
        let params = to_bytes(&TESTATOR); 
        ctx.set_parameter(&params);
        let get_will = get_will(&ctx,&mut host);
        claim!(get_will.is_ok(),"Could not get will");
        let will = get_will.unwrap();
        claim!(will.is_some(), "will contain no components");

        // check set variables within will struct 
        claim_eq!(will.clone().unwrap().will_file, will_url, "Will url should match");
        claim_eq!(will.clone().unwrap().will_hash,WILL_HASH, "Will hash should match");
        claim_eq!(will.clone().unwrap().notary,NOTARY, "Will notary should match");
        claim_eq!(will.clone().unwrap().is_notarized,false, "Will should not be notarized");

    }


    #[concordium_test]
    // Test burning of a will.
    fn testing_burning() {
        // set up test case
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(Address::Account(TESTATOR));
  
        // Mint a will
        let param_object = WillParam {
            will_file: "file_url".to_string(),
            will_hash: WILL_HASH,
            notary: NOTARY,
        };
        let mut logger = TestLogger::init();
        let params = to_bytes(&param_object);
        ctx.set_parameter(&params);

        let mut state_builder = TestStateBuilder::new();
        let state = State::new(&mut state_builder);
        let mut host = TestHost::new(state,state_builder);

        // mint new will struct
        let _ = mint(&ctx,&mut host,&mut logger);

        let params = to_bytes(&TESTATOR);
        ctx.set_parameter(&params);

        // Check if will exist after minting
        let does_exist = will_exists(&ctx,&mut host);
        claim_eq!(does_exist,Ok(true),"Will should exist before burning");

        // burn will 
        let result = burn(&ctx,&mut host,&mut logger);
        claim!(result.is_ok(),"Burning failed.");

        // Check will existance after burning
        let doesnt_exist = will_exists(&ctx,&mut host);
        claim_eq!(doesnt_exist,Ok(false),"Will should not exist after burning");

    }

    #[concordium_test]
    fn testing_notarization() {
        // set up test case
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(Address::Account(TESTATOR));


        // minting parameters
        let will_url:String = "file_url".to_string();

        let will_params = WillParam {
            will_file: will_url.clone(),
            will_hash: WILL_HASH,
            notary: NOTARY,
        };

        let w_params = to_bytes(&will_params);
        ctx.set_parameter(&w_params);

        // create state
        let mut logger = TestLogger::init();
        let mut state_builder = TestStateBuilder::new();
        let state = State::new(&mut state_builder);
        let mut host = TestHost::new(state,state_builder);

        // mint will
        let _ = mint(&ctx,&mut host,&mut logger);


        // notary parameters 
        const TIME: u64 = 1;
        ctx.set_metadata_slot_time(Timestamp::from_timestamp_millis(TIME));

        let notary_params = NotaryParams {
            will_hash: WILL_HASH,
            will_id: TESTATOR,
            witness: WITNESS,
        };
        // set sender as notary
        ctx.set_sender(Address::Account(NOTARY));

        let n_params = to_bytes(&notary_params);
        ctx.set_parameter(&n_params);

        // notarize will 
        let result = notarize(&ctx,&mut host,&mut logger);
        claim!(result.is_ok(),"Notarization failed.");
        // check results
        ctx.set_sender(Address::Account(TESTATOR));
        
        let params = to_bytes(&TESTATOR);
        ctx.set_parameter(&params);
        // get notarized `will`
        let get_will = get_will(&ctx,&mut host);
        claim!(get_will.is_ok(),"Could not get will");
        // Unwrap optinal
        let will = get_will.unwrap();
        claim!(will.is_some(), "will contain no components");

        // check `will` struct variables
        claim_eq!(will.clone().unwrap().will_file, will_url, "Will url should match");
        claim_eq!(will.clone().unwrap().will_hash,WILL_HASH, "Will hash should match");
        claim_eq!(will.clone().unwrap().notary,NOTARY, "Will notary should match");
        claim_eq!(will.clone().unwrap().is_notarized,true, "Will should be be notarized");
        
        // check e seal docs
        let e_seal = will.clone().unwrap().e_seal;
        claim_eq!(e_seal.clone().timestamp, Some(Timestamp::from_timestamp_millis(TIME)), "Eseal time is incorrect");
        claim_eq!(e_seal.clone().witness, Some(WITNESS), "Witness address is incorrect");
    }
}
