# fluffhammer
> CLI tool for reading and manipulating resource files (*.bin)

As with everything in this repo, this is a work in progress.

## Info

This tool only works with world files at the moment. Right now it mainly serves as a reverse engineering helper and allows you to test certain assumptions about the file format. You can use it to play around with the binary, print a few offsets and values and see if you can make any sense of them.

Findings are documented here: [Data Structure Documentation](./re_datastructures.md)

## Usage

**Read wdmadrigal.bin world file**

```bash
cd fluffhammer
cargo run -- ../clients/v42/res/world/wdmadrigal/wdmadrigal.bin
```
