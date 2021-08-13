use {
    crate::{errors::*, misc, networking, utils::return_reqwest_client},
    postgres::NoTls,
    serde::de::DeserializeOwned,
    std::{collections::HashSet, time::Duration},
};

trait IntoSubdomains {
    fn into_subdomains(self) -> HashSet<String>;
}

impl IntoSubdomains for HashSet<String> {
    #[inline]
    fn into_subdomains(self) -> HashSet<String> {
        self
    }
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
struct SubdomainsCertSpotter {
    dns_names: Vec<String>,
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
struct SubdomainsCrtsh {
    name_value: String,
}
#[allow(clippy::upper_case_acronyms)]
#[allow(non_snake_case)]
struct SubdomainsDBCrtsh {
    NAME_VALUE: String,
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
struct SubdomainsVirustotal {
    id: String,
}

#[derive(Deserialize, Eq, PartialEq)]
struct ResponseDataVirusTotal {
    data: HashSet<SubdomainsVirustotal>,
}

impl IntoSubdomains for ResponseDataVirusTotal {
    fn into_subdomains(self) -> HashSet<String> {
        self.data.into_iter().map(|sub| sub.id).collect()
    }
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
struct SubdomainsFacebook {
    domains: Vec<String>,
}

#[derive(Deserialize, Eq, PartialEq)]
struct ResponseDataFacebook {
    data: HashSet<SubdomainsFacebook>,
}

impl IntoSubdomains for ResponseDataFacebook {
    fn into_subdomains(self) -> HashSet<String> {
        self.data
            .into_iter()
            .flat_map(|sub| sub.domains.into_iter())
            .collect()
    }
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
struct SubdomainsSpyse {
    domain: String,
}

#[derive(Deserialize, Eq, PartialEq)]
struct ResponseDataSpyse {
    records: HashSet<SubdomainsSpyse>,
}

impl IntoSubdomains for ResponseDataSpyse {
    fn into_subdomains(self) -> HashSet<String> {
        self.records.into_iter().map(|sub| sub.domain).collect()
    }
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct SubdomainsBufferover {
    FDNS_A: HashSet<String>,
}

impl IntoSubdomains for SubdomainsBufferover {
    fn into_subdomains(self) -> HashSet<String> {
        self.FDNS_A
            .iter()
            .map(|sub| sub.split(','))
            .flatten()
            .map(str::to_owned)
            .collect()
    }
}

#[derive(Deserialize)]
struct SubdomainsThreatcrowd {
    subdomains: HashSet<String>,
}

impl IntoSubdomains for SubdomainsThreatcrowd {
    fn into_subdomains(self) -> HashSet<String> {
        self.subdomains.into_iter().collect()
    }
}

#[derive(Deserialize)]
struct SubdomainsVirustotalApikey {
    subdomains: HashSet<String>,
}

impl IntoSubdomains for SubdomainsVirustotalApikey {
    fn into_subdomains(self) -> HashSet<String> {
        self.subdomains.into_iter().collect()
    }
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
struct SubdomainUrlscan {
    domain: String,
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
struct PageVecUrlscan {
    page: SubdomainUrlscan,
}

#[derive(Deserialize)]
struct ResponseDataUrlscan {
    results: HashSet<PageVecUrlscan>,
}

impl IntoSubdomains for ResponseDataUrlscan {
    fn into_subdomains(self) -> HashSet<String> {
        self.results
            .into_iter()
            .map(|sub| sub.page.domain)
            .collect()
    }
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
struct SubdomainsSecurityTrails {
    subdomains: Vec<String>,
}

#[derive(Deserialize)]
struct SubdomainsThreatminer {
    results: HashSet<String>,
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
pub struct SubdomainsCtsearch {
    #[serde(rename = "subjectDN")]
    pub subject_dn: String,
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
struct SubdomainsC99 {
    subdomain: String,
}

#[derive(Deserialize, Eq, PartialEq)]
struct ResponseDataC99 {
    subdomains: HashSet<SubdomainsC99>,
}

impl IntoSubdomains for ResponseDataC99 {
    fn into_subdomains(self) -> HashSet<String> {
        self.subdomains
            .into_iter()
            .map(|sub| sub.subdomain)
            .collect()
    }
}

impl IntoSubdomains for SubdomainsThreatminer {
    fn into_subdomains(self) -> HashSet<String> {
        self.results.into_iter().collect()
    }
}

fn get_from_http_api<T: DeserializeOwned + IntoSubdomains>(
    url: &str,
    name: &str,
) -> Option<HashSet<String>> {
    match return_reqwest_client(15).get(url).send() {
        Ok(data) => {
            if networking::check_http_response_code(name, &data) {
                match data.json::<T>() {
                    Ok(json) => Some(json.into_subdomains()),
                    Err(e) => {
                        check_json_errors(e, name);
                        None
                    }
                }
            } else {
                None
            }
        }
        Err(e) => {
            check_request_errors(e, name);
            None
        }
    }
}

pub fn get_certspotter_subdomains(
    url_api_certspotter: &str,
    certspotter_token: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("CertSpotter")
    }

    let mut request = return_reqwest_client(15).get(url_api_certspotter);

    if !certspotter_token.is_empty() {
        request = request.bearer_auth(certspotter_token);
    }

    match request.send() {
        Ok(data_certspotter) => {
            if networking::check_http_response_code("CertSpotter", &data_certspotter) {
                match data_certspotter.json::<HashSet<SubdomainsCertSpotter>>() {
                    Ok(domains_certspotter) => Some(
                        domains_certspotter
                            .into_iter()
                            .flat_map(|sub| sub.dns_names.into_iter())
                            .collect(),
                    ),
                    Err(e) => {
                        check_json_errors(e, "CertSpotter");
                        None
                    }
                }
            } else {
                None
            }
        }
        Err(e) => {
            check_request_errors(e, "CertSpotter");
            None
        }
    }
}

pub fn get_crtsh_subdomains(url_api_crtsh: &str, quiet_flag: bool) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("Crtsh")
    }
    match return_reqwest_client(15).get(url_api_crtsh).send() {
        Ok(data_crtsh) => {
            if networking::check_http_response_code("Crtsh", &data_crtsh) {
                match data_crtsh.json::<HashSet<SubdomainsCrtsh>>() {
                    Ok(domains_crtsh) => Some(
                        domains_crtsh
                            .iter()
                            .flat_map(|sub| sub.name_value.split('\n'))
                            .map(str::to_owned)
                            .collect(),
                    ),
                    Err(e) => {
                        check_json_errors(e, "Crtsh");
                        None
                    }
                }
            } else {
                None
            }
        }
        Err(e) => {
            check_request_errors(e, "Crtsh");
            None
        }
    }
}

pub fn get_securitytrails_subdomains(
    url_api_securitytrails: &str,
    target: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("SecurityTrails")
    }
    match return_reqwest_client(15).get(url_api_securitytrails).send() {
        Ok(data_securitytrails) => {
            if networking::check_http_response_code("SecurityTrails", &data_securitytrails) {
                match data_securitytrails.json::<SubdomainsSecurityTrails>() {
                    Ok(domains_securitytrails) => Some(
                        domains_securitytrails
                            .subdomains
                            .into_iter()
                            .map(|sub| format!("{}.{}", sub, target))
                            .collect(),
                    ),
                    Err(e) => {
                        check_json_errors(e, "SecurityTrails");
                        None
                    }
                }
            } else {
                None
            }
        }
        Err(e) => {
            check_request_errors(e, "SecurityTrails");
            None
        }
    }
}

pub fn get_crtsh_db_subdomains(
    crtsh_db_query: &str,
    url_api_crtsh: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("Crtsh database")
    }
    match postgres::config::Config::new()
        .connect_timeout(Duration::from_secs(5))
        .user("guest")
        .host("crt.sh")
        .port(5432)
        .dbname("certwatch")
        .connect(NoTls)
    {
        Ok(mut crtsh_db_client) => match crtsh_db_client.simple_query(crtsh_db_query) {
            Ok(crtsh_db_subdomains) => Some(
                crtsh_db_subdomains
                    .iter()
                    .map(|row| {
                        if let postgres::SimpleQueryMessage::Row(row) = row {
                            let subdomain = SubdomainsDBCrtsh {
                                NAME_VALUE: row.get("NAME_VALUE").unwrap().to_owned(),
                            };
                            subdomain.NAME_VALUE
                        } else {
                            String::new()
                        }
                    })
                    .collect(),
            ),
            Err(e) => {
                if !quiet_flag {
                    println!(
                    "❌ A error has occurred while querying the Crtsh database. Error: {}. Trying the API method...",
                    e);
                }
                get_crtsh_subdomains(url_api_crtsh, quiet_flag)
            }
        },
        Err(e) => {
            if !quiet_flag {
                println!(
                "❌ A error has occurred while connecting to the Crtsh database. Error: {}. Trying the API method...",
                e
            );
            }
            get_crtsh_subdomains(url_api_crtsh, quiet_flag)
        }
    }
}

pub fn get_virustotal_subdomains(
    url_api_virustotal: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("Virustotal")
    }
    get_from_http_api::<ResponseDataVirusTotal>(url_api_virustotal, "Virustotal")
}

pub fn get_sublist3r_subdomains(
    url_api_sublist3r: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("Sublist3r")
    }
    get_from_http_api::<HashSet<String>>(url_api_sublist3r, "Sublist3r")
}

pub fn get_facebook_subdomains(url_api_fb: &str, quiet_flag: bool) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("Facebook")
    }
    get_from_http_api::<ResponseDataFacebook>(url_api_fb, "Facebook")
}

pub fn get_spyse_subdomains(url_api_spyse: &str, quiet_flag: bool) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("Spyse")
    }
    get_from_http_api::<ResponseDataSpyse>(url_api_spyse, "Spyse")
}

pub fn get_anubisdb_subdomains(
    url_api_anubisdb: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("AnubisDB")
    }
    get_from_http_api::<HashSet<String>>(url_api_anubisdb, "AnubisDB")
}

pub fn get_bufferover_subdomains(
    url_api_bufferover: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("Bufferover")
    }
    get_from_http_api::<SubdomainsBufferover>(url_api_bufferover, "Bufferover")
}

pub fn get_threatcrowd_subdomains(
    url_api_threatcrowd: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("Threatcrowd")
    }
    get_from_http_api::<SubdomainsThreatcrowd>(url_api_threatcrowd, "Threatcrowd")
}

pub fn get_virustotal_apikey_subdomains(
    url_virustotal_apikey: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        println!("Searching in the Virustotal API using apikey... 🔍");
    }
    get_from_http_api::<SubdomainsVirustotalApikey>(
        url_virustotal_apikey,
        "Virustotal API using apikey",
    )
}

pub fn get_urlscan_subdomains(url_api_urlscan: &str, quiet_flag: bool) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("Urlscan.io")
    }
    get_from_http_api::<ResponseDataUrlscan>(url_api_urlscan, "Urlscan.io")
}

pub fn get_threatminer_subdomains(
    url_api_threatminer: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("Threatminer")
    }
    get_from_http_api::<SubdomainsThreatminer>(url_api_threatminer, "Threatminer")
}

pub fn get_c99_subdomains(url_api_c99: &str, quiet_flag: bool) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("C99")
    }
    get_from_http_api::<ResponseDataC99>(url_api_c99, "C99")
}

pub fn get_ctsearch_subdomains(
    url_api_ctsearch: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("Ctsearch")
    }
    match return_reqwest_client(15).get(url_api_ctsearch).send() {
        Ok(data_ctsearch) => {
            if networking::check_http_response_code("Ctsearch", &data_ctsearch) {
                match data_ctsearch.json::<HashSet<SubdomainsCtsearch>>() {
                    Ok(domains_ctsearch) => Some(
                        domains_ctsearch
                            .iter()
                            .map(|sub| {
                                let str_vec =
                                    sub.subject_dn.split(&['=', ','][..]).collect::<Vec<&str>>();
                                if str_vec.len() > 1 {
                                    str_vec[1]
                                } else {
                                    ""
                                }
                            })
                            .map(str::to_owned)
                            .collect(),
                    ),
                    Err(e) => {
                        check_json_errors(e, "Ctsearch");
                        None
                    }
                }
            } else {
                None
            }
        }
        Err(e) => {
            check_request_errors(e, "Ctsearch");
            None
        }
    }
}

pub fn get_archiveorg_subdomains(
    url_api_archiveorg: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        misc::show_searching_msg("Archive.org")
    }
    match return_reqwest_client(300).get(url_api_archiveorg).send() {
        Ok(data_archiveorg) => {
            if networking::check_http_response_code("Archive.org", &data_archiveorg) {
                match data_archiveorg.json::<Vec<Vec<String>>>() {
                    Ok(domains_archiveorg) => Some(
                        if !domains_archiveorg.is_empty() && domains_archiveorg.len() > 1 {
                            domains_archiveorg[1..]
                                .iter()
                                .flat_map(|sub| {
                                    sub.iter().map(|sub| {
                                        let str_vec = sub
                                            .split(&['/', ':', '?', '&'][..])
                                            .collect::<Vec<&str>>();
                                        if str_vec.len() > 3 {
                                            str_vec[3]
                                        } else {
                                            ""
                                        }
                                    })
                                })
                                .map(str::to_owned)
                                .collect()
                        } else {
                            HashSet::new()
                        },
                    ),
                    Err(e) => {
                        check_json_errors(e, "Archive.org");
                        None
                    }
                }
            } else {
                None
            }
        }
        Err(e) => {
            check_request_errors(e, "Archive.org");
            None
        }
    }
}
