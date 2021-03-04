# yarner-toc

[![Test status](https://github.com/mlange-42/yarner-toc/actions/workflows/tests.yml/badge.svg)](https://github.com/mlange-42/yarner-toc/actions/workflows/tests.yml)
[![Crate](https://img.shields.io/crates/v/yarner-toc.svg)](https://crates.io/crates/yarner-toc)

A [Yarner](https://github.com/mlange-42/yarner) plugin to generate a Table of Content.

## Installation

**Binaries**

1. Download the [latest binaries](https://github.com/mlange-42/yarner-toc/releases) for your platform
2. Unzip somewhere
3. Add the parent directory of the executable to your `PATH` environmental variable

**Using `cargo`**

```
> cargo install yarner-toc
```

## Usage

Add a section `plugin.toc` to your `Yarner.toml`:

```toml
[plugin.toc]
```

## Options

The plugin allows for different options, which are all optional:

```toml
[plugin.toc]
placeholder = "[[_TOC_]]"
min-level = 2
max-level = 5
```

| Option             | Details                             | Default     |
|--------------------|-------------------------------------|-------------|
| `placeholder`      | Placeholder to replace with the TOC | `[[_TOC_]]` |
| `min-level`        | Minimum heading level to include    | `2`         |
| `max-level`        | Maximum heading level to include    | `5`         |
