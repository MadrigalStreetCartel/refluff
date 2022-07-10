# FlyffU Data Structures
> This is a work in progress. If you know anything about this and want to help, get in contact via Discord!

## World Data

### Info

Endianness: Big (not sure if correct, seems to make the most sense right now)

You can play around with the file format using fluffhammer (requires Rust knowledge): [Fluffhammer Docs](./fluffhammer.md)

### Structure

Each world data file has the same 4-byte magic number.

| Offset | Hex Value | UTF-8 | Remarks |
| ------ | --------- | ----- | ------- |
| 0x0000 | 0x25      | %     |         |
| 0x0001 | 0x43      | C     |         |
| 0x0002 | 0x4A      | J     |         |
| 0x0003 | 0x53      | S     |         |

Afterwards, it's getting a bit tricky.

| Offset | Hex Value | Remarks                           |
| ------ | --------- | --------------------------------- |
| 0x0004 | 0x01      | Maybe version number? Always 0x01 |
| 0x0005 | ?         |                                   |
| 0x0006 | ?         |                                   |
| 0x0007 | ?         | Always between 0x01 and 0x09      |
| 0x0008 | 0x00      | Always 0x00                       |
| 0x0009 | ?         |                                   |
| 0x000a | ?         |                                   |
| 0x000b | ?         | Always 0x00 or 0x01               |
| 0x000c | 0x00      | Always 0x00                       |

I'm guessing `0x0007-0x0008` should be read as a `uint16`. It's always a multiple of 256 in BigEndian.
