#![feature(test)]

extern crate test;
extern crate rand;
#[macro_use] extern crate sarkara;

use test::Bencher;
use rand::OsRng;
use sarkara::kex::{ NewHope, KeyExchange };
use sarkara::aead::{ Ascon, General, AeadCipher };
use sarkara::stream::HC256;
use sarkara::auth::HMAC;
use sarkara::hash::Blake2b;

type HHBCipher = General<HC256, HMAC<Blake2b>>;


macro_rules! bench_box {
    ( secretbox $name:ident $ty:ident, $len:expr ) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            use sarkara::secretbox::SecretBox;

            let key = rand!($ty::key_length());
            let data = rand!(bytes $len);
            let mut rng = OsRng::new().unwrap();
            b.bytes = data.len() as u64;
            b.iter(|| {
                let ciphertext = $ty::seal_with_rng(&mut rng, &key, &data);
                $ty::open(&key, &ciphertext)
            });
        }
    };
    ( sealedbox $name:ident $kty:ident $cty:ident, $len:expr ) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            use sarkara::sealedbox::SealedBox;

            let (sk, pk) = $kty::keygen();
            let data = rand!(bytes $len);
            b.bytes = data.len() as u64;
            b.iter(|| {
                let ciphertext = $cty::seal::<$kty>(&pk, &data);
                $cty::open::<$kty>(&sk, &ciphertext)
            });
        }
    }
}

bench_box!(secretbox bench_secretbox_ascon_10 Ascon, 10);
bench_box!(sealedbox bench_sealedbox_ascon_10 NewHope Ascon, 10);
bench_box!(secretbox bench_secretbox_ascon_1k Ascon, 1024);
bench_box!(sealedbox bench_sealedbox_ascon_1k NewHope Ascon, 1024);
bench_box!(secretbox bench_secretbox_ascon_64k Ascon, 65536);
bench_box!(sealedbox bench_sealedbox_ascon_64k NewHope Ascon, 65536);

bench_box!(secretbox bench_secretbox_hhb_10 HHBCipher, 10);
bench_box!(sealedbox bench_sealedbox_hhb_10 NewHope HHBCipher, 10);
bench_box!(secretbox bench_secretbox_hhb_1k HHBCipher, 1024);
bench_box!(sealedbox bench_sealedbox_hhb_1k NewHope HHBCipher, 1024);
bench_box!(secretbox bench_secretbox_hhb_64k HHBCipher, 65536);
bench_box!(sealedbox bench_sealedbox_hhb_64k NewHope HHBCipher, 65536);
