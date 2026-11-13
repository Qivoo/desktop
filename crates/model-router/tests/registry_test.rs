use super::*;

#[test]
fn test_model_info_creation() {
    let info = ModelInfo {
        id: "test-model".to_string(),
        provider: "test-provider".to_string(),
        name: "Test Model".to_string(),
        modalities: vec!["text".to_string(), "image".to_string()],
        supports_tools: true,
        free_tier: true,
        context_length: 32768,
        max_tokens: 4096,
        cooldown_ms: 0,
        last_successful_request: None,
        capability_tags: vec!["general".to_string(), "coding".to_string()],
    };

    assert!(info.is_free());
    assert!(info.supports_tools());
    assert!(info.supports_modality("text"));
    assert!(!info.supports_modality("audio"));
}

#[test]
fn test_registry_creation() {
    let registry = ModelRegistry::new("/tmp/test-registry.json".to_string());
    assert!(registry.list_all().is_empty());
}

#[test]
fn test_registry_insert_and_get() {
    let mut registry = ModelRegistry::new("/tmp/test-registry.json".to_string());
    let model = ModelInfo {
        id: "test-model".to_string(),
        provider: "test-provider".to_string(),
        name: "Test Model".to_string(),
        modalities: vec!["text".to_string()],
        supports_tools: false,
        free_tier: true,
        context_length: 32768,
        max_tokens: 8192,
        cooldown_ms: 0,
        last_successful_request: None,
        capability_tags: vec!["general".to_string()],
    };

    registry.insert(model);
    assert_eq!(registry.list_all().len(), 1);
    assert!(registry.get("test-model").is_some());
}

#[test]
fn test_registry_list_free_models() {
    let mut registry = ModelRegistry::new("/tmp/test-registry.json".to_string());
    registry.insert(ModelInfo {
        id: "free-model".to_string(),
        provider: "test-provider".to_string(),
        name: "Free Model".to_string(),
        modalities: vec!["text".to_string()],
        supports_tools: false,
        free_tier: true,
        context_length: 32768,
        max_tokens: 8192,
        cooldown_ms: 0,
        last_successful_request: None,
        capability_tags: vec!["general".to_string()],
    });

    registry.insert(ModelInfo {
        id: "paid-model".to_string(),
        provider: "test-provider".to_string(),
        name: "Paid Model".to_string(),
        modalities: vec!["text".to_string()],
        supports_tools: false,
        free_tier: false,
        context_length: 32768,
        max_tokens: 8192,
        cooldown_ms: 0,
        last_successful_request: None,
        capability_tags: vec!["general".to_string()],
    });

    let free_models = registry.list_free_models();
    assert_eq!(free_models.len(), 1);
    assert_eq!(free_models[0].id, "free-model");
}

#[test]
fn test_registry_list_models_with_tools() {
    let mut registry = ModelRegistry::new("/tmp/test-registry.json".to_string());
    registry.insert(ModelInfo {
        id: "tool-model".to_string(),
        provider: "test-provider".to_string(),
        name: "Tool Model".to_string(),
        modalities: vec!["text".to_string()],
        supports_tools: true,
        free_tier: true,
        context_length: 32768,
        max_tokens: 8192,
        cooldown_ms: 0,
        last_successful_request: None,
        capability_tags: vec!["general".to_string(), "coding".to_string()],
    });

    registry.insert(ModelInfo {
        id: "no-tool-model".to_string(),
        provider: "test-provider".to_string(),
        name: "No Tool Model".to_string(),
        modalities: vec!["text".to_string()],
        supports_tools: false,
        free_tier: true,
        context_length: 32768,
        max_tokens: 8192,
        cooldown_ms: 0,
        last_successful_request: None,
        capability_tags: vec!["general".to_string()],
    });

    let tool_models = registry.list_models_with_tools();
    assert_eq!(tool_models.len(), 1);
    assert_eq!(tool_models[0].id, "tool-model");
}

#[test]
fn test_registry_resolve_alias() {
    let mut registry = ModelRegistry::new("/tmp/test-registry.json".to_string());
    registry.insert(ModelInfo {
        id: "free-model".to_string(),
        provider: "test-provider".to_string(),
        name: "Free Model".to_string(),
        modalities: vec!["text".to_string()],
        supports_tools: false,
        free_tier: true,
        context_length: 32768,
        max_tokens: 8192,
        cooldown_ms: 0,
        last_successful_request: None,
        capability_tags: vec!["general".to_string()],
    });

    assert_eq!(registry.resolve_alias("openrouter/free"), Some("free-model".to_string()));
    assert_eq!(registry.resolve_alias("unknown"), None);
}

#[test]
fn test_model_info_from_openrouter() {
    let data = OpenRouterModelData {
        model_id: "test-model".to_string(),
        name: "Test Model".to_string(),
        context_length: Some(32768),
        context_window: Some(32768),
        modalities: vec!["text".to_string()],
        max_tokens: Some(8192),
        input: None,
        output: None,
        pricing: None,
        AAD: None,
        archived: None,
        color: None,
        config: None,
        created: None,
        credential: None,
        disabled: None,
        context_window_auto_adjust: None,
        do_text_embeddings: None,
        do_audio_transcription: None,
        do_text_to_speech: None,
        do_image_generation: None,
        do_object_detection: None,
        hardware: None,
        home: None,
        input_price: None,
        max_batch_size: None,
        max_image_input_size: None,
        max_completion_tokens: Some(8192),
        max_output_tokens: None,
        num_retries: None,
        owner: None,
        per_input_txn_price: None,
        per_minute: None,
        per_output_txn_price: None,
        prompt: None,
        quality: None,
        recency: None,
        revenue_share: None,
        sub: None,
        speed: None,
        status: Some("free".to_string()),
        supported_parameters: None,
        tunable: None,
        vendor: Some("test-provider".to_string()),
        vram: None,
    };

    let model_info = ModelInfo::from_openrouter(&data).unwrap();
    assert_eq!(model_info.id, "test-model");
    assert_eq!(model_info.provider, "test-provider");
    assert_eq!(model_info.name, "Test Model");
    assert!(model_info.is_free());
    assert_eq!(model_info.context_length, 32768);
    assert_eq!(model_info.max_tokens, 8192);
    assert_eq!(model_info.modalities, vec!["text".to_string()]);
}

#[test]
fn test_model_info_extract_tags() {
    let data = OpenRouterModelData {
        model_id: "test-model".to_string(),
        name: "Test Model".to_string(),
        context_length: Some(32768),
        context_window: Some(32768),
        modalities: vec!["text".to_string()],
        max_tokens: Some(8192),
        input: None,
        output: None,
        pricing: None,
        AAD: None,
        archived: None,
        color: None,
        config: None,
        created: None,
        credential: None,
        disabled: None,
        context_window_auto_adjust: None,
        do_text_embeddings: Some(true),
        do_audio_transcription: Some(true),
        do_text_to_speech: None,
        do_image_generation: None,
        do_object_detection: None,
        hardware: None,
        home: None,
        input_price: None,
        max_batch_size: None,
        max_image_input_size: None,
        max_completion_tokens: Some(8192),
        max_output_tokens: None,
        num_retries: None,
        owner: None,
        per_input_txn_price: None,
        per_minute: None,
        per_output_txn_price: None,
        prompt: None,
        quality: None,
        recency: None,
        revenue_share: None,
        sub: None,
        speed: None,
        status: Some("free".to_string()),
        supported_parameters: None,
        tunable: None,
        vendor: Some("test-provider".to_string()),
        vram: None,
    };

    let tags = ModelInfo::extract_tags(&data);
    assert!(tags.contains(&"text_embeddings".to_string()));
    assert!(tags.contains(&"audio".to_string()));
    assert!(!tags.contains(&"image_generation".to_string()));
}
