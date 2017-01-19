extern crate prime_factors;

#[cfg(test)] #[macro_use] extern crate hamcrest;

mod prime_factors_test {
    use prime_factors::generate;
    use hamcrest::prelude::*;
    use hamcrest::core::MatchResult;
    use hamcrest::core::success;
    use std::fmt;

    struct GeneratesList {
        list: Vec<i32>,
    }

    impl fmt::Display for GeneratesList {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", &self.list)
        }
    }

    impl HamcrestMatcher<i32> for GeneratesList {
        fn matches(&self, actual: i32) -> MatchResult {
            let v = generate(actual);

            if v == self.list {
                success()
            } else {
                Err(format!("was {:?} (via {})", v, actual))
            }
        }
    }

    fn generates_list (list: Vec<i32>) -> GeneratesList {
        GeneratesList{list: list}
    }

    #[test]
    fn test_one() {
        let empty: Vec<i32> = Vec::new();

        assert_that!(1, generates_list(empty));
    }

    #[test]
    fn test_23456789() {
        assert_that!(2, generates_list(vec![2]));
        assert_that!(3, generates_list(vec![3]));
        assert_that!(4, generates_list(vec![2, 2]));
        assert_that!(5, generates_list(vec![5]));
        assert_that!(6, generates_list(vec![2, 3]));
        assert_that!(7, generates_list(vec![7]));
        assert_that!(8, generates_list(vec![2, 2, 2]));
        assert_that!(9, generates_list(vec![3, 3]));
    }

    #[test]
    fn test_large() {
        assert_that!(2*2*3*3*5*11*13*13,
                     generates_list(vec![2, 2, 3, 3, 5, 11, 13, 13]));
    }
}
