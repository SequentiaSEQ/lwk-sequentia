use std::fmt;
use lwk_wollet::HTLC;

#[derive(uniffi::Object, PartialEq, Eq, Debug)]
#[uniffi::export(Display)]
pub struct HTLCScript {
    inner:HTLC,
}

impl From<HTLC> for HTLCScript {
    fn from(inner: HTLC) -> Self {
        Self { inner }
    }
}

impl fmt::Display for HTLCScript {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[uniffi::export]
impl HTLCScript {
    pub fn address(&self) -> String {
        self.inner.address.to_string()
    }

    pub fn redeem_script(&self) -> Vec<u8> {
        self.inner.redeem_script.to_vec()
    }

    pub fn seed_hash(&self) -> Vec<u8> {
        self.inner.seed_hash.to_vec()
    }

    pub fn seed(&self) -> Option<Vec<u8>> {
        self.inner.seed.clone().map(|s| s.to_vec())
    }
}