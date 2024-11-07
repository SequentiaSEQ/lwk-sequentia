use crate::elements::{AddressParams, AssetId};
use crate::error::Error;
use std::str::FromStr;

const SEQUENTIA_POLICY_ASSET_STR: &str =
    "6f0279e9ed041c3d710a9f57d0c02928416460c4b722ae3457a11eec381c526d";
const SEQUENTIA_TESTNET_POLICY_ASSET_STR: &str =
    "c8eccacf0953e1931cd31e434d8319101cc36e6c38b0e2104d8687552fae3e40";

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum ElementsNetwork {
    Sequentia,
    SequentiaTestnet,
    ElementsRegtest { policy_asset: AssetId },
}

impl ElementsNetwork {
    pub fn policy_asset(&self) -> AssetId {
        match self {
            ElementsNetwork::Sequentia => {
                AssetId::from_str(SEQUENTIA_POLICY_ASSET_STR).expect("can't fail on const")
            }
            ElementsNetwork::SequentiaTestnet => {
                AssetId::from_str(SEQUENTIA_TESTNET_POLICY_ASSET_STR).expect("can't fail on const")
            }
            ElementsNetwork::ElementsRegtest { policy_asset } => *policy_asset,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ElementsNetwork::Sequentia => "mainnet",
            ElementsNetwork::SequentiaTestnet => "testnet",
            ElementsNetwork::ElementsRegtest { .. } => "elementsregtest",
        }
    }

    pub fn address_params(&self) -> &'static AddressParams {
        match self {
            ElementsNetwork::Sequentia => &AddressParams::SEQUENTIA,
            ElementsNetwork::SequentiaTestnet => &AddressParams::SEQUENTIA_TESTNET,
            ElementsNetwork::ElementsRegtest { .. } => &AddressParams::ELEMENTS,
        }
    }

    pub fn default_regtest() -> ElementsNetwork {
        let policy_asset =
            AssetId::from_str("5ac9f65c0efcc4775e0baec4ec03abdde22473cd3cf33c0419ca290e0751b225")
                .expect("static");

        ElementsNetwork::ElementsRegtest { policy_asset }
    }

    /// Return the dynamic epoch length of this network
    pub fn dynamic_epoch_length(&self) -> u32 {
        // taken from elements chainparams.cpp
        // TODO upstream to rust elements
        match self {
            ElementsNetwork::Sequentia => 20160,
            ElementsNetwork::SequentiaTestnet => 1000,
            ElementsNetwork::ElementsRegtest { policy_asset: _ } => 10,
        }
    }

    /// Return the dynamic epoch length of this network
    pub fn total_valid_epochs(&self) -> u32 {
        // taken from elements chainparams.cpp
        // TODO upstream to rust elements
        match self {
            ElementsNetwork::Sequentia => 2,
            ElementsNetwork::SequentiaTestnet => 0,
            ElementsNetwork::ElementsRegtest { policy_asset: _ } => 0,
        }
    }

    #[cfg(feature = "bindings")]
    pub fn tx_builder(&self) -> crate::TxBuilder {
        crate::TxBuilder::new(*self)
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Config {
    network: ElementsNetwork,
}

impl Config {
    pub fn new(network: ElementsNetwork) -> Result<Self, Error> {
        Ok(Config { network })
    }

    pub fn address_params(&self) -> &'static AddressParams {
        self.network.address_params()
    }

    pub fn policy_asset(&self) -> AssetId {
        self.network.policy_asset()
    }

    pub fn network(&self) -> ElementsNetwork {
        self.network
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use super::Config;

    #[test]
    fn test_config_hash() {
        let config = Config::new(crate::ElementsNetwork::Sequentia).unwrap();
        let mut hasher = DefaultHasher::new();
        config.hash(&mut hasher);
        assert_eq!(13646096770106105413, hasher.finish());
    }
}
