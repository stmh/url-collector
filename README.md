# url collector (Current release: 0.11.6)

a small cli utility that collects all urls from one (hierarchical) sitemap and offers a random selection of all urls. You can use JSON or YAML as output format

## Installation


1. Make sure, you have the rust and cargo tooling installed
2. Run `cargo install --git https://github.com/stmh/url-collector.git`

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

## Development

### Creating a Release

To create a new release:

1. **Create the release tag and version bump:**
   ```shell
   cargo release patch --execute  # for patch releases (x.y.z)
   cargo release minor --execute  # for minor releases (x.y.0)
   cargo release major --execute  # for major releases (x.0.0)
   ```
   
   This will:
   - Bump the version in `Cargo.toml`
   - Generate/update the changelog using `git-cliff`
   - Update the version in this README
   - Create a signed git commit and tag
   - Push to the remote repository

2. **Create a GitHub Release:**
   After the tag is pushed, go to the [GitHub Releases page](https://github.com/stmh/url-collector/releases) and create a new release from the tag. This step is **required** to trigger the GitHub Actions workflow that builds and uploads the executables for different platforms.

   The executables will be automatically built and attached to the release for:
   - Linux (x86_64)
   - macOS (Intel and Apple Silicon)
   - Windows (x86_64)

## Sponsors

This project is sponsored by [Factorial GmbH](https://www.factorial.io) - a digital agency that designs, implements, and transforms digital ecosystems with custom software development and digital transformation strategies.

Have fun
