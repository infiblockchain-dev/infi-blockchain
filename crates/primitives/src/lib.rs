use std::fmt;
use std::str::FromStr;

pub const INVERTX_DECIMALS: u8 = 18;
pub const INFI_TESTNET_CHAIN_ID: u64 = 98_402;
pub const DEVNET_CHAIN_ID: u64 = INFI_TESTNET_CHAIN_ID;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChainConfig {
    pub chain_name: String,
    pub chain_id: u64,
    pub native_token: NativeToken,
    pub block_time_ms: u64,
}

impl ChainConfig {
    pub fn devnet() -> Self {
        Self {
            chain_name: "INFI Testnet".to_string(),
            chain_id: INFI_TESTNET_CHAIN_ID,
            native_token: NativeToken::test_invertx(),
            block_time_ms: 1_000,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeToken {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

impl NativeToken {
    pub fn invertx() -> Self {
        Self {
            name: "Invertx".to_string(),
            symbol: "TBD".to_string(),
            decimals: INVERTX_DECIMALS,
        }
    }

    pub fn test_invertx() -> Self {
        Self {
            name: "test InvertX".to_string(),
            symbol: "tInvertX".to_string(),
            decimals: INVERTX_DECIMALS,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Address(pub [u8; 20]);

impl Address {
    pub const ZERO: Self = Self([0_u8; 20]);

    pub fn repeat(byte: u8) -> Self {
        Self([byte; 20])
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl FromStr for Address {
    type Err = AddressParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let hex = value.strip_prefix("0x").unwrap_or(value);
        if hex.len() != 40 {
            return Err(AddressParseError::WrongLength(hex.len()));
        }

        let mut bytes = [0_u8; 20];
        for index in 0..20 {
            let start = index * 2;
            let end = start + 2;
            bytes[index] = u8::from_str_radix(&hex[start..end], 16)
                .map_err(|_| AddressParseError::InvalidHex)?;
        }

        Ok(Self(bytes))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AddressParseError {
    WrongLength(usize),
    InvalidHex,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Amount(pub u128);

impl Amount {
    pub const ZERO: Self = Self(0);

    pub fn from_invertx_units(units: u128) -> Self {
        Self(units)
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Account {
    pub address: Address,
    pub balance: Amount,
    pub nonce: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transaction {
    pub from: Address,
    pub to: Option<Address>,
    pub value: Amount,
    pub gas_limit: u64,
    pub gas_price: Amount,
    pub nonce: u64,
    pub input: Vec<u8>,
}

impl Transaction {
    pub fn simple_transfer(from: Address, to: Address, value: Amount, nonce: u64) -> Self {
        Self {
            from,
            to: Some(to),
            value,
            gas_limit: 21_000,
            gas_price: Amount(1),
            nonce,
            input: Vec::new(),
        }
    }

    pub fn fee(&self) -> Amount {
        Amount(self.gas_limit as u128 * self.gas_price.0)
    }

    pub fn hash(&self) -> Hash {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.from.0);
        if let Some(to) = self.to {
            bytes.extend_from_slice(&to.0);
        } else {
            bytes.extend_from_slice(&Address::ZERO.0);
        }
        bytes.extend_from_slice(&self.value.0.to_be_bytes());
        bytes.extend_from_slice(&self.gas_limit.to_be_bytes());
        bytes.extend_from_slice(&self.gas_price.0.to_be_bytes());
        bytes.extend_from_slice(&self.nonce.to_be_bytes());
        bytes.extend_from_slice(&self.input);
        Hash::from_bytes(&bytes)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockHeader {
    pub parent_hash: Hash,
    pub number: u64,
    pub state_root: Hash,
    pub tx_root: Hash,
    pub proposer: Address,
    pub timestamp_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    pub const ZERO: Self = Self([0_u8; 32]);

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut output = [0_u8; 32];
        let mut state = [
            0x6a09_e667_f3bc_c908_u64,
            0xbb67_ae85_84ca_a73b_u64,
            0x3c6e_f372_fe94_f82b_u64,
            0xa54f_f53a_5f1d_36f1_u64,
        ];

        for (index, byte) in bytes.iter().enumerate() {
            let lane = index % state.len();
            state[lane] ^= *byte as u64;
            state[lane] = state[lane]
                .rotate_left(5)
                .wrapping_mul(0x100_0000_01b3)
                .wrapping_add(index as u64);
        }

        for (index, lane) in state.iter().enumerate() {
            output[index * 8..index * 8 + 8].copy_from_slice(&lane.to_be_bytes());
        }

        Self(output)
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}
