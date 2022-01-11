use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String,i32>{
  let mut tree: BTreeMap<String,i32> = BTreeMap::new();
  for (i, vec) in input {
    println!("{:?}",vec);
    for s in  vec {
      tree.insert(s.to_lowercase(), i.clone());
    }
  }
//  tree.insert("world".to_string(), 1);
  return tree;
}