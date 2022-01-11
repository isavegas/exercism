/* Bob is a lackadaisical teenager. In conversation, his responses are very limited.

Bob answers 'Sure.' if you ask him a question.

He answers 'Whoa, chill out!' if you yell at him.

He says 'Fine. Be that way!' if you address him without actually saying
anything.

He answers 'Whatever.' to anything else. */


// * receive an &str and ensure it lives as long as the function
// (I'm learning about lifetimes, so best not let it be magic)
// * we can accept &str because it is guaranteed to live until the
// end of the function. using String would copy the whole thing, whereas
// &str simply points to a set of bytes to view as a str. I think..?
// * return String, as returing &str would cause our function to retain
// ownership when it ends, meaning that the caller now has a reference to
// a pointer to a dead object. returning String moves the ownership completely.

pub fn reply<'a>(message: &'a str) -> String {
  if message.len() == 0 { return "Fine. Be that way!".to_string(); }
  if message.chars().last().unwrap_or_default() == '?' {
    return "Sure.".to_string();
  } else if message.find(char::is_lowercase) == None {
    return "Whoa, chill out!".to_string();
  } else {
    return "Whatever.".to_string();
  }
}