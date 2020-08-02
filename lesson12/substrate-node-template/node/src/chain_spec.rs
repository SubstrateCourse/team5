use sp_core::{Pair, Public, crypto::UncheckedInto, sr25519};
use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::ChainType;
use hex_literal::hex;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Aura
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		|| testnet_genesis(
			vec![
				authority_keys_from_seed("Alice"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
			true,
		),
		vec![],
		None,
		None,
		None,
		None,
	)
}

pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		|| testnet_genesis(
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			true,
		),
		vec![],
		None,
		None,
		None,
		None,
	)
}

// public staging network
pub fn insect_staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"Insect Staging Testnet",
		"insect_staging",
		ChainType::Live,
		|| testnet_genesis(
			// for i in 1 2; do for j in aura; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
			// and
			// for i in 1 2; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
			vec![(
				// 5GgY6McwhdxsRNitNBRauHb2vTvZ1JZ2QvBtHUVmCprvPKxT
				hex!["cc44e658aafd6c2472c3ecf6d99a8d1abad0e70127041597c0ec37a5f5ef0f4b"].unchecked_into(),
				// 5HSzP5V6gQYCnvjZnBEZCE9NaJmEEnEbisZYKyWMSRsUYhLL
				hex!["ee2c312128765da1fbb6cf79b99d09c3a10be8e68bf62f032b89eca86aa38a5a"].unchecked_into(),
			),(
				// 5Cwnmec9rnek11RfNgvhAuJ84xWGCNfgyfT6ZomMrCr2B8QF
				hex!["d08105ca0ad4117f9069f9b3a4fdbdb84a5791ec893c2ac85cdc9329a293d0e4"].unchecked_into(),
				// 5CXY9uFexop6877F9bGLSzjfVWh75somcebSZQmxCfM8VF5t
				hex!["25fcb8aa9a10812c6f64b50dadb6920da4359ad371a0dcd5744d4267afe030f7"].unchecked_into(),
			)],
			// subkey inspect "$SECRET//insect"
			hex![
				// 5DAbwPGEg2CreNdB6TNcqLRytW4Yqt2pD4sdF5SwjVm8gRg3
				"30bbc82344f39b0f1b74220c2f75981985d9e9a1accd0c247074e09e5d6b7915"
			].into(),
			vec![
				// 5DAbwPGEg2CreNdB6TNcqLRytW4Yqt2pD4sdF5SwjVm8gRg3
				hex!["30bbc82344f39b0f1b74220c2f75981985d9e9a1accd0c247074e09e5d6b7915"].into(),
				// 5HSx9MgMe93Y88PaMDH2ffcMj5kM7uWaaoVrkdsPqJhF6Mht
				hex!["ee24a9b7e14623764387c9ad67d52c24f890d93f6b074bf63780d28397b25d7f"].into(),
			],
			true,
		),
		boot_nodes,
		None,
		None,
		None,
		None,
	)
}

fn testnet_genesis(initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool) -> GenesisConfig {
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
	}
}
