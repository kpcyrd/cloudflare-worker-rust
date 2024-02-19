# Cloudflare worker example project

Build a Hello World WebAssembly web-service with Rust and run it locally with Cloudflare's workerd:

```
pacman -S wrangler
git clone https://github.com/kpcyrd/cloudflare-worker-rust
cd cloudflare-worker-rust
wrangler dev
```

- [worker-rs Documentation](https://docs.rs/worker)
- [worker-rs Repository](https://github.com/cloudflare/workers-rs)

<a href="https://repology.org/project/wrangler/versions"><img align="right" src="https://repology.org/badge/vertical-allrepos/wrangler.svg" alt="Packaging status"></a>

## Using `npx wrangler`

If your Operating System does not provide a wrangler package, you can also run it through the Node.js package manager:

```
npx wrangler dev
```

<sup>There's currently a hack in wrangler.toml to make this possible</sup>

## License

`MIT OR Apache-2.0`
