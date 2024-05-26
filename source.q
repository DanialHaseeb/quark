//# a simple recursive function
func fibonacci(n) {
    if n < 1 {
        return n;
    } else {
    
    return fibonacci(n - 1) + fibonacci(n - 2);
  }
}
echo fibonacci(3);

//# Adding matrices

let a = [1, 2|
	|3, 5];
        
let b = [1, 2 | 2, 3];

echo a + b;

//# Simple while loop
let count = 0;
while true
{
  a = a + b;
  count = count + 1;
  if count > 10 {
    break;
  }
}

echo a;

