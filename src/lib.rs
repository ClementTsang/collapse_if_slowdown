#[inline]
pub fn slow(mut ret: u64) -> u64 {
    let mask = (1 << 38) - 1;

    for _ in 0..100_000 {
        let mut speed = 0.0;
        let mut z: f64 = speed;
        speed += 0.200000001;

        for _ in 2..14 {
            z += speed;

            if (z.to_bits() >> 8) & mask == 0 && z % 0.0625 < 1e-13 {
                println!("{}", z % 0.0625);
                ret += 1;
            }
        }
    }

    eprintln!("ret: {ret}");
    ret
}

#[inline]
pub fn fast(mut ret: u64) -> u64 {
    let mask = (1 << 38) - 1;

    for _ in 0..100_000 {
        let mut speed = 0.0;
        let mut z: f64 = speed;
        speed += 0.200000001;

        for _ in 2..14 {
            z += speed;

            if (z.to_bits() >> 8) & mask == 0 {
                if z % 0.0625 < 1e-13 {
                    println!("{}", z % 0.0625);
                    ret += 1;
                }
            }
        }
    }

    eprintln!("ret: {ret}");
    ret
}
