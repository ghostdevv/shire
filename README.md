# Shire

Shire is a simple no fuss ddns client for Cloudflare. It allows you to update Cloudflare DNS record(s) with your current IP address.

## Installation

Grab your operating systems binary from the [Releases Tab](https://github.com/ghostdevv/shire/releases). You can then run this from terminal.

### Linux One Liner

If you're using linux you can use the following command to install it easily:

```bash
curl -sL -o shire https://github.com/ghostdevv/shire/releases/latest/download/shire-linux-amd64 \
  && chmod +x shire \
  && sudo mv shire /usr/local/bin \
  && sudo chown root:root /usr/local/bin/shire
```

## Usage

```bash
$ shire --help
Shire is a simple no fuss ddns client for Cloudflare

Usage: shire [OPTIONS] --zone-id <ZONE_ID> --key <KEY>

Options:
  -r, --records <RECORDS>          Comma seperated list of the record names to update
  -z, --zone-id <ZONE_ID>          The Cloudflare Zone Id for your domain
  -k, --key <KEY>                  Your Cloudflare API key
  -i, --ip-resolver <IP_RESOLVER>  The IP resolver url to use, this defaults to ip.willow.sh [default: https://ip.willow.sh]
  -h, --help                       Print help
  -V, --version                    Print version
```

### Cron

Shire is intended to be run as a cron job on linux, here is an example cron configuration:

```
0 0 * * * /bin/bash /usr/local/bin/shire -k CF_API_KEY -z ZONE_ID -r RECORDS
```

### Example

If we wanted to update the record `test` we could do

```bash
shire --key CF_API_KEY --zone-id bab32631af40d574ag246741013k40z3 --records test
# or
shire -k CF_API_KEY -z bab32631af40d574ag246741013k40z3 -r test
```

## Configuration

### Cloudflare API Key

You can generate an API Key by visiting the [Cloudflare Dashboard](https://dash.cloudflare.com/profile/api-tokens). Once you click the "Create Token" button, if available click the "Edit zone DNS" template, and then fill in similar to this:

![](./.github/cloudflare-api-key.webp)

Add as many zones as you are using with shire, or allow access to all zones. 

### Zone Id

To get your Zone's Id visit your domain on the [Cloudflare Dashboard](https://dash.cloudflare.com?to=/:account/:zone). Click on the "Overview" tab, and scroll until you see the "API" on the right side of the page. You can then click to copy your Zone Id.

![](./.github/zone-id.webp)

## Plans

- [ ] Support IPv6
- [ ] Configurable IP resolver
