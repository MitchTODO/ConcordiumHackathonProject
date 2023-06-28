//! # A Concordium V1 smart contract
//#![cfg_attr(not(feature = "std"), no_std)]
use concordium_std::*;
use core::fmt::Debug;

//const TOKEN_METADATA_BASE_URL: &str = "IPFS"; Think about how base url can be implemented
//  concordium-client --grpc-ip node.testnet.concordium.com --grpc-port 20000  module deploy a.wasm.v1 --name ewillv2 --sender account1
// Module successfully deployed with reference: '1ec09182ce7b7a0c8cd0ac46904174162930bb380b2a231fe1e7447a8fc444d0'.
// Module reference 1ec09182ce7b7a0c8cd0ac46904174162930bb380b2a231fe1e7447a8fc444d0 was successfully named 'ewillv2'.

// c872cae61c6cad07cf03b734363d5ea30618e4820fd625ea668bae1d2296dafb
// Contract successfully initialized with address: {"index":5142,"subindex":0}

// params for view methods
#[derive(Serialize, SchemaType, Clone)]
pub struct WillIdParams {
    will_id: u8,
    owner: AccountAddress,
}

// params for notarization
#[derive(Serialize, SchemaType, Clone)]
pub struct NotaryParams {
    will_hash:HashSha2256,
    will_id: u8,
    testator: AccountAddress,
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

// Will state.
#[derive(Serial, DeserialWithState, Deletable, StateClone)]
#[concordium(state_parameter = "S")]
struct WillState<S> {
    active_will: Option<Will>,      // active will is the lastest will to be notarized
    will_count: u8,                // amount of will user has created
    wills: StateMap<u8,Will,S>,    // mapping of wills to u8 NOTE could us will hash 
    operators: StateSet<Address,S>, // operators allowed to modifiy wills?
}

impl<S:HasStateApi> WillState<S> {

    fn empty(state_builder: &mut StateBuilder<S>) -> Self {
        WillState {
            active_will:    None,                       // No active wills
            will_count:     0,                          // init with zero wills
            wills:          state_builder.new_map(),    // past wills used
            operators:      state_builder.new_set(),
        }
    }
}

// contract state.
#[derive(Serial, DeserialWithState, StateClone)]
#[concordium(state_parameter = "S")]
struct State<S> {
    // Map address to mutple wills
    state: StateMap<AccountAddress,WillState<S>,S>,
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
    //WillAlreadyOwnedError,
    //NoWillToBurn,
    IncorrectHash,
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
            state: state_builder.new_map(),
        }
    }

    // add 
    fn add(
        &mut self,
        owner: &AccountAddress,
        will: Will, 
        state_builder: &mut StateBuilder<S>) -> Result<u8, ContractError>
        {
            let mut owner_state = self.state.entry(*owner).or_insert_with(|| WillState::empty(state_builder));
            let wc = owner_state.will_count;

            owner_state.wills
            .entry(wc)
            .or_insert(will);

            owner_state.will_count += 1;
            Ok(owner_state.will_count)
        }
    

    // updates owner active will via notary
    fn update_active_will(
        &mut self,
        owner:&AccountAddress,
        index:&u8,
        will:Will,
        ) 
        {
            self.state.entry(*owner).and_modify(|will_state| {
                will_state.active_will = Some(will.clone());
                will_state.wills.entry(*index).and_modify(|old_will| {
                    *old_will = will.clone();
                });

            });
        }

    /* TODO allow for notary change
    Only allow on wills that have not been notarized or is not active
    fn change_notary(
        &mut self,
        owner:&AccountAddress,
        index:&u8,
        notary:&AccountAddress,
    ) -> bool {
        self.state.entry(*owner).and_modify(|will_state| {
            will_state.wills.entry(*index).and_modify(|old_will| {
                *old_will = will.clone();
            });

        });
    }
    */

    // will exist 
    fn will_exists(
        &self,
        owner:&AccountAddress,
        index:&u8
    ) -> bool {
        self.state
        .get(owner)
        .map(|will_state| will_state.wills.get(index).is_some())
        .unwrap_or(false)
    }

    // will count of owner
    fn will_count(
        &self,
        owner:&AccountAddress,
    ) -> u8
        {
            self
            .state
            .get(owner)
            .map(|will_state| will_state.will_count.clone())
            .unwrap_or(0)
        }
    
    // return get will at index of will address of array of owner
    fn get_will(
        &self,
        owner:&AccountAddress,
        index:&u8,
    ) -> Result<Option<Will>,()>
    {
        let will = self
        .state
        .get(&owner)
        .map(|will_state| will_state.wills.get(&index).map(|v| v.clone()))
        .unwrap();

        Ok(will)
    }
    
    // return active will of address
    fn get_active_will(
        &self,
        owner:&AccountAddress,
    ) -> Option<Will>
    {
        self
        .state
        .get(&owner)
        .map(|will_state| will_state.active_will.clone())
        .unwrap_or(None)
    }
}

/*********************** Contract Events  ***********************/

// Tagged events to be serialized for the event log.
#[derive(Debug, Serial, SchemaType)]
enum Event {
    Notarization(NotarizationEvent),
    Mint(MintEvent),
}

// The NotarizationEvent is logged when a new file hash is registered.
#[derive(Debug, Serialize, SchemaType)]
pub struct NotarizationEvent {
    will_id:   u8,
    // Testator (will owner)
    testator: AccountAddress,
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
#[init(contract = "ewills156")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
    // Build new contract state
    Ok(State::new(state_builder))
}


// Mints a will struct owned by senders address
#[receive(
    contract = "ewills156",
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
) -> Result<u8, ContractError> {

    // bail if sender is contract address
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(ContractError::OnlyAccount.into()),
        Address::Account(account_address) => account_address,
    };

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

    let (state, builder) = host.state_and_builder();
    
    // add `will` struct into sender address key within contract state `wills` array
    let will_count = state.add(&sender_account,will,builder)?;

    // log new mint event
    logger.log(&Event::Mint(MintEvent {
        will_id:Address::Account(sender_account),
        notary:param.notary,
    }))?;

    Ok(will_count)
}


// Allows a will to be notarized 
#[receive(
    contract = "ewills156",
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
    let params: NotaryParams = ctx.parameter_cursor().get()?;

    ensure!(host.state().will_exists(&params.testator,&params.will_id),ContractError::NoWill);

    // get `will` struct if exist
    let will = match host.state().get_will(&params.testator,&params.will_id).unwrap(){
        Some(will_object) => will_object,
        None => return Err(ContractError::NoWill.into()),
    };

    // clone `will` struct
    let mut c_will = will.clone();

    // get minted file hash 
    let file_hash:HashSha2256 = will.will_hash;
    // ensure notry will file hashes 
    ensure_eq!(params.will_hash,file_hash,ContractError::IncorrectHash);

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
        witness : Some(params.witness),
    };
    
    // update `will` struct 
    host.state_mut().update_active_will(&params.testator, &params.will_id, c_will);

    // log notarization event
    logger.log(&Event::Notarization(NotarizationEvent {
        will_id:params.will_id,
        testator:params.testator,
        notary:sender_account,
        file_hash:file_hash,
        witness: sender_account,
        timestamp:timestamp,
    }))?;
    
    Ok(())
}


/********************** Contract Views **********************/

#[receive(
    contract = "ewills156",
    name = "active_will",
    parameter = "WillIdParams",
    return_value = "Option<Will>"
)]
fn active_will<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Option<Will>> {

    // get input parameters
    let param: WillIdParams = ctx.parameter_cursor().get()?;

    // output param
    let active_result = host.state().get_active_will(&param.owner);
    Ok(active_result)
}


#[receive(
    contract = "ewills156",
    name = "get_will",
    parameter = "WillIdParams",
    return_value = "Option<Will>"
)]
fn get_will<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Option<Will>> {

    // get input parameters
    let param: WillIdParams = ctx.parameter_cursor().get()?;

    // output param
    let active_result = host.state().get_will(&param.owner,&param.will_id).unwrap();
    Ok(active_result)
}


#[receive(
    contract = "ewills156",
    name = "will_count",
    parameter = "WillIdParams",
    return_value = "u8"
)]
fn will_count<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<u8> {
    let param: WillIdParams = ctx.parameter_cursor().get()?;
    Ok(host.state().will_count(&param.owner)) 
}


#[receive(
    contract = "ewills156",
    name = "will_exists",
    parameter = "WillIdParams",
    return_value = "bool"
)]
fn will_exists<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<bool> {

    // get input parameters
    let param: WillIdParams = ctx.parameter_cursor().get()?;

    // output bool true/false if `will` exist
    Ok(host.state().will_exists(&param.owner,&param.will_id))
}


#[receive(
    contract = "ewills156",
    name = "is_notarized",
    parameter = "WillIdParams",
    return_value = "bool",
    error = "ContractError",
)]
fn is_notarized<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<bool> {

    // get AccountAddress input parameter
    let params: WillIdParams = ctx.parameter_cursor().get()?;

    // match input parameter to `will` within 
    ensure!(host.state().will_exists(&params.owner,&params.will_id),ContractError::NoWill.into());
    let will = match host.state().get_will(&params.owner,&params.will_id).unwrap(){
        Some(will_object) => will_object,
        None => return Err(ContractError::NoWill.into()),
    };

    // return bool true/false if will has be notarized
    Ok(will.is_notarized)
}

#[receive(
    contract = "ewills156",
    name = "is_contract",
    return_value = "bool",
)]
fn is_contract<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<bool> {
    Ok(true)
}


/********************** Contract Testing  **********************/

#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    const TESTATOR: AccountAddress = AccountAddress([0u8; 32]);
    const NOTARY: AccountAddress = AccountAddress([2u8; 32]);
    const WITNESS: AccountAddress = AccountAddress([4u8; 32]);
    const WILL_1_HASH :HashSha2256 = HashSha2256([5u8; 32]);
    const WILL_2_HASH :HashSha2256 = HashSha2256([6u8; 32]);

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
            will_hash: WILL_1_HASH,
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
        let get_will_params_object = WillIdParams {
            will_id: 0,
            owner: TESTATOR,
        };

        let params = to_bytes(&get_will_params_object); 
        ctx.set_parameter(&params);

        let will_count = will_count(&ctx,&mut host);
        
        // check will count 
        claim_eq!(will_count.unwrap(),1);

        // set params 
        //let get_will_params = to_bytes(&get_will_params_object);
        //ctx.set_parameter(&get_will_params);

        // Get Will object
        let get_will = get_will(&ctx,&mut host);
         claim!(get_will.is_ok(),"Could not get will");
        let will = get_will.unwrap();
        claim!(will.is_some(), "will contain no components");

        // check set variables within will struct 
        claim_eq!(will.clone().unwrap().will_file, will_url, "Will url should match");
        claim_eq!(will.clone().unwrap().will_hash, WILL_1_HASH, "Will hash should match");
        claim_eq!(will.clone().unwrap().notary, NOTARY, "Will notary should match");
        claim_eq!(will.clone().unwrap().is_notarized, false, "Will should not be notarized");

        // get active will params 
        //let active_will_params = to_bytes(&TESTATOR);
        //ctx.set_parameter(&active_will_params);

        // check active will for testator should be none
        let active_will = active_will(&ctx,&mut host).unwrap();
        claim!(active_will.is_none(), "Active Will should be empty");
    }
    
    
    #[concordium_test]
    // Test burning of a will.
    fn testing_multiple_will_mint() {
        // set up test case
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(Address::Account(TESTATOR));

        let mut state_builder = TestStateBuilder::new();
        let state = State::new(&mut state_builder);

        let mut host = TestHost::new(state,state_builder);
  
        // Mint a first will
        let will_url_1:String = "will 1".to_string();
        let param_object = WillParam {
            will_file: will_url_1.clone(),
            will_hash: WILL_1_HASH,
            notary: NOTARY,
        };

        let mut logger = TestLogger::init();
        let params = to_bytes(&param_object);
        ctx.set_parameter(&params);

        let _ = mint(&ctx,&mut host,&mut logger);

        // Mint a second will
        let will_url_2:String = "will 2".to_string();
        let param_object_will_2 = WillParam {
            will_file: will_url_2.clone(),
            will_hash: WILL_2_HASH,
            notary: NOTARY,
        };

        let mut logger = TestLogger::init();
        let params_2 = to_bytes(&param_object_will_2);
        ctx.set_parameter(&params_2);

        let _ = mint(&ctx,&mut host,&mut logger);

        let count_params = to_bytes(&TESTATOR);
        ctx.set_parameter(&count_params);

        // Check will 1
        // get will parameters
        let get_will_params_object = WillIdParams {
            will_id: 0,
            owner: TESTATOR,
        };
        
        // set params 
        let get_will_params = to_bytes(&get_will_params_object);
        ctx.set_parameter(&get_will_params);

        // Check will amount 
        let will_count = will_count(&ctx,&mut host);
        claim_eq!(will_count.unwrap(),2,"Incorrect Will count");

        // get will one 
        let get_will_one = get_will(&ctx,&mut host);
        // check if none
        claim!(get_will_one.is_ok(),"Could not get will");
        let will_one = get_will_one.unwrap();
        claim!(will_one.is_some(), "will contain no components");

        // check set variables within will struct 
        claim_eq!(will_one.clone().unwrap().will_file, will_url_1, "Will url should match");
        claim_eq!(will_one.clone().unwrap().will_hash, WILL_1_HASH, "Will hash should match");
        claim_eq!(will_one.clone().unwrap().notary, NOTARY, "Will notary should match");
        claim_eq!(will_one.clone().unwrap().is_notarized, false, "Will should not be notarized");
        
        // Check will 2
        // get will parameters
        let get_will_params_object_2 = WillIdParams {
            will_id: 1,
            owner: TESTATOR,
        };

        let get_will_params_2 = to_bytes(&get_will_params_object_2);
        ctx.set_parameter(&get_will_params_2);
       
        let get_will_two = get_will(&ctx,&mut host);
        
        claim!(get_will_two.is_ok(),"Could not get will");
        let will_two = get_will_two.unwrap();
        claim!(will_two.is_some(), "will contain no components");

        // check set variables within will struct 
        claim_eq!(will_two.clone().unwrap().will_file, will_url_2, "Will 2 url should match");
        claim_eq!(will_two.clone().unwrap().will_hash, WILL_2_HASH, "Will 2 hash should match");
        claim_eq!(will_two.clone().unwrap().notary, NOTARY, "Will 2 notary should match");
        claim_eq!(will_two.clone().unwrap().is_notarized, false, "Will 2 should not be notarized");
        
    }

    #[concordium_test]
    fn testing_notarization() {
        // set up test case
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(Address::Account(TESTATOR));

        // minting parameters
        let will_url:String = "will 2".to_string();

        let will_params = WillParam {
            will_file: will_url.clone(),
            will_hash: WILL_1_HASH,
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
            will_hash: WILL_1_HASH,
            will_id: 0,
            testator:TESTATOR,
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
        
        let get_will_params_object_2 = WillIdParams {
            will_id: 0,
            owner: TESTATOR,
        };

        let params = to_bytes(&get_will_params_object_2);
        ctx.set_parameter(&params);
        // get notarized `will`
        let get_will = get_will(&ctx,&mut host);
        claim!(get_will.is_ok(),"Could not get will");

        // Unwrap optinal
        let will = get_will.unwrap();
        claim!(will.is_some(), "will contain no components");

        // check `will` struct variables
        claim_eq!(will.clone().unwrap().will_file, will_url, "Will url should match");
        claim_eq!(will.clone().unwrap().will_hash,WILL_1_HASH, "Will hash should match");
        claim_eq!(will.clone().unwrap().notary,NOTARY, "Will notary should match");
        claim_eq!(will.clone().unwrap().is_notarized,true, "Will should be be notarized");
        
        // check e seal docs
        let e_seal = will.clone().unwrap().e_seal;
        claim_eq!(e_seal.clone().timestamp, Some(Timestamp::from_timestamp_millis(TIME)), "Eseal time is incorrect");
        claim_eq!(e_seal.clone().witness, Some(WITNESS), "Witness address is incorrect");

        // check testators active will 
        let active_will = active_will(&ctx,&mut host);
        claim!(active_will.is_ok(),"Could not get active will");

        let a_will = active_will.unwrap();
        claim!(a_will.is_some(), "Active will is none");

        claim_eq!(a_will.clone().unwrap().will_file, will_url, "Will url should match");
        claim_eq!(a_will.clone().unwrap().will_hash,WILL_1_HASH, "Will hash should match");
        claim_eq!(a_will.clone().unwrap().notary,NOTARY, "Will notary should match");
        claim_eq!(a_will.clone().unwrap().is_notarized,true, "Will should be be notarized");
    }

    #[concordium_test]
    fn testing_activation_change() {
        // set up test case
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(Address::Account(TESTATOR));

        // create state
        let mut logger = TestLogger::init();
        let mut state_builder = TestStateBuilder::new();
        let state = State::new(&mut state_builder);
        let mut host = TestHost::new(state,state_builder);

        // minting parameters
        let will_url_one:String = "will 1".to_string();
        let will_url_two:String = "will 2".to_string();

        let will_params_one = WillParam {
            will_file: will_url_one.clone(),
            will_hash: WILL_1_HASH,
            notary: NOTARY,
        };

        let will_params_two = WillParam {
            will_file: will_url_two.clone(),
            will_hash: WILL_2_HASH,
            notary: NOTARY,
        };

        // set parameters and mint first will
        let w_params_one = to_bytes(&will_params_one);
        ctx.set_parameter(&w_params_one);
        let _ = mint(&ctx,&mut host,&mut logger);


        // set parameters and mint second will 
        let w_params_two = to_bytes(&will_params_two);
        ctx.set_parameter(&w_params_two);
        let _ = mint(&ctx,&mut host,&mut logger);

    
        // notary parameters 
        const TIME: u64 = 1;
        ctx.set_metadata_slot_time(Timestamp::from_timestamp_millis(TIME));

        // notary parameters for first will
        let notary_params_one = NotaryParams {
            will_hash: WILL_1_HASH,
            will_id: 0,
            testator:TESTATOR,
            witness: WITNESS,
        };

        // notary parameters for second will
        let notary_params_two = NotaryParams {
            will_hash: WILL_2_HASH,
            will_id: 1, // set will id
            testator:TESTATOR,
            witness: WITNESS,
        };

        // set sender as notary
        ctx.set_sender(Address::Account(NOTARY));

        // set params notarize will
        let n_params = to_bytes(&notary_params_two);
        ctx.set_parameter(&n_params);
        let result = notarize(&ctx,&mut host,&mut logger);
        claim!(result.is_ok(),"Notarization failed.");

        // check testators active will 
  
        //let params = to_bytes(&TESTATOR);
        let check_active_params_object_1 = WillIdParams {
            will_id: 0,
            owner: TESTATOR,
        };
        let cap1 = to_bytes(&check_active_params_object_1);
        ctx.set_parameter(&cap1);

        let active_will_one = active_will(&ctx,&mut host);
        claim!(active_will_one.is_ok(),"Could not get active will");
        let a_will = active_will_one.unwrap();
        claim!(a_will.is_some(), "Active will is none");
        claim_eq!(a_will.clone().unwrap().will_file, will_url_two, "Will url should match");
        claim_eq!(a_will.clone().unwrap().will_hash,WILL_2_HASH, "Will hash should match");
        claim_eq!(a_will.clone().unwrap().notary,NOTARY, "Will notary should match");
        claim_eq!(a_will.clone().unwrap().is_notarized,true, "Will should be be notarized");

        
        // Notarize second will
        // set sender as notary
        ctx.set_sender(Address::Account(NOTARY));
        let n_params = to_bytes(&notary_params_one);
        ctx.set_parameter(&n_params);

        // notarize will
        let result = notarize(&ctx,&mut host,&mut logger);
        claim!(result.is_ok(),"Notarization failed.");

        // Check results of active will
        ctx.set_sender(Address::Account(TESTATOR));
        
        let check_active_params_object_2 = WillIdParams {
            will_id: 0,
            owner: TESTATOR,
        };

        let cap2 = to_bytes(&check_active_params_object_2);
        ctx.set_parameter(&cap2);
        // check testators active will 
        let active_will = active_will(&ctx,&mut host);
        claim!(active_will.is_ok(),"Could not get active will");

        let a_will = active_will.unwrap();

        claim!(a_will.is_some(), "Active will is none");
        
        claim_eq!(a_will.clone().unwrap().will_file, will_url_one, "Will url should match");
        claim_eq!(a_will.clone().unwrap().will_hash,WILL_1_HASH, "Will hash should match");
        claim_eq!(a_will.clone().unwrap().notary,NOTARY, "Will notary should match");
        claim_eq!(a_will.clone().unwrap().is_notarized,true, "Will should be be notarized");
        
    }
}
