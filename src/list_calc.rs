use std::collections::HashMap;

pub fn get_mean (vec: &Vec<i32>) -> i32 {
    let len = vec.len() as i32;
    let mut sum = 0;
    for i in vec {
        sum += i;
    }
    return sum /len;
}

pub fn get_median (vec: &Vec<i32>) -> i32 {
    let mut _vec = vec.clone();
    _vec.sort();
    let len = _vec.len();

    let med: i32 = if len % 2 == 0  {
        (_vec[len/2] + _vec[len/2-1] )/2
    } else {
        _vec[len /2]
    };
    
    return med;
}

pub fn get_mode (vec: &Vec<i32>) -> i32 {
    let mut mode_count: HashMap<i32, i32>  = HashMap::new();
    for i in vec {
        if mode_count.contains_key(i){
            mode_count.insert(*i, mode_count[i] + 1);
        } else {
            mode_count.insert(*i, 1);
        }
    }

    let maxMap = mode_count.iter().fold((&0,&0), |left,right| 
        if left.1 < right.1 {
            right
        } else {
            left
        });
    
    return *maxMap.0;
}