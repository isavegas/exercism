// on every year that is evenly divisible by 4
//   except every year that is evenly divisible by 100
//     unless the year is also evenly divisible by 400


pub fn is_leap_year(year: i64) -> bool {
//y % 4 == 0 and (y % 400 == 0 or y % 100 ~= 0)
	return year % 4 == 0 && (year % 400 == 0 || year % 100 != 0);
}