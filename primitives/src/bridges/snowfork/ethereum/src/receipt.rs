use crate::{Bloom, Log};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

#[derive(Clone, Default, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Receipt {
    pub post_state_or_status: Vec<u8>,
    pub cumulative_gas_used: u64,
    pub bloom: Bloom,
    pub logs: Vec<Log>,
}

impl Receipt {
    pub fn contains_log(&self, log: &Log) -> bool {
        self.logs.iter().any(|l| l == log)
    }

    fn decode_list(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let mut iter = rlp.iter();

        let post_state_or_status: Vec<u8> = match iter.next() {
            Some(data) => data.as_val()?,
            None =>
                return Err(rlp::DecoderError::Custom(
                    "Expected receipt post state or status",
                )),
        };

        let cumulative_gas_used: u64 = match iter.next() {
            Some(data) => data.as_val()?,
            None =>
                return Err(rlp::DecoderError::Custom(
                    "Expected receipt cumulative gas used",
                )),
        };

        let bloom: Bloom = match iter.next() {
            Some(data) => data.as_val()?,
            None => return Err(rlp::DecoderError::Custom("Expected receipt bloom")),
        };

        let logs: Vec<Log> = match iter.next() {
            Some(data) => data.as_list()?,
            None => return Err(rlp::DecoderError::Custom("Expected receipt logs")),
        };

        Ok(Self {
            post_state_or_status,
            cumulative_gas_used,
            bloom,
            logs,
        })
    }
}

impl rlp::Decodable for Receipt {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        if rlp.is_data() {
            // Typed receipt
            let data = rlp.as_raw();
            match data[0] {
                // 1 = EIP-2930, 2 = EIP-1559
                1 | 2 => {
                    let receipt_rlp = &rlp::Rlp::new(&data[1..]);
                    if !receipt_rlp.is_list() {
                        return Err(rlp::DecoderError::RlpExpectedToBeList)
                    }
                    Self::decode_list(&rlp::Rlp::new(&data[1..]))
                },
                _ => Err(rlp::DecoderError::Custom("Unsupported receipt type")),
            }
        } else if rlp.is_list() {
            // Legacy receipt
            Self::decode_list(rlp)
        } else {
            Err(rlp::DecoderError::RlpExpectedToBeList)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Receipt;
    use hex_literal::hex;

    const RAW_RECEIPT: [u8; 1242] = hex!(
        "
		f904d701830652f0b901000420000000000000000000008002000000000001000000000001000000
		00000000000000000000000000000000000000020000000800000000000000002000000000000000
		00000000000008000000220000000000400010000000000000000000000000000000000000000000
		00000000000000000004000000001000010000000000080000000000400000000000000000000000
		00000800000040000000000200000000000200000000000000000000000000000000000000000000
		04000000000002000000000100000000000000000000000000001000000002000020000010200000
		000000010000000000000000000000000000000000000010000000f903ccf89b9421130f34829b4c
		343142047a28ce96ec07814b15f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a116
		28f55a4df523b3efa00000000000000000000000007d843005c7433c16b27ff939cb37471541561e
		bda0000000000000000000000000e9c1281aae66801fa35ec404d5f2aea393ff6988a00000000000
		0000000000000000000000000000000000000000000005d09b7380f89b9421130f34829b4c343142
		047a28ce96ec07814b15f863a08c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200a
		c8c7c3b925a00000000000000000000000007d843005c7433c16b27ff939cb37471541561ebda000
		00000000000000000000007a250d5630b4cf539739df2c5dacb4c659f2488da0ffffffffffffffff
		ffffffffffffffffffffffffffffffffffffffcc840c6920f89b94c02aaa39b223fe8d0a0e5c4f27
		ead9083c756cc2f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523
		b3efa0000000000000000000000000e9c1281aae66801fa35ec404d5f2aea393ff6988a000000000
		00000000000000007a250d5630b4cf539739df2c5dacb4c659f2488da00000000000000000000000
		0000000000000000000000000003e973b5a5d1078ef87994e9c1281aae66801fa35ec404d5f2aea3
		93ff6988e1a01c411e9a96e071241c2f21f7726b17ae89e3cab4c78be50e062b03a9fffbbad1b840
		000000000000000000000000000000000000000000000000000001f1420ad1d40000000000000000
		000000000000000000000000000000014ad400879d159a38f8fc94e9c1281aae66801fa35ec404d5
		f2aea393ff6988f863a0d78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159
		d822a00000000000000000000000007a250d5630b4cf539739df2c5dacb4c659f2488da000000000
		00000000000000007a250d5630b4cf539739df2c5dacb4c659f2488db88000000000000000000000
		000000000000000000000000000000000005d415f332000000000000000000000000000000000000
		00000000000000000000000000000000000000000000000000000000000000000000000000000000
		00000000000000000000000000000000000000000000000000000000000003e973b5a5d1078ef87a
		94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f842a07fcf532c15f0a6db0bd6d0e038bea71d
		30d808c7d98cb3bf7268a95bf5081b65a00000000000000000000000007a250d5630b4cf539739df
		2c5dacb4c659f2488da000000000000000000000000000000000000000000000000003e973b5a5d1
		078e
	"
    );

    #[test]
    fn decode_legacy_receipt() {
        let receipt: Receipt = rlp::decode(&RAW_RECEIPT).unwrap();
        assert_eq!(receipt.post_state_or_status, vec!(1));
        assert_eq!(receipt.cumulative_gas_used, 414448);
        assert_eq!(
            receipt.bloom,
            (&hex!(
                "
				042000000000000000000000800200000000000100000000000100000000000000000000
				000000000000000000000000020000000800000000000000002000000000000000000000
				000000080000002200000000004000100000000000000000000000000000000000000000
				000000000000000000000400000000100001000000000008000000000040000000000000
				000000000000000800000040000000000200000000000200000000000000000000000000
				000000000000000000040000000000020000000001000000000000000000000000000010
				000000020000200000102000000000000100000000000000000000000000000000000000
				10000000
			"
            ))
                .into(),
        );
        assert_eq!(receipt.logs.len(), 6);
    }
}
