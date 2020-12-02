pub fn one_int_per_line(input:& str)->Vec<i64>{
    input.split('\n')
                     .map(str::trim)
                     .filter_map(|s|s.parse::<i64>().ok())
                     .collect()
}