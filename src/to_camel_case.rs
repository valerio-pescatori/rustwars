/// Complete the method/function so that it converts dash/underscore delimited words into camel casing.
/// The first word within the output should be capitalized only if the original word was capitalized
/// (known as Upper Camel Case, also often referred to as Pascal case). The next words should be always capitalized.
/// Examples
/// "the-stealth-warrior" gets converted to "theStealthWarrior"
/// "The_Stealth_Warrior" gets converted to "TheStealthWarrior"
/// "The_Stealth-Warrior" gets converted to "TheStealthWarrior"
pub fn to_camel_case(text: &str) -> String {
    let split_iter = text.split(['-', '_']).collect::<Vec<&str>>();
    if split_iter.len() == 1 {
        return text.to_owned();
    }
    let mut result = String::from("");
    for (i, element) in split_iter.iter().enumerate() {
        let (first, _rest) = element.split_at(1);
        let capitalized = capitalize(element);
        let capitalized_str = capitalized.as_str();
        if i == 0 {
            result.push_str(if first.to_uppercase() == first {
                capitalized_str
            } else {
                element
            })
        } else {
            result.push_str(capitalized_str)
        }
    }
    result
}

fn capitalize(string: &str) -> String {
    let (first, rest) = string.split_at(1);
    let mut first = first.to_uppercase();
    first.push_str(rest);
    first
}

pub fn to_better_camel_case(text: &str) -> String {
    let mut should_capitalize = false;
    let mut result = String::from("");
    text.chars().enumerate().for_each(|(i, c)| {
        if i == 0 {
            result.push(c);
        }
        if c == '-' || c == '_' {
            should_capitalize = true;
        }
        if should_capitalize {
            result.extend(c.to_uppercase());
        } else {
            result.push(c);
        }
    });
    return result;
}
