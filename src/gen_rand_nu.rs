extern crate rand;

use rand::Rng;

mod numeric {
    pub fn gen_rand(seed: u32) -> u32 {
        let mut rand_nu: u32 = rand::thread_rng().gen_range(1, seed);
        println!("1~{} 随机数 {}", seed, &mut rand_nu);
        return rand_nu;
    }

    pub fn fun(param: u32) -> u32 {
        param + 100;
    }
}
