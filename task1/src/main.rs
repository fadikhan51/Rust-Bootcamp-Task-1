fn main() {
    let str1 = String::from("Fahad ");
    let str2 = String::from("Khan");
    let concatenate_string = concatenate_strings(&str1[..], &str2[..]);

    println!("The concatenated string is {}", concatenate_string);
}

fn concatenate_strings(s1: &str, s2: &str) -> String{
    let mut result: String = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}
