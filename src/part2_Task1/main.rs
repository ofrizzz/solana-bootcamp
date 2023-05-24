fn concatenate_strings(st1: &str, st2: &str) -> String{
    let mut result: String = String::from("");
    result.push_str(st1);
    result.push_str(st2);
    return result;
}


fn main() {
    let string1: String = String::from("string1");
    let string2: String = String::from("string2");
    let concatenated_string: String = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_string);
}
