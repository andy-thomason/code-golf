
// use std::time::SystemTime;
use num_bigint::BigUint;
use std::time::SystemTime;
use num_traits::{Zero, FromPrimitive};

// Find the decimal square root of a bytes sting.
// Assumes non-zero length series of decimal digits.
fn decimal_sqrt<'a, 'tmp>(a: &'a [u8], tmp: &'tmp mut [u8]) -> &'tmp [u8] {
    type Acc = u128;

    // println!("a={}", std::str::from_utf8(a).unwrap());
    // println!("len={}", a.len());

    let mut a = a;

    // The current value:
    // last two digits are the from the string.
    let mut c = Acc::zero();

    // The part of the root found so far
    // last digit is a digit from the result.
    let mut p = Acc::zero();

    // Function to calculate the next digit of the result.
    fn calc_result_digit(p: &Acc, c: &Acc) -> Acc {
        let mut x = 0;

        // This loop unrolls to cmovs with small integers.
        for i in 0..4 {
            let v = 8 >> i;
            let y = (x+v)*(20*p + (x+v));
            if &y <= c {
                x += v;
            }
        }
        x
    }

    let mut len = 0;

    // Handle odd digit at the start.
    if a.len() % 2 != 0 {
        c = Acc::from_u8(a[0] - b'0').unwrap();
        let x = calc_result_digit(&p, &c);
        let y = x*x;
        p = x;
        a = &a[1..];
        c -= y;
        tmp[len] = x as u8 + b'0';
        len += 1;
    }

    // Handle pairs of digits.
    for a in a.chunks_exact(2) {
        let digits = ((a[0] - b'0') * 10 + (a[1] - b'0')) as Acc;
        c = c * 100 + digits;
        let x = calc_result_digit(&p, &c);
        let y = x*(20*p + x);
        //println!("digits={:02} p={} c={:02} x={} y={}", digits, p, c, x, y);
        p = p * 10 + x;
        c -= y;
        tmp[len] = x as u8 + b'0';
        len += 1;
    }

    //println!("res={}", std::str::from_utf8(&tmp[0..len]).unwrap());
    // Note that c will be non-zero if the result is not a perfect square.
    &tmp[0..len]
}

fn main() {
    // Read the input.
    let text = std::fs::read("numbers-updated.txt")
        .unwrap();

    // A buffer for results.
    let mut res = text.clone();

    // Vector of slices of source
    let src : Vec<_> = text
        .as_slice()
        .split(|c| c == &b'\n')
        .collect();

    // Vector of slices of result
    let mut dest : Vec<_> = res
        .as_mut_slice()
        .split_mut(|c| c == &b'\n')
        .collect();
    
    let start = SystemTime::now();

    let results : Vec<_> = src
        .iter()
        .zip(dest.iter_mut())
        .take(7000)
        .map(|(s, d)| decimal_sqrt(*s, *d))
        .collect();
    
    println!("{} μs", start.elapsed().unwrap().as_micros());

    src
        .iter()
        .zip(results.iter())
        .take(100)
        .for_each(|(s, d)| {
            let src = std::str::from_utf8(*s).unwrap();
            let dest = std::str::from_utf8(*d).unwrap();
            let bigint_sqrt = src.parse::<BigUint>().unwrap().sqrt();
            let decimal_sqrt = dest.parse::<BigUint>().unwrap();
            assert_eq!(bigint_sqrt, decimal_sqrt);
        });
}

