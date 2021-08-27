use std::fmt::Debug;

use thiserror::Error;

use casper_execution_engine::{
    core::engine_state, shared::stored_value::StoredValue, storage::trie::Trie,
};
use casper_types::{EraId, Key, ProtocolVersion, PublicKey, U512};

use crate::{
    components::{contract_runtime::BlockExecutionError, fetcher::FetcherError},
    crypto,
    types::{
        Block, BlockHash, BlockHeader, BlockHeaderWithMetadata, BlockSignatures, BlockWithMetadata,
        Deploy,
    },
};
use num::rational::Ratio;
use std::collections::BTreeMap;

#[derive(Error, Debug)]
pub(crate) enum FinalitySignatureError {
    #[error(
        "Block signatures do not correspond to block header. \
         block header: {block_header:?} \
         block hash: {block_hash:?} \
         block signatures: {block_signatures:?}"
    )]
    SignaturesDoNotCorrespondToBlockHeader {
        block_header: Box<BlockHeader>,
        block_hash: Box<BlockHash>,
        block_signatures: Box<BlockSignatures>,
    },

    #[error(transparent)]
    CryptoError(#[from] crypto::Error),

    #[error(
        "Block signatures contain bogus validator. \
         trusted validator weights: {trusted_validator_weights:?}, \
         block signatures: {block_signatures:?}, \
         bogus validator public key: {bogus_validator_public_key:?}"
    )]
    BogusValidator {
        trusted_validator_weights: BTreeMap<PublicKey, U512>,
        block_signatures: Box<BlockSignatures>,
        bogus_validator_public_key: Box<PublicKey>,
    },

    #[error(
        "Insufficient weight for finality. \
         trusted validator weights: {trusted_validator_weights:?}, \
         block signatures: {block_signatures:?}, \
         signature weight: {signature_weight}, \
         total validator weight: {total_validator_weight}, \
         finality threshold fraction: {finality_threshold_fraction}"
    )]
    InsufficientWeightForFinality {
        trusted_validator_weights: BTreeMap<PublicKey, U512>,
        block_signatures: Box<BlockSignatures>,
        signature_weight: Box<U512>,
        total_validator_weight: Box<U512>,
        finality_threshold_fraction: Ratio<u64>,
    },
}

#[derive(Error, Debug)]
pub(crate) enum LinearChainSyncError<I>
where
    I: Eq + Debug + 'static,
{
    #[error(transparent)]
    ExecutionEngineError(#[from] engine_state::Error),

    #[error(
        "Cannot get trusted validators for such an early era. \
         trusted header: {trusted_header:?}, \
         last emergency restart era id: {maybe_last_emergency_restart_era_id:?}"
    )]
    TrustedHeaderEraTooEarly {
        trusted_header: Box<BlockHeader>,
        maybe_last_emergency_restart_era_id: Option<EraId>,
    },

    #[error(
        "Current version is {current_version}, but retrieved block header with future version: \
         {block_header_with_future_version:?}"
    )]
    RetrievedBlockHeaderFromFutureVersion {
        current_version: ProtocolVersion,
        block_header_with_future_version: Box<BlockHeader>,
    },
    #[error(transparent)]
    BlockHeaderFetcherError(#[from] FetcherError<BlockHeader, I>),

    #[error(transparent)]
    BlockHeaderWithMetadataFetcherError(#[from] FetcherError<BlockHeaderWithMetadata, I>),

    #[error(transparent)]
    BlockWithMetadataFetcherError(#[from] FetcherError<BlockWithMetadata, I>),

    #[error(transparent)]
    DeployWithMetadataFetcherError(#[from] FetcherError<Deploy, I>),

    #[error(transparent)]
    TrieFetcherError(#[from] FetcherError<Trie<Key, StoredValue>, I>),

    #[error(
        "Executed block is not the same as downloaded block. \
         Executed block: {executed_block:?}, \
         Downloaded block: {downloaded_block:?}"
    )]
    ExecutedBlockIsNotTheSameAsDownloadedBlock {
        executed_block: Box<Block>,
        downloaded_block: Box<Block>,
    },

    #[error(transparent)]
    BlockExecutionError(#[from] BlockExecutionError),

    #[error(
        "Joining with trusted hash before emergency restart not supported. \
         Find a more recent hash from after the restart. \
         Last emergency restart era: {last_emergency_restart_era}, \
         Trusted hash: {trusted_hash:?}, \
         Trusted block header: {trusted_block_header:?}"
    )]
    TryingToJoinBeforeLastEmergencyRestartEra {
        last_emergency_restart_era: EraId,
        trusted_hash: BlockHash,
        trusted_block_header: Box<BlockHeader>,
    },

    #[error(
        "Can't download block before genesis. \
         Genesis block header: {genesis_block_header}"
    )]
    CantDownloadBlockBeforeGenesis {
        genesis_block_header: Box<BlockHeader>,
    },
}