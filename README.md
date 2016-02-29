# Fetcher

[![Build Status](https://travis-ci.org/zyphrus/fetcher.svg)](https://travis-ci.org/zyphrus/fetcher)
[![Coverage Status](https://coveralls.io/repos/zyphrus/fetcher/badge.svg)](https://coveralls.io/r/zyphrus/fetcher)

An automated media fetcher written in rust.


Primarily used for getting scheduled released content like podcasts or
web series.


This repo provides two binaries `libfetcher` and `fetcherd`

### libfecther

To provide a library for fetcher

### fetcherd

A daemon to fetch periodically using libfetcher.

## Goal

To fetch from multiple sources in a modular fashion with a focus on plug and download.
For example to be able to source the media to fetch from a local file or a Web
API, like [fetch-django](https://github.com/zyphrus/fetch-django).
Then using the media info scrap and fetch it from various sources.

## Build

- run `cargo build`
