# `Oxi::Test`

This is just a test Ruby on Rust Rust gem. It is a simple example to display
show a fully working Rust gem can be built and published to RubyGems.org.

Use this as inspiration for your own Rust gems. If you are looking for a more
productionized setup, [check out the config for `wasmtime-rb`][wasmtime-rb] as
well.

## Features

✅ Binary Gems for 8 Platforms ([code](Rakefile))
✅ GitHub Actions CI ([code](.github/workflows/ci.yml))
✅ GitHub Actions Cross Compilation ([code](.github/workflows/cross-gem.yml))

## Tricks and Tips

### Interactive Cross Compiling

Inside this repo, you can shell into a cross compilation container with by
running the following command. This will drop you into a shell inside the
container, where you can run build the gem for the specified platform. Works
with any `rb-sys` gem.

```sh
(host)      $ bundle exec rb-sys-dock --platform aarch64-linux
(container) $ bundle install && bundle exec rake native:aarch64-linux
```

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

[wasmtime-rb]: https://github.com/bytecodealliance/wasmtime-rb
