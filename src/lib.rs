//! Asynchronous library and cli to solve proof-of-work challenges generated with the kctf scheme.
//!
//! ```rust
//! use kctf::KctfPow;
//! let challenge = KctfPow::from_challenge("s.AAU5.AACV7mM375HM8wElUbxsknqD").unwrap();
//!
//! // Solve one
//! let solution = challenge.clone().solve();
//! println!("{}", solution);
//! assert_eq!(solution, "s.LR15WHZE5YO/8EEY9BF7pdvxiJxwkDi7mdS52bg7eVUdHbAwBVxfahl/qxceccZV2PHkj4wQTQ9Ng837/KD9IWQL4v2GmRyjc5O9MxiAXBtxn7FYjjA2as/17lF2lEtQtABbSEUgxam+sIsdfDJMAUzn4fYsS7vOarXh7iY6ZYknrwt1S8EHyQeYkoTUzkpUIVAuSvl8jExcPzvmuaoM6A==");
//!
//! // Verify solution
//! assert_eq!(challenge.verify(&solution), Ok(true));
//! assert_eq!(challenge.verify("s.invalid"), Ok(false));
//!
//! // Generate a challenge
//! let challenge = KctfPow::gen_challenge(50);
//! println!("{}", challenge.serialize_challenge());
//! ```

use base64::prelude::*;
use once_cell::sync::Lazy;
use rand::prelude::*;
use rug::integer::Order;
use rug::ops::Pow;
use rug::Integer;
use std::str;

const VERSION: &str = "s";
struct KctfParams {
    /// The modulus as defined by kCTF, which is 2 ** 1279 - 1
    pub modulus: Integer,
    /// The exponent as defined by kCTF, which is (modulus + 1) / 4
    pub exponent: Integer,
}

impl KctfParams {
    fn new() -> Self {
        let big_num = Integer::from(2).pow(1279);
        KctfParams {
            modulus: big_num.clone() - 1,
            exponent: big_num / 4,
        }
    }
}

static CUSTOM_BASE64: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
    &base64::alphabet::STANDARD,
    base64::engine::GeneralPurposeConfig::new()
        .with_decode_allow_trailing_bits(true)
        .with_encode_padding(true)
        .with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent),
);

static KCTF_PARAMS: Lazy<KctfParams> = Lazy::new(|| KctfParams::new());

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum KctfErrors {
    /// The given challenge/solution has an unknown version.
    ///
    /// Challenges should start with an `s.`
    UnknownVersion,
    /// The given challenge/solution does not follow the kCTF format.
    ///
    /// Challenges should follow the format `s.<Base64 encoded difficulty>.<Base64 encoded value>`,
    /// while solutions should follow the format `s.<Base64 encoded value>`
    FormatError,
    /// The given challenge/solution parts cannot be properly decoded from base64.
    ///
    /// Note that kctf does not require strict following of the base64 rules, like
    /// padding, but will fail to decode if the parts does not follow the rules
    /// `[a-zA-Z0-9]+`
    DecodeError,
    /// The give challenge has a large difficulty that can't fit in a u32 variable.
    LargeDifficulty,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KctfPow {
    /// The difficulty of the challenge
    pub difficulty: u32,
    /// The starting value of the challenge
    pub value: Integer,
}

impl KctfPow {
    /// Used to set the challenge difficulty and starting value manually.
    ///
    /// This is not recommended for direct use unless the remote instances
    /// uses a different format that kctf could decode.
    pub fn from_difficulty_and_value(difficulty: u32, value: Integer) -> Self {
        KctfPow { difficulty, value }
    }

    /// Decodes a challenge and returns an instance of `KctfPow` on success
    /// and a variant of `KctfErrors` if not.
    pub fn from_challenge(challenge: &str) -> Result<Self, KctfErrors> {
        let mut challenge_parts = challenge.split('.');

        // Check if kctf version is known
        if challenge_parts.next() != Some(VERSION) {
            return Err(KctfErrors::UnknownVersion);
        }

        let challenge_parts: Vec<&str> = challenge_parts.collect();

        // Check if the number of remaining parts are expected
        if challenge_parts.len() != 2 {
            return Err(KctfErrors::FormatError);
        }

        // Decode all parts
        let decoded_parts: Vec<Vec<u8>> = challenge_parts
            .into_iter()
            .map(|x| CUSTOM_BASE64.decode(x).map_err(|_| KctfErrors::DecodeError))
            .collect::<Result<_, KctfErrors>>()?;

        let decoded_difficulty = &decoded_parts[0];
        let decoded_value = &decoded_parts[1];

        let difficulty: u32 = if decoded_difficulty.len() > 4 {
            if (&decoded_difficulty[..decoded_difficulty.len() - 4])
                .iter()
                .any(|&x| x != 0)
            {
                return Err(KctfErrors::LargeDifficulty);
            }
            u32::from_be_bytes((&decoded_difficulty[..4]).try_into().unwrap())
        } else {
            let mut difficulty_array = [0; 4];
            difficulty_array[4 - decoded_difficulty.len()..].copy_from_slice(&decoded_difficulty);
            u32::from_be_bytes(difficulty_array)
        };

        Ok(KctfPow {
            difficulty,
            value: Integer::from_digits(decoded_value, Order::Msf),
        })
    }

    /// Solves a challenge. This must be called after initialization.
    pub fn solve(mut self) -> String {
        for _ in 0..self.difficulty {
            let _ = self
                .value
                .pow_mod_mut(&KCTF_PARAMS.exponent, &KCTF_PARAMS.modulus);
            self.value ^= 1;
        }

        format!(
            "{}.{}",
            VERSION,
            CUSTOM_BASE64.encode(self.value.to_digits(Order::Msf))
        )
    }

    /// Generates a challenge.
    pub fn gen_challenge(difficulty: u32) -> Self {
        let mut bytes: [u8; 16] = [0; 16];
        thread_rng().fill(&mut bytes[..]);
        KctfPow {
            difficulty: difficulty,
            value: Integer::from_digits(&bytes, Order::Msf),
        }
    }

    /// Serializes a challenge after it is generated.
    pub fn serialize_challenge(&self) -> String {
        format!(
            "{}.{}.{}",
            VERSION,
            CUSTOM_BASE64.encode(self.difficulty.to_be_bytes()),
            CUSTOM_BASE64.encode(self.value.to_digits(Order::Msf))
        )
    }

    /// Verifies a solution.
    pub fn verify(&self, solution: &str) -> Result<bool, KctfErrors> {
        let mut decoded_solution = decode_solution(solution)?;

        for _ in 0..self.difficulty {
            decoded_solution ^= 1;
            let _ = decoded_solution.pow_mod_mut(&2.into(), &KCTF_PARAMS.modulus);
        }

        Ok(self.value == decoded_solution
            || Integer::from(&KCTF_PARAMS.modulus - &self.value) == decoded_solution)
    }
    /// Solves a challenge asynchronously.
    pub async fn async_solve(mut self) -> String {
        for _ in 0..self.difficulty {
            async {
                let _ = self
                    .value
                    .pow_mod_mut(&KCTF_PARAMS.exponent, &KCTF_PARAMS.modulus);
                self.value ^= 1;
            }
            .await
        }

        format!(
            "{}.{}",
            VERSION,
            CUSTOM_BASE64.encode(self.value.to_digits(Order::Msf))
        )
    }

    /// Verifies a solution asynchronously.
    pub async fn async_verify(&self, solution: &str) -> Result<bool, KctfErrors> {
        let mut decoded_solution = decode_solution(solution)?;

        for _ in 0..self.difficulty {
            async {
                decoded_solution ^= 1;
                let _ = decoded_solution.pow_mod_mut(&2.into(), &KCTF_PARAMS.modulus);
            }
            .await
        }
        Ok(self.value == decoded_solution
            || Integer::from(&KCTF_PARAMS.modulus - &self.value) == decoded_solution)
    }
}

/// Decodes a solution.
///
/// Not really meant for public consumption, but it could be useful.
pub fn decode_solution(solution: &str) -> Result<Integer, KctfErrors> {
    let mut solution_parts = solution.split('.');
    if solution_parts.next() != Some(VERSION) {
        return Err(KctfErrors::UnknownVersion);
    }

    let decoded_solution = match solution_parts.next() {
        Some(v) => Integer::from_digits(
            &CUSTOM_BASE64
                .decode(v)
                .map_err(|_| KctfErrors::DecodeError)?,
            Order::Msf,
        ),

        None => {
            return Err(KctfErrors::FormatError);
        }
    };

    if solution_parts.next().is_some() {
        return Err(KctfErrors::FormatError);
    }

    Ok(decoded_solution)
}
