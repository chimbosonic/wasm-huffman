async function main(){
	const huffman = await import("../pkg/index.js").catch(console.error);

	console.log("Testing addition: " + huffman.add(3,7));

	function print_addition_on_click() {
		const button = document.getElementById("addition");
	
		button.addEventListener('click', (event) => {
			console.log(huffman.add(10,10)); 
		})
	}

	print_addition_on_click();

}

main();