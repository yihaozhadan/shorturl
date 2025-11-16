use shorturl::services::url_shortener::InMemoryUrlStore;

#[test]
fn get_or_create_is_idempotent_for_same_url() {
    let store = InMemoryUrlStore::new();

    let url = "https://example.com/abc";

    let first = store.get_or_create(url);
    let second = store.get_or_create(url);

    assert_eq!(first.short_code, second.short_code);
    assert_eq!(first.long_url, second.long_url);
}
