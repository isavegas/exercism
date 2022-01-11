extern crate chrono;
use chrono::DateTime;
use chrono::UTC;
use chrono::Duration;

pub fn after(date: DateTime<UTC>) -> DateTime<UTC> {
	return date + Duration::seconds(10_i64.pow(9));
}