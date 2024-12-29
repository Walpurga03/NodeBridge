use super::*;
use bitcoincore_rpc::Error;
use serde_json::json;
use std::collections::HashMap;

#[derive(Clone)]
struct MockBitcoinRPC {
    responses: HashMap<String, Value>,
}

impl MockBitcoinRPC {
    fn new() -> Self {
        Self {
            responses: HashMap::new(),
        }
    }

    fn expect_response(&mut self, address: &str, response: Value) {
        self.responses.insert(address.to_string(), response);
    }
}

impl ExplorerAddress for MockBitcoinRPC {
    fn get_explorer_address(&self, address: &str) -> Result<Value, Error> {
        Ok(self.responses.get(address)
            .cloned()
            .unwrap_or_else(|| json!({})))
    }
}

#[test]
fn test_address_types() {
    let test_cases = vec![
        (
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", // Legacy (Satoshi's Address)
            "Legacy (P2PKH)",
            json!({
                "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
                "type": "pubkeyhash",
                "scriptPubKey": {
                    "type": "pubkeyhash",
                    "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
                }
            })
        ),
        (
            "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy", // SegWit
            "Nested SegWit (P2SH)",
            json!({
                "address": "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy",
                "type": "scripthash",
                "scriptPubKey": {
                    "type": "scripthash",
                    "address": "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy"
                }
            })
        ),
        // ... weitere Testfälle ...
    ];

    for (address, expected_type, mock_response) in test_cases {
        let mut mock_client = MockBitcoinRPC::new();
        mock_client.expect_response(address, mock_response);

        let mode = AddressMode {
            address: address.to_string()
        };

        let paragraph = render(Some(&mode), &Some(mock_client));
        
        // Text aus dem Widget extrahieren
        let text = format!("{:?}", paragraph);

        assert!(text.contains(expected_type), 
            "Adresstyp '{}' nicht gefunden für Adresse {}", expected_type, address);
    }
} 