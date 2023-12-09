# url collector

a small cli utility that collects all urls from one (hierarchical) sitemap and offers a random selection of all urls. You can use JSON or YAML as output format

## Installation


1. Make sure, you have the rust and cargo tooling installed
2. Run `cargo install --git ssh://git@source.factorial.io:2222/shuber/url-collector.git`

Or 

* Download the binary from the releases

## Usage

Get help for all arguments

```shell
url-collector --help
```

Typical usage

```shell
# Will collect 10 random urls from the sitemap
url-collector --url https://spiegel.de --num-urls 10   
# Will get 100000 urls from a basic auth site and print yaml from it
url-collector --url https://myprivatewebsite.org --authentication user:seCret --num-urls 100000 --output yaml
```

Have fun
