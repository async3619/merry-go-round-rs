<h1 align="center">
  <br />
  🎠
  <br />
  Merry Go Round
  <sup>
    <br />
    <br />
  </sup>
</h1>

<div align="center">
    <a href="https://www.npmjs.com/package/@async3619/merry-go-round">
        <img alt="npm (tag)" src="https://img.shields.io/npm/v/@async3619/merry-go-round?style=flat-square">
    </a>
    <a href="https://github.com/async3619/merry-go-round-rs/blob/main/LICENSE">
        <img src="https://img.shields.io/github/license/async3619/merry-go-round-rs.svg?style=flat-square" alt="MIT License" />
    </a>
    <br />
  <sup>Node.js native module for handling media files</sup>
  <br />
  <br />
</div>

## Introduction

<img align="right" src="https://raw.githubusercontent.com/async3619/merry-go-round-rs/main/docs/merry-go-round.gif" />

This is a node.js native module to handle information of media files. as you already know, most of the media files out there have their
own tag system such as ID3, APE and so on. this module is developed for this purpose!

<br />
<br />
<br />
<br />

## Installation

```bash
npm install @async3619/merry-go-round

# or

yarn add @async3619/merry-go-round

# or

pnpm add @async3619/merry-go-round
```

## Usage

```js
const { Audio } = require('merry-go-round');

const audio = Audio.fromFile('path/to/file.mp3');
// or you can use Audio.fromBuffer(buffer) to read from buffer

console.log(audio.title); // => 'title'
console.log(audio.artist); // => 'artist'
console.log(audio.album); // => 'album'
console.log(audio.year); // => 'year'
console.log(audio.genre); // => 'genre'

// ...
```