const path = require('path');
const fs = require('fs');

const SHAPES = {
	A: 1,
	X: 1,
	B: 2,
	Y: 2,
	C: 3,
	Z: 3,
};

const input = fs
	.readFileSync(path.join(__dirname, 'puzzle_input.txt'), 'utf8')
	.toString()
	.trim()
	.split('\n')
	.map((v) => v.split(' '));

const partOneRounds = input.map(([left_shape, right_shape]) => {
	const left = SHAPES[left_shape];
	const right = SHAPES[right_shape];
	const diff = Math.abs(left - right);

	if (left === right) {
		// Draw - A X, B Y, C Z
		return right + 3;
	} else if ((diff === 1 && right > left) || (right === 1 && left === 3)) {
		// Win - A Y, B Z, C X
		return right + 6;
	} else {
		// Lose - A Z, B X, C Y
		return right;
	}
});

const partTwoRounds = input.map(([left_shape, right_shape]) => {
	const left = SHAPES[left_shape];

	if (right_shape === 'X') {
		// Lose
		let right = left - 1 || 3; // If 0, loop to 3 (paper)
		return right;
	} else if (right_shape === 'Y') {
		// Draw
		return left + 3;
	} else {
		// Win
		let right = (left + 1) % 3 || 3; // If 0, loop to 3 (paper)
		return right + 6;
	}
});

console.log('Part One:', partOneRounds.reduce((a, b) => a + b, 0));
console.log('Part Two:', partTwoRounds.reduce((a, b) => a + b, 0));
