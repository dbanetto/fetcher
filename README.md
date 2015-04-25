# Fetcher

[![Build Status](https://travis-ci.org/zyphrus/fetcher.svg)](https://travis-ci.org/zyphrus/fetcher)

An automated media fetcher written in rust.
Primarily used for getting scheduled released content like
podcasts or web series.

This repo provides two binaries `libfetcher` and `fetchd`

### libfecther

To provide a library for fetcher

### fetchd

A daemon to fetch periodically

## Goal

To fetch from multiple sources in a modular fashion with a focus on plug and download.
For example to be able to source the media to fetch from a local file or a Web API
and to scrap and fetch those listed media from various sites.

## Build

- run `cargo build`
