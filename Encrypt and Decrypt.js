const CharList = [
	"0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
	"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
	"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
	"`", "¬", "!", "\"", "£", "$", "%", "^", "&", "*", "(", ")", "€", "-", "=", "_", "+", "[", "]", "{", "}", ";", ":", "'", "@", "#", "~", ",", ".", "<", ">", "/", "?", "\\", "|",
	"á", "é", "í", "ó", "ú", "ý", "à", "è", "ì", "ò", "ù", "ä", "ë", "ï", "ö", "ü", "â", "ê", "î", "ô", "û",
	"Á", "É", "Í", "Ó", "Ú", "Ý", "À", "È", "Ì", "Ò", "Ù", "Ä", "Ë", "Ï", "Ö", "Ü", "Â", "Ê", "Î", "Ô", "Û",
	" ", "\n", "\t",
	"←", "↑", "→", "↓", "↔", "↕", "↖", "↗", "↘", "↙", "↻", "➔", "➜", "➞", "➟", "➠", "➡",
	"■", "□", "▢", "▣", "▤", "▥", "▦", "▧", "▨", "▩", "▪", "▫", "▬", "▮", "▯", "▰", "▱", "▲", "△", "▴",
	"♠", "♡", "♢", "♣", "♤", "♥", "♦", "♧", "♈", "♉", "♊", "♋", "♌", "♍", "♎", "♏", "♐", "♑", "♒", "♓",
	"π", "Σ", "Δ", "Ω", "∞", "√", "≈", "≠", "≤", "≥", "±", "÷", "×", "∫", "∂", "∇", "∅", "∈", "∉", "∏", "⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹", "·",
	"★", "☆", "☇", "☈", "☉", "☊", "☋", "☌", "☍", "☎", "☏", "☐", "☑", "☒", "☓", "☕", "♿", "☤", "☥", "☦", "☧", "☨", "☩", "☪", "☫", "☬"
];

const input = document.querySelector('#userInput');
const output = document.querySelector('#output');

document.querySelector('#toBin').addEventListener('click', () => {
	const val = input.value;
	const shift = Math.floor(Math.random() * 256);
	let outputBits = [shift.toString(2).padStart(8, '0')];
	for (let i = 0; i < val.length; i++) {
		const listIndex = CharList.indexOf(val[i]);
		if (listIndex === -1) {
			outputBits.push(val[i]);
		} else {
			const shiftedIndex = (listIndex + shift) % 256;
			outputBits.push(shiftedIndex.toString(2).padStart(8, '0'));
		}
	}
	output.textContent = outputBits.join("");
});

document.querySelector('#toChar').addEventListener('click', () => {
	const val = input.value;
	if (val.length < 8) return;
	const shift = parseInt(val.substring(0, 8), 2);
	let decodedText = "";
	let i = 8;
	while (i < val.length) {
		if (val[i] === "0" || val[i] === "1") {
			const chunk = val.substring(i, i + 8);
			const shiftedIndex = parseInt(chunk, 2);
			const originalIndex = (shiftedIndex - shift + 256) % 256;
			decodedText += CharList[originalIndex] || "";
			i += 8;
		} else {
			decodedText += val[i];
			i++;
		}
	}
	output.textContent = decodedText;
});

document.querySelector('#copyBtn').addEventListener('click', () => {
	navigator.clipboard.writeText(output.textContent).then(() => {
		alert("Copied!");
	});
});