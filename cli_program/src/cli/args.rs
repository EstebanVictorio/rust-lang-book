use crate::utils::{debug_println, is_debug};
use std::boxed::Box;
use std::env;
use std::error::Error;
use std::fs::read_to_string;

const INVALID_EMPTY_ARGUMENT: &str = "invalid argument";
const NOT_ENOUGH_ARGUMENTS: &str = "not enough arguments, missing query and/or location";

pub fn get_args() -> Vec<String> {
  let args = env::args().collect();
  if is_debug() {
    let string = format!("Debug output - Args {:?}", args);
    debug_println(&string);
  }
  args
}

pub struct Search {
  pub query: String,
  pub location: String,
  case_insensitive: bool,
}

impl Search {
  fn get_invalid_arg_error_message(name: &str) -> String {
    format!("{}, arg name: {}", INVALID_EMPTY_ARGUMENT, name)
  }

  pub fn new(args: &Vec<String>) -> Result<Search, String> {
    if args.len() < 3 {
      return Err(format!("{}", NOT_ENOUGH_ARGUMENTS));
    }

    let invalid_query_err_msg = Search::get_invalid_arg_error_message("query");
    let invalid_location_err_msg = Search::get_invalid_arg_error_message("location");

    let query = args.get(1).expect(&invalid_query_err_msg);
    let location = args.get(2).expect(&invalid_location_err_msg);

    Ok(Search {
      query: String::from(query),
      location: String::from(location),
      case_insensitive: !env::var("CASE_INSENSITIVE").is_err(),
    })
  }

  pub fn run(&self) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(&self.location)?;

    if is_debug() {
      let query_debug_string = format!("Query {:?}", self.query);
      debug_println(&query_debug_string);
      let location_debug_string = format!("Location {:?}", self.location);
      debug_println(&location_debug_string);
    }

    let results = if self.case_insensitive {
      search_case_insensitive(&self.query, &contents)
    } else {
      search(&self.query, &contents)
    };

    for line in results {
      println!("{}", line);
    }

    Ok(())
  }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = vec![];
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = vec![];
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }
  results
}
