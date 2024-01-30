# Stepper

## What Does This Do

This program steps up or down collections of values.

## How Does It Do This

It takes a "current value", a "direction", and a file containing a "collection of values",
and then walks a "direction" of "up" or "down" to find the next larger or smaller values in the "collection".

## Usage

```
 $ stepper up 40 ~/.config/example/config
50
```

## Config File Format

The "config" file is just a list of floating point numbers, separated by line breaks.

```
0
25
50
75
100
```
