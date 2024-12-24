use kctf::{KctfErrors, KctfPow};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_google_kctf_challenge() {
        let result = KctfPow::from_challenge("s.AAU5.AACV7mM375HM8wElUbxsknqD").unwrap();
        assert!(result.verify("s.LR15WHZE5YO/8EEY9BF7pdvxiJxwkDi7mdS52bg7eVUdHbAwBVxfahl/qxceccZV2PHkj4wQTQ9Ng837/KD9IWQL4v2GmRyjc5O9MxiAXBtxn7FYjjA2as/17lF2lEtQtABbSEUgxam+sIsdfDJMAUzn4fYsS7vOarXh7iY6ZYknrwt1S8EHyQeYkoTUzkpUIVAuSvl8jExcPzvmuaoM6A==").unwrap());
    }

    #[test]
    fn bad_challenge_unknown_version() {
        assert_eq!(
            KctfPow::from_challenge("v.AAAR+CA/uJcJHw.AACV7mM375HM8wElUbxsknqD"),
            Err(KctfErrors::UnknownVersion)
        );
    }

    #[test]
    fn bad_challenge_bad_format() {
        assert_eq!(
            KctfPow::from_challenge("s.AAAR+CA/uJcJHwAACV7mM375HM8wElUbxsknqD"),
            Err(KctfErrors::FormatError)
        );
    }

    #[test]
    fn bad_challenge_decode_error() {
        assert_eq!(
            KctfPow::from_challenge("s.AAA;[********R+CA/uJcJHw.AACV7mM375HM8wElUbxsknqD"),
            Err(KctfErrors::DecodeError)
        );
    }

    #[test]
    fn bad_challenge_large_difficulty() {
        assert_eq!(
            KctfPow::from_challenge("s.AAAR+CA/uJcJHw.AACV7mM375HM8wElUbxsknqD"),
            Err(KctfErrors::LargeDifficulty)
        );
    }

    #[test]
    fn self_test_difficulty_50() {
        let challenge = KctfPow::gen_challenge(50);
        let solution = challenge.clone().solve();
        assert!(challenge.verify(&solution).unwrap());
    }

    #[test]
    fn self_test_difficulty_1337() {
        let challenge = KctfPow::gen_challenge(1337);
        let solution = challenge.clone().solve();
        assert!(challenge.verify(&solution).unwrap());
    }

    #[test]
    fn expensive_self_test_difficulty_31337() {
        let challenge = KctfPow::gen_challenge(31337);
        let solution = challenge.clone().solve();
        assert!(challenge.verify(&solution).unwrap());
    }

    #[test]
    fn expensive_google_kctf_challenge() {
        let result = KctfPow::from_challenge("s.AAU5.AACV7mM375HM8wElUbxsknqD")
            .unwrap()
            .solve();
        assert_eq!(result, "s.LR15WHZE5YO/8EEY9BF7pdvxiJxwkDi7mdS52bg7eVUdHbAwBVxfahl/qxceccZV2PHkj4wQTQ9Ng837/KD9IWQL4v2GmRyjc5O9MxiAXBtxn7FYjjA2as/17lF2lEtQtABbSEUgxam+sIsdfDJMAUzn4fYsS7vOarXh7iY6ZYknrwt1S8EHyQeYkoTUzkpUIVAuSvl8jExcPzvmuaoM6A==");
    }

    #[tokio::test]
    async fn async_verify() {
        let result = KctfPow::from_challenge("s.AAU5.AACV7mM375HM8wElUbxsknqD").unwrap();
        assert!(result.async_verify("s.LR15WHZE5YO/8EEY9BF7pdvxiJxwkDi7mdS52bg7eVUdHbAwBVxfahl/qxceccZV2PHkj4wQTQ9Ng837/KD9IWQL4v2GmRyjc5O9MxiAXBtxn7FYjjA2as/17lF2lEtQtABbSEUgxam+sIsdfDJMAUzn4fYsS7vOarXh7iY6ZYknrwt1S8EHyQeYkoTUzkpUIVAuSvl8jExcPzvmuaoM6A==").await.unwrap());
    }

    #[tokio::test]
    async fn async_self_test_50() {
        let challenge = KctfPow::gen_challenge(50);
        let solution = challenge.clone().async_solve().await;
        assert!(challenge.async_verify(&solution).await.unwrap());
    }
}
