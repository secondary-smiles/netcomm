# Netcomm

Communicate with servers interactively.



**Basically, netcat with fewer features made in Rust**



## Usage

```bash
$ netcomm [OPTIONS] <DOMAIN> <PORT>
```

## Where

**OPTIONS** are `-l` or `--listen`

**DOMAIN** is an IP or URL 

**PORT** is a valid port number (u16)



```bash
$ netcomm -h
Communicate with servers interactively

Usage: netcomm [OPTIONS] <DOMAIN> <PORT>

Arguments:
  <DOMAIN>
  <PORT>

Options:
  -l, --listen   Run in listen mode
  -h, --help     Print help information
  -V, --version  Print version information
```

## Demonstration

![Usage demonstration](/Users/phantomphreak/Documents/Projects/Code/rust/netcomm/social/showcase.gif)
