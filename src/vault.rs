tonic::include_proto!("vault"); // import the generated grpc package from running build.rs

pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("vault_descriptor"); // import the file descriptor for reflection

pub fn tokenize_string(req: TokenizeRequest) -> TokenizeResponse {
    let tokenized_field = req.field + "test";
    let tokenized_value = req.value + "test";

    TokenizeResponse {
        tokenized_field,
        tokenized_value,
    }
}

pub fn detokenize_string(req: DetokenizeRequest) -> DetokenizeResponse {
    DetokenizeResponse {
        detokenized_field: req.field,
        detokenized_value: req.value,
    }
}

#[cfg(test)]
#[test]
fn tokenize_should_add_string() {
    let req: TokenizeRequest = TokenizeRequest {
        value: String::from("Hello"),
        field: String::from("Strange"),
    };

    let TokenizeResponse {
        tokenized_value,
        tokenized_field,
    } = tokenize_string(req);

    assert!(tokenized_field.contains("test"));
    assert!(tokenized_value.contains("test"));
}

#[test]
fn detokenize_does_nothing() {
    let req: DetokenizeRequest = DetokenizeRequest {
        value: String::from("Something"),
        field: String::from("Good"),
    };

    assert_eq!(req.field, detokenize_string(req.clone()).detokenized_field);
    assert_eq!(req.value, detokenize_string(req.clone()).detokenized_value);
}
