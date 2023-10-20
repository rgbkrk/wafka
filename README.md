![WAFKA Logo](https://github.com/rgbkrk/wafka/assets/836375/21e3db2b-6fdc-46f6-836d-537274e990d6)

# WAFKA - Web Assembly bindings for Kafka

> [!WARNING]
> This is a Work in Progress. `wasm-bindgen` can't automagically generated based on the `rdkafka` rust crate out of the box.

## Goal

A reliable, fast, and secure way to write Kafka producers for running on edge networks (Deno, Netlify, etc.). 

## Get Hacking

### Build

This package relies on [`wasmbuild`](https://github.com/denoland/wasmbuild) to make creating Web Assembly from Rust very easy. Install `deno` then run:

```
deno task wasmbuild
```
