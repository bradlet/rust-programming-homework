# HW3 - Chomp CLI Game & AI

Bradley Thompson and Bart Massey 2023

This code plays the game of
[Chomp](https://en.wikipedia.org/wiki/Chomp). It comprises
a library containing a perfect AI player, together with a
command-line program to play against it.

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.

# Writeup

## What I Did
I started out with a roadblock because I misinterpreted part of the
instructions / expectations. I thought that we had to dynamically
size the arrays that the Chomp board used. I tried a lot of ways to
figure out how to do it without making the signature some combination
of slices, or vectors. Pretty sure it is not possible.

Once I figured out that sizing the arrays was actually very simple,
because it was in fact using compile-time constants for sizing, I
started out by implementing Display for my Chomp struct. I had
known that's how to make a struct formattable, just hadn't had a
chance to do it before and it seemed fun to try. I realized *after*
that the display piece was already provided. Oh well!

I finished enough of the CLI, as well as the board creation and
`make_move()` implementation, so that it was possible to at least
sort-of play the game. No win checks, no AI. After that, I worked
on the `winning_move()` negamax algorithm. My initial implementation
was not entirely working, but I didn't realize that until I completed
the rest of the program that it wasn't quite right. Finished up by
fixing it, and then doing some formatting things, and giving the
AI a sassy personality.

## How It Went

Well to start, I just have to say gosh DANG do I like working in Rust.
I've been writing almost entirely Kotlin (some Java, JS and Python too)
code at work across 3 companies over the last 3+ years, and still to
this day need to sometimes write a quick little test to remind my self
how values are passed around, what is a reference, why some value isn't
changing, etc. For the most part it can kinda hide all of that away,
but when it can't, it really sucks. I really love knowing explicitly,
_exactly_ what kind of data I am working with.

Other than that, the assignment went well! When I first completed it,
I started playing around with it a bit. I thought I was just really
good at Chomp until I realized that the AI wasn't detecting any
winning moves from any position within row or col 0 to 1.

## How I Tested My Work
I setup a really light initial unit test harness with the plans of
my testing mostly being manual since it's a CLI game. That let me
make sure, at least foundationally, that my code was working alright.

Once I finished up the initial implementation, as mentioned previously,
I started playing around with the game. I played like 9 times, winning
every time, before I thought something was up. It seemed like such
a simple game that I was like "Huh I must be really good at this." It
was convincing because at times the AI still made reasonably different
moves from the "computer move". But then I realized, after fixing 
some of the output structure and formatting, and the amount of times 
that I printed the board state, that win states were being missed.

So, I added an additional unit test describing that case. That test
helped guide me to a solution which I then manually tested for other
similar cases. Once the unit tests passed, I felt assured through many
games that the AI was working exactly right.