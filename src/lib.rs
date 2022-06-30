use std::env;
use std::error::Error;
use std::process;
use std::thread;
use std::time;
use chrono::{DateTime, Utc};
use rand::{thread_rng,Rng};

pub const USER_SPECIFIED_VALUE: u32 = 10;

pub const MAX_TOKEN_SIZE: usize = 100;

pub const MAX_RANDOM_POSSIBLE_VAL: u32 = 50;

pub fn simulate_time_request(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(time::Duration::from_secs(2));
    intensity
}

#[derive(Debug)]
pub struct Training {
    id: String,
    pub user_spec_val: u32,
    pub random_int: u32,
    created_at: DateTime<Utc>
}

impl Training {
    pub fn new(val: u32,id: String) -> Training {
        Training {
            id,
            random_int: Self::generate_random_number(),
            created_at: Utc::now(),
            user_spec_val: val,
        }
    }
    pub fn generate_token_id(token_length: usize) -> Result<String,Box<dyn Error>> {
        let chars_list = String::from("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-.");
        let mut random_thread = thread_rng();
        let mut chars: Vec<u8> = vec![];
        loop {
            if chars.len().eq(&token_length) {
                break
            } else {
                let mut random_index: usize = random_thread.gen_range(0..chars_list.len());
                let char = chars_list.bytes().nth(random_index).unwrap();
                chars.push(char);
            }
        }

        let string = String::from_utf8(chars)?;

        Ok(string)

    }
    fn generate_random_number() -> u32 {
        let mut random_thread_gen = thread_rng() ;
        let random_num = random_thread_gen.gen_range(0..MAX_RANDOM_POSSIBLE_VAL);
        random_num
    }
    pub fn generate_workout(&self) -> Result<(),String> {
        let expensive_calc = |num| {
            println!("calculating slowly...");
            thread::sleep(time::Duration::from_secs(2));
            num
        };

        let val: u32 = expensive_calc(self.user_spec_val);

        if self.user_spec_val < 25 {
            println!("Today do: {} push-ups!",val);
            println!("Next do: {} sit-ups!",val);
            Ok(())
        } else {
            if self.random_int == 2 { Err(String::from("You have trained hard, so far. Please rest today.")) }
            else if self.random_int > 5 { Err(String::from("The doctor does not recommend you to train this week.")) }
            else {
                println!("Just run: {} miles",val);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_training_success() {
        let random_token = Training::generate_token_id(MAX_TOKEN_SIZE).unwrap_or_else(|err| {
            eprintln!("Error found: {}",err);
            process::exit(1);
        });

        let week_one = Training::new(USER_SPECIFIED_VALUE,random_token);

        println!("{:#?}",week_one);

        let zero: usize = 0;
        let hundred: usize = 100;

        assert_eq!(week_one.user_spec_val,USER_SPECIFIED_VALUE);
        assert!(week_one.id.len().gt(&zero));
        assert!(week_one.id.len().le(&hundred));
    }

    #[test]
    #[should_panic]
    fn create_training_failure() {
        let random_token = Training::generate_token_id(MAX_TOKEN_SIZE).unwrap_or_else(|err| {
            eprintln!("Error found: {}",err);
            process::exit(1);
        });

        let week_one = Training::new(USER_SPECIFIED_VALUE,random_token);

        println!("{:#?}",week_one);

        let zero: usize = 0;
        let hundred: usize = 100;

        assert_ne!(week_one.user_spec_val,USER_SPECIFIED_VALUE);
        assert!(week_one.id.len().lt(&zero));
        assert!(week_one.id.len().ne(&hundred));
    }
}