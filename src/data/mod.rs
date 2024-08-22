use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Address {
    #[serde(rename = "Address")]
    pub(crate) address: u8,

    #[serde(rename = "BitRange")]
    pub(crate) bit_range: String,

    #[serde(rename = "PrivateKeyRange")]
    pub(crate) private_key_range: String,

    #[serde(rename = "PrivateKeyRangeStart")]
    pub(crate) private_key_range_start: String,

    #[serde(rename = "PrivateKeyRangeEnd")]
    pub(crate) private_key_range_end: String,

    #[serde(rename = "PrivateKey(HEX)")]
    pub(crate) private_key_hex: String,

    #[serde(rename = "PublicKey(HEX)")]
    pub(crate) public_key_hex: String,

    #[serde(rename = "BitcoinAddress")]
    pub(crate) bitcoin_address: String,

    #[serde(rename = "PercentOfRange")]
    pub(crate) percent_of_range: f32,

    #[serde(rename = "ResolutionDate")]
    pub(crate) resolution_date: String,

    #[serde(rename = "Solver")]
    pub(crate) solver: String,

    #[serde(rename = "Solved")]
    pub(crate) solved: bool,
}
