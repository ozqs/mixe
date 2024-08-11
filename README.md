<h1 align="center">MIX Computer Emulator</h1>

<p align="center">
  <img src="https://img.shields.io/badge/language-rust-orange.svg" alt="language rust">
  <img src="https://img.shields.io/github/license/ozqs/mixe" alt="license">
</p>

<p align="center">A MIX computer emulator written in Rust</p>

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Example](#example)

## Introduction

MIX is a hypothetical computer introduced by Donald Knuth in his classic book "The Art of Computer Programming". This project is a MIX computer simulator written in Rust, designed to help people learn and understand this computational model.

## Features

- Supports the MIX instruction set
- Simulates MIX memory and registers
- Provides a simple and user-friendly command-line interface
- Detailed error messages and debugging support

## Installation

To install this simulator, make sure you have Rust installed. Then, run the following commands:

```bash
git clone https://github.com/ozqs/mixe.git
cd mixe
cargo run --release
```

## Example

Here is a simple example of a MIX program:
```assembly
3991 ENT1 0
3992 MOVE 3995
3993 MOVE 0(43)
3994 JMP 3993
3995 HLT 0
```

Save this as `a.asm`.
Then run the commands below.

```
>> PARSE a.asm
Set memory 3991 to ENT1 0 : ENT1 0(2)
Set memory 3992 to MOVE 3995 : MOVE 3995
Set memory 3993 to MOVE 0(43) : MOVE 0(43)
Set memory 3994 to JMP 3993 : JMP 3993(0)
Set memory 3995 to HLT 0 : HLT 0(2)
>> START 3991
start at location 3991
>> PRINT 3-5
(HLT 0(2)) 0 00000085 | 000000000000000000000010000101
(HLT 0(2)) 0 00000085 | 000000000000000000000010000101
(HLT 0(2)) 0 00000085 | 000000000000000000000010000101
```