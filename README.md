<p align="center">
    <img src="/static/logo.png" width="300" />
    <h3 align="center">JuMP</h3>
    <p align="center">A Command Line Tool for Fast System Navigation in Rust.</p>
    <p align="center">
        <a href="https://github.com/Clivern/Jump/actions"><img src="https://github.com/Clivern/Jump/actions/workflows/build.yml/badge.svg"></a>
        <a href="https://github.com/Clivern/Jump/releases"><img src="https://img.shields.io/badge/Version-v0.2.0-green.svg"></a>
        <a href="https://github.com/Clivern/Jump/blob/main/LICENSE"><img src="https://img.shields.io/badge/LICENSE-MIT-green.svg"></a>
    </p>
</p>


## Usage

Here is how to install and use `ju` command line tool.

```zsh
$ cargo install ju

# Store a new path with name root
$ ju new root
> What is the project path? /

# To remove a path
$ ju remove $name
```

Then add a function to jump

```zsh
jut () {
    cd $(ju to $1)
}
``

It can be used like this

```zsh
jut root
```


## Versioning

For transparency into our release cycle and in striving to maintain backward compatibility, Jump is maintained under the [Semantic Versioning guidelines](https://semver.org/) and release process is predictable and business-friendly.

See the [Releases section of our GitHub project](https://github.com/clivern/jump/releases) for changelogs for each release version of Jump. It contains summaries of the most noteworthy changes made in each release.


## Bug tracker

If you have any suggestions, bug reports, or annoyances please report them to our issue tracker at https://github.com/clivern/jump/issues


## Security Issues

If you discover a security vulnerability within Jump, please send an email to [hello@clivern.com](mailto:hello@clivern.com)


## Contributing

We are an open source, community-driven project so please feel free to join us. see the [contributing guidelines](CONTRIBUTING.md) for more details.


## License

© 2022, clivern. Released under [MIT License](https://opensource.org/licenses/mit-license.php).

**Jump** is authored and maintained by [@Clivern](http://github.com/clivern).
