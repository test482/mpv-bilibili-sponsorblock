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
cp ./target/release/libmpv_bilibili_sponsorblock.so ~/.config/mpv/scripts/bilibili_sponsorblock.so
```

</details>

### Windows

<details>
<summary>MPV</summary>

Installation for Windows is available : <https://mpv.io/manual/stable/#c-plugins>.

I don't use Windows. Feel free to make a pull request.

</details>

## Configuration

Copy the exemple configuration file `bilibili-sponsorblock.toml` into your **MPV** folder:

```bash
cp ./bilibili-sponsorblock.toml ~/.config/mpv/bilibili-sponsorblock.toml
```

If no configuration file is found, only the sponsors segments will be skipped as specified by the [API](https://github.com/hanydd/BilibiliSponsorBlock/wiki/API).

A segment is the combination of a category and an action type.

## Usage

Play a Bilibili video and segments you chose in the configuration file will be skipped or muted.
