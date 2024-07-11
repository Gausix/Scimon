<div align='center'>
    <img src="https://i.imgur.com/ZZ9a1DU.png"/>
</div>

<p align='center'><b>Unleash your knowledge.</b></p>

<p align='center'>
	<a href='https://github.com/Scibun/Scibun/actions/workflows/rust.yml'><img src='https://img.shields.io/github/actions/workflow/status/scibun/scimon/rust.yml?style=flat-square'/></a>
	<img src='https://img.shields.io/github/license/Scibun/Scimon?style=flat-square'/>
</p>

<p align='center'>
    <img src='https://i.imgur.com/brFeOKa.png' />
</p>

## Documentation

For more help and document, see our documentation:

- [How to build](https://scibun.github.io/ScimonDocs/build.html)
- [Basic usage](https://scibun.github.io/ScimonDocs/basic-usage.html)
- [Flags](https://scibun.github.io/ScimonDocs/flags.html)
- [Downloads Block](https://scibun.github.io/ScimonDocs/download-block.html)
- [Readme Block](https://scibun.github.io/ScimonDocs/readme-block.html)
- [Commands Block](https://scibun.github.io/ScimonDocs/commands-block.html) (Experimental)
- [Open links](https://scibun.github.io/ScimonDocs/open-links.html)
- [Directives and Comments](https://scibun.github.io/ScimonDocs/directives.html)
- [Markdown render](https://scibun.github.io/ScimonDocs/markdown-render.html)
- [Scrape](https://scibun.github.io/ScimonDocs/scrape.html)
- [Providers](https://scibun.github.io/ScimonDocs/providers.html)
- [Scimon.yml file](https://scibun.github.io/ScimonDocs/scimon.yml-file.html)
- [.env file](https://scibun.github.io/ScimonDocs/env-file.html)
- [External Resources Usage](https://scibun.github.io/ScimonDocs/external-resources.html)

## Example of code and execute

```monset
path = "downloads/"
open = "https://github.com/kremilly"

downloads {
    https://arxiv.org/pdf/2405.01513
    https://olacesar.com/e-books/protegido.pdf
    https://raw.githubusercontent.com/facebook/react/main/README.md !ignore
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
scimon -r scimon.mon
```
