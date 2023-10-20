![WAFKA Logo -- the Waluigi of Kafka](https://github.com/rgbkrk/wafka/assets/836375/ee6747d3-3511-4515-80bd-3e1740a5aac0)


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
