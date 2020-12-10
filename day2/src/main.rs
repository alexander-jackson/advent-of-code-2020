use day2::PasswordConstraint;

fn main() {
    let constraints = day2::read_input();

    println!(
        "Part 1 Solution: {}",
        day2::valid_passwords(&constraints, PasswordConstraint::basic)
    );

    println!(
        "Part 2 Solution: {}",
        day2::valid_passwords(&constraints, PasswordConstraint::complex)
    );
}
