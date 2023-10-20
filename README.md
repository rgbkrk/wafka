![WAFKA Logo -- the Waluigi of Kafka](https://github.com/rgbkrk/wafka/assets/836375/0caee473-3183-4d4c-afd2-f59bb69cbca2)


# WAFKA - Web Assembly bindings for Kafka

> [!WARNING]
> This is a Work in Progress. `wasm-bindgen` can't automagically generate based on the `rdkafka` rust crate out of the box.

## Goal

A reliable, fast, and secure way to write Kafka producers for running on edge networks (Deno, Netlify, etc.). 

## Get Hacking

### Build

This package relies on [`wasmbuild`](https://github.com/denoland/wasmbuild) to make creating Web Assembly from Rust very easy. Install `deno` then run:

```
deno task wasmbuild
```
