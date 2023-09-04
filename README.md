# Lox

An implimentation of the Lox tree walk interpreter from https://craftinginterpreters.com/ in Rust. After completing the base implimentation I changed some syntax and added some features.

# Language

## Variables
```rust
var a = 1;
a = true;
a = "hello";

num n = 2.0;
string s = "hello";
bool b = true;
```
### List
List can be nested and jagged. List may also contain multiple types. += will append to a list and -= will remove at index from a list. List index will always wrap around the list length so no index is invalid. That being said if you do `list -= -1` it will remove the last element. This allows list to operate much like stacks when needed.
```rust
var l = [0,1,2,3,4];
l += 5; //[0,1,2,3,4,5]
l -= 0;//[1,2,3,4,5]
var l2 = [true, false, [0,2,3]];
var length_of_l = #l;
var l_reversed = !l;
```
### Strings 
Strings may be indexed and manipulated like list.
```rust
string hello = "";
hello += "Hello";
hello += " World!";
hello -= -1;
var h = hello[0];
hello = !hello;
//hello -> dlroW olleH 
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
## Classes
```rust
class a {
    a(val : num) {
        this.val = val;
    }

    get_val() {
        return this.val;
    }

    incr() {
        this.val++;
    }
}

class b : a {
    b(val : num) {
        this.val = val;
    }

    incr() {
        super.incr();
        this.val++;
    }
}
```
## Standard Library 
```rust
print(v); //print without newline
println(v); //print with newline
debug(v); //print internal representation of a value
len(lst); //return length of list
clock(); //current tick of program
random(0, 2); //random between 0-2
hashcode(v); //get hash for value

var s = Stack();
s.push(1);
s.push(2);
s.push(3);
println(s.count());
var v = s.pop();

var q = Queue();
q.enqueue(1);
q.enqueue(2);
q.enqueue(3);
q.dequeue();
println(s.front());
println(s.back());

var h = Hashmap();
h.insert("a", 1);
h.insert("b", 2);
h.insert("c", 3);
println(h.get("a"));
h.remove("a");
```