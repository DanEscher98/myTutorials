pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// # A dummy macro
/// This is my first macro
macro_rules! myvec {
    [ $( $x:expr ),* ] => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! foo {
    (x => $e:expr) => (println!("mode X: {}", $e));
    (y => $e:expr) => (println!("mode Y: {}", $e));
}

macro_rules! twoargs {
    ($var:expr, $call:expr) => {
        {
            let $var =  $call else {
                return Err(());
            };
        }
    };
}

/// 
macro_rules! mapsum {
    ($( $x:expr; [ $( $y:expr ),* ]);*) => {
        &[ $($( $x + $y ),*),* ]
    }
}

macro_rules! map {
    ($($k:expr => $v:expr),*) => {
        {
            let mut ret = ::std::collections::HashMap::new();
            $(
                ret.insert($k, $v);
            )*
            ret
        }
    };
}

fn dummy() {
    let x = myvec![1, 2, 3];
    vec!(1, 2, 3);
    foo!(x => 3);
    mapsum!(10; [1, 2, 3, 4]; 20; []);

    matches!(Some(3), Some(x) if x < 4);
    let dict = map!(1 => 2, 3 => 4, 5 => 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
