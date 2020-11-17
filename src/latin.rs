pub fn to_latin (str: &String) -> String {
    let vowel = ['a','e','i','o','u'];
    let char_vec = str.chars();
    let first_str = str.chars().nth(0).unwrap();
    let mut rest_str = String::new();
    for (i,c) in char_vec.enumerate() {
        if i>0 { rest_str.push(c);} 
    }
    match vowel.contains(&first_str) {
        true => { return format!("{}{}",str , "-hay")},
        false => {return format!("{}{}{}{}", rest_str, "-",first_str, "ay")}
    }
}