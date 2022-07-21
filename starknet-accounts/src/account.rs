use crate::Call;

use async_trait::async_trait;
use starknet_core::types::{AddTransactionResult, BlockId, FeeEstimate, FieldElement};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct AttachedAccountCall<'a, A> {
    pub calls: Vec<Call>,
    pub nonce: Option<FieldElement>,
    pub max_fee: Option<FieldElement>,
    pub(crate) account: &'a A,
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

    async fn execute(&self, calls: &[Call]) -> Result<AttachedAccountCall<Self>, Self::SignTransactionError>;

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

impl<'a, A> AttachedAccountCall<'a, A>
where
    A: Account + Sync,
{
    pub async fn estimate_fee(&self) -> Result<FeeEstimate, A::SignTransactionError> {
        self.account.estimate_fee(self).await
    }

    pub async fn send(&self) -> Result<AddTransactionResult, A::SendTransactionError> {
        self.account.send_transaction(self).await
    }
}

impl<'a, A> AccountCall for AttachedAccountCall<'a, A> {
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
            account: self.account,
            transaction_hash: self.transaction_hash
        }
    }

    fn max_fee(self, max_fee: FieldElement) -> Self {
        Self {
            calls: self.calls,
            nonce: self.nonce,
            max_fee: Some(max_fee),
            account: self.account,
            transaction_hash: self.transaction_hash
        }
    }
}
