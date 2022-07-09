# FlyffU Resource URLs
> This is a work in progress. If you know anything about this and want to help, get in contact via Discord!

## General Information

**Base URL**

Base URL (Client CDN): https://gcpcdn-universe.flyff.com/client

**Hash Suffix**

All files seem to have some kind of hash as a suffix. From local testing, the resources can be downloaded just fine without the hash, so I'm guessing it's just for cache invalidation.

## Resource URLs

### Program
> This endpoint only seems to contain the main client wasm file.

| Files |
| ---
| program/web/main-wasm32.js

### World
> World data consists of many parts. Part numbering seems to be fairly deterministic. There are two variable parts in the URL. The first part ranges from `29-36` (inclusive) and the second part ranges from `09-16` (inclusive).

| Files |
| ---
| world/wdmadrigal/wdmadrigal.bin?A305FF1
| world/wdmadrigal/wdmadrigal<29-36>-<09-16>.bin?A2317A8

### Map

| Files |
| ---
| map/dxt/madrigalloadinglow.bin?A111A4D

### UI

| Files |
| ---
| ui/world.bin?A5A8977
| ui/world_v19.bin?A111979

### Skill

| Files |
| ---
| skill/assist.bin?A111A65

### Item

| Files |
| ---
| item/item.bin?A5BCD3F

### Env

| Files |
| ---
| env/texturemid/dxt/default.bin?A10F743
| env/sun.bin?A1118D9
| env/dxt/moon.bin?A1118D8
| env/texturemid/dxt/default.bin?A10F743

### Cursor

| Files |
| ---
| cursor/curbase.cur
| cursor/curbase.png
| cursor/curdelay.cur
| cursor/curdelay.png
| cursor/hori.cur
| cursor/hori.png
