use std::process;

use clsr::{Training,MAX_TOKEN_SIZE,USER_SPECIFIED_VALUE};

fn main() {

    let random_token = Training::generate_token_id(MAX_TOKEN_SIZE).unwrap_or_else(|err| {
        eprintln!("Error found: {}",err);
        process::exit(1);
    });

    let week_one = Training::new(USER_SPECIFIED_VALUE,random_token);

    week_one.generate_workout();

}


