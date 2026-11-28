# Variables 

1. Variables unintialised cannot infer type from value so they should have annotated types

```rust
fn main(){
    let x;
}
```
Thus above code have error 

fix for this error
```rust 
let x:i32;
```
So now rust now determines type 

2. Variables unintialised gets warning but if they used we get error that it is not binding 

```rust
fn main(){
    let s:&str;
    println!("{}",s);
}
```

3. Variables are immutable by default 

```rust
fn main(){
    let x = 2;
    x = 3;
    println!("{}",x);
}
```
The above code throws error so fix is use mut keyword 

4. mut keyword can make variable mutable 

```rust
fn main(){
    let mut x = 2;
    x +=3;
    println!("{}",x);
}
```

5. mutable only changes the value when you change the type you will get mismatched types error


```rust
let mut x = 12;
x = "Hello";
```
The above code results in error "mismatched types"

# Constants 
* Constants are the one which are immutable whose value cannot be changed 

```rust
const x = 12;
```
The above code fails because they should have annotated types 

fix:
```rust
const x:i32 = 32;
```
* Constants cannot have runtime expressions 

```rust
let x = 2;
const y:i32 = x+2;
```
The above code produce error because  in constants we can use only compile time expressions not runtime expressions


* Constants cannot be made as mutable by mut keyword 
```rust
const mut x:i32 = 12;
```
The above code produces error as constants cannot be made as mutable

# Shadowing 
Shadowing is the concept in which the variable with same name if used 
hides the previous one with same name until scope gets destroyed 

```rust
let x = 23;
let x = 1345;
```

Shadowing allows you to change the type since it creates entirely new variable
```rust
fn main(){
    let x =  23;
    let x = "hello";
}
```
# Scope
Scope is the block in which program in which defines how we can access the variable

## Constants can have global while variables cannot

```rust
const x:i32 = 23;
fn main(){

}
```
## Constants in global can overshadow by one in local 
```rust
const x:i32 = 5;
fn main(){
    const x:i32 = 24;
}
```
## Constants with same name cannot be present in same local scope or same global scope 

```rust
const x:i32 = 5;
const x:i32 = 10;
fn main(){

}
```
The above code ends in error 

```rust 
fn main(){
    const x:i32 = 24;
    const x:i32 = 123;
}
```
The above code ends in error 
