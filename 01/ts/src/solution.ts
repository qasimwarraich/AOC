import fs from 'fs';

const file = fs.readFileSync('./input.txt').toString('utf8');
let elftotals: number[] = [];

file.split('\n\n').forEach((e) => {
    elftotals.push(e.split('\n').reduce((acc, obj) => acc + Number(obj), 0));
});

//part 1
console.log(Math.max(...elftotals));
//part 2
console.log(
    elftotals
        .sort((n1, n2) => n1 - n2)
        .slice(-3)
        .reduce((a, b) => a + b)
);
