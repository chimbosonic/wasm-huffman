use wasm_huffman::*;
use std::collections::HashMap;

// All public functions must be tested here. One test per function unless impossible.

#[test]
fn test_add(){
    assert_eq!(10,add(1,9));
}

#[test]
fn test_huffman_encode(){
    let input_data = "My super test string".to_string();

    let expected_data_encoded_string = "b6bcefa0bec4df94d157".to_string();
    let mut expected_data_encoding_map: HashMap<char, String> = HashMap::new();
    expected_data_encoding_map.insert('M',"0110".to_string());
    expected_data_encoding_map.insert('g',"0111".to_string());
    expected_data_encoding_map.insert(' ',"111".to_string());
    expected_data_encoding_map.insert('y',"1100".to_string());
    expected_data_encoding_map.insert('u',"11011".to_string());
    expected_data_encoding_map.insert('p',"11010".to_string());
    expected_data_encoding_map.insert('e',"000".to_string());
    expected_data_encoding_map.insert('n',"0101".to_string());
    expected_data_encoding_map.insert('t',"101".to_string());
    expected_data_encoding_map.insert('r',"001".to_string());
    expected_data_encoding_map.insert('i',"0100".to_string());
    expected_data_encoding_map.insert('s',"100".to_string());

    let test_output = huffman_encode(&input_data);

    assert_eq!(expected_data_encoded_string,test_output.encoded_data);
    assert_eq!(expected_data_encoding_map,test_output.encoding_map);
}

#[test]
fn test_huffman_decode(){
    let input_encoded_string = "b6bcefa0bec4df94d157".to_string();
    let mut input_encoding_map: HashMap<char, String> = HashMap::new();
    input_encoding_map.insert('M',"0110".to_string());
    input_encoding_map.insert('g',"0111".to_string());
    input_encoding_map.insert(' ',"111".to_string());
    input_encoding_map.insert('y',"1100".to_string());
    input_encoding_map.insert('u',"11011".to_string());
    input_encoding_map.insert('p',"11010".to_string());
    input_encoding_map.insert('e',"000".to_string());
    input_encoding_map.insert('n',"0101".to_string());
    input_encoding_map.insert('t',"101".to_string());
    input_encoding_map.insert('r',"001".to_string());
    input_encoding_map.insert('i',"0100".to_string());
    input_encoding_map.insert('s',"100".to_string());
    let input_data = HuffmanData{encoded_data: input_encoded_string,encoding_map: input_encoding_map};

    let expected_data = "My super test string".to_string();

    let test_output: String = huffman_decode(&input_data);

    assert_eq!(expected_data,test_output);
}

