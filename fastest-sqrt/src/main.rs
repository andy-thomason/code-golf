
// use std::time::SystemTime;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::time::SystemTime;

macro_rules! gen_isqrt {
    ($name: ident, $type: ty, $fix: expr) => {
        fn $name(a: $type) -> $type {
            // Compute the lower bound.
            // It is important to always under estimate to prevent
            // overflows.
            let mut x = 1 << (a.leading_zeros()/2);
        
            // Refine the lower bound using the Newton inverse square root
            // in fixed point.
            // This converges much faster than the Woo abacus method
            // and does not use a divide.
            // However the Woo method may be faster for small numbers.
            for _ in 0..4 {
                let ax = a * x >> $fix;
                let axx = ax * x >> $fix+1;
                let half = 1 << $fix-1;
                let dx = x * (half - axx) >> $fix;
                println!("x   ={:032x}", x);
                println!("a   ={:032x}", a);
                println!("ax  ={:032x}", ax);
                println!("axx ={:032x}", axx);
                println!("dx  ={:032x}", dx);
                println!("half={:032x}\n", half);
                x += dx;
            }
        
            // Convert inverse square root to square root.
            x = a * x >> $fix;
            println!("x={} a={}", x, a);
        
            // We may have underestimated too much.
            // Bump the result until it is the correct lower bound.
            if (x+1) * (x+1) <= a {
                x += 1;
            }
            assert_eq!(x*x,a);
            x
        }
    }
}

gen_isqrt!(isqrt32, u64, 32);
gen_isqrt!(isqrt64, u128, 64);

// https://web.archive.org/web/20120306040058/http://medialab.freaknet.org/martin/src/sqrt/sqrt.c
// Martin Guy's integer square root.
fn isqrt(x: u32) -> u32 {
    let mut op = x;
	let mut res = 0;

	// "one" starts at the highest power of four <= than the argument.
	let mut one = 1 << 30;	// second-to-top bit set
	while one > op { one >>= 2; }

	while one != 0 {
		if op >= res + one {
			op = op - (res + one);
			res = res +  2 * one;
		}
		res /= 2;
		one /= 4;
    }
    res
}

// https://web.archive.org/web/20120306040058/http://medialab.freaknet.org/martin/src/sqrt/sqrt.c
// Adaptation of Martin Guy's integer square root.
// fn isqrt_big(x: &[u32], res: &mut [u32]) {
//     for i in 0..res.len() {
//         res[i] = 0;
//     }

// 	// "one" starts at the highest power of four <= than the argument.
//     let mut one = 1 << 30;
//     let mut i = x.len()-1;
// 	while one > x[i] { one >>= 2; }

// 	loop {
// 		if x[i] >= res[i] + one {
// 			x[i] = x[i] - (res[i] + one);
// 			res[i] = res[i] + 2 * one;
// 		}
// 		res[i] /= 2;
//         one /= 4;
//         if one == 0 {
//             if i == 0 {
//                 break;
//             }
//             one = 1 << 30;
//             i -= 1;
//         }
//     }
//     res
// }

#[test]
fn testit() {
    println!("{}", isqrt(1234*1234));
}

// fn decimal_sqrt() {
//     let a = [01, 52, 27, 56];
//     let mut c = 0;
//     let mut p = 0;
//     let mut y = 0;

//     for j in 0..4 {
//         c = (c - y)* 100 + a[j];
//         let x = (0..10).position(|x| x*(20*p + x) > c).unwrap()-1;
//         y = x*(20*p + x);
//         p = p * 10 + x;
//     }
// }

// Find the decimal square root of a bytes sting.
// Assumes non-zero length series of decimal digits.
fn decimal_sqrt<'a, 'tmp>(a: &'a [u8], tmp: &'tmp mut [u8]) -> &'tmp [u8] {
    let mut a = a;
    // The current value:
    // last two digits are the from the string.
    let mut c = 0;

    // The part of the root found so far
    // last digit is a digit from the result.
    let mut p = 0;

    // Function to calculate the next digit of the result.
    fn calc_result_digit(p: usize, c: usize) -> usize {
        (0..10).position(|x| x*(20*p + x) > c).unwrap()-1
    }

    let mut len = 0;

    // let mut a = "152415765279684".as_bytes();
    if a.len() % 2 != 0 {
        c = (a[0] - b'0') as usize;
        let x = calc_result_digit(p, c);
        let y = x*x;
        p = x;
        a = &a[1..];
        c -= y;
        tmp[len] = x as u8 + b'0';
        len += 1;
    }

    for a in a.chunks_exact(2) {
        let digits = (a[0] - b'0') as usize * 10 + (a[1] - b'0') as usize;
        c = c * 100 + digits;
        let x = calc_result_digit(p, c);
        let y = x*(20*p + x);
        p = p * 10 + x;
        c -= y;
        tmp[len] = x as u8 + b'0';
        len += 1;
    }

    // Note that c will be non-zero if the result is not a perfect square.
    &tmp[0..len]
}

// println!("digits={:02} c={:02} x={}", digits, c, x);
// println!("c={:04} p={:04} y={:04} x={}", c, p, y, x);

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

    let results : Vec<_> = src.iter().zip(dest.iter_mut())
        .map(|(s, d)| decimal_sqrt(*s, *d))
        .collect();

    src.iter().zip(results.iter())
        .for_each(|(s, d)| {
            let src = std::str::from_utf8(*s).unwrap();
            let dest = std::str::from_utf8(*d).unwrap();
            let bigint_sqrt = src.parse::<BigUint>().unwrap().sqrt();
            let decimal_sqrt = dest.parse::<BigUint>().unwrap();
            assert_eq!(bigint_sqrt, decimal_sqrt);
        });
}

    // // Parse the numbers.
    // let mut numbers: Vec<_> = std::str::from_utf8(&text)
    //     .unwrap()
    //     .split(|c| c == '\n')
    //     .map(|s| s.parse::<BigUint>().unwrap())
    //     .collect();
    
    // let check = numbers.clone();

    // let start = SystemTime::now();

    // // Do the calculation
    // numbers.iter_mut().for_each(|s| {
    //     if s.bits() > 32 {
    //         let mut digits = s.to_u32_digits();
    //         sqrt(digits.as_mut_slice());
    //         panic!();
    //     }
    // });

    // // Print the time.
    // println!("{}us", start.elapsed().unwrap().as_micros());

    // for (n, a) in numbers.iter().zip(check.iter()) {
    //     assert_eq!(n, &a.sqrt());
    // }
