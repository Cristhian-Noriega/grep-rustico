use std::process::Command;

fn run_test(regex: &str, expected_output: &str) {
    let output = Command::new("cargo")
        .arg("run")
        .arg(regex)
        .arg("texts/test.txt")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output_str.trim(), expected_output);
}

#[test]
fn test_dot() {
    run_test("ab.cd", "abecd");
}

#[test]
fn test_star() {
    run_test("ab.*c", "abecd\nabzzzcd\nhola abcdefg");
}

#[test]
fn test_bracket() {
    run_test("a[bc]d", "abd\nacd");
}

#[test]
fn test_ranges() {
    run_test("ab{2,4}cd", "");
}

#[test]
fn test_or_plus() {
    run_test("abc|d+f", "df\nhola abcdefg");
}

#[test]
fn test_bracket_spaces() {
    run_test("la [aeiou] es una vocal", "la a es una vocal");
}

#[test]
fn test_bracket_negated() {
    run_test("la [^aeiou] no es una vocal", "la z no es una vocal");
}

#[test]
fn test_character_class_alpha() {
    run_test("hola [[:alpha:]]", "hola abcdefg\nhola mundo");
}

#[test]
fn test_character_class_digit() {
    run_test("el caracter [[:alnum:]] no es un simbolo", "el caracter a no es un simbolo");
}

#[test]
fn test_character_class_space() {
    run_test("hola[[:space:]]mundo", "hola mundo");
}

#[test]
fn test_character_class_upper() {
    run_test("[[:upper:]]ascal[[:upper:]]ase", "PascalCase");
}

#[test]
fn test_dollar() {
    run_test("es el fin$", "es el fin");
}