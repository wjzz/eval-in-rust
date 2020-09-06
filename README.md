# Eval

A simple expression evaluator written in Rust

~~~
Variables: v ::= [ascii_letter]+
(e.g. x, y, abc)
Numberss: n ::= [digit]+
(e.g. 123, 1, 0)
Expressions e ::= number | v | e + e | e * e | (e)
Statements  s :: e | x = e
~~~

To test the evaluator, run `cargo run` and enter statements

Example session:

~~~
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/evaluator`
Press ^C (ctrl+C) to quit.
> 1 + 4
5
> x
> Error! variable not bound: x
> x = 123
> x
123
> x = x + 3
> x
126
> x = x * x
> x
15876
> y = 2 + 2 * 2
> y
6
> (2+2)*2
8
> ^C
~~~

There are some unit tests avaiable, run `cargo test` to execute them.