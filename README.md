# Shire

Shire is a simple no fuss ddns client for Cloudflare. It allows you to update Cloudflare DNS record(s) with your current IP address.

## Installation

## Usage

```
$ shire --help
Shire is a simple no fuss ddns client for Cloudflare

Usage: shire [OPTIONS] --zone-id <ZONE_ID> --key <KEY>

Options:
  -r, --records <RECORDS>  Comma seperated list of the record names to update
  -z, --zone-id <ZONE_ID>  The Cloudflare Zone Id for your domain
  -k, --key <KEY>          Your Cloudflare API key
  -h, --help               Print help
  -V, --version            Print version
```

### Example

If we wanted to update the record `test` we could do

```
shire --key $CF_API_KEY --zone-id bab32631af40d574ag246741013k40z3 --records test
# or
shire -k $CF_API_KEY -z bab32631af40d574ag246741013k40z3 -r test
```

## Zone Id

To get your Zone's Id visit your domain on the [Cloudflare Dashboard](https://dash.cloudflare.com?to=/:account/:zone). Click on the "Overview" tab, and scroll until you see the "API" on the right side of the page. You can then click to copy your Zone Id.

![](./.github/zone-id.webp)

## Plans

- [ ] Support IPv6
- [ ] Configurable IP resolver