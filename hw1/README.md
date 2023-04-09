# Homework #1

Creating a short CLI program to calculate an exponential modulo.

Student: Bradley Thomspon | Odin Username: bradlet2

> CS 510 - Rust Programming _taught by_ Bart Massey
> Spring 2023
> Portland State University
> Maseeh College of Engineering and Computer Science

## What I did

I started out making a separate library crate. I didn't need to do this at all, but I wanted to set it up so that It would be easier to
centrally store helpers to be re-used in future assignments. After grading, I'll likely move the `parse_num` helper there, as well.

After setting up that lib dependency, I focused on making the input validation and parsing logic. I wanted to change the `parse_num`
helper a little bit because I wanted to have some fun colors (saw in chapter 2 of the textbook) -- and also because I didn't want to
have any unwrapping or error result handling. Just planned on using it in `main`, so I'd just have to catch those and then panic, or
report the error somehow, anyway. I'd remembered seeing the `#[should_panic]` annotation (what's the Rust word for this again?) while reading The Rust Book semi-recently, so I figured it would be a nice opportunity to verify how that works; I wasn't sure if it would be the equivalent of `@Throws(some_exception::class.java)` in JVM-Land, or if it would actually perform an assertion.

After that, I finished input validation, created the signature for modexp, and made a test for it with a few assertions. Gave the algo
a first-pass and was able to succeed on my test cases; however, after pulling in the provided cases I encountered failures. After reading
through the handout again, I paid more attention to the part mentioning that internal operations would need `u128`s. Looked at the test
cases provided, and realized that it was obviously overflowing `u64`. So I just shadowed the arguments and converted the types as needed,
as suggested in the handout.

## How it went

It went really well! I had read through the first ~12 chapters of The Rust Book several months back, but had not really touched it much since.
This was a fun project to refresh on, go over my prior practice repo and try some new things. Also, the `text-colorizer` was very fun.

I was also very pleased to learn about `rustdoc` and `clippy`! Hadn't realized that so many of the crate docs I saw online were automagically
generated! And *clippy* helped me confirm that `/=` is in fact an operator! I was going to just not assume that Rust had it and roll with
`y = y / 2` even though it's less concise. Clippy showed me the way.

## How I tested my work
Described mostly inline above; I tested a few pieces as I built them with some light TDD. With the implementation of `modexp`, I started out by using Python to get several test assertinos to compare to. That made it really easy to get `modexp` to a working state quickly!
