use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, CustomQuery, QuerierWrapper, StdResult, Uint256};

/// A number of Custom messages that can call into the TokenFactory bindings
#[cw_serde]
pub enum TokenFactoryMsg {
    /// CreateDenom creates a new factory denom, of denomination:
    /// factory/{creating contract address}/{Subdenom}
    /// Subdenom can be of length at most 44 characters, in [0-9a-zA-Z./]
    /// The (creating contract address, subdenom) pair must be unique.
    /// The created denom's admin is the creating contract address,
    /// but this admin can be changed using the ChangeAdmin binding.
    CreateDenom {
        subdenom: String,
        metadata: Option<DenomMetadata>,
    },
    /// ChangeAdmin changes the admin for a factory denom.
    /// If the NewAdminAddress is empty, the denom has no admin.
    ChangeAdmin {
        denom: String,
        new_admin_address: Addr,
    },
    /// Contracts can mint native tokens for an existing factory denom
    /// that they are the admin of.
    MintTokens {
        denom: String,
        amount: Uint256,
        mint_to_address: Addr,
    },
    /// Contracts can burn native tokens for an existing factory denom
    /// that they are the admin of.
    /// Currently, the burn from address must be the admin contract.
    BurnTokens {
        denom: String,
        amount: Uint256,
        burn_from_address: Addr,
    },
    /// Sets the metadata on a denom which the contract controls
    SetMetadata { metadata: DenomMetadata },
    /// Forces a transfer of tokens from one address to another.
    ForceTransfer {
        denom: String,
        from_address: Addr,
        to_address: Addr,
        amount: Uint256,
    },
}

/// TokenFactory-specific queries
#[cw_serde]
#[derive(QueryResponses)]
pub enum TokenFactoryQuery {
    #[returns(FullDenomResponse)]
    FullDenom {
        subdenom: String,
        creator_addr: Addr,
    },
    #[returns(AdminResponse)]
    Admin { denom: String },
    #[returns(MetadataResponse)]
    Metadata { denom: String },
    #[returns(DenomsByCreatorResponse)]
    DenomsByCreator { creator: Addr },
    #[returns(TokenParamsResponse)]
    Params {},
}

/// DenomUnit is used to describe a token for the Bank module; part of the SetDenomMetadata message
#[cw_serde]
pub struct DenomUnit {
    /// Denom represents the string name of the given denom unit (e.g uatom). pub denom: String,
    pub denom: String,
    /// Exponent represents power of 10 exponent that one must
    /// raise the base_denom to in order to equal the given DenomUnit's denom
    /// 1 denom = 1^exponent base_denom
    /// (e.g. with a base_denom of uatom, one can create a DenomUnit of 'atom' with
    /// exponent = 6, thus: 1 atom = 10^6 uatom).
    pub exponent: u32,
    /// Aliases is a list of string aliases for the given denom
    pub aliases: Vec<String>,
}

/// DenomMetadata is used to describe a token for the Bank module; part of the SetDenomMetadata message
#[cw_serde]
pub struct DenomMetadata {
    pub description: String,
    /// DenomUnits represents the list of DenomUnit's for a given coin
    pub denom_units: Vec<DenomUnit>,
    /// Base represents the base denom (should be the DenomUnit with exponent = 0).
    pub base: String,
    /// Display indicates the suggested denom that should be displayed in clients.
    pub display: String,
    /// Name defines the name of the token (eg: Cosmos Atom)
    pub name: String,
    /// Symbol is the token symbol usually shown on exchanges (eg: ATOM).
    /// This can be the same as the display.
    pub symbol: String,
}

#[cw_serde]
pub struct FullDenomResponse {
    pub denom: String,
}

#[cw_serde]
pub struct AdminResponse {
    pub admin: String,
}

#[cw_serde]
pub struct MetadataResponse {
    pub metadata: Option<DenomMetadata>,
}

#[cw_serde]
pub struct DenomsByCreatorResponse {
    pub denoms: Vec<String>,
}

#[cw_serde]
pub struct TokenParamsResponse {
    pub params: TokenParams,
}

#[cw_serde]
pub struct TokenParams {
    pub denom_creation_fee: Vec<DenomCreationFee>,
}

#[cw_serde]
pub struct DenomCreationFee {
    pub amount: Uint256,
    pub denom: String,
}

pub trait CreateTokenFactoryMsg: From<TokenFactoryMsg> {
    fn token_factory_create_denom(
        subdenom: String,
        metadata: Option<DenomMetadata>,
    ) -> StdResult<Self> {
        Ok(TokenFactoryMsg::CreateDenom { subdenom, metadata }.into())
    }
    fn token_factory_change_admin(denom: String, new_admin_address: Addr) -> StdResult<Self> {
        Ok(TokenFactoryMsg::ChangeAdmin {
            denom,
            new_admin_address,
        }
        .into())
    }
    fn token_factory_mint_tokens(
        denom: String,
        amount: Uint256,
        mint_to_address: Addr,
    ) -> StdResult<Self> {
        Ok(TokenFactoryMsg::MintTokens {
            denom,
            amount,
            mint_to_address,
        }
        .into())
    }
    fn token_factory_burn_tokens(
        denom: String,
        amount: Uint256,
        burn_from_address: Addr,
    ) -> StdResult<Self> {
        Ok(TokenFactoryMsg::BurnTokens {
            denom,
            amount,
            burn_from_address,
        }
        .into())
    }
    fn token_factory_set_metadata(metadata: DenomMetadata) -> StdResult<Self> {
        Ok(TokenFactoryMsg::SetMetadata { metadata }.into())
    }
    fn token_factory_force_transfer(
        denom: String,
        from_address: Addr,
        to_address: Addr,
        amount: Uint256,
    ) -> StdResult<Self> {
        Ok(TokenFactoryMsg::ForceTransfer {
            denom,
            from_address,
            to_address,
            amount,
        }
        .into())
    }
}

impl<T> CreateTokenFactoryMsg for T where T: From<TokenFactoryMsg> {}

pub trait TokenFactoryQuerier {
    fn query_token_factory_full_denom(
        &self,
        subdenom: String,
        creator_addr: Addr,
    ) -> StdResult<FullDenomResponse>;

    fn query_token_factory_admin(&self, denom: String) -> StdResult<AdminResponse>;

    fn query_token_factory_metadata(&self, denom: String) -> StdResult<MetadataResponse>;

    fn query_token_factory_denoms_by_creator(
        &self,
        creator: Addr,
    ) -> StdResult<DenomsByCreatorResponse>;

    fn query_token_factory_params(&self) -> StdResult<TokenParamsResponse>;
}

impl<'a, T> TokenFactoryQuerier for QuerierWrapper<'a, T>
where
    T: CustomQuery + From<TokenFactoryQuery>,
{
    fn query_token_factory_full_denom(
        &self,
        subdenom: String,
        creator_addr: Addr,
    ) -> StdResult<FullDenomResponse> {
        let custom_query: T = TokenFactoryQuery::FullDenom {
            subdenom,
            creator_addr,
        }
        .into();
        self.query(&custom_query.into())
    }

    fn query_token_factory_admin(&self, denom: String) -> StdResult<AdminResponse> {
        let custom_query: T = TokenFactoryQuery::Admin { denom }.into();
        self.query(&custom_query.into())
    }

    fn query_token_factory_metadata(&self, denom: String) -> StdResult<MetadataResponse> {
        let custom_query: T = TokenFactoryQuery::Metadata { denom }.into();
        self.query(&custom_query.into())
    }

    fn query_token_factory_denoms_by_creator(
        &self,
        creator: Addr,
    ) -> StdResult<DenomsByCreatorResponse> {
        let custom_query: T = TokenFactoryQuery::DenomsByCreator { creator }.into();
        self.query(&custom_query.into())
    }

    fn query_token_factory_params(&self) -> StdResult<TokenParamsResponse> {
        let custom_query: T = TokenFactoryQuery::Params {}.into();
        self.query(&custom_query.into())
    }
}

// This export is added to all contracts that import this package, signifying that they require
// "token_factory" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_token_factory() {}
