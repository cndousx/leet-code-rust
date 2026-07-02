pub fn to_string_vec(vs: Vec<&str>) -> Vec<String> {
    vs.iter().map(|s| s.to_string()).collect::<Vec<String>>()
}
