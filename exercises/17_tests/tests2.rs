// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(2 + 2, 4); // Fait passer le test en comparant deux valeurs égales
    }
    fn you_can_assert_eq2() {
        assert_eq!(2 + 2, 5); // Fait échouer le test en comparant deux valeurs différentes
    }
}