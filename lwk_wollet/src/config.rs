use crate::elements::{AddressParams, AssetId};
use crate::error::Error;
use std::str::FromStr;

const LIQUID_POLICY_ASSET_STR: &str =
    "6f0279e9ed041c3d710a9f57d0c02928416460c4b722ae3457a11eec381c526d";
const LIQUID_TESTNET_POLICY_ASSET_STR: &str =
    "144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49";

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum ElementsNetwork {
    Sequentia,
    SequentiaTestnet,
    SequentiaRegtest { policy_asset: AssetId },
}

impl ElementsNetwork {
    pub fn policy_asset(&self) -> AssetId {
        match self {
            ElementsNetwork::Sequentia => {
                AssetId::from_str(LIQUID_POLICY_ASSET_STR).expect("can't fail on const")
            }
            ElementsNetwork::SequentiaTestnet => {
                AssetId::from_str(LIQUID_TESTNET_POLICY_ASSET_STR).expect("can't fail on const")
            }
            ElementsNetwork::SequentiaRegtest { policy_asset } => *policy_asset,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ElementsNetwork::Sequentia => "liquid",
            ElementsNetwork::SequentiaTestnet => "liquid-testnet",
            ElementsNetwork::SequentiaRegtest { .. } => "liquid-regtest",
        }
    }

    pub fn address_params(&self) -> &'static AddressParams {
        match self {
            ElementsNetwork::Sequentia => &AddressParams::SEQUENTIA,
            ElementsNetwork::SequentiaTestnet => &AddressParams::SEQUENTIA_TESTNET,
            ElementsNetwork::SequentiaRegtest { .. } => &AddressParams::SEQUENTIA_REGTEST,
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
