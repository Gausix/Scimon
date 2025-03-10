<div align='center'>
    <img src="https://i.imgur.com/ZZ9a1DU.png"/>
</div>

<p align='center'><b>Unleash your knowledge.</b></p>

<p align='center'>
	<a href='https://github.com/Scibun/Scibun/actions/workflows/rust.yml'><img src='https://img.shields.io/github/actions/workflow/status/scibun/scimon/rust.yml?style=flat-square'/></a>
	<img src='https://img.shields.io/github/license/Scibun/Scimon?style=flat-square'/>
</p>

<p align='center'>
    <img src='https://i.imgur.com/RRPMQ2j.png' />
</p>

## What is Scimon?

Scimon is a tool designed for batch downloading PDF files using its own dedicated language, Monset (.mon). Monset features a very simple and quick-to-write syntax, making it easy to use. The Scimon interpreter is both fast and secure, as it is written in Rust, leveraging the language's best practices.

## What is Monset?

Monset is a language designed specifically for downloading files. It offers a streamlined syntax that makes the process of retrieving files from the internet straightforward and efficient. By focusing on simplicity, Monset ensures that users can quickly grasp its fundamentals and start downloading files with minimal effort.

The key strength of Monset lies in its user-friendly design. The syntax is intuitive, reducing the learning curve typically associated with programming languages. This makes it accessible to both beginners and experienced developers, allowing them to integrate file downloading capabilities into their projects seamlessly. Monset abstracts the complexities involved in file transfers, providing a clear and concise way to handle downloads.

## Documentation

For more help and document, see our documentation:

- [How to build](https://scimon.dev/build)
- [Basic usage](https://scimon.dev/basic-usage)
- [Flags](https://scimon.dev/flags)
- [Scrape](https://scimon.dev/scrape)
- [Providers](https://scimon.dev/providers)
- [Monset](https://scimon.dev/monset/what-is)
  - [Downloads Block](https://scimon.dev/monset/download-block)
  - [Readme Block](https://scimon.dev/monset/readme-block)
  - [Commands Block](https://scimon.dev/monset/commands-block)
  - [Compress folder](https://scimon.dev/monset/compress)
  - [Open links](https://scimon.dev/monset/open-links)
  - [Markdown render](https://scimon.dev/monset/markdown-render)
  - [Style](https://scimon.dev/monset/style)
  - [Print](https://scimon.dev/monset/prints)
  - [Covers](https://scimon.dev/monset/covers)
  - [QR Code](https://scimon.dev/monset/qrcode)
  - [Math](https://scimon.dev/monset/math)
- [Configs](https://scimon.dev/configs/index)
  - [Scimon.yml file](https://scimon.dev/configs/scimon.yml-file)
  - [.env file](https://scimon.dev/configs/env-file)
- [External Resources Usage](https://scimon.dev/external-resources)

## Example of code and execute

```monset
path "downloads/"
open "https://github.com/kremilly"

compress "folder.zip"

covers "covers/"

qrcode "qrcodes/"

print "Hello, World!"

math "2 + 2 = 4" > path/to/output.png

style "https://raw.githubusercontent.com/sindresorhus/github-markdown-css/main/github-markdown.css"

readme "https://gist.githubusercontent.com/Kremilly/5fd360d994bb0fe108b648d0e4c9e92f/raw/5f180716411e11fc352188c805c0707ac96d70a0/readme-example.md"

downloads {
    https://arxiv.org/pdf/2405.01513 !ignore
    https://raw.githubusercontent.com/facebook/react/main/README.md
    https://pt.wikisource.org/wiki/Manifesto_da_Guerrilha_do_Livre_Acesso !ignore
}

commands {
    https://gist.githubusercontent.com/Kremilly/e0e0db11e43269da179adab610f38bb1/raw/6820be26a936a54bac713d03deb49edf804d0b6b/index.py
}
```

> [!note]
>
> Save as `scimon.mon`

Run the command:

```bash
scimon run scimon.mon
```
