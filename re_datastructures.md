# FlyffU Data Structures
> This is a work in progress. If you know anything about this and want to help, get in contact via Discord!

## World Data

### Header

**Info**

Each world data file has a fixed 5-byte header. Since 5 bytes is a pretty weird length for header, I'm guessing it's actually a 4-byte (uint32) header and the fifth byte means something different, but is coincidentally always the same.

**Structure**

| Offset | Hex Value | UTF-8 | Remarks                                 |
| ------ | --------- | ----- | --------------------------------------- |
| 0x00   | 0x25      | %     |                                         |
| 0x01   | 0x43      | C     |                                         |
| 0x02   | 0x4A      | J     |                                         |
| 0x03   | 0x53      | S     |                                         |
| 0x04   | 0x01      |       | Not sure if actually part of the header |
