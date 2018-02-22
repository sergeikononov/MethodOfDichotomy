use std::f32;
use std::f32::EPSILON;

fn f(x: f32) -> f32 {
    let eps:f32 = 2.71;
    return 4.0-x.exp() - 2.0*(x.powf(2.0));


}

fn main() {
    let mut a:f32; let mut b:f32; let mut c:f32;
    a = 0 as f32;
    b = 1 as f32;
    while (b-a) > EPSILON {
        c = (a+b) / 2.0;
        if f(b)*f(c) < 0 as f32 {
            a = c;
        } else {
            b = c;
        }
    }
    println!("{}", (a+b)/2.0);
}