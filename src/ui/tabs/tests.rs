#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
            (
                "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4", // Native SegWit
                "Native SegWit (bech32)",
                json!({
                    "address": "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
                    "type": "witness_v0_keyhash",
                    "scriptPubKey": {
                        "type": "witness_v0_keyhash",
                        "address": "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"
                    }
                })
            ),
            (
                "bc1p38hzyl8p5yyqnzgkcxttr6ac0wc0ae8gpv7rld79df88qkrva38s78e8wd", // Taproot
                "Taproot (P2TR)",
                json!({
                    "address": "bc1p38hzyl8p5yyqnzgkcxttr6ac0wc0ae8gpv7rld79df88qkrva38s78e8wd",
                    "type": "taproot",
                    "scriptPubKey": {
                        "type": "taproot",
                        "address": "bc1p38hzyl8p5yyqnzgkcxttr6ac0wc0ae8gpv7rld79df88qkrva38s78e8wd"
                    }
                })
            ),
            (
                "0279BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", // Public Key
                "Public Key (Hex)",
                json!({
                    "address": "0279BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
                    "type": "pubkey",
                    "scriptPubKey": {
                        "type": "pubkey",
                        "asm": "0279BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798 OP_CHECKSIG"
                    }
                })
            )
        ];

        for (address, expected_type, mock_response) in test_cases {
            // Mock RPC Client erstellen
            let mock_client = MockBitcoinRPC::new()
                .expect_get_explorer_address()
                .with(eq(address))
                .return_once(move |_| Ok(mock_response));

            // AddressMode erstellen
            let mode = AddressMode {
                address: address.to_string()
            };

            // Render aufrufen
            let paragraph = render(Some(&mode), &Some(mock_client));

            // Prüfen ob der erwartete Adresstyp im Text enthalten ist
            let text = paragraph.lines.iter()
                .map(|line| line.spans.iter().map(|span| span.content.as_str()).collect::<String>())
                .collect::<Vec<String>>()
                .join("\n");

            assert!(text.contains(expected_type), 
                "Adresstyp '{}' nicht gefunden für Adresse {}", expected_type, address);
        }
    }

    // Mock für BitcoinRPC
    use mockall::*;
    mock! {
        BitcoinRPC {
            fn get_explorer_address(&self, address: &str) -> Result<Value, Error>;
        }
    }
} 