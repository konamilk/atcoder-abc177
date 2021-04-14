use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut ans = std::i32::MAX;

    for i in 0..=(s.len() - t.len()){
        let mut icchi = 0;

        let slice = &s[i..(i+t.len())];

        // dbg!(slice);
        for j in 0..t.len(){
            if slice[j] == t[j] {
                icchi += 1;
            }
        }

        // println!("{}", icchi);
        ans = std::cmp::min(ans, t.len() as i32 - icchi)
    }

    println!("{}", ans);
}
