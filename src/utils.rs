use {
    crate::{args, utils},
    headless_chrome::{Browser, LaunchOptionsBuilder},
    rand::{seq::SliceRandom, thread_rng},
    reqwest::blocking::Client,
    std::io::{self, Read},
};

lazy_static! {
    static ref USER_AGENTS: Vec<String> = {
        let args = args::get_args();
        args.user_agent_strings
    };
}

pub fn return_reqwest_client(secs: u64) -> Client {
    let user_agent = utils::return_random_string(USER_AGENTS.clone());
    Client::builder()
        .user_agent(user_agent)
        .timeout(std::time::Duration::from_secs(secs))
        .build()
        .unwrap()
}

pub fn return_headless_browser(sandbox: bool) -> Browser {
    match Browser::new(
        LaunchOptionsBuilder::default()
            .sandbox(sandbox)
            .window_size(Some((1920, 2500)))
            .build()
            .expect("Could not find appropriate Chrome binary."),
    ) {
        Ok(browser) => browser,
        Err(e) => {
            eprintln!("Error getting the Chrome/Chromium instance, make sure that it's properly installed.
Chromium/Chrome from Snap are known to cause problems, if you have installed it from there,
please uninstall it and reinstall without using Snap. Error: {}", e);
            std::process::exit(1)
        }
    }
}

pub fn calculate_timeout(threads: usize, timeout: u64) -> u64 {
    if timeout <= 500 {
        if threads >= 50 {
            timeout + 200
        } else if threads >= 100 {
            timeout + 300
        } else if threads >= 200 {
            timeout + 400
        } else if threads >= 300 {
            timeout + 500
        } else {
            timeout + 100
        }
    } else {
        timeout
    }
}

pub fn read_stdin() -> Vec<String> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut buffer)
        .expect("Error getting input list.");
    let mut targets: Vec<String> = buffer.lines().map(str::to_owned).collect();
    targets.sort();
    targets.dedup();
    targets
}

pub fn return_random_string(strings: Vec<String>) -> String {
    if strings.is_empty() {
        String::new()
    } else {
        strings.choose(&mut thread_rng()).unwrap().to_string()
    }
}
