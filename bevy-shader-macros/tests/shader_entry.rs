use bevy_shader_macros::ShaderEntry;

#[test]
fn test_macro() {
    trait ShaderEntry {
        fn as_key(&self) -> usize;
    }

    #[derive(ShaderEntry)]
    enum TestEntry {
        Main,
        Update
    }

    assert_eq!(TestEntry::Main.as_key(), 0);    
    assert_eq!(TestEntry::Update.as_key(), 1);    
}