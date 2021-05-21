async function main(){
	const huffman = await import("../pkg/index.js").catch(console.error);

	function user_input() {
		const encode_data = document.getElementById("encode_data");
		const encoding_map = document.getElementById("encoding_map");
		const decode_data = document.getElementById("decode_data");
		const button_encode = document.getElementById("encode");
		const button_decode = document.getElementById("decode");

		button_encode.addEventListener('click', (event) => {
			console.log("Encoding");
			data_encoded = huffman.encode(decode_data.value);
			
			encode_data.value = data_encoded.get_data();
			encoding_map.value = data_encoded.get_map();
		})

		button_decode.addEventListener('click', (event) => {
			console.log("Decoding");
			data_decoded = huffman.decode(huffman.Huffdata.new(encode_data.value,encoding_map.value));
			
			decode_data.value = data_decoded;
		})
	}

	user_input();
}

main();