fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result = String::new();
    result.push_str(str1);
    result.push_str(str2);
    result
}

fn main() {
    let str1 = String::from("I Love ");
    let str2 = String::from("GDSC!");

    let concatenated_string = concatenate_strings(&str1, &str2);
    println!("{}", concatenated_string);
}
