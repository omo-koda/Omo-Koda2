#[cfg(test)]
mod hermetic_tests {
    use omokoda_hermetic::HermeticState;

    #[test]
    fn same_seed_produces_same_state() {
        let seed = [0u8; 32];
        let a = HermeticState::from_odu_seed(&seed);
        let b = HermeticState::from_odu_seed(&seed);
        assert_eq!(a.fingerprint(), b.fingerprint());
        assert_eq!(a.mentalism(), b.mentalism());
        assert_eq!(a.gender(), b.gender());
    }

    #[test]
    fn different_seed_produces_different_state() {
        let seed1 = [1u8; 32];
        let seed2 = [2u8; 32];
        let a = HermeticState::from_odu_seed(&seed1);
        let b = HermeticState::from_odu_seed(&seed2);
        assert_ne!(a.fingerprint(), b.fingerprint());
    }

    #[test]
    fn fingerprint_is_fixed_length() {
        let state = HermeticState::from_odu_seed(&[0u8; 32]);
        assert_eq!(state.fingerprint().len(), 64); // 32 bytes as hex
    }

    #[test]
    fn principle_values_are_in_range() {
        let state = HermeticState::from_odu_seed(&[0u8; 32]);
        assert!((0.0..=1.0).contains(&state.mentalism()));
        assert!((0.0..=1.0).contains(&state.correspondence()));
        assert!((0.0..=1.0).contains(&state.vibration()));
        assert!((0.0..=1.0).contains(&state.polarity()));
        assert!((0.0..=1.0).contains(&state.rhythm()));
        assert!((0.0..=1.0).contains(&state.cause_effect()));
        assert!((0.0..=1.0).contains(&state.gender()));
    }

    #[test]
    fn think_policy_returns_valid_depth() {
        let state = HermeticState::from_odu_seed(&[1u8; 32]);
        let depth = state.think_abstraction_depth();
        assert!((0.0..=1.0).contains(&depth));
        assert_eq!(depth, state.mentalism());
    }

    #[test]
    fn act_cooldown_is_deterministic() {
        let seed = [3u8; 32];
        let state = HermeticState::from_odu_seed(&seed);
        let same_state = HermeticState::from_odu_seed(&seed);
        assert_eq!(state.act_cooldown_ms(), same_state.act_cooldown_ms());
    }
}
