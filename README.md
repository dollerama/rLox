# Lox

An implimentation of the Lox tree walk interpreter from https://craftinginterpreters.com/ in Rust. Some syntax was changed and features added.

# Language

## Variables
```lox
var a = 1;
a = true;
a = "hello";
num n = 2.0;
//n = "hello"; **error
string s = "hello";
bool b = true;
```
## Conditionals
```lox
if a == 2 {
    println("a equals 2");
}
else if a == 3 {
    printl("a equals 3");
}
else {
    println("default");
}
```