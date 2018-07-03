pub fn hash(token: &String) -> u64 {
    let mut hash: u64 = 0;
    let mut i = 1;
    for c in token.chars().rev() {
        hash += (c as u64) * i;
        i += 1;
    }
    433_494_437 / hash
}
