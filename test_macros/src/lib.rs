#[macro_export]
macro_rules! describe_item_tests {
    ($test_category:ident , $param_type:ty , $param_def:item , $result_type:ty , $params_var:ident , $act_block:block , $($test:item),*) => {
        #[cfg(test)]
        mod $test_category {
            use super::*;

            $param_def

            fn act($params_var: $param_type) -> $result_type $act_block

            $(
                $test
            )*
        }
    };
}

#[macro_export]
macro_rules! describe_test {
    ($input_desc:ident , $param_type:ty , $arrange_block:block , $result_desc:ident , $result_var:ident , $result_block:block) => {
        mod $input_desc {
            use super::*;

            fn arrange() -> $param_type $arrange_block

            #[test]
            fn $result_desc () {
                let params = arrange();
                let result = act(params);

                let result_fn = |$result_var| $result_block;

                result_fn(result);
            }
        }
    };
}
