use chalk_rs::Chalk;

pub fn is_debug() -> bool {
  cfg!(debug_assertions)
}

pub fn debug_print(text: &str) {
  let mut chalk = Chalk::new();
  let string = format!("Debug {:?}", text);
  chalk.rgb(0, 0, 0).bg_yellow().bold().println(&string);
}

pub fn debug_println(text: &str) {
  let mut chalk = Chalk::new();
  let string = format!("Debug {:?}\n", text);
  chalk.rgb(0, 0, 0).bg_yellow().bold().println(&string);
}
