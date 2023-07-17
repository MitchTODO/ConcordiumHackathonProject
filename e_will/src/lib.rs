//! # A Concordium V1 smart contract
#![cfg_attr(not(feature = "std"), no_std)]
use concordium_cis2::*;
use concordium_std::*;

//const TOKEN_METADATA_BASE_URL: &str = "IPFS"; Think about how base url can be implemented
//  concordium-client --grpc-ip node.testnet.concordium.com --grpc-port 20000  module deploy a.wasm.v1 --name ewillv2 --sender account1
// Module successfully deployed with reference: '1ec09182ce7b7a0c8cd0ac46904174162930bb380b2a231fe1e7447a8fc444d0'.
// Module reference 1ec09182ce7b7a0c8cd0ac46904174162930bb380b2a231fe1e7447a8fc444d0 was successfully named 'ewillv2'.

// c872cae61c6cad07cf03b734363d5ea30618e4820fd625ea668bae1d2296dafb
// Contract successfully initialized with address: {"index":5142,"subindex":0}


/// Contract Arch
/// NFTID -> Address:Will OBJECT -> WillState

/// Baseurl
const TOKEN_METADATA_BASE_URL:&str = "https://cloudflare-ipfs.com/ipfs/";

/// List of supported standards by this contract address.
const SUPPORTS_STANDARDS: [StandardIdentifier<'static>; 2] =
    [CIS0_STANDARD_IDENTIFIER, CIS2_STANDARD_IDENTIFIER];

/// Contract token ID type.
/// To save bytes we use a token ID type limited to a `u32`.
/// New: Amount of eWill NFT's that have been minted
type ContractTokenId = TokenIdU32;

/// Contract token amount.
/// Since the tokens are non-fungible the total supply of any token will be at
/// most 1 and it is fine to use a small type for representing token amounts.
type ContractTokenAmount = TokenAmountU8;

// params for view methods
#[derive(Serialize, SchemaType, Clone)]
pub struct WillParams {
    token_id: u32,
    owner: AccountAddress,
}

// Minting parameters 
#[derive(Serialize, SchemaType, Clone)]
pub struct MintParams {
    will_file:String,
    will_hash:HashSha2256,
    notary: AccountAddress,
}

// params for notarization
#[derive(Serialize, SchemaType, Clone)]
pub struct NotaryParams {
    will_hash:HashSha2256,
    token_id: u32,
    testator: AccountAddress,
    witness: Vec<AccountAddress>,
}


// will modifications
#[derive(Serialize, SchemaType, Clone)]
enum WillModType {
    Mint,
    Notarized,
    Revoke,
    Revived,
}



#[derive(Serialize, SchemaType, Clone)]
pub struct WillMod {
    mod_type: WillModType,
    timestamp: Timestamp
}

impl WillMod {
    fn new(mod_type:WillModType,time:Timestamp) -> Self {
        Self {
            mod_type: mod_type,
            timestamp: time,
        }
    }
}


// Will file state for esealing
#[derive(Serial, Deserial, Clone, SchemaType)]
struct FileState {
    // The timestamp when this file hash was registered.
    timestamp: Option<Timestamp>,
    // Witnesses (sender_account) that witness file notary.
    witness: Vec<AccountAddress>,
}

// Note file hash is contained within will struct
impl FileState {
    fn new() -> Self {
        Self {
            timestamp: None,
            witness: Vec::new(),
        }
    }
}

/// Contract Will Structs
#[derive(Serialize, SchemaType, Clone)]
pub struct Will {
    will_file:String,
    will_hash:HashSha2256,      // TODO check when notrizing if hashes match same will file shouldn't be notrarized twice
    notary:AccountAddress,
    notarized: bool,
    e_seal:FileState,
    mod_history:Vec<WillMod>,   // will modification history 
}


/// Will object state
/// Address are mapped to 
#[derive(Serial, DeserialWithState, Deletable, StateClone)]
#[concordium(state_parameter = "S")]
struct WillState<S> {
    active_will: Option<Will>,      // active will is the lastest will to be notarized
    will_index: u32,                 // amount of will user has created ie (owned_tokens)
    wills: StateMap<u32,Will,S>,     // mapping of wills to u8 NOTE could us will hash 
    operators: StateSet<Address,S>, // addresses allowed to modifiy wills? notaized is removed
}

impl<S:HasStateApi> WillState<S> {

    fn empty(state_builder: &mut StateBuilder<S>) -> Self {
        WillState {
            active_will:    None,                       // No active wills
            will_index:     0,                          // init with zero 
            wills:          state_builder.new_map(),    // past wills used
            operators:      state_builder.new_set(),
        }
    }
}

/********************** Contract State **********************/

// contract state.
#[derive(Serial, DeserialWithState, StateClone)]
#[concordium(state_parameter = "S")]
struct State<S> {
    // Map account address to will state
    state: StateMap<AccountAddress,WillState<S>, S>,

    /// Map with contract addresses providing implementations of additional
    /// standards.
    implementors: StateMap<StandardIdentifierOwned, Vec<ContractAddress>, S>,
}



/// Follows NFT standard
/// The parameter type for the contract function `setImplementors`.
/// Takes a standard identifier and list of contract addresses providing
/// implementations of this standard.
#[derive(Debug, Serialize, SchemaType)]
struct SetImplementorsParams {
    /// The identifier for the standard.
    id:           StandardIdentifierOwned,
    /// The addresses of the implementors of the standard.
    implementors: Vec<ContractAddress>,
}



/********************** Contract Errors **********************/

// contract errors.
#[derive(Serialize, Debug, PartialEq, Eq, Reject, SchemaType)]
enum CustomContractError {
    // Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
    // Failed logging: Log is full.
    LogFull,
    // Failed logging: Log is malformed.
    LogMalformed,
    // Only accounts can mint will.
    OnlyAccount,

    // Will errors
    IncorrectHash,
    NoWill,
    
    // Notary errors
    NotaryCantBeTestator,
    WillAlreadyNotarized,
    IncorrectNotary,

    InvokeContractError,
    InvalidTokenId,
}


/// Wrapping the custom errors in a type with CIS2 errors.
type ContractError = Cis2Error<CustomContractError>;
type ContractResult<A> = Result<A, ContractError>;

/// Mapping the logging errors to CustomContractError.
impl From<LogError> for CustomContractError {
    fn from(le: LogError) -> Self {
        match le {
            LogError::Full => Self::LogFull,
            LogError::Malformed => Self::LogMalformed,
        }
    }
}

/// Mapping errors related to contract invocations to CustomContractError.
impl<T> From<CallContractError<T>> for CustomContractError {
    fn from(_cce: CallContractError<T>) -> Self { Self::InvokeContractError }
}

/// Mapping CustomContractError to ContractError
impl From<CustomContractError> for ContractError {
    fn from(c: CustomContractError) -> Self { Cis2Error::Custom(c) }
}


/********************** State Functions  **********************/

impl<S: HasStateApi> State<S> {

    // create a new state with no files registered.
    fn new(state_builder: &mut StateBuilder<S>) -> Self {
        State {
            state:           state_builder.new_map(),
            implementors:    state_builder.new_map(),
        }
    }

    // mints new will struct under 
    fn mint(
        &mut self,
        owner: &AccountAddress,
        will: Will,
        state_builder: &mut StateBuilder<S>,) -> Result<TokenIdU32,ContractError>
        {
            let mut owner_state = self.state.entry(*owner).or_insert_with(|| WillState::empty(state_builder));
            let will_index = owner_state.will_index; // get current will count

            // insert will object 
            owner_state.wills
            .entry(will_index)
            .or_insert(will);

            owner_state.will_index += 1;
            Ok(concordium_cis2::TokenIdU32(owner_state.will_index))
        }

    // updates owner active will via notary
    fn update_active_will(
        &mut self,
        owner:&AccountAddress,
        index:&u32,
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

    // This will need to be tested 
    // Adds a modification to will mod
    fn modifiy(
        &mut self,
        token_id:&u32,
        owner:&AccountAddress,
        will_mod:WillMod,
    )
    {
        self.state.entry(*owner).and_modify(|will_state| {
            will_state.wills.entry(*token_id).and_modify(|old_will| {
                old_will.mod_history.push(will_mod);
            });
        });
    }

    #[inline(always)]
    fn contains_token(&self,owner:&AccountAddress,token_id: &u32) -> bool {
        self.state
        .get(owner)
        .map(|will_state| will_state.wills.get(token_id).is_some())
        .unwrap_or(false)
    }

    // will balance 
    fn balance(
        &mut self,
        owner:&AccountAddress,
        token_id:&u32,
    ) -> ContractResult<u32> {
        ensure!(self.contains_token(owner,token_id),ContractError::InvalidTokenId);
        Ok(self
        .state
        .get(owner)
        .map(|will_state| will_state.will_index.clone())
        .unwrap_or(0))
    }
    
    // Revokes active will from user
    fn revoke(
        &mut self,
        token_id:&u32,
        owner:&AccountAddress,
    )
    {
        self.state.entry(*owner).and_modify(|will_state| {
            will_state.active_will = None;
        });
    }
    
    // Revives a existing or newly notarized will to active will 
    fn revive(
        &mut self,
        owner:&AccountAddress,
        token_id:&u32,
        will:Will,
    ) 
    {
        self.state.entry(*owner).and_modify(|will_state| {
            will_state.active_will = Some(will.clone());
            will_state.wills.entry(*token_id).and_modify(|old_will| {
                *old_will = will.clone();
            });

        });
    }
    
    // Allows will notary to be changed 
    fn change_notary(
        &mut self,
        owner:&AccountAddress,
        token_id:&u32,
        notary:&AccountAddress,
    ) {
        self.state.entry(*owner).and_modify(|will_state| {
            will_state.wills.entry(*token_id).and_modify(|old_will| {
                old_will.notary = *notary;
            });
        });
    }
    
    // return option Will 
    fn get_will(
        &self,
        owner:&AccountAddress,
        token_id:&u32,
    ) -> Result<Option<Will>,()>
    {
        let will = self
        .state
        .get(&owner)
        .map(|will_state| will_state.wills.get(&token_id).map(|v| v.clone()))
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

    // will exist 
    fn will_exists(
        &self,
        owner:&AccountAddress,
        index:&u32
    ) -> bool {
        self.state
        .get(owner)
        .map(|will_state| will_state.wills.get(index).is_some())
        .unwrap_or(false)
    }

    // will count of owner
    // TODO merge with balance method
    fn will_count(
        &self,
        owner:&AccountAddress,
    ) -> u32
        {
            self
            .state
            .get(owner)
            .map(|will_state| will_state.will_index.clone())
            .unwrap_or(0)
        }

}

/*********************** Contract Events  ***********************/

// Tagged events to be serialized for the event log.
#[derive(Debug, Serial, SchemaType)]
enum Event {
    Notarization(NotarizationEvent),
}

// The NotarizationEvent is logged when a new file hash is registered.
#[derive(Debug, Serialize, SchemaType)]
pub struct NotarizationEvent {
    will_id:   u32,
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


/********************** Contract Functions  **********************/

// Init creates a new smart contract instance.
#[init(contract = "ewills2")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
    // Build new contract state
    Ok(State::new(state_builder))
}

/****** Tools ********/

/// Build a string from TOKEN_METADATA_BASE_URL appended with the token ID
/// encoded as hex.
fn build_token_metadata_url(token_id: &String) -> String {
    let mut token_metadata_url = String::from(TOKEN_METADATA_BASE_URL);
    token_metadata_url.push_str(&token_id);
    token_metadata_url
}

/********************** Contract Views **********************/


/// Mint new will token
/// Can only be called by the contract owner.
/// Logs a `Mint` and a `TokenMetadata` event for each token.
/// The url for the token metadata is the token ID encoded in hex, appended on
/// the `TOKEN_METADATA_BASE_URL`.
///
/// It rejects if:
/// - The sender is not the contract instance owner.
/// - Fails to parse parameter.
/// - Any of the tokens fails to be minted, which could be if:
///     - The minted token ID already exists.
///     - Fails to log Mint event
///     - Fails to log TokenMetadata event
///
/// Note: Can at most mint 32 token types in one call due to the limit on the
/// number of logs a smart contract can produce on each function call.
#[receive(
    contract = "ewills2",
    name = "mint",
    parameter = "MintParams",
    error = "ContractError",
    mutable,
    enable_logger
)]
fn contract_mint<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> Result<TokenIdU32, ContractError> {

    // bail if sender is contract address
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(ContractError::Unauthorized),
        Address::Account(account_address) => account_address,
    };

    // parse method parameters
    let param: MintParams = ctx.parameter_cursor().get()?;

    // ensure notary address is not sender account
    ensure_ne!(param.notary, sender_account,ContractError::Unauthorized);

    // timestamp for modfication
    let timestamp = ctx.metadata().slot_time();

    // mint modification to will
    // sense mod history vector is init add new mode event
    let will_mod = WillMod::new(WillModType::Mint,timestamp);
    let mut mod_hist_vec = Vec::new();
    mod_hist_vec.push(will_mod);

    // will file state | none until notarized
    let file_state = FileState::new();

    // Params for wills TODO should only be using pointers
    let will_file = build_token_metadata_url(&param.will_file);
    let will_hash = param.will_hash;

    // create new `will` struct
    let will = Will{
        will_file:will_file.clone(),  // CID IPFS content identifiers
        will_hash:will_hash,  // Hash of will file
        notary:param.notary,
        notarized:false,
        e_seal:file_state,
        mod_history:mod_hist_vec,
    };

    let (state, builder) = host.state_and_builder();
    
    // add `will` struct into sender address key within contract state `wills` array
    let token_id = state.mint(&sender_account,will,builder)?;

    // Event for minted NFT.
    logger.log(&Cis2Event::Mint(MintEvent {
        token_id,
        amount: ContractTokenAmount::from(1),
        owner: concordium_std::Address::Account(sender_account),
    }))?;

    // Metadata URL for the NFT.
    logger.log(&Cis2Event::TokenMetadata::<_, ContractTokenAmount>(TokenMetadataEvent {
        token_id,
        metadata_url: MetadataUrl {
            url:  build_token_metadata_url(&will_file),
            hash: Some(will_hash.0),
        },
    }))?;
    
    Ok(token_id)
}

#[receive(
    contract = "ewills2",
    name = "will_count",
    parameter = "WillParams",
    return_value = "u32"
)]
fn will_count<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<u32> {
    let param: WillParams = ctx.parameter_cursor().get()?;
    Ok(host.state().will_count(&param.owner)) 
}


#[receive(
    contract = "ewills2",
    name = "active_will",
    parameter = "WillParams",
    return_value = "Option<Will>"
)]
fn active_will<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Option<Will>> {

    // get input parameters
    let param: WillParams = ctx.parameter_cursor().get()?;

    // output param
    let active_result = host.state().get_active_will(&param.owner);
    Ok(active_result)
}

// Allows a will to be notarized 
#[receive(
    contract = "ewills2",
    name = "notarize",
    parameter = "NotaryParams",
    error = "CustomContractError",
    mutable,
    enable_logger
)]
fn notarize<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> Result<(), CustomContractError> {

    // parse notary parameters
    let params: NotaryParams = ctx.parameter_cursor().get()?;

    //ensure!(host.state().will_exists(&params.testator,&params.token_id),CustomContractError::NoWill);

    // get `will` struct if exist
    let will = match host.state().get_will(&params.testator,&params.token_id).unwrap(){
        Some(will_object) => will_object,
        None => return Err(CustomContractError::NoWill),
    };

    // clone `will` struct
    let mut c_will = will.clone();

    // get minted file hash 
    let file_hash:HashSha2256 = will.will_hash;
    // ensure notry will file hashes 
    ensure_eq!(params.will_hash,file_hash,CustomContractError::IncorrectHash);

    // match sender to address account / only accounts can notarize a will.
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(CustomContractError::OnlyAccount),
        Address::Account(account_address) => account_address,
    };

    // check if `will` has been already notarized
    ensure!(!will.notarized,CustomContractError::WillAlreadyNotarized);

    // only callable by notry within the `will` struct
    ensure_eq!(will.notary, sender_account, CustomContractError::IncorrectNotary);

    // get block time stamp
    let timestamp = ctx.metadata().slot_time();
    
    c_will.notarized = true;
    
    c_will.e_seal = FileState {
        timestamp : Some(timestamp),
        witness : params.witness,
    };
    
    // update `will` struct 
    host.state_mut().update_active_will(&params.testator, &params.token_id, c_will);

    let will_mod = WillMod::new(WillModType::Notarized,timestamp);
    host.state_mut().modifiy(&params.token_id,&params.testator,will_mod);

    // log notarization event
    logger.log(&Event::Notarization(NotarizationEvent {
        will_id:params.token_id,
        testator:params.testator,
        notary:sender_account,
        file_hash:file_hash,
        witness: sender_account,
        timestamp:timestamp,
    }))?;
    
    Ok(())
}

#[receive(
    contract = "ewills2",
    name = "get_will",
    parameter = "WillParams",
    return_value = "Option<Will>"
)]
fn get_will<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Option<Will>> {

    // get input parameters
    let param: WillParams = ctx.parameter_cursor().get()?;

    // output param
    let active_result = host.state().get_will(&param.owner,&param.token_id).unwrap();
    Ok(active_result)
}


#[receive(
    contract = "ewills2",
    name = "revoke_active_will",
    parameter = "WillParams",
    return_value = "bool"
)]
fn revoke_active_will() {

}


#[receive(
    contract = "ewills2",
    name = "revoke_active_will",
    parameter = "WillParams",
    return_value = "bool"
)]
fn revive_active_will() {

}


#[receive(
    contract = "ewills2",
    name = "will_exists",
    parameter = "WillParams",
    return_value = "bool"
)]
fn will_exists<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<bool> {

    // get input parameters
    let param: WillParams = ctx.parameter_cursor().get()?;

    // output bool true/false if `will` exist
    Ok(host.state().will_exists(&param.owner,&param.token_id))
}


#[receive(
    contract = "ewills2",
    name = "is_notarized",
    parameter = "WillParams",
    return_value = "bool",
    error = "CustomContractError",
)]
fn is_notarized<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<bool> {

    // get AccountAddress input parameter
    let params: WillParams = ctx.parameter_cursor().get()?;

    // match input parameter to `will` within 
    ensure!(host.state().will_exists(&params.owner,&params.token_id),CustomContractError::NoWill.into());
    let will = match host.state().get_will(&params.owner,&params.token_id).unwrap(){
        Some(will_object) => will_object,
        None => return Err(CustomContractError::NoWill.into()),
    };

    // return bool true/false if will has be notarized
    Ok(will.notarized)
}



/************* CIS Standard *****************/

/// Parameter type for the CIS-2 function `balanceOf` specialized to the subset
/// of TokenIDs used by this contract.
type ContractBalanceOfQueryParams = BalanceOfQueryParams<ContractTokenId>;
/// Response type for the CIS-2 function `balanceOf` specialized to the subset
/// of TokenAmounts used by this contract.
type ContractBalanceOfQueryResponse = BalanceOfQueryResponse<ContractTokenAmount>;

#[receive(
    contract = "ewills2",
    name = "balanceOf",
    parameter = "ContractBalanceOfQueryParams",
    return_value = "ContractBalanceOfQueryResponse",
    mutable,
    error = "ContractError"
)]
fn contract_balance_of<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<ContractBalanceOfQueryResponse> {

    // bail if sender is contract address
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(ContractError::Unauthorized.into()),
        Address::Account(account_address) => account_address,
    };

    let (state, builder) = host.state_and_builder();

    let params: ContractBalanceOfQueryParams = ctx.parameter_cursor().get()?;
    // Build the response.
    let mut response = Vec::with_capacity(params.queries.len());
    for query in params.queries {
        let token_id = query.token_id.0;
        // Query the state for balance.
        // Switch us
        let amount = state.balance(&sender_account,&token_id)?;
        if amount > 0 {
            response.push(concordium_cis2::TokenAmountU8(1));
        }else{
            response.push(concordium_cis2::TokenAmountU8(0));
        }
        
    }
    let result = ContractBalanceOfQueryResponse::from(response);
    Ok(result)
}



/*
type TransferParameter = TransferParams<ContractTokenId, ContractTokenAmount>;

/// Execute a list of token transfers, in the order of the list.
///
/// Logs a `Transfer` event and invokes a receive hook function for every
/// transfer in the list.
///
/// It rejects if:
/// - It fails to parse the parameter.
/// - Any of the transfers fail to be executed, which could be if:
///     - The `token_id` does not exist.
///     - The sender is not the owner of the token, or an operator for this
///       specific `token_id` and `from` address.
///     - The token is not owned by the `from`.
/// - Fails to log event.
/// - Any of the receive hook function calls rejects.
#[receive(
    contract = "ewills2",
    name = "transfer",
    parameter = "TransferParameter",
    error = "ContractError",
    enable_logger,
    mutable
)]
fn contract_transfer<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> ContractResult<()> {
  
    Ok(())
}

/// Enable or disable addresses as operators of the sender address.
/// Logs an `UpdateOperator` event.
///
/// It rejects if:
/// - It fails to parse the parameter.
/// - Fails to log event.
#[receive(
    contract = "ewills2",
    name = "updateOperator",
    parameter = "UpdateOperatorParams",
    error = "ContractError",
    enable_logger,
    mutable
)]
fn contract_update_operator<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> ContractResult<()> {

    Ok(())
}
*/


/********************** Contract Testing  **********************/

#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    const TESTATOR: AccountAddress = AccountAddress([0u8; 32]);
    const NOTARY: AccountAddress = AccountAddress([2u8; 32]);

    const WITNESS1: AccountAddress = AccountAddress([4u8; 32]);
    const WITNESS2: AccountAddress = AccountAddress([5u8; 32]);

    const WILL_1_HASH :HashSha2256 = HashSha2256([6u8; 32]);
    const WILL_2_HASH :HashSha2256 = HashSha2256([7u8; 32]);

    const TOKEN_0:u32 = 0;
    const TOKEN_1:u32 = 1;

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

        // set timestamp 
        const TIME: u64 = 1;
        ctx.set_metadata_slot_time(Timestamp::from_timestamp_millis(TIME));

        // construct input parameters
        let will_url:String = "file_url".to_string();
        let mut logger = TestLogger::init();

        let param_object = MintParams {
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
        let result = contract_mint(&ctx,&mut host,&mut logger);
        claim!(result.is_ok(),"Mint failed.");
        
        // get & check will struct from contract
        let get_will_params_object = WillParams {
            token_id: TOKEN_0,
            owner: TESTATOR,
        };
        // encode input parameters
        let params = to_bytes(&get_will_params_object); 
        ctx.set_parameter(&params);

        // check will count
        let will_count = will_count(&ctx,&mut host);
        claim_eq!(will_count.unwrap(),1);

        // check will values
        let will = get_will(&ctx,&mut host);

        // Get Will object
        let get_will = get_will(&ctx,&mut host);
         claim!(get_will.is_ok(),"Could not get will");
        let will = get_will.unwrap();
        claim!(will.is_some(), "will contain no components");

        // check set variables within will struct 
        claim_eq!(will.clone().unwrap().will_file, "https://cloudflare-ipfs.com/ipfs/file_url", "Will url should match");
        claim_eq!(will.clone().unwrap().will_hash, WILL_1_HASH, "Will hash should match");
        claim_eq!(will.clone().unwrap().notary, NOTARY, "Will notary should match");
        claim_eq!(will.clone().unwrap().notarized, false, "Will should not be notarized");
        claim_eq!(will.clone().unwrap().mod_history.len(),1,"Will mod history is incorrect");
        // How are enum types checked??
        //claim_eq!(will.clone().unwrap().mod_history[0].mod_type, WillModType::Mint,"Will mod history is incorrect");

        // check active will for testator should be none
        let active_will = active_will(&ctx,&mut host).unwrap();
        claim!(active_will.is_none(), "Active Will should be empty");
    }
    
    #[concordium_test]
    fn testing_multiple_will_mint() {
        // set up test case
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(Address::Account(TESTATOR));

        let mut state_builder = TestStateBuilder::new();
        let state = State::new(&mut state_builder);

        let mut host = TestHost::new(state,state_builder);

        // set timestamp 
        const TIME: u64 = 1;
        ctx.set_metadata_slot_time(Timestamp::from_timestamp_millis(TIME));
  
        // Mint a first will
        let will_url_1:String = "will-1".to_string();
        let param_object = MintParams {
            will_file: will_url_1.clone(),
            will_hash: WILL_1_HASH,
            notary: NOTARY,
        };

        let mut logger = TestLogger::init();
        let params = to_bytes(&param_object);
        ctx.set_parameter(&params);

        let _ = contract_mint(&ctx,&mut host,&mut logger);

        // Mint a second will
        let will_url_2:String = "will-2".to_string();
        let param_object_will_2 = MintParams {
            will_file: will_url_2.clone(),
            will_hash: WILL_2_HASH,
            notary: NOTARY,
        };

        let mut logger = TestLogger::init();
        let params_2 = to_bytes(&param_object_will_2);
        ctx.set_parameter(&params_2);

        let _ = contract_mint(&ctx,&mut host,&mut logger);

        let count_params = to_bytes(&TESTATOR);
        ctx.set_parameter(&count_params);

        // Check will 1
        // get will parameters
        let get_will_params_object = WillParams {
            token_id: TOKEN_0,
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
        claim_eq!(will_one.clone().unwrap().will_file, "https://cloudflare-ipfs.com/ipfs/will-1", "Will url should match");
        claim_eq!(will_one.clone().unwrap().will_hash, WILL_1_HASH, "Will hash should match");
        claim_eq!(will_one.clone().unwrap().notary, NOTARY, "Will notary should match");
        claim_eq!(will_one.clone().unwrap().notarized, false, "Will should not be notarized");
        
        // Check will 2
        // get will parameters
        let get_will_params_object_2 = WillParams {
            token_id: TOKEN_1,
            owner: TESTATOR,
        };

        let get_will_params_2 = to_bytes(&get_will_params_object_2);
        ctx.set_parameter(&get_will_params_2);
       
        let get_will_two = get_will(&ctx,&mut host);
        
        claim!(get_will_two.is_ok(),"Could not get will");
        let will_two = get_will_two.unwrap();
        claim!(will_two.is_some(), "will contain no components");

        // check set variables within will struct 
        claim_eq!(will_two.clone().unwrap().will_file, "https://cloudflare-ipfs.com/ipfs/will-2", "Will 2 url should match");
        claim_eq!(will_two.clone().unwrap().will_hash, WILL_2_HASH, "Will 2 hash should match");
        claim_eq!(will_two.clone().unwrap().notary, NOTARY, "Will 2 notary should match");
        claim_eq!(will_two.clone().unwrap().notarized, false, "Will 2 should not be notarized");
    }
    
    #[concordium_test]
    fn testing_notarization() {
        // set up test case
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(Address::Account(TESTATOR));

        // minting parameters
        let will_url:String = "will-2".to_string();

        let will_params = MintParams {
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

        // set timestamp 
        const TIME: u64 = 1;
        ctx.set_metadata_slot_time(Timestamp::from_timestamp_millis(TIME));

        // mint will
        let _ = contract_mint(&ctx,&mut host,&mut logger);

        // notary parameters 
        let mut witness = Vec::new();
        witness.push(WITNESS1);
        witness.push(WITNESS2);

        let notary_params = NotaryParams {
            will_hash: WILL_1_HASH,
            token_id: 0,
            testator:TESTATOR,
            witness: witness.clone(),
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
        
        let get_will_params_object_2 = WillParams {
            token_id: 0,
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
        claim_eq!(will.clone().unwrap().will_file, "https://cloudflare-ipfs.com/ipfs/will-2", "Will url should match");
        claim_eq!(will.clone().unwrap().will_hash,WILL_1_HASH, "Will hash should match");
        claim_eq!(will.clone().unwrap().notary,NOTARY, "Will notary should match");
        claim_eq!(will.clone().unwrap().notarized,true, "Will should be be notarized");
        claim_eq!(will.clone().unwrap().mod_history.len(),2,"Will mod history is incorrect");

        
        // check e seal docs
        let e_seal = will.clone().unwrap().e_seal;
        claim_eq!(e_seal.clone().timestamp, Some(Timestamp::from_timestamp_millis(TIME)), "Eseal time is incorrect");
        claim_eq!(e_seal.clone().witness, witness, "Witness address is incorrect");

        // check testators active will 
        let active_will = active_will(&ctx,&mut host);
        claim!(active_will.is_ok(),"Could not get active will");

        let a_will = active_will.unwrap();
        claim!(a_will.is_some(), "Active will is none");

        claim_eq!(a_will.clone().unwrap().will_file, "https://cloudflare-ipfs.com/ipfs/will-2", "Will url should match");
        claim_eq!(a_will.clone().unwrap().will_hash,WILL_1_HASH, "Will hash should match");
        claim_eq!(a_will.clone().unwrap().notary,NOTARY, "Will notary should match");
        claim_eq!(a_will.clone().unwrap().notarized,true, "Will should be be notarized");
        // Mint + Notary = 2
        claim_eq!(will.clone().unwrap().mod_history.len(),2,"Will mod history is incorrect");
    }

    #[concrodium_test]
    fn revoke_active_will() {

    }


    #[concrodium_test]
    fn revive_active_will() {

    }

    
    /*
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
    */
}

