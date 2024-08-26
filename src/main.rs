mod to_camel_case;

fn main() {
    println!(
        "the_stealth_warrior -> {}",
        to_camel_case::to_camel_case("the_stealth_warrior")
    );
    println!("'' -> {}", to_camel_case::to_camel_case(""));
    println!(
        "The-Stealth-Warrior -> {}",
        to_camel_case::to_camel_case("The-Stealth-Warrior")
    );
    println!("A-B-C -> {}", to_camel_case::to_camel_case("A-B-C"));
}
