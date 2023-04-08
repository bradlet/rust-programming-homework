# bradlet2-rust-programming

Home for all homework assignment submissions for:

> CS 510 - Rust Programming _taught by_ Bart Massey
> Spring 2023
> Portland State University
> Maseeh College of Engineering and Computer Science

Student: Bradley Thomspon | Odin Username: bradlet2

## Project Structure

While this project will contain subdirectories for each assignment, it is not a single cargo monorepo which would use [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). Each homework directory is a separate cargo crate. I didn't have anything
to benefit from easily sharing build artifacts across these projects, because they are distinct assignments.

So there will be one directory per create like:

> -   hw1
> -   hw2
>     ...

There will also be (not necessarily after the first assignment) a separate common library crate hosted on my own github. This is where
I'll centralize helpers (not necessarily cool, just common helper functions that I'd like to re-use) to be re-used across assignments.
