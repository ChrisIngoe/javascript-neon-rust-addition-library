const { add_ten, add_hundred, add_thousand } = require('./addition.node');

const value = process.argv[2] || null;
const number = parseInt(value);

if (isNaN(number)) {
  console.log('Please provide a number');
  return;
}

const addTen = add_ten(number);
console.log(addTen);

const addHundred = add_hundred(number);
console.log(addHundred);

const addThousand = add_thousand(number);
console.log(addThousand);
