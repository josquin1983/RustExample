pub mod coin {

    type Gramm = f32;

    #[derive(Debug)]
    pub struct Coin {
        weight: Gramm,
    }

    impl Coin {
        pub fn new(weight: f32) -> Coin {
            Coin { weight: weight }
        }
    }

    pub fn is_bad_coin(data: &[Coin]) -> bool {
        let length = data.len();

        let result = match length {
            0 => false,

            1 => data[0].weight != 9.5,

            2 => !(data[0].weight == data[1].weight),

            _ => {
                let mid = length / 2;

                let first = &data[..mid];

                let last = &data[mid..];

                is_bad_coin(first) || is_bad_coin(last)
            }
        };

        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn is_bad_coin_test() {
            assert_eq!(
                is_bad_coin(&[
                    Coin::new(8.0),
                    Coin::new(8.0),
                    Coin::new(8.0),
                    Coin::new(9.5)
                ]),
                true
            );
        }

        #[test]
        fn no_bad_coins() {
            assert_eq!(is_bad_coin(&[Coin::new(8.0), Coin::new(8.0)]), false);
        }
    }
}
