use test_macros::{self, describe_item_tests};

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

describe_item_tests!(
    add_tests,
    AddParams,
    struct AddParams {
        a: i32,
        b: i32,
    },
    i32,
    params,
    { add(params.a, params.b) },
    test_macros::describe_test!(
        when_provided_positive_integers,
        AddParams,
        { AddParams { a: 4, b: 7 } },
        should_add_correctly,
        result,
        {
            assert_eq!(result, 11);
        }
    );,
    test_macros::describe_test!(
        when_provided_negative_integers,
        AddParams,
        { AddParams { a: -4, b: -7 } },
        should_add_correctly,
        result,
        {
            assert_eq!(result, -11);
        }
    );
);

// The above should expand to something like this:
// mod add_tests {
//     use super::*;

//     struct AddParams {
//         a: i32,
//         b: i32,
//     }

//     fn act(params: AddParams) -> i32 {
//         add(params.a, params.b)
//     }

//     mod when_provided_positive_integers {
//         use super::*;

//         fn arrange() -> AddParams {
//             AddParams { a: 4, b: 7 }
//         }

//         #[test]
//         fn should_add_correctly() {
//             let params = arrange();
//             let result = act(params);

//             let result_fn = |result| {
//                 assert_eq!(result, -11);
//             };

//             result_fn(result);
//         }
//     }

//     mod when_provided_negative_integers {
//         use super::*;

//         fn arrange() -> AddParams {
//             AddParams { a: -4, b: -7 }
//         }

//         #[test]
//         fn should_add_correctly() {
//             let params = arrange();
//             let result = act(params);

//             let result_fn = |result| {
//                 assert_eq!(result, -11);
//             };

//             result_fn(result);
//         }
//     }
// }
