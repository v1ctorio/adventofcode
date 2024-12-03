const i = await Deno.readTextFile('input.txt')


let input: string[] = i.split('').filter(isValidChar)

const numbers = '0123456789'.split('')

let lasteElement = ''

let startValid = false;

for (let index = 0; index < input.length; index++) {
  const element = input[index];
  
  if (element == 'm') {
    lasteElement = element
    startValid = true
    continue;
  }

  if (!startValid) continue;
  if (element == 'u' && lasteElement == 'm') {
    lasteElement = element
    startValid = true
    continue;
  }
  if (element == 'l' && lasteElement == 'u') {
    lasteElement = element
    startValid = true
    continue;
  }
  if (element == '(' && lasteElement == 'l') {
    lasteElement = element
    startValid = true
    continue;
  }
  if (numbers.includes(element))


  
  startValid = false
}



console.log(input)

function isValidChar(c: string) {
  let valid_chars = '0 1 2 3 4 5 6 7 8 9 ( ) , m u l'.split(' ');
  if (valid_chars.includes(c)) return true;
  return false;
}
