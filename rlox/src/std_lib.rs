use std::any::Any;
use rand::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::types::*;
use crate::tokens::*;
use crate::interpreter::*;

macro_rules! function_container {
    ($func : ident) => {
        Box::new($func::new())
    }
}

macro_rules! native_function {
    ($($name:ident $s:expr, $arity:expr => {$($fun:tt)*})*) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name;

            impl $name {
                pub fn new() -> Self {
                    Self {}
                }
            }

            impl LoxCallable for $name {
                fn get_name(&self) -> Token {
                    Token::new(TokenType::Identifier, $s, None, 0)
                }

                fn arity(&self) -> usize {
                    $arity as usize
                }
                
                $($fun)*
                
                fn clone_dyn(&self) -> Box<dyn LoxCallable> {
                    Box::new(self.clone())
                }
                
                fn as_any(&self) -> &dyn Any {
                    self
                }
            }
        )*
    }
}

pub const STD_LIB_SCRIPT: &str = "
    class Entry {
        Entry(key, val) {
            this.key = key;
            this.val = val;
            this.next = nil;
        }
    }

    class Hashmap {
        Hashmap() {
            this.buckets = [];
            for i < 16 {
                this.buckets += nil;
            }
            this.size = 0;
            this.capacity = 16;
        }

        resize() {
            var new_capacity = this.capacity*2;
            var new_table = [];
            for i < this.capacity {
                new_table += nil;
            }
            for i < this.capacity {
                var node = new_table[i];
                while node != null {
                    var next = node.next;
                    var index = hashcode(key)%this.capacity;
                    node.next = new_table[index];
                    new_table[index] = node;
                    node = next;
                }
            }

            this.buckets = new_table;
            this.capacity = new_capacity;
        }
        
        insert(key, value) {
            var new_entry = Entry(key, value);
            var hash = hashcode(key)%this.capacity;

            if this.buckets[hash] == nil {
                this.buckets[hash] = new_entry;
            }
            else {
                var head = this.buckets[hash];
                this.buckets[hash] = head;

                while head != nil {
                    if head.key == key {
                        head.value = value;
                        return nil;
                    }
                    head = head.next;
                }
                new_entry.next = this.buckets[hash];
                this.buckets[hash] = new_entry;
                this.size++;
                if this.size > this.capacity * 0.75 {
                    this.resize();
                }
            }
        }

        remove(key) {
            var index = hashcode(key)%this.capacity;
            var node = this.buckets[index];
            var prev = nil;
            while node != nil {
                if node.key == key {
                    if prev == nil {
                        this.buckets[index] = node.next;
                    }
                    else {
                        prev.next = node.next;
                    }
                    this.size = (this.size-1 < 0) ? 0 : this.size-1;
                    return;
                }
                prev = node;
                node = node.next;
            }
        }

        get(key) {
            var hash = hashcode(key)%this.capacity;
            var head = this.buckets[hash];

            while head != nil {
                if head.key == key {
                    break;
                    return head.val;
                }
                head = head.next;
            }

            return nil;
        }
    }

    class Stack {
        Stack() {
            this.items = [];
        }

        push(item) {
            this.items += item;
        }

        pop() {
            var item = this.items[-1];
            this.items -= -1;
            return item;
        }

        count() {
            return #this.items;
        }
    }

    class Queue {
        Queue() {
            this.items = [];
        }

        front() {
            return this.items[0];
        }

        back() {
            return this.items[-1];
        }

        enqueue(item) {
            this.items += item;
        }

        dequeue() {
            var ret = this.items[0];
            this.items -= 0;
            return ret;
        }
    }
";

native_function! {
    DebugFunction "debug", 1 => {
        fn call(&self, _interpreter : &mut Interpreter, _callee : Token, arguments : Vec<Option<Literal>>, _auto_clean : bool) -> RuntimeError<Option<Literal>> {
            println!("{:#?}", arguments[0].clone());
            Ok(None)
        }
    }

    CollectFunction "collect_garbage", 0 => {
        fn call(&self, interpreter : &mut Interpreter, _callee : Token, _arguments : Vec<Option<Literal>>, _auto_clean : bool) -> RuntimeError<Option<Literal>> {
            interpreter.collect_garbage();
            Ok(None)
        }
    }

    LenFunction "len", 1 => {
        fn call(&self, _interpreter : &mut Interpreter, _callee : Token, arguments : Vec<Option<Literal>>, _auto_clean : bool) -> RuntimeError<Option<Literal>> {           
            if let Some(Literal::Collection(c)) = arguments[0].clone() {
                Ok(Some(Literal::Number(c.len() as f64)))
            }
            else {
                Ok(Some(Literal::Number(1.0)))
            }
        }
    }

    ClockFunction "clock", 0 => {
        fn call(&self, interpreter : &mut Interpreter, _callee : Token, _arguments : Vec<Option<Literal>>, _auto_clean : bool) -> RuntimeError<Option<Literal>> {
            Ok(Some(Literal::Number(interpreter.time.elapsed().as_millis() as f64)))
        }
    }

    RandomFunction "random", 2  => {
        fn call(&self, _interpreter : &mut Interpreter, _callee : Token, arguments : Vec<Option<Literal>>, _auto_clean : bool) -> RuntimeError<Option<Literal>> {
            if let Some(Literal::Number(x)) = arguments[0].clone() {
                if let Some(Literal::Number(y)) = arguments[1].clone() {
                    Ok(Some(Literal::Number(rand::thread_rng().gen_range(x..y))))
                }
                else {
                    Ok(None)
                }
            }
            else {
                Ok(None)
            }
        }
    }

    HashFunction "hashcode", 1 => {
        fn call(&self, _interpreter : &mut Interpreter, callee : Token, arguments : Vec<Option<Literal>>, _auto_clean : bool) -> RuntimeError<Option<Literal>> {
            match arguments[0].clone() {
                Some(Literal::Number(x)) => {
                    let mut s = DefaultHasher::new();
                    (x as i32).hash(&mut s);
                    Ok(Some(Literal::Number(s.finish() as f64)))
                }
                Some(Literal::String(x)) => {
                    let mut s = DefaultHasher::new();
                    x.hash(&mut s);
                    Ok(Some(Literal::Number(s.finish() as f64)))
                }
                _ => Err((callee.clone(), "Invalid hashcode input.".to_string()))
            }
        }
    }
}