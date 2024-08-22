![Logo](https://github.com/codaholichq/mailpass/blob/main/docs/banner.webp)

<div align="center">
	<img alt="status" src="https://github.com/codaholichq/mailpass/actions/workflows/rust.yml/badge.svg?branch=main">
    <img alt="status" src="https://img.shields.io/badge/status-developing-brightgree">
	<img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/codaholichq/mailpass">
	<img alt="GitHub closed issues" src="https://img.shields.io/github/issues-closed/codaholichq/mailpass">
	<img alt="GitHub stars" src="https://img.shields.io/github/stars/codaholichq/mailpass">
	<img alt="visitors" src="https://visitor-badge.laobi.icu/badge?page_id=mailpass">
	<a href="https://x.com/intent/tweet?hashtags=mailpass,rust,axum,oss&text=A+dApp+that+checks+if+an+email+address+exists+without+sending+a+mail&url=https%3A%2F%2Fgithub.com%2Fcodaholichq%2Fmailpass&via=codaholichq">
		<img alt="𝕏" src="https://img.shields.io/twitter/url/http/shields.io.svg?style=flat&logo=twitter">
	</a>
</div>

<br/>

Welcome to `MailPass`, a (dApp) decentralized application that checks if an email address exists before sending any email.

Read the [Changelog file](https://github.com/codaholichq/mailpass/blob/main/docs/CHANGELOG.md) to see the new changes.

Clone this repository to get the latest unreleased version.

## Purpose of this Project
This project helps newsletter owners to:
- Prevent email from getting blacklisted
- Spot emails that don't exist
- Fish out trash emails

## Technologies
- Rust 1.80.1
- Cargo Watch
- Axum 0.7.5
- Tokio 1.39.2
- Vue 3.3.4
- Pinia 2.1.6
- DFinity
- VSCode 1.92.2

## Features
- [x] Swagger
- [x] Concurrency
- [x] Message Queuing

To start working on the backend, enter the following commands:

```bash
cd mailpass/
cargo watch -x run -C backend
```

The backend will be available at `http://localhost:3000`.

To start working on the frontend, enter the following commands:

```bash
cd mailpass/
npm start --prefix frontend
```

The frontend will be available at `http://localhost:8080`.

## Bugs? Suggestions?
Having any issues or trouble getting started? [Get in touch with me](https://www.codaholic.com/contact) or [Raise a Bug or Feature Request](https://github.com/codaholichq/configa/issues/new/choose). Always happy to help.

## Give a Star ⭐
If you find this project helpful, do give it a star. Thanks! <br/>
If you are feeling really generous, send me ETH: <code>0x9F4942911f2406E5897669Db99184d47B3078E99</code>

## Share it!
There are many improvements and fixes along the way from the day I started out. Thanks to the community for the support and suggestions.
Please share this Repository within your developer community, if you think this would make a difference! Thanks.

## About the Author
### Emmy Steven
- [𝕏](https://x.com/emmystevenx)
- [Blog](https://www.codaholic.com)
- [Linkedin](https://www.linkedin.com/in/emmysteven/)

## Licensing
This project is licensed with the [MIT License](https://github.com/codaholichq/mailpass/blob/main/docs/LICENSE).
