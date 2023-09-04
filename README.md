# Lox

An implimentation of the Lox tree walk interpreter from https://craftinginterpreters.com/ in Rust. Some syntax was changed and features added.

# Language

## Variables
```rust
var a = 1;
a = true;
a = "hello";
num n = 2.0;
//n = "hello"; **error
string s = "hello";
bool b = true;
```
## Conditionals
```rust
if a == 2 {
    println("a equals 2");
}
else if a == 3 {
    println("a equals 3");
}
else {
    println("default");
}
```
## Loops
```rust
var i = 0;
while i < 5 {
    println(i);
    i++;
}

for var i=0; i < 5; i++ {
    println(i);
}

for i < 5 {
    println(i);
}
```
when looping through a list you will have a read-only slice the list and the index for mutations. the index is the specified slice name plus "_iter" in this case "i_iter".
```rust 
var d = [1,2,3];

for i in d {
    println(i);
    d[i_iter] += 5;
}
```