#[cfg(test)]
mod privacy_tests {
    use omokoda_core::providers::{ProviderClass, ProviderMetadata, ProviderRegistry};

    #[tokio::test]
    async fn allows_local_in_private() {
        let registry = ProviderRegistry::new();
        let ollama = ProviderMetadata {
            name: "Ollama".to_string(),
            class: ProviderClass::Local,
            endpoint: "http://localhost:11434".to_string(),
        };
        // validate_think was removed, we use route_think now or just check is_allowed_in_private
        assert!(registry.is_allowed_in_private(&ollama));
    }

    #[tokio::test]
    async fn blocks_external_in_private() {
        let registry = ProviderRegistry::new();
        let claude = ProviderMetadata {
            name: "Claude".to_string(),
            class: ProviderClass::External,
            endpoint: "https://api.anthropic.com".to_string(),
        };
        assert!(!registry.is_allowed_in_private(&claude));
    }

    #[tokio::test]
    async fn hard_fail_on_no_local_in_private() {
        let registry = ProviderRegistry::new();
        // Ollama will fail (unless running on localhost, which it might be, but we can't guarantee)
        // We'll use with_mock to test the error path if we want, or just assume localhost is down.
        
        let result = registry.route_think("hello", &[], true).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("HARD FAIL"));
    }
}
