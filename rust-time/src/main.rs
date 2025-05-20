use std::time::Instant;
use quanta::Clock;

fn main() {
    //unused_mut = 使われてないmut
    // unnecessary_mut = 無駄なmut（ツールによっては同義）
    #[allow(unused_mut)]
    let mut clock = Clock::new();
    const N:u32 = 1_000_000;

    //Instant::now() を 1,000,000 回呼び出し、最初と最後の差を計測
    let istart = Instant::now();
    let mut istop = istart;
    for _ in 1..N {
        istop = Instant::now();
    }

    println!("std::time:Instant::now() overhead = {:?}",istop.duration_since(istart));

    let start = clock.now();
    let mut stop = start;
    for _ in 1..1_000_000{
        stop = clock.now();
    }
    println!("quanta::clock::now() overhead = {:?}",stop.duration_since(start));
}
//unknown lint:`unnecessary_mut `