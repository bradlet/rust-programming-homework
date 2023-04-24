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

There is also a separate common library crate hosted on my own github. Check that out here: [Bradley's Random Rust Helpers](https://github.com/bradlet/bradleys-random-rust-helpers)

## Course Project Proposal

Project Name: Postgres Flatten

Student: Bradley Thompson

MCECS Gitlab Repo: https://gitlab.cecs.pdx.edu/bradlet2/postgres-flatten

### Project Topic Overview
In my work, on multiple occasions I've ran into a situation where it is desirable to flatten some nested data structure.
Moreoften than not, this is because I want to make it easy to input data into a relational (always PostgresSql, to be 
honest) database. Rust has [pretty solid postgres support](https://crates.io/crates/postgres). Alongside two client 
libraries, one synchronous and one asynchronous, there is a `postgres-types` extension library. This library provides
derivable traits that help convert postgres types to Rust types. While I am still new to the API, I'm pretty sure
it enables the creation of rust structs, which can exactly match the schema for a given table in the database,
as well as the automatically generated Postgres type which defines a row in the given table. This allows for cleaner,
automatic deserialization from the generic `Row` container type, to a strongly-typed struct.

While Postgres supports object columns, among other issues these columns can't be indexed. As a result, it's nice to be
able to flatten data so that you can interact with it more efficiently. But this doesn't eliminate the desire to have
nested data structures within your business logic.

### Project Vision
The goal of this project is to provide, essentially, an additional extension crate to `postgres` which would allow for
derivate `ToSqlFlattened` and `FromSqlFlattened` traits. This would allow clients of the library to create struct types,
which in turn have fields that are also of some other well known struct type, which can be easily converted to and from
the Row result returned by the `postgres` client.

This is not an ORM. These types are essentially just serialization / deserialization helpers. The goal is to make it easy
to strongly define the result of some standard query, as well as define the structure for data that you want to input into
the db.

There's a chance that the original use-case I defined in the topic overview section isn't supported out of the box
(essentially, providing `into` and `try_into` implementations for the postgres `Row` to convert into the struct type easily).
I highly doubt it, and probably just haven't figured it out yet in my initial exploration; however, if not, I plan to make
that part of the mvp for the project. The goal is to have the option to easily convert a `Row` to whatever complex type we
define in our Rust code, whether that is a flat data structure, or a heavily nested data structure.

### Discussion
I plan to implement this library crate, as well as a light web app to exemplify the library. I want to do the driver 
application because, well first of all it's mentioned in the project proposal handout, but also because I'm really interested
in checking out a newer web framework called "Axum".

I have this pipe dream of converting some core services at my work from Kotlin to Rust. I really love Kotlin as a language,
but some ambiguity that it allows has made bugs hard to track down, and we really need to squeeze as much performance out of 
our code as possible because we provide an algorithmic trading engine. I think Rust would be a great fit, so I've been 
exploring the Rust ecosystem trying to figure out if all the pieces needed are present. This project is attainable in scope,
and while it doesn't provide a REQUIRED piece of functionality -- one that would be a blocker to convert services over to 
Rust if not available -- it would provide a good deal of convenience in interacting with the persistence layer.

I almost certainly won't have time to go into stretch goals, but a large part of the above "conversion" process would
consist of re-implementing GRPC services in Rust. I've also been checking out Rust's `grpc` and `protobuf` crates.
It seems like there is already decent support for generating rust message types from a protobuf schema. I'd like to look
into adding an additional extension library that would allow clients of the library to make their generated protobuf
messages ALSO have the `ToSqlFlattened` and `FromSqlFlattened` traits. This might not really need it's own library,
though, still undetermined as there might be some way to essentially do this as part of `protoc` without the need for
a separate library.