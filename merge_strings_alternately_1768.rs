pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut i1 = 0;
    let mut i2 = 0;
    let mut cap = word1.len() + word2.len();
    cap = cap + (cap / 2);
    let mut result = String::with_capacity(cap);
    loop {
        match (word1.as_bytes().get(i1), word2.as_bytes().get(i2)) {
            (Some(&c1), Some(&c2)) => {
                result.push(c1 as char);
                result.push(c2 as char);
                i1 += 1;
                i2 += 1;
            }
            (Some(&c1), None) => {
                result.push(c1 as char);
                i1 += 1;
            }
            (None, Some(&c2)) => {
                result.push(c2 as char);
                i2 += 1;
            }
            (None, None) => {
                break;
            }
        }
    }
    result
}
