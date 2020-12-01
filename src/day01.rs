pub fn find_with_sum_exclude(input:&Vec<i64>,target:i64,exclusion_start:usize,exclusion_stop:usize)->Option<(usize,usize)>{
        let mut i = 0; let mut j = input.len()-1;
        while i<j && i<exclusion_start && exclusion_stop<j{
            let sum = input[i] + input[j];
            if sum == target {println!("elements [{}]{} and [{}]{} -> {}",i,input[i],j,input[j],input[i]*input[j]); return Some((i,j))}
            else if sum > target {j = j-1; }
            else if sum < target {i = i+1;}
        }
        None
}


pub fn find_with_sum(input:&Vec<i64>,target:i64)->Option<(usize,usize)>{
        let mut i = 0; let mut j = input.len()-1;
        while i<j{
            let sum = input[i] + input[j];
            if sum == target {println!("elements [{}]{} and [{}]{} -> {}",i,input[i],j,input[j],input[i]*input[j]); return Some((i,j))}
            else if sum > target {j = j-1; }
            else if sum < target {i = i+1;}
        }
        None
}

pub fn find_three_with_sum(input:&Vec<i64>,target:i64)->Option<(usize,usize,usize)>{
    let mut k = input.iter().position(|x| *x >= target/3).unwrap_or(input.len()-2);
    let mut l = k;
    let mut cur = &k;
    let mut dec = true; //first turn decrement k, next increment l, then start again
    while k>1 || l<input.len()-2{
        if let Some((i,j)) = find_with_sum_exclude(input,target-input[*cur],k,l){
            println!("elements [{}]{} , [{}]{} and [{}]{} -> {}",i,input[i],j,input[j],*cur,input[*cur],input[i]*input[j]*input[*cur]);
            return Some((i,j,*cur))
        }else{
            if (dec && k>2) || l>= input.len()-2{
                k = k-1;
                cur = &k;
                dec = false;
            }else{
                l = l+1;
                cur = &l;
                dec = true;
            }
        }
    }
    None
}