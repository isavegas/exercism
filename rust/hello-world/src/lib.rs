pub fn hello(option: Option<&str>) -> String {
  let mut name_string = option.unwrap_or_default();
  if name_string == "" {
    name_string = "World";
  }
  return format!("Hello, {}!", name_string).to_string();
}