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
### List
List can be nested and jagged. List may also contain multiple types.
```rust
var l = [0,1,2,3,4];
var l2 = [true, false, [0,2,3]];
var length_of_l = #l;
var l_reversed = !l;
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
when looping through a list you will have a read-only slice of the list and the index for mutations. the index is the specified slice name plus "_iter" in this case "i_iter".
```rust 
var d = [1,2,3];

for i in d {
    println(i);
    d[i_iter] += 5;
}
```
## Functions
Functions are declared with fn and parameters may be restricted to type using var : type syntax.
```rust
fn hello_fun(msg : string) {
    return "Hello "+msg;
}

var c = hello_fun("World"); // c = "Hello World"

var hello = hello_fun;
var d = hello("World2");
```
Anonymous functions will automatically return when they're only one statement
```rust
var a => |a, b| a+b;

var cmp => |a, b| {
    if a < b {
        return false;
    }
    else {
        return true;
    }
};

var b = a(2,5);
var bb = cmp(b, 3);
```