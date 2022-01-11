pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        String::new()
    } else {
        let mut proverb: String = list.windows(2).map(|p| format!("For want of a {} the {} was lost.\n", p[0], p[1])).collect();
        proverb.push_str(format!("And all for the want of a {}.", list[0]).as_str());
        proverb
    }
}
