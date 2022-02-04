# Digital Garden CLI

A CLI tool for the creation and maintenance of Digital Gardens.

This project was written following the [Create a Digital Garden CLI with Rust](https://egghead.io/courses/creating-a-digital-garden-cli-with-rust-34b8) course.

## Commands

### Setting the path to the garden

```shell
GARDEN_PATH=~/work/posts garden write
garden -p ~/work/posts write
garden --garden-path ~/work/posts write

````

### write

Open a new file to write in our digital garden. Since we don't necessarily know what we want to title what we're writing, we'll leave the title as optional and ask the user for it later if they don't provide it up-front.

```shell
garden write
garden write -t "My title"

```
