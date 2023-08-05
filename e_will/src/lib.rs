//! # A Concordium V1 smart contract
#![cfg_attr(not(feature = "std"), no_std)]
use concordium_cis2::*;
use concordium_std::*;

// cargo concordium build -e -o a.wasm.v1 -s schema.bin
// concordium-client --grpc-ip node.testnet.concordium.com --grpc-port 20000  module deploy a.wasm.v1 --name ewillv2 --sender account1
// Module successfully deployed with reference: '32b913ead0761a4dfce08a9ddf32462dfb73c36542f5e70b0e673913131f1a98'.
// Module reference 85b36104059fd514ca13f88271bf40be6ab3453f1ee8bd6775b9944960030543 was successfully named 'ewillv2'.

// concordium-client --grpc-ip node.testnet.concordium.com --grpc-port 20000  contract init ewills --contract ewills  --sender account1 --energy 10000
// 
// Contract successfully initialized with address: {"index":5659,"subindex":0}
// cargo concordium build --schema-base64-out -

/// Base meta url using IPFS
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


// will mod struct
// mod event 
// mod timestamp
#[derive(Serialize, SchemaType, Clone)]
pub struct WillMod {
    mod_type: WillModType,
    timestamp: Timestamp
}

// mod implentation
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
    // Witnesses that witness file notary. Check readme for info
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
    will_hash:HashSha2256,  
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
    active_will: Option<u32>,       // token_id of active will
    will_index: u32,                // amount of will user has created ie (owned_tokens)
    wills: StateMap<u32,Will,S>,    // mapping of wills to u8 NOTE could us will hash 
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
    NoActiveWill,
    
    // Notary errors
    NotaryCantBeTestator,
    WillAlreadyNotarized,
    IncorrectNotary,
    NotNotarized,

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

    /// mints new will struct under sender account address as owner
    /// returns token_id
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
            
            let tokenId = owner_state.will_index;
            // increment sender will count
            owner_state.will_index += 1;
            // return new token_id 
            Ok(concordium_cis2::TokenIdU32(tokenId))
        }
    

    /// updates owner active will via notary
    fn update_active_will(
        &mut self,
        owner:&AccountAddress,
        index:&u32,
        will:&Will
    )
    {
        self.state.entry(*owner).and_modify(|will_state| {
            will_state.active_will = Some(*index);
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
        &self,
        token_id:&u32,
        owner:&AccountAddress,
    ) -> ContractResult<ContractTokenAmount> {
        //ensure!(self.contains_token(owner,token_id),ContractError::InvalidTokenId);
        let will_count = self
        .state
        .get(owner)
        .map(|will_state| will_state.will_index.clone())
        .unwrap_or(0);

        if will_count > 0 {
            Ok(concordium_cis2::TokenAmountU8(1))
        }else{
            Ok(concordium_cis2::TokenAmountU8(0))
        }
    }
    
    // Revokes active will
    fn revoke(
        &mut self,
        owner:&AccountAddress,
    )
    {
        self.state.entry(*owner).and_modify(|will_state| {
            will_state.active_will = None;
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
    
    // Return option Will 
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

        let active_index = self
        .state
        .get(&owner)
        .map(|will_state| will_state.active_will.clone())
        .unwrap_or(None);

        let token_id = match active_index {
            None => return None,
            Some(u32) => active_index.unwrap(),
        };

        return self.get_will(&owner,&token_id).unwrap();
    }

    fn get_active_will_id(
        &self,
        owner:&AccountAddress,
    ) -> Option<u32>
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

    /********** CI2 Standard *************/
    /// Check if state contains any implementors for a given standard.
    fn have_implementors(&self, std_id: &StandardIdentifierOwned) -> SupportResult {
        if let Some(addresses) = self.implementors.get(std_id) {
            SupportResult::SupportBy(addresses.to_vec())
        } else {
            SupportResult::NoSupport
        }
    }

    /// Set implementors for a given standard.
    fn set_implementors(
        &mut self,
        std_id: StandardIdentifierOwned,
        implementors: Vec<ContractAddress>,
    ) {
        self.implementors.insert(std_id, implementors);
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
// TODO check if CIS2 event need to emitted 
// event = "Cis2Event<ContractTokenId, ContractTokenAmount>")
#[init(contract = "ewills893")]
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
    contract = "ewills893",
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
    contract = "ewills893",
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
    contract = "ewills893",
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
    contract = "ewills893",
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
    host.state_mut().update_active_will(&params.testator, &params.token_id,&c_will);

    // add will mod after update active will 
    let will_mod = WillMod::new(WillModType::Notarized,timestamp);
    host.state_mut().modifiy(&params.token_id,&params.testator,will_mod);

    // log notarization event
    logger.log(&Event::Notarization(NotarizationEvent {
        will_id:params.token_id,
        testator:params.testator,
        notary: sender_account, // sender is notary 
        file_hash:file_hash,
        witness: sender_account, // TODO change to witness vectors
        timestamp:timestamp,
    }))?;
    Ok(())
}

#[receive(
    contract = "ewills893",
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
    contract = "ewills893",
    name = "revoke_active_will",
    error = "CustomContractError",
    mutable,
    return_value = "bool",
)]
fn revoke_active_will<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<bool, CustomContractError> {

    // match sender to address account
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(CustomContractError::OnlyAccount),
        Address::Account(account_address) => account_address,
    };
    let token_id_w = host.state().get_active_will_id(&sender_account);
    let token_id = match token_id_w {
        None => bail!(CustomContractError::NoActiveWill),
        Some(u32) => token_id_w.unwrap(), 
    };

    host.state_mut().revoke(&sender_account);

    // get block time stamp
    let timestamp = ctx.metadata().slot_time();

    // add will mod after update active will 
    let will_mod = WillMod::new(WillModType::Revoke,timestamp);
    host.state_mut().modifiy(&token_id,&sender_account,will_mod);

    Ok(true)
}


#[receive(
    contract = "ewills893",
    name = "revive_active_will",
    parameter = "WillParams",
    error = "CustomContractError",
    mutable,
    return_value = "bool",
)]
fn revive_active_will<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<bool, CustomContractError>
 {
    // get input parameters
    let param: WillParams = ctx.parameter_cursor().get()?;

    // match sender to address account / only accounts.
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(CustomContractError::OnlyAccount),
        Address::Account(account_address) => account_address,
    };

    // get will to revive
    let will = host.state().get_will(&sender_account,&param.token_id).unwrap();

    // check if will is not option
    let will_op = match will {
        None => bail!(CustomContractError::NoWill),
        Some(will) => will, 
    };

    // check if will has be notarized 
    ensure_eq!(will_op.notarized, true, CustomContractError::NotNotarized);
    
    // revive prior notarized will
    //host.state_mut().revive(&sender_account,&param.token_id, &will_op);
    host.state_mut().update_active_will(&sender_account, &param.token_id, &will_op);

    // get block time stamp
    let timestamp = ctx.metadata().slot_time();

    // add will mod after update active will 
    let will_mod = WillMod::new(WillModType::Revived,timestamp);
    host.state_mut().modifiy(&param.token_id,&sender_account,will_mod);

    Ok(true)
}


#[receive(
    contract = "ewills893",
    name = "will_exists",
    parameter = "WillParams",
    return_value = "bool",
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
    contract = "ewills893",
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
    contract = "ewills893",
    name = "balanceOf",
    parameter = "ContractBalanceOfQueryParams",
    return_value = "ContractBalanceOfQueryResponse",
    error = "ContractError"
)]
fn contract_balance_of<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<ContractBalanceOfQueryResponse> {

    // bail if sender is contract address
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(ContractError::Unauthorized.into()),
        Address::Account(account_address) => account_address,
    };

    let params: ContractBalanceOfQueryParams = ctx.parameter_cursor().get()?;
    // Build the response.
    let mut response = Vec::with_capacity(params.queries.len());
    for query in params.queries {
        let token_id = query.token_id.0;
        // Query the state for balance.
        // Switch us
        let amount = host.state().balance(&token_id,&sender_account)?;
        //if amount > 0 {
        response.push(concordium_cis2::TokenAmountU8(amount.into()));
        //}else{
        //    response.push(concordium_cis2::TokenAmountU8(0));
        //}
    }
    let result = ContractBalanceOfQueryResponse::from(response);
    Ok(result)
}


type TransferParameter = TransferParams<ContractTokenId, ContractTokenAmount>;

/// eWill NFT's aren't transferable 
#[receive(
    contract = "ewills893",
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
/// TODO add operators
#[receive(
    contract = "ewills893",
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

#[receive(
    contract = "ewills893",
    name = "operatorOf",
    parameter = "OperatorOfQueryParams",
    return_value = "OperatorOfQueryResponse",
    error = "ContractError"
)]
fn contract_operator_of<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<OperatorOfQueryResponse> {
    // Parse the parameter.
    let params: OperatorOfQueryParams = ctx.parameter_cursor().get()?;
    // Build the response.
    let mut response = Vec::with_capacity(params.queries.len());
    for query in params.queries {
        // Query the state for address being an operator of owner.
        //let is_operator = host.state().is_operator(&query.address, &query.owner);
        response.push(true);
    }
    let result = OperatorOfQueryResponse::from(response);
    Ok(result)
}

/// Parameter type for the CIS-2 function `tokenMetadata` specialized to the
/// subset of TokenIDs used by this contract.
type ContractTokenMetadataQueryParams = TokenMetadataQueryParams<ContractTokenId>;

/// Get the token metadata URLs and checksums given a list of token IDs.
///
/// It rejects if:
/// - It fails to parse the parameter.
/// - Any of the queried `token_id` does not exist.
#[receive(
    contract = "ewills893",
    name = "tokenMetadata",
    parameter = "ContractTokenMetadataQueryParams",
    return_value = "TokenMetadataQueryResponse",
    error = "ContractError"
)]
fn contract_token_metadata<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<TokenMetadataQueryResponse> {
    // Parse the parameter.
    let params: ContractTokenMetadataQueryParams = ctx.parameter_cursor().get()?;
    // Build the response.
    let mut response = Vec::with_capacity(params.queries.len());

    // Bail if sender is contract address
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(ContractError::Unauthorized.into()),
        Address::Account(account_address) => account_address,
    };

    for token_id in params.queries {

        // Check the token exists.
        //ensure!(host.state().contains_token(&sender_account, &token_id.0), ContractError::InvalidTokenId);

        let will = match host.state().get_will(&sender_account, &token_id.0).unwrap(){
            Some(will_object) => will_object,
            None => return Err(CustomContractError::NoWill.into()),
        };

        let metadata_url = MetadataUrl {
            url:  will.will_file,
            hash: Some(will.will_hash.0),
        };
        response.push(metadata_url);
    }
    
    let result = TokenMetadataQueryResponse::from(response);
    Ok(result)
}


/// Get the supported standards or addresses for a implementation given list of
/// standard identifiers.
///
/// It rejects if:
/// - It fails to parse the parameter.
#[receive(
    contract = "ewills893",
    name = "supports",
    parameter = "SupportsQueryParams",
    return_value = "SupportsQueryResponse",
    error = "ContractError"
)]
fn contract_supports<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<SupportsQueryResponse> {
    // Parse the parameter.
    let params: SupportsQueryParams = ctx.parameter_cursor().get()?;

    // Build the response.
    let mut response = Vec::with_capacity(params.queries.len());
    for std_id in params.queries {
        if SUPPORTS_STANDARDS.contains(&std_id.as_standard_identifier()) {
            response.push(SupportResult::Support);
        } else {
            response.push(host.state().have_implementors(&std_id));
        }
    }
    let result = SupportsQueryResponse::from(response);
    Ok(result)
}

#[receive(
    contract = "ewills893",
    name = "setImplementors",
    parameter = "SetImplementorsParams",
    error = "ContractError",
    mutable
)]
fn contract_set_implementor<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    // Authorize the sender.
    ensure!(ctx.sender().matches_account(&ctx.owner()), ContractError::Unauthorized);
    // Parse the parameter.
    let params: SetImplementorsParams = ctx.parameter_cursor().get()?;
    // Update the implementors in the state
    host.state_mut().set_implementors(params.id, params.implementors);
    Ok(())
}


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

    const NO_TOKEN:u32 = 0;
    const HAS_TOKEN:u32 = 1;

    const TOKEN_0:u32 = 0;
    const TOKEN_1:u32 = 1;
    const NO_TOKEN_0:u32 = 42;

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
        
        // check balance before minting 
        //let balance0 = host.state().balance(&TOKEN_0,&TESTATOR);
        //claim_eq!(balance0, 0.into(), "Tokens should be owned by the given address 0");

        // Call test minting method
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
        // Test balance from state 
        let balance0 = host.state().balance(&TOKEN_0, &TESTATOR).expect_report("Token is expected to exist");
        claim_eq!(balance0, 1.into(), "Tokens should be owned by the given testator address");

        let balance1 = host.state().balance(&TOKEN_0, &WITNESS1).expect_report("Token is expected to exist");
        claim_eq!(balance1, 0.into(), "Token should not be owned");
        
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

        claim!(
            logger.logs.contains(&to_bytes(&Cis2Event::Mint(MintEvent {
                owner:concordium_std::Address::Account(TESTATOR),
                token_id:TokenIdU32(TOKEN_0),
                amount: ContractTokenAmount::from(1),
            }))),
            "Expected an event for minting TOKEN_0"
        );
        /*
        // URL FAILS under token metadata
        claim!(
            logger.logs.contains(&to_bytes(&Cis2Event::TokenMetadata::<_, ContractTokenAmount>(
                TokenMetadataEvent {
                    token_id: TokenIdU32(TOKEN_0),
                    metadata_url: MetadataUrl {
                        url: format!("{}file_url",TOKEN_METADATA_BASE_URL),
                        hash: Some(WILL_1_HASH.0),
                    },
                }
            ))),
            "Expected an event for token metadata for TOKEN_0"
        );
        */
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

        // setup witness vec parameters 
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
        claim_eq!(a_will.clone().unwrap().mod_history.len(),2,"Active Will mod history is incorrect");
        // Mint + Notary = 2
        claim_eq!(will.clone().unwrap().mod_history.len(),2,"Will mod history is incorrect");
    }
    /*
    #[concrodium_test]
    fn revoke_active_will() {

    }


    #[concrodium_test]
    fn revive_active_will() {

    }
    */
    
    #[concordium_test]
    fn testing_activation_change() {
        // set up test case
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(Address::Account(TESTATOR));

        // set timestamp 
        const TIME: u64 = 1;
        ctx.set_metadata_slot_time(Timestamp::from_timestamp_millis(TIME));

        // create state
        let mut logger = TestLogger::init();
        let mut state_builder = TestStateBuilder::new();
        let state = State::new(&mut state_builder);
        let mut host = TestHost::new(state,state_builder);

        // minting parameters
        let will_url_one:String = "will-1".to_string();
        let will_url_two:String = "will-2".to_string();

        let will_params_one = MintParams {
            will_file: will_url_one.clone(),
            will_hash: WILL_1_HASH,
            notary: NOTARY,
        };

        let will_params_two = MintParams {
            will_file: will_url_two.clone(),
            will_hash: WILL_2_HASH,
            notary: NOTARY,
        };
        
        // setup witness vec for notary 
        let mut witness = Vec::new();
        witness.push(WITNESS1);
        witness.push(WITNESS2);

        // set parameters and mint first will
        let w_params_one = to_bytes(&will_params_one);
        ctx.set_parameter(&w_params_one);
        let _ = contract_mint(&ctx,&mut host,&mut logger);


        // set parameters and mint second will 
        let w_params_two = to_bytes(&will_params_two);
        ctx.set_parameter(&w_params_two);
        let _ = contract_mint(&ctx,&mut host,&mut logger);

        // notary parameters for first will
        let notary_params_one = NotaryParams {
            will_hash: WILL_1_HASH,
            token_id: 0,
            testator:TESTATOR,
            witness: witness.clone(),
        };

        // notary parameters for second will
        let notary_params_two = NotaryParams {
            will_hash: WILL_2_HASH,
            token_id: 1, // set will id
            testator:TESTATOR,
            witness: witness.clone(),
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
        let check_active_params_object_1 = WillParams {
            token_id: 0,
            owner: TESTATOR,
        };
        let cap1 = to_bytes(&check_active_params_object_1);
        ctx.set_parameter(&cap1);

        let active_will_one = active_will(&ctx,&mut host);
        claim!(active_will_one.is_ok(),"Could not get active will");
        let a_will = active_will_one.unwrap();
        claim!(a_will.is_some(), "Active will is none");
        claim_eq!(a_will.clone().unwrap().will_file,"https://cloudflare-ipfs.com/ipfs/will-2", "Will url should match");
        claim_eq!(a_will.clone().unwrap().will_hash,WILL_2_HASH, "Will hash should match");
        claim_eq!(a_will.clone().unwrap().notary,NOTARY, "Will notary should match");
        claim_eq!(a_will.clone().unwrap().notarized,true, "Will should be be notarized");
        claim_eq!(a_will.clone().unwrap().mod_history.len(),2,"Mod history is incorrect");

        
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
        
        let check_active_params_object_2 = WillParams {
            token_id: 0,
            owner: TESTATOR,
        };

        let cap2 = to_bytes(&check_active_params_object_2);
        ctx.set_parameter(&cap2);
        // check testators active will 
        let active_will = active_will(&ctx,&mut host);
        claim!(active_will.is_ok(),"Could not get active will");

        let a_will = active_will.unwrap();

        claim!(a_will.is_some(), "Active will is none");
        
        claim_eq!(a_will.clone().unwrap().will_file, "https://cloudflare-ipfs.com/ipfs/will-1", "Will url should match");
        claim_eq!(a_will.clone().unwrap().will_hash,WILL_1_HASH, "Will hash should match");
        claim_eq!(a_will.clone().unwrap().notary,NOTARY, "Will notary should match");
        claim_eq!(a_will.clone().unwrap().notarized,true, "Will should be be notarized");
    }
}

