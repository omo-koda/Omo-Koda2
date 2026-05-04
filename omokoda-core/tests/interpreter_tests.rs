#[cfg(test)]
mod interpreter_tests {
    use omokoda_core::interpreter::Steward;
    use omokoda_core::parser::parse;

    #[test]
    fn birth_creates_agent_at_tier_zero() {
        let mut steward = Steward::new();
        let stmts = parse(r#"birth "luna""#).unwrap();
        steward.dispatch(stmts[0].clone()).unwrap();
        assert_eq!(steward.reputation(), 0.000);
        assert_eq!(steward.tier(), 0);
    }

    #[test]
    fn think_does_not_produce_receipt() {
        let mut steward = Steward::new();
        parse(r#"birth "luna""#).unwrap().into_iter()
            .for_each(|s| { steward.dispatch(s).unwrap(); });
        let stmts = parse(r#"think "hello world""#).unwrap();
        let result = steward.dispatch(stmts[0].clone()).unwrap();
        assert!(result.receipt.is_none());
    }

    #[test]
    fn think_private_sets_private_mode() {
        let mut steward = Steward::new();
        parse(r#"birth "luna""#).unwrap().into_iter()
            .for_each(|s| { steward.dispatch(s).unwrap(); });
        let stmts = parse(r#"think "secret" /private"#).unwrap();
        let result = steward.dispatch(stmts[0].clone()).unwrap();
        assert!(result.private_mode);
    }

    #[test]
    fn act_produces_receipt() {
        let mut steward = Steward::new();
        parse(r#"birth "luna""#).unwrap().into_iter()
            .for_each(|s| { steward.dispatch(s).unwrap(); });
        let stmts = parse(r#"act "web_search" "bitcoin""#).unwrap();
        let result = steward.dispatch(stmts[0].clone()).unwrap();
        assert!(result.receipt.is_some());
        let receipt = result.receipt.unwrap();
        assert!(!receipt.dry_run);
        assert!(!receipt.receipt_id.is_empty());
    }

    #[test]
    fn act_increases_reputation_via_dynamic_formula() {
        let mut steward = Steward::new();
        parse(r#"birth "luna""#).unwrap().into_iter()
            .for_each(|s| { steward.dispatch(s).unwrap(); });
        let before = steward.reputation();
        let stmts = parse(r#"act "web_search" "query""#).unwrap();
        steward.dispatch(stmts[0].clone()).unwrap();
        let after = steward.reputation();
        assert!(after > before);
        // At rep ~0, gain should be close to ACT_TIER_0 base (0.040)
        // not a flat +2
        assert!(after - before < 0.1);
        assert!(after - before > 0.0);
    }

    #[test]
    fn reputation_gain_decreases_as_rep_grows() {
        let mut steward = Steward::new();
        parse(r#"birth "luna""#).unwrap().into_iter()
            .for_each(|s| { steward.dispatch(s).unwrap(); });
        // Gain at low rep
        let stmts = parse(r#"act "web_search" "q""#).unwrap();
        steward.dispatch(stmts[0].clone()).unwrap();
        let gain_low = steward.reputation();
        // Manually set rep to 50 to simulate higher reputation
        steward.set_reputation_for_test(50.0);
        let before_high = steward.reputation();
        let stmts2 = parse(r#"act "web_search" "q""#).unwrap();
        steward.dispatch(stmts2[0].clone()).unwrap();
        let gain_high = steward.reputation() - before_high;
        assert!(gain_low > gain_high);
    }

    #[test]
    fn act_rejected_for_tool_above_current_tier() {
        let mut steward = Steward::new();
        parse(r#"birth "luna""#).unwrap().into_iter()
            .for_each(|s| { steward.dispatch(s).unwrap(); });
        // agent_orchestration requires Tier 4 (rep >= 80)
        // new agent is Tier 0
        let stmts = parse(r#"act "agent_orchestration" "task""#).unwrap();
        let result = steward.dispatch(stmts[0].clone());
        assert!(result.is_err());
    }

    #[test]
    fn reputation_decay_on_inactivity() {
        let mut steward = Steward::new();
        parse(r#"birth "luna""#).unwrap().into_iter()
            .for_each(|s| { steward.dispatch(s).unwrap(); });
        steward.set_reputation_for_test(10.0);
        steward.apply_daily_decay(1); // 1 day inactive
        assert!(steward.reputation() < 10.0);
        assert!((steward.reputation() - 9.992).abs() < 0.001);
    }

    #[test]
    fn reputation_cannot_go_below_zero() {
        let mut steward = Steward::new();
        parse(r#"birth "luna""#).unwrap().into_iter()
            .for_each(|s| { steward.dispatch(s).unwrap(); });
        steward.set_reputation_for_test(0.001);
        steward.apply_daily_decay(100); // massive inactivity
        assert_eq!(steward.reputation(), 0.000);
    }

    #[test]
    fn multi_statement_executes_in_order() {
        let mut steward = Steward::new();
        let input = r#"birth "luna"think "hello"act "web_search" "query""#;
        let stmts = parse(input).unwrap();
        assert_eq!(stmts.len(), 3);
        for stmt in stmts {
            steward.dispatch(stmt).unwrap();
        }
        // Reputation must have increased (from the act)
        assert!(steward.reputation() > 0.0);
    }

    #[test]
    fn steward_state_persists_between_dispatches() {
        let mut steward = Steward::new();
        parse(r#"birth "luna""#).unwrap().into_iter()
            .for_each(|s| { steward.dispatch(s).unwrap(); });
        parse(r#"act "web_search" "first""#).unwrap().into_iter()
            .for_each(|s| { steward.dispatch(s).unwrap(); });
        let rep_after_first = steward.reputation();
        parse(r#"act "web_search" "second""#).unwrap().into_iter()
            .for_each(|s| { steward.dispatch(s).unwrap(); });
        let rep_after_second = steward.reputation();
        assert!(rep_after_second > rep_after_first);
    }
}
