fn main() {
    const DEFAULT_ITERS: u32 = 100_000_000;

    const DATA: [[u32; 4]; 4] = [[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]];
    const ONES: [[u32; 4]; 4] = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]];
    const ZERO: [[u32; 4]; 4] = [[0; 4]; 4];

    let iters = std::env::args()
        .nth(1)
        .and_then(|n| n.parse().ok())
        .unwrap_or(DEFAULT_ITERS);

    println!("{iters} iterations set..");

    // C test
    {
        use c::*;

        let a = Mat { data: DATA };
        let b = Mat { data: ONES };
        let mut c = Mat { data: ZERO };
        let secs = bench(iters, || /* unsafe lol xD */ unsafe {
            mult(&a, &b, &mut c);
        });
        assert_eq!(c.data, DATA.map(|v| v.map(|x| x * iters)));
        println!("  c: {secs:.4}");
    }

    // Rust test
    {
        use rs::*;

        let a = Mat { data: DATA };
        let b = Mat { data: ONES };
        let mut c = Mat { data: ZERO };
        let secs = bench(iters, || {
            mult(&a, &b, &mut c);
        });
        assert_eq!(c.data, DATA.map(|v| v.map(|x| x * iters)));
        println!(" rs: {secs:.4}");
    }
}

fn bench<F>(iters: u32, mut f: F) -> f64
where
    F: FnMut(),
{
    use std::time::Instant;

    let start = Instant::now();
    for _ in 0..iters {
        f();
    }
    let end = Instant::now();
    end.duration_since(start).as_secs_f64()
}

mod c {
    const N: usize = 4;

    #[repr(C)]
    pub struct Mat {
        pub data: [[u32; N]; N],
    }

    extern "C" {
        pub fn mult(a: *const Mat, b: *const Mat, c: *mut Mat);
    }
}
