#![allow(unused_macros)]
#![allow(unstable_features)]
#![feature(trace_macros)]

macro_rules! rpn {
    // handler to catch operators
    (@op [ $b:expr, $a:expr $(, $stack:expr)* ] $op:tt $($rest:tt)*) => {
        // ex: `[2, 3, 4, 1] * 2` => `[3 * 2, 4, 1] 2`
        rpn!([ $a $op $b $(, $stack)* ] $($rest)*)
    };

    (@op $stack:tt $op:tt $($rest:tt)*) => {
        compile_error!(concat!(
            "Could not apply operator `",
            stringify!($op),
            "` to current stack: ",
            stringify!($stack)
        ))
    };

    ($stack:tt + $($rest:tt)*) => {
        rpn!(@op $stack + $($rest)*)
    };
    
    ($stack:tt - $($rest:tt)*) => {
        rpn!(@op $stack - $($rest)*)
    };

    ($stack:tt * $($rest:tt)*) => {
        rpn!(@op $stack * $($rest)*)
    };
    
    ($stack:tt / $($rest:tt)*) => {
        rpn!(@op $stack / $($rest)*)
    };

    // mutation by modified copy and recursion
    ([ $($stack:expr),* ] $num:tt $($rest:tt)*) => {
        // ex: `[1] 2 3` => `[1 , 2] 3`
        rpn!([ $num $(, $stack)* ] $($rest)*)
    };

    ([ $result:expr ]) => {
        $result
    };

    // branch to be when there are no more tokens nor final value
    ([ $($stack:expr),* ]) => {
        compile_error!(concat!(
            "Could not find final value. Final stack: ",
            stringify!([ $($stack),* ])
        ))
    };

    // branch to be used as entry API
    ($($tokens:tt)*) => {
        rpn!([] $($tokens)*)
    };
}

macro_rules! logic_expr {
    ($a:ident ^ $b:ident $($tail:tt)*) => {
        $a && logic_expr!($b $($tail)*)
    };
    ($a:ident v $b:ident $($tail:tt)*) => {
        $a || logic_expr!($b $($tail)*)
    };
    ($($a:tt)*) => {
        $($a)*
    }
}

fn main() {
    println!("World of macros");

    trace_macros!(true);

    println!("{}", rpn!(2 7 + 4 *));
    println!("{}", logic_expr!(true v true ^ false));

    trace_macros!(false);
}
