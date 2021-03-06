// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    executor_proxy::ExecutorProxyTrait, state_synchronizer::SynchronizationState,
    tests::mock_storage::MockStorage,
};
use anyhow::{format_err, Result};
use diem_config::config::HANDSHAKE_VERSION;
use diem_crypto::{hash::ACCUMULATOR_PLACEHOLDER_HASH, test_utils::TEST_SEED, x25519, Uniform};
use diem_infallible::RwLock;
use diem_network_address::{
    encrypted::{TEST_SHARED_VAL_NETADDR_KEY, TEST_SHARED_VAL_NETADDR_KEY_VERSION},
    NetworkAddress, Protocol,
};
use diem_types::{
    contract_event::ContractEvent, ledger_info::LedgerInfoWithSignatures,
    on_chain_config::ValidatorSet, proof::TransactionListProof,
    transaction::TransactionListWithProof, validator_config::ValidatorConfig,
    validator_info::ValidatorInfo, validator_signer::ValidatorSigner,
    validator_verifier::random_validator_verifier,
};
use memsocket::MemoryListener;
use rand::{rngs::StdRng, SeedableRng};
use std::sync::Arc;

pub(crate) struct SynchronizerEnvHelper;

impl SynchronizerEnvHelper {
    // Returns the initial peers with their signatures
    pub(crate) fn initial_setup(
        count: usize,
    ) -> (
        Vec<ValidatorSigner>,
        Vec<ValidatorInfo>,
        Vec<x25519::PrivateKey>,
        Vec<NetworkAddress>,
    ) {
        let (signers, _verifier) = random_validator_verifier(count, None, true);

        // Setup identity public keys.
        let mut rng = StdRng::from_seed(TEST_SEED);
        let network_keys: Vec<_> = (0..count)
            .map(|_| x25519::PrivateKey::generate(&mut rng))
            .collect();

        let mut validator_infos = vec![];
        let mut network_addrs = vec![];

        for (idx, signer) in signers.iter().enumerate() {
            let peer_id = signer.author();

            // Reserve an unused `/memory/<port>` address by binding port 0; we
            // can immediately discard the listener here and safely rebind to this
            // address later.
            let port = MemoryListener::bind(0).unwrap().local_addr();
            let addr = NetworkAddress::from(Protocol::Memory(port));
            let addr = addr.append_prod_protos(network_keys[idx].public_key(), HANDSHAKE_VERSION);

            let enc_addr = addr.clone().encrypt(
                &TEST_SHARED_VAL_NETADDR_KEY,
                TEST_SHARED_VAL_NETADDR_KEY_VERSION,
                &peer_id,
                0, /* seq_num */
                0, /* addr_idx */
            );

            // The voting power of peer 0 is enough to generate an LI that passes validation.
            let voting_power = if idx == 0 { 1000 } else { 1 };
            let validator_config = ValidatorConfig::new(
                signer.public_key(),
                bcs::to_bytes(&vec![enc_addr.unwrap()]).unwrap(),
                bcs::to_bytes(&vec![addr.clone()]).unwrap(),
            );
            let validator_info = ValidatorInfo::new(peer_id, voting_power, validator_config);
            validator_infos.push(validator_info);
            network_addrs.push(addr);
        }
        (signers, validator_infos, network_keys, network_addrs)
    }

    pub(crate) fn genesis_li(validators: &[ValidatorInfo]) -> LedgerInfoWithSignatures {
        LedgerInfoWithSignatures::genesis(
            *ACCUMULATOR_PLACEHOLDER_HASH,
            ValidatorSet::new(validators.to_vec()),
        )
    }
}

pub(crate) type MockRpcHandler = Box<
    dyn Fn(TransactionListWithProof) -> Result<TransactionListWithProof> + Send + Sync + 'static,
>;

pub(crate) struct MockExecutorProxy {
    handler: MockRpcHandler,
    storage: Arc<RwLock<MockStorage>>,
}

impl MockExecutorProxy {
    pub(crate) fn new(handler: MockRpcHandler, storage: Arc<RwLock<MockStorage>>) -> Self {
        Self { handler, storage }
    }
}

impl ExecutorProxyTrait for MockExecutorProxy {
    fn get_local_storage_state(&self) -> Result<SynchronizationState> {
        Ok(self.storage.read().get_local_storage_state())
    }

    fn execute_chunk(
        &mut self,
        txn_list_with_proof: TransactionListWithProof,
        ledger_info_with_sigs: LedgerInfoWithSignatures,
        intermediate_end_of_epoch_li: Option<LedgerInfoWithSignatures>,
    ) -> Result<()> {
        self.storage.write().add_txns_with_li(
            txn_list_with_proof.transactions,
            ledger_info_with_sigs,
            intermediate_end_of_epoch_li,
        );
        Ok(())
    }

    fn get_chunk(
        &self,
        known_version: u64,
        limit: u64,
        target_version: u64,
    ) -> Result<TransactionListWithProof> {
        let start_version = known_version
            .checked_add(1)
            .ok_or_else(|| format_err!("Known version too high"))?;
        let txns = self
            .storage
            .read()
            .get_chunk(start_version, limit, target_version);
        let first_txn_version = txns.first().map(|_| start_version);
        let txns_with_proof = TransactionListWithProof::new(
            txns,
            None,
            first_txn_version,
            TransactionListProof::new_empty(),
        );
        (self.handler)(txns_with_proof)
    }

    fn get_epoch_proof(&self, epoch: u64) -> Result<LedgerInfoWithSignatures> {
        self.storage.read().get_epoch_changes(epoch)
    }

    fn get_epoch_ending_ledger_info(&self, version: u64) -> Result<LedgerInfoWithSignatures> {
        self.storage.read().get_epoch_ending_ledger_info(version)
    }

    fn get_version_timestamp(&self, _version: u64) -> Result<u64> {
        // Only used for logging purposes so no point in mocking
        Ok(0)
    }

    fn publish_on_chain_config_updates(&mut self, _events: Vec<ContractEvent>) -> Result<()> {
        Ok(())
    }
}
