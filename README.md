# Quark

A programming language designed from the ground-up for quantum computing.

## Example

Quark focuses on short and concise code:
```
A = [1, 2 || 3 4];
B = [5, 6|
    |7, 8];
C = A + B;
```
This code is translated to:
```python
import numpy
A = numpy.array([[1, 2], [3, 4]])
B = numpy.array([[5, 6], [7, 8]])
C = A + B
```
