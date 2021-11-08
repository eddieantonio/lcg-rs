# LCG

This is my practice repo for learning how to write Rust code to provide
a library. I'm new to Rust, so this is not intended to be
a production-grade library—just me messing around with things. See
the [`rand` crate][rand] if you want a real random number generator.

# What is an LCG?

A **linear congruential generator** (LCG) is a psuedorandom number
generator that is straightforward to implement and can produce entirely
decent psuedorandom numbers for non-critical applications, like video
games. Don't use it for cryptography.

Basically everything I learned about them I learned from the
corresponding [Wikipedia article][wiki].

I used implementing this as an exercise to create a Rust library with
C and Python bindings.

[rand]: https://github.com/rust-random/rand
[wiki]: https://en.wikipedia.org/wiki/Linear_congruential_generator

# License

Unlicensed.
