use alloy::primitives::U256;
use ark_bn254::Fr;
use light_poseidon::{Poseidon, PoseidonBytesHasher};

pub fn poseidon4(a: U256, b: U256, c: U256, d: U256) -> Result<U256, anyhow::Error> {
    let mut poseidon = Poseidon::<Fr>::new_circom(4)?;

    let a = a.to_be_bytes::<32>();
    let b = b.to_be_bytes::<32>();
    let c = c.to_be_bytes::<32>();
    let d = d.to_be_bytes::<32>();

    let hashed = poseidon.hash_bytes_be(&[&a, &b, &c, &d])?;

    Ok(U256::from_be_bytes(hashed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poseidon4_test() {
        let hash = poseidon4(
            U256::from(77),
            U256::from(78),
            U256::from(79),
            U256::from(80),
        )
        .unwrap();

        let answer = U256::from_str_radix(
            "15259998661006299095149775441429875008500637153142570752325914879195522286115",
            10,
        )
        .unwrap();
        assert_eq!(hash, answer);
    }
}
