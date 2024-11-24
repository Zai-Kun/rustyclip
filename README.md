# RustyClip

RustyClip is a powerful clipboard management tool written in Rust. It allows you to easily manage and access your clipboard history.

## Features

- **Clipboard History**: Keep track of your clipboard history and access previous entries.
- **Multi-Media Support**: Supports pretty much all media types such as mp4, png, jpg, text, bin, etc.
- **Blazingly Fast**: Super fast because of how it stores clipboard data and it's coded in rust.
- **Easy To Use**: Simple and easy to remeber commands.

## Installation

### Downloading Binary from Release

1. Go to the [Releases](https://github.com/zai-kun/rustyclip/releases) page.
2. Download the appropriate binary for your operating system.
3. Extract the downloaded file.
4. Move the binary to a directory in your PATH.

### Compiling from Source

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone the repository:
    ```sh
    git clone https://github.com/zai-kun/rustyclip
    ```
3. Navigate to the project directory:
    ```sh
    cd rustyclip
    ```
4. Build the project:
    ```sh
    cargo build --release
    ```
5. The compiled binary will be located in `target/release/`.

---

## Usage
It was inspired by [Cliphist](https://github.com/sentriz/cliphist) so the usage is pretty similer to that.

```
Usage: rustyclip <command> [<args>]

A clipboard management tool

Options:
  --help            display usage information

Commands:
  list              List all stored clipboard items
  store             Store a new clipboard item
  get               Get a clipboard item by query
  remove            Remove a clipboard item by query
  clear             Clear all clipboard items
```

#### Listen for clipboard changes

`$ wl-paste --watch path/to/rustyclip store`  
This will listen for changes on your primary clipboard and write them to the history.  
Call it once per session - for example in your sway config.

#### Select old item
How it works is a bit interesting. Unlike `cliphist decode`, `rustyclip get` returns two things: a file path and a mime type seprated by a `\n`. As you may have guessed, the file holds the contents of the clipboard and the mime type is, well, the mime type (eg, image/png, text/plain, etc). Read [How it works](https://github.com/zai-kun/rustyclip#how-it-works) to find out why is that the case.
Here is a simple script of how you may use it:

```bash
#!/bin/bash

RUSTYCLIP=/path/to/rustyclip
PICKER=fuzzel
WLCOPY=wl-copy
NOTIFY_SEND=notify-send

# List clipboard entries and select one using a picker (fuzzel in this example)
output=$($RUSTYCLIP list | $PICKER -d| $RUSTYCLIP get)

# Extract the file path and MIME type from the output
file_path=$(echo "$output" | head -n 1)
mime_type=$(echo "$output" | tail -n 1)

# Check if the file exists and MIME type is non-empty
if [[ -f "$file_path" && -n "$mime_type" ]]; then
    # Copy the file content to the clipboard with the specified MIME type
    $WLCOPY -t "$mime_type" < "$file_path"
    
    # Notify the user
    $NOTIFY_SEND "Copied to clipboard" -t 500
fi
```

Here is a one-liner:
```bash
output=$(/path/to/rustyclip list | fuzzel -d | /path/to/rustyclip get); file_path=$(echo "$output" | head -n 1); mime_type=$(echo "$output" | tail -n 1); [[ -f "$file_path" && -n "$mime_type" ]] && wl-copy -t "$mime_type" < "$file_path"
```

#### Delete old item

`$ path/to/rustyclip list | fuzzel | path/to/rustyclip remove`

#### Clear database

`$ path/to/rustyclip clear`

Note: You can pass an index (starting from 0) to `rustyclip get`. The index can be provided directly or as a string separated by `:` where the first part is the index (an int).

---

## Todo
Empty for now

---

## How It Works

Instead of storing clipboard data in a single file or database, RustyClip stores each clipboard entry in a separate file and uses a manifest JSON file to hold information related to the file, such as mime type, preview text, etc. This way, it doesn't have to load a huge file just to perform small operations like listing, making it super fast. The downside of this method is that it creates a new file even for clipboard entries that are as small as 1 byte.


The reason it returns a mime type is, so you `wl-copy` doesn't have to infer the mimetype using `xdg-mime` maaking things much faster. For me, it sometimes took over 7 seconds for `xdg-mime` to infer the type with `cliphist`.

## Why?

Simple. I just couldn't find a clipboard manager that I liked. Until now, I was using `cliphist`, but it was super slow at times and that was frustrating. I made this project with speed in mind.

---

Feel free to ask any questions or contribute to the project by submitting issues or pull requests on the [GitHub repository](https://github.com/zai-kun/rustyclip).
