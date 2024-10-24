use std::env;

pub fn set_test_env() {
    env::set_var(
        "NSEC_SECRET_KEY",
        "nsec1yrltn3whr35te648whx89gsc7krr6x5qeyuscen05e7x9cmde6gq7f82u7",
    );
}
