# Motivation

Some users do not want to fight with the setup of a monitoring server including database configuration, buying a dedicated VPS, creating cron jobs or systemd timers/services and more. Findomain+ monitoring server is a easy way to get the most of this powerful tool only editing two files: the list of domains to monitor and a configuration file to set up your API keys (optional) and webhooks (required).

# About the Findomain+ server

Findomain+ monitoring server that is included in the tiers from Findomain+ VIP Patron onwards is a dedicated VPS hosted in Amazon, this server is specifically designed for subdomains monitoring. When you got a tier equal or major to Findomain+ VIP you got access to this server via a chrooted FTP environment, there you can modify a `targets.txt` (the domains list to monitor) and `config.toml` (the configuration) files. Additionally you got access to a "logs" folder inside the FTP access, that folder contains all the logs for every Findomain Plus execution, there is one file per target. When new subdomains are found the old files are renamed to `*.old.txt`, so that you can difference they. I recommmend you to use a FTP client such as [Filezilla](https://filezilla-project.org/) to connect to server.

When you configure it, a systemd timer is launched and start the monitoring process for new subdomains according to your configuration file, the frequency is based on your tier as well starting from every 30 hours.

The server will send you the alerts to the webhook(s) you have configured previously according to the frequency that you got in your tier.

# Getting access

[Recommended read](https://github.com/Edu4rdSHL/findomain#findomain-plus-version).

## Patreon

<a href="https://www.patreon.com/bePatron?u=30383720"><img src="https://c5.patreon.com/external/logo/become_a_patron_button@2x.png" alt="Patreon donate button" /> </a>

## Paypal

**By default donations to Paypal will not give access to the Plus version. If for any reason you want to make your monthly suscription using Paypal instead of Patreon, please reach me out in [Twitter](https://twitter.com/edu4rdshl) and I will send you the binaries. Anyways I recommend that you use Patreon for the Findomain+ suscription.**

[![paypal](https://www.paypalobjects.com/en_US/i/btn/btn_donateCC_LG.gif)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=37K3XKBUD2JJ8)
