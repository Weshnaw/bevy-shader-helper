use bevy_shazzy::internals::prelude::ShaderEntry;

#[test]
fn test_entry_macro() {
    #[derive(ShaderEntry)]
    enum TestEntry {
        Main,
        Update
    }

    assert_eq!(TestEntry::Main.as_key(), 0);    
    assert_eq!(TestEntry::Update.as_key(), 1);    
}