const i = await Deno.readTextFile('input.txt')
const exp = /mul\((\d+),(\d+)\)/g

let input: string[] = i.split('').filter(isValidChar).join('')

const numbers = '0123456789'.split('')

let lasteElement = ''

let startValid = false;


let ans1 = 0;
let m;

while ((m = exp.exec(input)) !== null) {
  let firstN = m[1]
  let secondN = m[2]
  
  console.log({firstN,secondN});

  let res = firstN * secondN
  ans1 = ans1 + res
  
}

console.log(`The first ans is ${ans1}`)

function isValidChar(c: string) {
  let valid_chars = '0 1 2 3 4 5 6 7 8 9 ( ) , m u l'.split(' ');
  if (valid_chars.includes(c)) return true;
  return false;
}
