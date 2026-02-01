use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use light_poseidon::{Poseidon, PoseidonBytesHasher};

pub fn poseidon4(a: Fr, b: Fr, c: Fr, d: Fr) -> Result<Fr, anyhow::Error> {
    let mut poseidon = Poseidon::<Fr>::new_circom(4)?;

    let a = a.into_bigint().to_bytes_be();
    let b = b.into_bigint().to_bytes_be();
    let c = c.into_bigint().to_bytes_be();
    let d = d.into_bigint().to_bytes_be();

    let hashed = poseidon.hash_bytes_be(&[&a, &b, &c, &d])?;

    Ok(Fr::from_be_bytes_mod_order(&hashed))
}

pub fn poseidon3(a: Fr, b: Fr, c: Fr) -> Result<Fr, anyhow::Error> {
    let mut poseidon = Poseidon::<Fr>::new_circom(3)?;

    let a = a.into_bigint().to_bytes_be();
    let b = b.into_bigint().to_bytes_be();
    let c = c.into_bigint().to_bytes_be();

    let hashed = poseidon.hash_bytes_be(&[&a, &b, &c])?;

    Ok(Fr::from_be_bytes_mod_order(&hashed))
}

pub fn poseidon2(a: Fr, b: Fr) -> Result<Fr, anyhow::Error> {
    let mut poseidon = Poseidon::<Fr>::new_circom(2)?;

    let a = a.into_bigint().to_bytes_be();
    let b = b.into_bigint().to_bytes_be();

    let hashed = poseidon.hash_bytes_be(&[&a, &b])?;

    Ok(Fr::from_be_bytes_mod_order(&hashed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poseidon4_test() {
        let hash = poseidon4(Fr::from(77), Fr::from(78), Fr::from(79), Fr::from(80)).unwrap();

        let expected =
            "15259998661006299095149775441429875008500637153142570752325914879195522286115";
        assert_eq!(hash.to_string(), expected);
    }

    #[test]
    fn poseidon3_test() {
        let hash = poseidon3(Fr::from(77), Fr::from(78), Fr::from(79)).unwrap();

        let expected =
            "12210189975798879106793997699810717700312462076733147998063543371951372855322";
        assert_eq!(hash.to_string(), expected);
    }

    #[test]
    fn poseidon2_test() {
        let hash = poseidon2(Fr::from(77), Fr::from(78)).unwrap();

        let expected =
            "20026229866168919606600644007201798921387591145480254371130649313907815874811";
        assert_eq!(hash.to_string(), expected);
    }
}
