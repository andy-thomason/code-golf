
use num_bigint::BigUint;
use std::time::SystemTime;
use num_traits::{Zero, FromPrimitive, ToPrimitive};
use std::ops::{MulAssign, AddAssign, SubAssign, Add, Mul};
use std::cmp::PartialOrd;
use std::fmt::Display;

// Function to calculate the next digit of the result.
fn calc_result_digit<'a, Acc>(p: &'a Acc, c: &'a Acc) -> u8
where
    Acc : Zero,
    Acc : MulAssign<Acc>,
    Acc : AddAssign<Acc>,
    Acc : SubAssign<Acc>,
    Acc : Add<Acc, Output=Acc>,
    Acc : Mul<Acc, Output=Acc>,
    Acc : FromPrimitive,
    Acc : ToPrimitive,
    Acc : PartialOrd<Acc>,
    Acc : Clone,
    Acc : Display,
{
    // let px = p.to_f64().unwrap();
    // let cx = c.to_f64().unwrap();
    let mut x = 0_u8;
    let mut _20p = p.clone();
    _20p *= Acc::from_u8(20).unwrap();

    for i in 0..4 {
        let xt = Acc::from_u8(x+(8 >> i)).unwrap();
        let y = xt.clone() * (_20p.clone() + xt);
        if y <= c.clone() {
            x += 8 >> i;
        }
    }

    // solve x * (20p + x) - c = 0
    // x * x + 20px - c = 0
    // a = 1, b=20p, c= -c
    // (-20p +/- sqrt(400p*p + 4c))/2
    // sqrt(100p*p + c) - 10p
    // let guess = ((100.*px*px + cx).sqrt() - 10.*px) as u8;
    //println!("x={} guess={}", x, guess);

    if x >= 10 {
        println!("x={} p={} c={}", x, p, c);
        panic!("bad");
    }
    x
}


// Find the decimal square root of a bytes sting.
// Assumes non-zero length series of decimal digits.
fn decimal_sqrt<'a, 'tmp, Acc>(a: &'a [u8], tmp: &'tmp mut [u8]) -> &'tmp [u8]
where
    Acc : Zero,
    Acc : MulAssign<Acc>,
    Acc : AddAssign<Acc>,
    Acc : SubAssign<Acc>,
    Acc : Add<Acc, Output=Acc>,
    Acc : Mul<Acc, Output=Acc>,
    Acc : FromPrimitive,
    Acc : ToPrimitive,
    Acc : PartialOrd<Acc>,
    Acc : Clone,
    Acc : Display,
{
    //println!("a={}", std::str::from_utf8(a).unwrap());
    // println!("len={}", a.len());

    let mut a = a;

    // The current value:
    // last two digits are the from the string.
    let mut c = Acc::zero();

    // The part of the root found so far
    // last digit is a digit from the result.
    let mut p = Acc::zero();

    let mut len = 0;
    let _100 = Acc::from_u8(100).unwrap();
    let _10 = Acc::from_u8(10).unwrap();
    let _20 = Acc::from_u8(20).unwrap();

    // Handle odd digit at the start.
    if a.len() % 2 != 0 {
        c = Acc::from_u8(a[0] - b'0').unwrap();
        let x = calc_result_digit::<Acc>(&p, &c);
        let xx = Acc::from_u8(x).unwrap();
        let y = xx.clone() * xx.clone();
        //println!("    p={} c={:02} x={} y={}", p, c, x, y);
        p = Acc::from_u8(x).unwrap();
        a = &a[1..];
        c -= y;
        tmp[len] = x as u8 + b'0';
        len += 1;
    }

    // Handle pairs of digits.
    for a in a.chunks_exact(2) {
        let digits = Acc::from_u8((a[0] - b'0') * 10 + (a[1] - b'0')).unwrap();
        c = c * _100.clone() + digits;
        let x = calc_result_digit::<Acc>(&p, &c);
        let xx = Acc::from_u8(x).unwrap();
        let y = xx.clone() * (_20.clone() * p.clone() + xx.clone());
        // println!("digits={:02} p={} c={:02} x={} y={}", digits, p, c, x, y);
        p *= _10.clone();
        p += xx;
        c -= y;
        tmp[len] = x + b'0';
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
        //.take(7000)
        .map(|(s, d)| decimal_sqrt::<BigUint>(*s, *d))
        .collect();
    
    println!("{} μs", start.elapsed().unwrap().as_micros());

    src
        .iter()
        .zip(results.iter())
        .for_each(|(s, d)| {
            let src = std::str::from_utf8(*s).unwrap();
            let dest = std::str::from_utf8(*d).unwrap();
            let bigint_sqrt = src.parse::<BigUint>().unwrap().sqrt();
            let decimal_sqrt = dest.parse::<BigUint>().unwrap();
            assert_eq!(bigint_sqrt, decimal_sqrt);
        });
}

