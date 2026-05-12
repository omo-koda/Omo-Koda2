#[cfg(test)]
mod pet_tests {
    use omokoda_core::identity::pet::{PetIdentity, MASKS, MOODS};
    use omokoda_core::identity::odu::OduIdentity;
    use omokoda_hermetic::HermeticState;

    #[test]
    fn pet_has_31_masks() {
        assert_eq!(MASKS.len(), 31);
    }

    #[test]
    fn pet_derivation_is_hermetic_driven() {
        let odu = OduIdentity {
            primary_index: 0,
            mnemonic: "test".to_string(),
        };
        
        let h1 = HermeticState::from_seed("agent1", 123456);
        let h2 = HermeticState::from_seed("agent2", 654321);

        let pet1 = PetIdentity::derive(&odu, &h1, 0);
        let pet2 = PetIdentity::derive(&odu, &h2, 0);

        assert!(MOODS.contains(&pet1.mood.as_str()));
        assert!(MOODS.contains(&pet2.mood.as_str()));
    }

    #[test]
    fn pet_mask_evolves_with_tier() {
        let odu = OduIdentity {
            primary_index: 0,
            mnemonic: "test".to_string(),
        };
        let h = HermeticState::from_seed("agent", 123);

        let pet_tier0 = PetIdentity::derive(&odu, &h, 0);
        let pet_tier1 = PetIdentity::derive(&odu, &h, 1);

        assert_ne!(pet_tier0.mask, pet_tier1.mask);
        assert_eq!(pet_tier0.mask, MASKS[0]);
        assert_eq!(pet_tier1.mask, MASKS[1]);
    }

    #[test]
    fn pet_mood_evolves_with_tier_and_hermetic() {
        let odu = OduIdentity {
            primary_index: 0,
            mnemonic: "test".to_string(),
        };
        let h = HermeticState::from_seed("agent", 123);

        let pet_tier0 = PetIdentity::derive(&odu, &h, 0);
        let pet_tier1 = PetIdentity::derive(&odu, &h, 1);

        assert_ne!(pet_tier0.mood, pet_tier1.mood);
    }
}
