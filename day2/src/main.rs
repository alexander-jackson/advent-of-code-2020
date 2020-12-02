use day2::PasswordConstraint;

fn main() {
    let constraints = day2::read_input();

    println!(
        "Valid: {}",
        day2::valid_passwords(&constraints, PasswordConstraint::basic)
    );

    println!(
        "Valid: {}",
        day2::valid_passwords(&constraints, PasswordConstraint::complex)
    );
}
