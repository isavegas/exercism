// str does not have a defined size,
// meaning we cannot allocate space for it
// on the stackbefore we actually create it.
//this means we have to put it on the heap
// and pass back a reference, rather than
// the value itself. however, due to it being
// a reference, the value is disposed of
// as soon as this function ends. uh oh.
// therefore, String it is!
pub fn raindrops(i: i64) -> String {
	let mut out = String::from("");
	if i % 3 == 0 { out.push_str("Pling"); }
        if i % 5 == 0 { out.push_str("Plang"); }
        if i % 7 == 0 { out.push_str("Plong"); }
	if out != "" { return out; }
	return i.to_string();
}