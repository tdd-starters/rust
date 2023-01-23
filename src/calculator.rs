/// Evaluate a string as a mathematical expression and return the result
pub fn evaluate(expression: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {

    use super::evaluate;
    use rstest::rstest;

    #[rstest]
    #[case::single_digit("0", 0)]
    //#[case::add("1 + 2", 3)]
    // ... more tests here
    //#[case::complex("(1 + 2) * 5 / 2 + -3 * 7", -14)]
    fn test_expression(#[case] expression: &str, #[case] expected: i32) {
        // When the expression is evaluated
        let result = evaluate(expression);

        // Then the result is as expected
        assert_eq!(result, expected)
    }

    /*
    #[rstest]
    #[case::string("foo")]
    // ... more tests here
    fn test_errors(#[case] expression: &str) {
        // When the expression is evaluated
        let result = evaluate(expression);

        // Then the result should be an error
        assert!(result.is_err());
    }
    */
}
