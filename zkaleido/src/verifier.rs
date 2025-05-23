use std::fmt::Debug;

use borsh::BorshDeserialize;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    ProofReceipt, PublicValues, VerifyingKey, VerifyingKeyCommitment, ZkVmError, ZkVmProofError,
    ZkVmResult,
};

/// A trait implemented by the prover ("host") of a zkVM program.
pub trait ZkVmVerifier: Send + Sync + Clone + Debug + 'static {
    /// The proof receipt type, specific to this host, that can be
    /// converted to and from a generic [`ProofReceipt`].
    ///
    /// This allows flexibility for different proof systems or proof representations
    /// while still providing a way to convert back to a standard [`ProofReceipt`].
    type ZkVmProofReceipt: TryFrom<ProofReceipt, Error = ZkVmProofError>;

    /// Returns the Verification key for the loaded program
    fn vk(&self) -> VerifyingKey;

    /// Returns the commitment of the verification key for the loaded program
    fn vk_commitment(&self) -> VerifyingKeyCommitment;

    /// Extracts the public output from the public values using ZkVm's `serde`
    /// serialization/deserialization.
    fn extract_serde_public_output<T: Serialize + DeserializeOwned>(
        public_values: &PublicValues,
    ) -> ZkVmResult<T>;

    /// Extracts the public output from the given proof assuming the data was serialized using
    /// Borsh.
    fn extract_borsh_public_output<T: BorshDeserialize>(
        public_values: &PublicValues,
    ) -> ZkVmResult<T> {
        borsh::from_slice(public_values.as_bytes())
            .map_err(|e| ZkVmError::OutputExtractionError { source: e.into() })
    }

    /// Verifies the proof generated by the ZkVm
    fn verify_inner(&self, proof: &Self::ZkVmProofReceipt) -> ZkVmResult<()>;

    /// Verifies the [`ProofReceipt`]
    fn verify(&self, proof: &ProofReceipt) -> ZkVmResult<()> {
        self.verify_inner(&proof.clone().try_into()?)
    }
}
