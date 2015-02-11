extern crate test;

macro_rules! benches {
    ($f:expr, $var:ident) => {
        use std::iter::repeat;
        #[bench]
        fn str_small(b: &mut ::test::Bencher) {
            let $var = "foo";
            b.iter(|| $f);
        }

        #[bench]
        fn str_medium(b: &mut ::test::Bencher) {
            let s = repeat('a').take(500).collect::<String>();
            let $var = &*s;
            b.iter(|| $f);
        }

        #[bench]
        fn str_long(b: &mut ::test::Bencher) {
            let s = repeat('a').take(10000).collect::<String>();
            let $var = &*s;
            b.iter(|| $f);
        }
    }
}

mod c {
    extern {
        fn siphash(out: *mut u8, input: *const u8, len: u64, k: *const u8) -> i32;
    }

    fn hash(b: &[u8]) -> u64 {
        let mut ret = 0u64;
        let keys = [0u64, 0u64];
        unsafe {
            siphash(&mut ret as *mut _ as *mut _,
                    b.as_ptr(),
                    b.len() as u64,
                    keys.as_ptr() as *const _);
        }
        ret
    }

    benches!(hash(b.as_bytes()), b);
}

mod rust {
    use std::hash::{SipHasher, Writer, Hasher};

    fn hash(b: &[u8]) -> u64 {
        let mut s = SipHasher::new_with_keys(0, 0);
        s.write(b);
        s.finish()
    }

    benches!(hash(b.as_bytes()), b);
}