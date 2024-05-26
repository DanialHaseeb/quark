# Quark

A programming language designed from the ground-up for quantum computing.

## Example

Quark focuses on expressive and elegant code:
```nim
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

while true {
  a = a + b;
  count = count + 1;
  if count > 10 {
    break;
  }
}

echo a;
```
This code is translated to:
```python
import numpy as np
def fibonacci(n):
    if n < 1:
        return n
    else:
        return fibonacci(n - 1) + fibonacci(n - 2)
        
    
print(fibonacci(3))
a = [1, 2, 3, 5]
b = np.array([[1, 0],[0, 1],])
print(a + b)
count = 0

while True:
    a = a + b
    count = count + 1
    if count > 10:
        break
        
    
print(a)
```
