use local_community_feed_lib::crypto::{verify, Identity};

#[test]
fn test_sign_and_verify() {
    let identity = Identity::new();
    let message = "This is a test message.";
    let signature = identity.sign(message);
    let is_valid = verify(message, &signature, &identity.keypair.pk);
    assert!(is_valid);
}

#[test]
fn test_invalid_signature() {
    let identity1 = Identity::new();
    let identity2 = Identity::new();
    let message = "This is a test message.";
    let signature = identity1.sign(message);
    let is_valid = verify(message, &signature, &identity2.keypair.pk);
    assert!(!is_valid);
}

#[test]
fn test_username_generation() {
    let identity = Identity::new();
    assert!(!identity.username.is_empty());
}
