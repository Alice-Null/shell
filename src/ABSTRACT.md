## this shell is not posix compliant

A shell has syntax
This file is a descriptor of the syntax for this shell

# Variables
`let` keyword to define (scope, name, and type)
`var: type` to set type
inferred automatically when possible 
eg; 
```
let foo: int = 1234;
let bar = 1234;
assert [[foo == bar]];
let fizz: string = "1234";
let buzz = "1234";
assert [[fizz == buzz]];
```

explicitly typed variables (eg, foo, fizz) have one type.
implicitly typed variables (eg; bar, buzz) are dynamically typed.

variables may be called by using a `$`, similarly to posix sh
eg;
```
let foo = "i like crows";
assert [["i like crows" == $foo]]
```
variables must only contain ascii alphanumeric characters, and underscores
variables may not begin with a number

# delimiters
there are special characters that seperate commands
the most common ones are pipes, and basic delimiters
pipes take the output of one command and sends it to another
basic delimiters run one command and when complete runs the other

there are also conditional commands, like && which only runs if the previous command succeeds

# logic flow
if statements and loops
