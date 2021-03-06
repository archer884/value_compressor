Value Compressor
================

An experiment in:

1. Writing raw data to disk.
2. Using specialized compression?
3. Compressing in-memory streams?

I don't know how to do most of this in Rust, although I'm pretty familiar with a lot of it in C#, so it's been interesting so far.

To view this program's output, try `hexdump -C compress.out` or `hexdump -C uncompress.out`.

## Discoveries

1. Not going to bother with specialized compression for the time being because getting the normal stuff working has been such a chore.
2. There's no such thing as an "in-memory stream" in Rust because byte arrays, slices, and vectors apparently all serve pretty much *exactly* that purpose, which I found pretty cool except that most things expect an array and don't want a vector, which is actually pretty annoying in the grand scheme of things.
3. That last thing is made somewhat less annoying by the existence of `std::io::copy()`, which lets you skip most of the (awful) drudge work of actually copying data from one stream to another (kind of like what `copy()` does for you in C#, except it's a function instead of a method).

## Status

`value_compressor 1000 false` prints the expected output: a file with a length of 4000, or the equivalent length of 1000 32-bit integers. At present, `value_compressor 1000 true` prints something pretty surprising: a file with a length of *more* than 4000, or the equivalent length of... Well, 1000 32-bit integers with some extra garbage thrown in. This is pretty much exactly the opposite of what I would have expected to happen.

[The man page](http://www.bzip.org/1.0.5/bzip2.txt) for bzip2 suggests that files smaller than 100 bytes will tend to increase in size, which I'm cool with, but this file is actually *500* bytes. This behavior still holds true when the command calls for 10,000 integers instead of 1,000, which suggests to me that it's *definitely* not just a case of aberrant behavior on the part of the compression algorithm unless it is simply totallhy incapable of compressing integers...

But what the hell am I doing wrong?
