use crate::Call;

use async_trait::async_trait;
use starknet_core::types::{AddTransactionResult, BlockId, FeeEstimate, FieldElement};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct AttachedTxInfoCall {
    pub calls: Vec<Call>,
    pub nonce: Option<FieldElement>,
    pub max_fee: Option<FieldElement>,
    pub transaction_hash: FieldElement,
}

pub trait AccountCall {
    fn get_calls(&self) -> &[Call];

    fn get_nonce(&self) -> &Option<FieldElement>;

    fn get_max_fee(&self) -> &Option<FieldElement>;

    fn nonce(self, nonce: FieldElement) -> Self;

    fn max_fee(self, max_fee: FieldElement) -> Self;
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait Account: Sized {
    type GetNonceError: Error + Send;
    type SignTransactionError: Error + Send;
    type SendTransactionError: Error + Send;

    async fn get_nonce(
        &self,
        block_identifier: BlockId,
    ) -> Result<FieldElement, Self::GetNonceError>;

    async fn execute(&self, calls: &[Call]) -> Result<AttachedTxInfoCall, Self::SignTransactionError>;

    async fn estimate_fee<C>(&self, call: &C) -> Result<FeeEstimate, Self::SignTransactionError>
    where
        C: AccountCall + Sync;

    async fn send_transaction<C>(
        &self,
        call: &C,
    ) -> Result<AddTransactionResult, Self::SendTransactionError>
    where
        C: AccountCall + Sync;
}

impl AccountCall for AttachedTxInfoCall {
    fn get_calls(&self) -> &[Call] {
        &self.calls
    }

    fn get_nonce(&self) -> &Option<FieldElement> {
        &self.nonce
    }

    fn get_max_fee(&self) -> &Option<FieldElement> {
        &self.max_fee
    }

    fn nonce(self, nonce: FieldElement) -> Self {
        Self {
            calls: self.calls,
            nonce: Some(nonce),
            max_fee: self.max_fee,
            transaction_hash: self.transaction_hash
        }
    }

    fn max_fee(self, max_fee: FieldElement) -> Self {
        Self {
            calls: self.calls,
            nonce: self.nonce,
            max_fee: Some(max_fee),
            transaction_hash: self.transaction_hash
        }
    }
}
