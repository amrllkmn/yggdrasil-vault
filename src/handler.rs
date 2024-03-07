pub async fn decrypt() -> String {
    String::from("Hello World")
}

#[cfg(test)]
#[tokio::test]
async fn test_decrypt() {
    let received_string = decrypt().await;
    assert_eq!(received_string, "Hello World");
}

#[tokio::test]
async fn test_decrypt_should_be_neq() {
    let received_string = decrypt().await;
    assert_ne!(received_string, "Not equal to this");
}
