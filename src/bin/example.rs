extern crate miracl_core;

use miracl_core::rand::{RAND_impl, RAND};

use functional_encryption_schemes::dmcfe_ip::*;
use functional_encryption_schemes::define::{G1, G2Vector}; 
use num_bigint::BigInt;

fn main() {
    //let num_clients: usize = 5;
    //let bound = BigInt::from(10000);
    //let mut clients: Vec<Dmcfe> = Vec::with_capacity(num_clients);
    //let mut pub_keys: Vec<G1> = Vec::with_capacity(num_clients);
    //let mut ciphers: Vec<G1> = Vec::with_capacity(num_clients);
    //let mut fe_key: Vec<G2Vector> = Vec::with_capacity(num_clients);
    //let mut temp: G1;
    //let mut raw: [u8; 100] = [0; 100];

    //let mut rng = RAND_impl::new();
    //rng.clean();
    //for i in 0..100 {
        //raw[i] = i as u8
    //}
    //rng.seed(100, &raw);

    //for i in 0..num_clients {
        //clients.push(Dmcfe::new(&mut rng, i));
    //}

    //for i in 0..num_clients {
        //temp = clients[i].client_pub_key.clone();
        //pub_keys.push(temp);
    //}

    //for i in 0..num_clients {
        //clients[i].set_share(&pub_keys);
    //}

    //let label = "dmcfe-label";
    //let mut x: Vec<BigInt> = Vec::with_capacity(num_clients);
    //let mut y = vec![BigInt::from(1); num_clients];
    //y[0] = BigInt::from(-17);
    //println!("y[0]: {:?}", y[0].to_str_radix(16));

    //for i in 0..num_clients {
        //x.push(BigInt::from(i*1000))
    //}

    //for i in 0..num_clients {
        //ciphers.push(clients[i].encrypt(&x[i], label));
        //fe_key.push(clients[i].derive_fe_key_share(&y[..]));
    //}
    //println!("decrypt starts");
    //use std::time::Instant;
    //let now = Instant::now();
    //let xy = Dmcfe::decrypt(&ciphers, &y[..], &fe_key, label, &bound);
    //let elapsed = now.elapsed();
    //println!("Elapsed: {:.2?}", elapsed);

    //println!("xy {:?}", xy);

    let x: Vec<f32> = vec![0.34362, 2.63588, 1.8803, 1.12673, -0.90941, 0.59397, 0.5232, 0.68602];
    let int_x: Vec<i64> = x.iter().map(|&xi| (xi * 100.0).round() as i64).collect(); 
    println!("int_x: {:?}", int_x);

}
