use crate::constant_time_compare;

#[test]
fn test_constant_time_comparison() {
    assert!(!constant_time_compare!(
        "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762fd75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be", 
        "56b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762fd75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be"
    ));

    assert!(constant_time_compare!(
        "The quick brown fox jumps over the lazy dog",
        "The quick brown fox jumps over the lazy dog"
    ));
}
