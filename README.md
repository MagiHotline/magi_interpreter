# Interpreter for Magi language

This very simple interpreter for my own custom programming language called "Magi" is made by using [antlr4rust](https://github.com/rrevenantt/antlr4rust), an ANTLR4 parser generator runtime for Rust programming language.

## Prerequisites

You need to have **Rust** and **Cargo** installed on your machine. If you haven't installed them yet, you can do so via [rustup.rs](https://rustup.rs/).

- Download required dependencies:
   - [antlr4 fork with Rust target support version 0.3.0-beta](https://github.com/rrevenantt/antlr4rust/releases/tag/antlr4-4.8-2-Rust0.3.0-beta)
   - Java JDK

- Add the path to the antlr4 jar file to the `ANTLR_JAR` environment variable

```
export ANTLR_JAR=/path/to/antlr4-4.8-2-SNAPSHOT-complete.jar
```

- Build the project

```
cargo build
```

## Usage

You can either run it using simple ```cargo run``` (and then you would be
requested to write Magi code (look [Syntax](README#Syntax))) or you can 
pass a file path to the source code.

```
cargo run path/to/source.magi
```


## Syntax

This is an example of syntax for the Magi language:

```
a := 5;
while(a > 0)
{
  a = a - 1;
  print(a);
}
```

Should output:

```
4
3
2
1
0
```
