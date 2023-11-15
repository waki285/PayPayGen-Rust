use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

fn main() {
  loop {
    let mut rng = thread_rng();
    let chars: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();
    println!("https://pay.paypay.ne.jp/{}", chars);
  }
}
