const i = await Deno.readTextFile('input.txt')
const exp = /mul\((\d+),(\d+)\)/g
const secondExp = /mul\((\d+),(\d+)\)|do\(\)|don\'t\(\)/gm

let input: string = i//.split('').filter(isValidChar).join('')

const numbers = '0123456789'.split('')

let lasteElement = '';

let startValid = false;


let ans1 = 0;
let m;

var dog = true;

while ((m = secondExp.exec(input)) !== null) {
  let parsed = m[0]
  console.log({parsed});

  if (parsed.startsWith("don't")){
    console.log('dog disabled')
    
    dog = false
    continue
  }
  
  if (parsed.startsWith('do')) {
    console.log('dog is true')
    dog = true
    continue
  }

  if (dog == false) continue
  let firstN = m[1] as unknown as number
  let secondN = m[2] as unknown as number
  

  let res = firstN * secondN
  ans1 = ans1 + res
  
}

console.log(`The second ans is ${ans1}`)

function isValidChar(c: string) {
  let valid_chars = "d o n ' t 0 1 2 3 4 5 6 7 8 9 ( ) , m u l".split(' ');
  if (valid_chars.includes(c)) return true;
  return false;
}
