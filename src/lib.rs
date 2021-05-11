extern crate hex;
extern crate serde;

use wasm_bindgen::prelude::*;
use self::serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
pub struct HuffmanData {
    pub encoded_data: String,
    pub encoding_map: HashMap<char,String>,
}

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    freq: i64,
    value: Option<char>,
}



#[wasm_bindgen]
pub fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}


//Takes a Hex String and a HashMap containing the Encoding map
//Huffman Decodes it using the encoding map returns a string
pub fn huffman_decode(huffman_encoded_data: &HuffmanData) -> String {
    let encoded_data_u8_vec = hex_string_to_u8_vec(&huffman_encoded_data.encoded_data);
    let encoded_data_bin_string_padded = u8_vec_to_bin_string(&encoded_data_u8_vec);
    let encoded_data_bin_string = unpad_encoded_data(&encoded_data_bin_string_padded);
    let decoded_data = huffman_decode_bin_string(&encoded_data_bin_string,&huffman_encoded_data.encoding_map);
    
    return decoded_data;
}

//Takes a String and Huffman Encodes it returning a Hex version of the string and a HashMap containing the Encoding map
pub fn huffman_encode(data: &String,debug: bool) -> HuffmanData {
    let frequency_map = build_frequency_map(&data);
    let huffman_tree = build_huffman_tree(&frequency_map);
    let mut encoding_map:HashMap<char, String> = HashMap::new();
    build_encoding_map(&huffman_tree,&mut encoding_map,"".to_string());
    let encoded_data_bin = huffman_encode_string(&data,&encoding_map);
    let padded_encoded_data_bin = pad_encoded_data(&encoded_data_bin);
    let encoded_data_u8_vec = bin_string_to_u8_vec(&padded_encoded_data_bin);
    let encoded_data_hex = u8_vec_to_hex_string(&encoded_data_u8_vec);
    let huffman_encoded_data = HuffmanData{encoded_data: encoded_data_hex,encoding_map: encoding_map};

    if debug {
        print_data(&data,"Provided String to encode".to_string());
        print_frequency_map(&frequency_map);
        print_node(&huffman_tree,true);
        println!("Before Padding:");
        print_stats(&data,&encoded_data_bin);
        println!("After Padding:");
        print_stats(&data,&padded_encoded_data_bin);
        print_huffman_encoded_data(&huffman_encoded_data);
    }

    return huffman_encoded_data;
}
    


//All of our builders
//TESTED
//Creates a HashMap containing Nodes with the frequency of every char in given String
fn build_frequency_map(data: &String) -> HashMap<char, i64> {
    let mut frequency_map: HashMap<char, i64> = HashMap::new();
    for character in data.chars() {
        match frequency_map.get_mut(&character){
            Some(result) => {
                *result = *result + 1;
            }
            None => {
                frequency_map.insert(character, 1);
            }
        }
    }

    return frequency_map;
}

//TESTED
//Creates a a Huffman Coding Tree with given Frequency Map
// We sort the frequency list alphabetically then we sort it by frequency to give us consitancy in the tree we generate
fn build_huffman_tree(frequency_map: &HashMap<char, i64>) -> Node {
    //Create a Vector of Nodes containing each char and their frequency
    let mut freq_list: Vec<Node> = Vec::new();
    for (data, freq) in frequency_map {
        freq_list.push(Node{left: None, right: None,value: Some(*data),freq: *freq});
    }
    //Sort the Vector
    freq_list.sort_by(|a, b| b.value.cmp(&a.value));
    freq_list.sort_by(|a, b| b.freq.cmp(&a.freq));
    
    while freq_list.len() != 1 {
        let  left_node = freq_list.pop().unwrap();
        let  right_node = freq_list.pop().unwrap();
        let  new_node_freq = left_node.freq + right_node.freq;
        let  new_node = Node{left: Some(Box::new(left_node)), right: Some(Box::new(right_node)),value: None,freq: new_node_freq};
        freq_list.push(new_node);
        freq_list.sort_by(|a, b| b.freq.cmp(&a.freq));
    }
    return freq_list.pop().unwrap();
}

//TESTED
//Creates a Hash Map of the encoding of every char within a given Huffman Tree. Left node edges are 0s and right node edges are 1s
fn build_encoding_map(node: &Node,encoding_map: &mut HashMap<char, String>,code: String){
    match node.value {
        Some(value) => {
            encoding_map.insert(value, code);
        }
        None => {
            match &node.left {
                Some(left) => {
                    build_encoding_map(left,encoding_map,code.clone() + "0");
                }
                None => {}
            }
            match &node.right {
                Some(right) => {
                    build_encoding_map(right,encoding_map,code.clone() + "1");
                }
                None => {}
            }
        }
    }

}


// TESTED
//Decodes a Binary string to a Vector of u8 ==> Tested by test_binary_string_encoder_decoder
fn bin_string_to_u8_vec(bin_string: &String) -> Vec<u8>{
    let mut temp_byte: String = String::new();
    let mut u8_vec: Vec<u8> = Vec::new();

    for bit in bin_string.chars() {
        if temp_byte.len() == 8 {
            let u8_byte = u8::from_str_radix(temp_byte.as_str(), 2).unwrap();
            u8_vec.push(u8_byte);
            temp_byte = "".to_string();
        }
        temp_byte.push(bit);    
    }
    let u8_value = u8::from_str_radix(temp_byte.as_str(), 2).unwrap();
    u8_vec.push(u8_value);
    return u8_vec
}

// TESTED
//Encodes a Vector of u8 to a Binary string ==> Tested by test_binary_string_encoder_decoder
fn u8_vec_to_bin_string(u8_vec: &Vec<u8>) -> String{
    let mut bin_string: String = String::new();
    for byte in u8_vec {
        bin_string = bin_string + format!("{:b}", byte).as_str();
    }
    return bin_string;
}

// TESTED
// Decodes a hex string to  a Vector containing u8's ==> Tested by test_hex_encoder_decoder
fn hex_string_to_u8_vec(s: &String) -> Vec<u8> {
    match hex::decode(s) {
        Ok(result) => {
            let u8_vec: Vec<u8> = result;
            return u8_vec;
        },
        Err(error) => {
            panic!("{}",error);
        }
    }
}

// TESTED
//Encodes a Vector containing u8's to a hex string ==> Tested by test_hex_encoder_decoder
fn u8_vec_to_hex_string(u8_vec: &Vec<u8>) -> String{
    let hex_string: String = hex::encode(u8_vec);
    return hex_string;
}


// TESTED
//Pads a given binary string by prefixing a 1 to every 7 bits ==> Tested by test_pad_encoded_data
fn pad_encoded_data(encoded_data: &String) -> String {
    let mut padded_encoded_data: String = String::new();
    let mut temp_padded_byte: String = "1".to_string();
    
    for bit in encoded_data.chars() {
        if temp_padded_byte.len() > 7 {
            padded_encoded_data = padded_encoded_data + temp_padded_byte.as_str();
            temp_padded_byte = "1".to_string();
            
        } 
        temp_padded_byte = temp_padded_byte + &bit.to_string();
    }
    padded_encoded_data = padded_encoded_data + temp_padded_byte.as_str();
    return padded_encoded_data;
}

// TESTED
//Removes padding
fn unpad_encoded_data(padded_data: &String) -> String {
    let mut data: String = String::new();
    let mut temp_padded_byte: String =  String::new();
    
    for bit in padded_data.chars() {
        if temp_padded_byte.len() > 7 {
            let(_,byte) = temp_padded_byte.split_at(1);
            data = data + byte;
            temp_padded_byte = String::new();
        }
        temp_padded_byte = temp_padded_byte + &bit.to_string();
    }
    let(_,byte) = temp_padded_byte.split_at(1);
    data = data + byte;
    return data;
}


// TESTED
//Encodes string with given HashMap
fn huffman_encode_string(data: &String,encoding_map: &HashMap<char, String>) -> String {
    let mut encoded_data = String::new();
    for c in data.chars() {
        match encoding_map.get(&c) {
            Some(code) => {
                encoded_data = encoded_data + code;
            }
            None =>{}
        }
    }
    return encoded_data;
}

// TESTED
//Decodes Huffman encoded binary string using provided encoding HashMap
fn huffman_decode_bin_string(encoded_data: &String,encoding_map: &HashMap<char, String>) -> String{
    let inverted_encoding_map = invert_encoding_map(&encoding_map);
    let mut data = String::new();
    let mut temp_code = String::new();
    let mut encoded_data_rev = encoded_data.chars().rev().collect::<String>();
    loop {
        match inverted_encoding_map.get(&temp_code) {
            Some(byte) => {
                temp_code = "".to_string();
                data.push(*byte);
            }
            None =>{
                match encoded_data_rev.pop() {
                    Some(code) => {
                        temp_code.push(code);
                    }
                    None => {
                        break;
                    }
                }               
            }
        }
        
    }
    return data;
}

//TESTED
//Inverts Keys and values for a given Encoding Map
fn invert_encoding_map(encoding_map: &HashMap<char, String>) -> HashMap<String, char>{
    let mut inverted_encoding_map: HashMap<String, char> = HashMap::new();

    for (key,value) in encoding_map {
        inverted_encoding_map.insert(value.to_owned(),*key);
    }
    return inverted_encoding_map;
}


//All of our Printing functions

fn print_frequency_map(frequency_map: &HashMap<char,i64>){
    println!("The frequency map:");
    println!("\"{:?}\"",frequency_map);
    println!("");
}

fn print_data(data: &String,title: String){
    println!("{}:",title);
    println!("\"{}\"",data);
    println!("");
}

fn print_huffman_encoded_data(huffman_encoded_data: &HuffmanData){
    let json = serde_json::to_string(&huffman_encoded_data).unwrap();
    println!("The Huffman encoded data:");
    println!("{}",json);
    println!("");
}

fn print_stats(data: &String,encoded_data: &String){
    let data_size = (data.len() * 8) as f32;
    let encoded_size = encoded_data.chars().count() as f32;
    let ratio = (1 as f32 - ( encoded_size / data_size ) as f32) * 100 as f32 ;
    
    println!("Stats:");
    println!("Data size in bits {}",data_size);
    println!("Encoded data size in bits {}",encoded_size);
    println!("Compression Ratio is {}%", ratio);
    println!("");
}

//Prints a node and all of its children as a Json Object
fn print_node(node: &Node,is_root: bool) {
    if is_root {
        println!("The Huffman tree as a JSON object:");
        print!("{{");
    }

    print!("\"frequency\": {},",node.freq);
    match node.value {
        Some(value) => {
            print!("\"value\": \"{}\"",value);
        }
        None => {
            print!("\"value\": \"\"");
        }
    }
    match &node.left {
        Some(left) => {
            print!(",\"left\":{{");
            print_node(&*left,false);
            print!("}}");
        }
        None => {}
    }
    match &node.right {
        Some(right) => {
            print!(",\"right\":{{");
            print_node(&*right,false);
            print!("}}");
        }
        None => {}
    }
    
    if is_root {
        print!("}}");
        println!("");
        println!("");
    }
}


// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_encoded_data(){
        let input_data = "1011100101010000010100000110100101110101001010011011111000111001111011101001001010111010111111100001100".to_string();

        let expected_data = "1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100".to_string();
    
        let test_output = pad_encoded_data(&input_data);

        assert_eq!(expected_data,test_output);
    }

    #[test]
    fn test_unpad_encoded_data(){
        let input_data = "1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100".to_string();

        let expected_data = "1011100101010000010100000110100101110101001010011011111000111001111011101001001010111010111111100001100".to_string();
        
        let test_output = unpad_encoded_data(&input_data);

        assert_eq!(expected_data,test_output);
    }

    #[test]
    fn test_hex_string_to_u8_vec() {
        let input_data = "746869732069732061207465737420666f72206d7920737472696e6721".to_string();

        let expected_data: Vec<u8> = vec![116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32, 102, 111, 114, 32, 109, 121, 32, 115, 116, 114, 105, 110, 103, 33];

        let test_output = hex_string_to_u8_vec(&input_data);

        assert_eq!(expected_data,test_output);
    }

    #[test]
    fn test_u8_vec_to_hex_string() {
        let input_data: Vec<u8> = vec![116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32, 102, 111, 114, 32, 109, 121, 32, 115, 116, 114, 105, 110, 103, 33];

        let expected_data = "746869732069732061207465737420666f72206d7920737472696e6721".to_string();

        let test_output = u8_vec_to_hex_string(&input_data);

        assert_eq!(expected_data,test_output);
    }

    #[test]
    fn test_bin_string_to_u8_vec(){
        let input_data = "1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100".to_string();

        let expected_data: Vec<u8> = vec![220, 212, 138, 134, 203, 212, 211, 190, 156, 251, 210, 171, 215, 248, 44];

        let test_output = bin_string_to_u8_vec(&input_data);

        assert_eq!(expected_data,test_output);
    }

    #[test]
    fn test_u8_vec_to_bin_string(){
        let input_data: Vec<u8> = vec![220, 212, 138, 134, 203, 212, 211, 190, 156, 251, 210, 171, 215, 248, 44];

        let expected_data = "1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100".to_string();
       
        let test_output = u8_vec_to_bin_string(&input_data);

        assert_eq!(expected_data,test_output);
    }

    #[test]
    fn test_build_frequency_map(){
        let input_data: String = "this is a test string!".to_string();

        let mut expected_data: HashMap<char, i64> = HashMap::new();
        expected_data.insert('h',1);
        expected_data.insert('a',1);
        expected_data.insert(' ',4);
        expected_data.insert('g',1);
        expected_data.insert('i',3);
        expected_data.insert('s',4);
        expected_data.insert('!',1);
        expected_data.insert('n',1);
        expected_data.insert('r',1);
        expected_data.insert('t',4);
        expected_data.insert('e',1);
       
        let test_output = build_frequency_map(&input_data);

        assert_eq!(expected_data,test_output);
    }

    #[test]
    fn test_build_huffman_tree_build_encoding_map(){
        let mut input_data: HashMap<char, i64> = HashMap::new();
        input_data.insert('h',1);
        input_data.insert('a',1);
        input_data.insert(' ',4);
        input_data.insert('g',1);
        input_data.insert('i',3);
        input_data.insert('s',4);
        input_data.insert('!',1);
        input_data.insert('n',1);
        input_data.insert('r',1);
        input_data.insert('t',4);
        input_data.insert('e',1);

        let mut expected_data: HashMap<char, String> = HashMap::new();
        expected_data.insert('h',"10010".to_string());
        expected_data.insert('a',"0011".to_string());
        expected_data.insert(' ',"01".to_string());
        expected_data.insert('g',"0001".to_string());
        expected_data.insert('i',"101".to_string());
        expected_data.insert('s',"110".to_string());
        expected_data.insert('!',"0010".to_string());
        expected_data.insert('n',"10011".to_string());
        expected_data.insert('r',"1000".to_string());
        expected_data.insert('t',"111".to_string());
        expected_data.insert('e',"0000".to_string());

        // Create a huffman tree (Can't really test the output of this without coming up with a way to print it and build it manually)
        let test_output_tree = build_huffman_tree(&input_data);

        // Create a encoding map from the tree this we can test better
        let mut test_output:HashMap<char, String> = HashMap::new();
        build_encoding_map(&test_output_tree,&mut test_output,"".to_string());

        assert_eq!(expected_data,test_output);

    }


    #[test]
    fn test_invert_encoding_map(){
        let mut input_data: HashMap<char, String> = HashMap::new();
        input_data.insert('h',"10010".to_string());
        input_data.insert('a',"0011".to_string());
        input_data.insert(' ',"01".to_string());
        input_data.insert('g',"0001".to_string());
        input_data.insert('i',"101".to_string());
        input_data.insert('s',"110".to_string());
        input_data.insert('!',"0010".to_string());
        input_data.insert('n',"10011".to_string());
        input_data.insert('r',"1000".to_string());
        input_data.insert('t',"111".to_string());
        input_data.insert('e',"0000".to_string());

        let mut expected_data: HashMap<String, char> = HashMap::new();
        expected_data.insert("10010".to_string(),'h');
        expected_data.insert("0011".to_string(),'a');
        expected_data.insert("01".to_string(),' ');
        expected_data.insert("0001".to_string(),'g');
        expected_data.insert("101".to_string(),'i');
        expected_data.insert("110".to_string(),'s');
        expected_data.insert("0010".to_string(),'!');
        expected_data.insert("10011".to_string(),'n');
        expected_data.insert("1000".to_string(),'r');
        expected_data.insert("111".to_string(),'t');
        expected_data.insert("0000".to_string(),'e');
        
        let test_output = invert_encoding_map(&input_data);

        assert_eq!(expected_data,test_output);

    }

    #[test]
    fn test_huffman_encode_string(){
        let input_data: String = "this is a test string!".to_string();
        
        let mut input_encoding_map: HashMap<char, String> = HashMap::new();
        input_encoding_map.insert('h',"10010".to_string());
        input_encoding_map.insert('a',"0011".to_string());
        input_encoding_map.insert(' ',"01".to_string());
        input_encoding_map.insert('g',"0001".to_string());
        input_encoding_map.insert('i',"101".to_string());
        input_encoding_map.insert('s',"110".to_string());
        input_encoding_map.insert('!',"0010".to_string());
        input_encoding_map.insert('n',"10011".to_string());
        input_encoding_map.insert('r',"1000".to_string());
        input_encoding_map.insert('t',"111".to_string());
        input_encoding_map.insert('e',"0000".to_string());

        let expected_data: String = "11110010101110011011100100110111100001101110111011110001011001100010010".to_string();

        let test_output = huffman_encode_string(&input_data,&input_encoding_map);

        assert_eq!(expected_data,test_output);
    }
    
    #[test]
    fn test_huffman_decode_bin_string(){
        let input_data: String = "11110010101110011011100100110111100001101110111011110001011001100010010".to_string();
        let mut input_encoding_map: HashMap<char, String> = HashMap::new();
        input_encoding_map.insert('h',"10010".to_string());
        input_encoding_map.insert('a',"0011".to_string());
        input_encoding_map.insert(' ',"01".to_string());
        input_encoding_map.insert('g',"0001".to_string());
        input_encoding_map.insert('i',"101".to_string());
        input_encoding_map.insert('s',"110".to_string());
        input_encoding_map.insert('!',"0010".to_string());
        input_encoding_map.insert('n',"10011".to_string());
        input_encoding_map.insert('r',"1000".to_string());
        input_encoding_map.insert('t',"111".to_string());
        input_encoding_map.insert('e',"0000".to_string());

        let expected_data: String = "this is a test string!".to_string();


        let test_output = huffman_decode_bin_string(&input_data,&input_encoding_map);

        assert_eq!(expected_data,test_output);
    }
}