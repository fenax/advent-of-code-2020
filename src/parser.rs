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
pub fn items_separated_by_whitespace_separated_by_blankline(input:& str)->Vec<Vec<String>>{
    input.split("\n\n").map(str::trim).filter(|x| !x.is_empty())
                        .map(|l| l.split(char::is_whitespace).filter(|x| !x.is_empty()).map(|x|x.to_string()).collect()).collect()
}