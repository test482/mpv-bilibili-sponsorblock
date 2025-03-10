<h1 align="center">Bilibili SponsorBlock plugin for MPV</h1>

Copy from [mpv-sponsorblock](https://github.com/TheCactusVert/mpv-sponsorblock), made changes for bilibili website.

## Build

Build the plugin:

```bash
cargo build --release --locked
```

## Installation

### Linux

<details>
<summary>MPV</summary>

Copy the lib generated to your `scripts` folder:

```bash
cp ./target/release/libmpv_bilibili_sponsorblock.so ~/.config/mpv/scripts/
```

</details>

### Windows

Installation for Windows is available : <https://mpv.io/manual/stable/#c-plugins>.

I don't use Windows. Feel free to make a pull request.

## Configuration

Copy the exemple configuration file `sponsorblock.toml` into your **MPV** (not Celluloid) folder:

```bash
cp ./bilibili-sponsorblock.toml ~/.config/mpv/script-opts/bilibili-sponsorblock.toml
```

If no configuration file is found, only the sponsors segments will be skipped as specified by the [API](https://wiki.sponsor.ajay.app/w/API_Docs).

A segment is the combination of a category and an action type.

## Usage

Play a YouTube video and segments you chose in the configuration file will be skipped or muted.
