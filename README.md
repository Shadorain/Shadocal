</div>

<div align="center">

<br>

# 🗓️ Shadocal 🗓️

> A blazingly fast, Google calendar (more in future) event formatter webserver tool.

<a href="https://docs.rs/shadocal/latest/shadocal/"> ![Docs](https://img.shields.io/docsrs/shadocal?color=37d4a7&logo=rust&style=for-the-badge)</a>
<a href="https://crates.io/crates/shadocal"> ![Crate](https://img.shields.io/crates/v/shadocal?color=ff4971&style=for-the-badge)</a>
<a href="/LICENSE"> ![License](https://img.shields.io/badge/license-GPL%20v3-blueviolet?style=for-the-badge)</a>
<a href="#development"> ![Status](https://img.shields.io/badge/status-WIP-informational?style=for-the-badge&color=ff69b4) </a>

[Usage](#usage)
•
[Notes](#notes)
•
[Examples](#examples)
•
[Development](#development)
<br>
[Docs](https://docs.rs/shadocal/latest/shadocal/)

</div>

## Usage

Currently in it's binary state, Shadocal can only be run as an executable from the command line.

It should be run with no arguments. Any configuration is done with environment variables.

### Environment

Shadocal spins up a webserver on an environment variable controlled port and ip:
```bash
# Default PORT and IP
export SHADOCAL_IP=127.0.0.1
export SHADOCAL_PORT=7117
```

### Authentication

> This step only has to be done once.

To authenticate with OAuth2 to your Google Calendar you must configure some things on the Google side. 
Use the beginning steps of [this handy guide](https://developers.google.com/calendar/api/quickstart/nodejs) from the NodeJS team to help with this.

- Enable the API
- Configure the OAuth consent screen
- Download the `credentials.json` file

Once you have your `credentials.json` file, get the contents and encode them in [`base64` format](https://www.base64encode.org/).
Now set this environment variable to the encoded string:
```bash
export GOOGLE_KEY_ENCODED=PUT_THE_BASE64_KEY_HERE
```

This can be put in a file (without "export") in your local data directory.  
On Linux: `$HOME/.local/share/shadocal/client`.

On the first run of Shadocal, it will ask you to authenticate with your Google account using the credentials you provided. It will store the refresh token in the same data directory.

### Installation

#### Building from Source

The Rust ecosystem must be installed, use the [Rustup toolchain](https://rustup.rs/) installer which makes the process incredibly easy.

To install the latest stable version of Shadocal using `cargo` use this command:
```rust
cargo install shadocal
```

#### Executable

Currently there are no executable builds for this project yet, see [Tasks](#tasks).


## Notes

### Server

Shadocal uses the backend actix-web for its speed and simplicity.

CORS is enabled to ensure safe usage of the API.

### Formatters

Currently there are only two hardcoded formatters available:
- Raw: Simple string representation of a Google Event
- Tana: Parsed into a format recognizeable by the [PKM system Tana](https://tana.inc)

I hope to improve this drastically by allowing the user to create custom formats.


### Examples

Be sure to visit the [Wiki](https://github.com/Shadorain/ShadoGCal/wiki) for examples and use cases!


## Development

> I'm happy to address any Issues or Pull Requests when I can.

### Tasks

> In the current state this project meets my needs for my Tana workflow but I
> will continue to work on this as I can to add these other features.

- [ ] Allow multiple Google account connections
  - Should be possible assuming OAuth2 authorization doesn't hold me back here
- [ ] Make this into an API library
- [ ] Allow for user defined custom `Format`s
- [ ] Add better customization of hard coded `Format`s
- [ ] Potentially add support for other calendar types
  - This adds uneeded complexity depending on how the Authentication is done.
- [ ] Potentially ship binary builds for Linux, Mac, and Windows.
  - This is quite a tedius and fallible process, so I will push this off unless it gains alot of traction.
- [ ] Graphical User Interface (depends on above)
  - This could be quite a pain and would definitely need shipping of executables
  - Also multi platform support isn't the easiest with this so can see how needed it is
