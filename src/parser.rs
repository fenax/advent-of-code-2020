pub fn one_int_per_line(input:& str)->Vec<i64>{
    input.split('\n')
                     .map(str::trim)
                     .filter_map(|s|s.parse::<i64>().ok())
                     .collect()
}
pub fn one_char_vec_per_line(input:& str)->Vec<Vec<char>>{
    input.split('\n').map(str::trim).filter(|x| !x.is_empty())
                     .map(|l| l.chars().collect()).collect()
}