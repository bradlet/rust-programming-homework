# Homework #2

Creating a toy RSA library crate.

Student: Bradley Thomspon | Odin Username: bradlet2

> CS 510 - Rust Programming _taught by_ Bart Massey
> Spring 2023
> Portland State University
> Maseeh College of Engineering and Computer Science

## What I did

I started out by just copying over the interface / contract defined in the homework handout.
Once I had the function signatures, I added just the simple unit testing that is currently
in lib.rs. I wanted to make sure that there was a simple test harness so that I could
understand the goal of the implementation, and have a scale to measure whether my
implementation was working correctly.

I didn't start out with randomized testing because I needed to take time to understand where
all of the values came from. At first it wasn't clear to me that both primes returned from
genkey acted as the private key, and yet their product was the pubkey. I had initially
misinterpreted this to mean that the first of those two values was the pubkey.

Once I got the original unit test cases passing, I implemented a separate crate 'hw2-cli'.
I didn't care to test it out, it was just a minimal cli implementation to manually test out
my code. Interacting with the cli is what got me to understand what values made up the private
key, how to get the pubkey, and how to actually use the functions that I had implemented
together.

Once I figured that out and made some slight adjustments to make that usage / interaction more
clear, I moved onto randomized testing. Had to look back at The Rust Book to remember how to
structure the `tests/` directory, but it wasn't too hard to setup a simple test that tested
out the lib relatively thoroughly with a high number of randomized test cases.

## How it went

It went well! I gave myself plenty of time to work on it a little bit at a time, which helped
me because I was able to always think through each piece with a fresh mind. The code itself
was extremely simple to implement, just had to take a little bit of time to grok how all the
pieces fit together.

## How I tested my work

This is essentially answered above, but for posterity:

1. Implement simple unit tests based on the provided test case in the handout.
2. Implement CLI and manually test until I understood how to use the lib, and was sure that
   the code worked.
3. Implement a randomized test that essentially generated 10,000 test cases per run to get
   extra verification that the library was working correctly.
