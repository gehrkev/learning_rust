macro_rules! op {
    ( $x: expr, $y: expr, 1) => {
        $x + $y
    };
    ( $x: expr, $y: expr, 2) => {
        $x - $y
    };
    ( $x: expr, $y: expr, 3) => {
        $x * $y
    };
    ( $x: expr, $y: expr, 4) => {
        $x / $y
    };
    ( $x: expr, $y: expr, 5) => {
        $x % $y
    };
    ( $x: expr, $y: expr, $operation: expr) => {
        -1 //default
    }; /*
       // Other students used match like this:
       ($a: expr, $b: expr, $c: expr) => {{
           match $c {
               1 => $a + $b,
               2 => $a - $b,
               3 => $a * $b,
               4 => $a / $b,
               5 => $a % $b,
               _ => -1,
           }
       }};
       */
}

fn main() {
    println!("{}", op!(5, 2, 1));
    println!("{}", op!(5, 2, 2));
    println!("{}", op!(5, 2, 3));
    println!("{}", op!(5, 2, 4));
    println!("{}", op!(5, 2, 5));
    println!("{}", op!(5, 2, 6));
}
