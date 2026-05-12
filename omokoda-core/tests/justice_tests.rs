#[cfg(test)]
mod justice_tests {
    use omokoda_core::interpreter::Steward;
    use omokoda_core::parser::parse;
    use omokoda_core::justice::{JusticeEngine, ActQuality};

    #[tokio::test]
    async fn slashing_ethics_reduces_reputation_by_25_percent() {
        let mut steward = Steward::new();
        steward.dispatch(parse(r#"birth "luna""#).unwrap()[0].clone()).await.unwrap();
        
        steward.set_reputation_for_test(100.0);
        steward.slash_ethics().unwrap();
        
        assert_eq!(steward.reputation(), 75.0);
    }

    #[tokio::test]
    async fn slashing_budget_reduces_reputation_by_10_percent() {
        let mut steward = Steward::new();
        steward.dispatch(parse(r#"birth "luna""#).unwrap()[0].clone()).await.unwrap();
        
        steward.set_reputation_for_test(100.0);
        steward.slash_budget().unwrap();
        
        assert_eq!(steward.reputation(), 90.0);
    }

    #[test]
    fn quality_evaluation_failed_multiplier_is_negative() {
        let justice = JusticeEngine::new();
        let quality = justice.evaluate_act("Error: failed", true);
        assert_eq!(quality, ActQuality::Failed);
        assert_eq!(quality.multiplier(), -0.5);
    }

    #[tokio::test]
    async fn quality_evaluation_useful_increases_reputation_more_than_basic() {
        let mut steward = Steward::new();
        steward.dispatch(parse(r#"birth "luna""#).unwrap()[0].clone()).await.unwrap();
        
        // Basic: very short output
        let test_file = "basic.txt";
        std::fs::write(test_file, "short").unwrap();
        steward.set_reputation_for_test(10.0);
        steward.dispatch(parse(r#"act "read_file" "basic.txt""#).unwrap()[0].clone()).await.unwrap();
        let gain_basic = steward.reputation() - 10.0;
        std::fs::remove_file(test_file).unwrap();

        // Useful: > 100 chars
        let useful_content = "A".repeat(150);
        let test_file2 = "useful.txt";
        std::fs::write(test_file2, &useful_content).unwrap();
        steward.set_reputation_for_test(10.0);
        steward.dispatch(parse(r#"act "read_file" "useful.txt""#).unwrap()[0].clone()).await.unwrap();
        let gain_useful = steward.reputation() - 10.0;
        std::fs::remove_file(test_file2).unwrap();

        assert!(gain_useful > gain_basic);
    }
}
