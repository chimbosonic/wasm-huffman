extern crate hex;
extern crate huff_tree_tap;

use huff_tree_tap::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Huffdata {
    data: String,
    map: String,
}

#[wasm_bindgen]
impl Huffdata {
    pub fn new(data: String, map: String) -> Huffdata {
        Huffdata {
            data: data,
            map: map,
        }
    }

    pub fn get_map(&self) -> String {
        self.map.clone()
    }

    pub fn get_data(&self) -> String {
        self.data.clone()
    }
}

#[wasm_bindgen]
pub fn encode(data: String) -> Huffdata {
    let encoded_data: HuffmanData = huffman_encode(&data.into_bytes());
    let hex_encoded_data = hex::encode(encoded_data.encoded_data);
    let encoding_map_string = serde_json::to_string(&encoded_data.encoding_map).unwrap();
    return Huffdata::new(hex_encoded_data, encoding_map_string);
}

#[wasm_bindgen]
pub fn decode(huffdata: Huffdata) -> String {
    let encoded_data = hex::decode(huffdata.get_data()).unwrap();
    let encoding_map_string = huffdata.get_map();
    let encoding_map: HashMap<u8, String> = serde_json::from_str(&encoding_map_string).unwrap();
    let decoded_data = huffman_decode(&HuffmanData {
        encoded_data: encoded_data,
        encoding_map: encoding_map,
    });

    return String::from_utf8_lossy(&decoded_data).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let input_data = "test".to_string();
        let input_encoded_data = "63".to_string();
        let input_map_string = "{\"101\":\"00\",\"116\":\"1\",\"115\":\"01\"}".to_string();
        let input_huffdata = Huffdata::new(input_encoded_data, input_map_string);

        let output_data = decode(input_huffdata);

        assert_eq!(input_data, output_data);
    }

    #[test]
    fn test_encode() {
        let input_data = "test".to_string();
        let input_map_string = "{\"101\":\"00\",\"116\":\"1\",\"115\":\"01\"}".to_string();
        let input_map: HashMap<u8, String> = serde_json::from_str(&input_map_string).unwrap();
        let input_encoded_data = "63".to_string();

        let output_huffdata = encode(input_data);
        let output_encoded_data = output_huffdata.get_data();
        let output_map_string = output_huffdata.get_map();
        let output_map: HashMap<u8, String> = serde_json::from_str(&output_map_string).unwrap();

        assert_eq!(input_encoded_data, output_encoded_data);

        assert_eq!(input_map, output_map);
    }

    #[test]
    fn test_huffdata() {
        let input_data = "test".to_string();
        let input_map = "test".to_string();

        let huffdata = Huffdata::new(input_data.clone(), input_map.clone());

        assert_eq!(input_data, huffdata.get_data());

        assert_eq!(input_data, huffdata.get_map());
    }

    #[test]
    fn test_huff_tree_tap() {
        let input_data: Vec<u8> = "this is a test string!".to_string().into_bytes();
        let test_huffman_data: HuffmanData = huffman_encode(&input_data);
        let test_output = huffman_decode(&test_huffman_data);

        assert_eq!(input_data, test_output)
    }
}
