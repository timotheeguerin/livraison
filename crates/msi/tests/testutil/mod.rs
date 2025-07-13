// ========================================================================= //

macro_rules! assert_error {
    ($e:expr_2021, $k:expr_2021, $d:expr_2021) => {
        let kind = $k;
        let description = $d;
        match $e {
            Ok(_) => panic!("Expected {:?} error, but result was Ok", kind),
            Err(error) => {
                if error.kind() != kind {
                    panic!(
                        "Expected {:?} error, but result was {:?} error \
                            with description {:?}",
                        kind,
                        error.kind(),
                        error.to_string()
                    );
                }
                if error.to_string() != description {
                    panic!(
                        "Expected {:?} error with description {:?}, but \
                            result had description {:?}",
                        kind,
                        description,
                        error.to_string()
                    );
                }
            }
        }
    };
}

// ========================================================================= //
