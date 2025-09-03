# Minsk

This is a Rust implementation of the **Minsk** programming language, originally
written in C# by Immo Landwerth (aka terrajobst on Github). I wanted to follow
along with his YouTube series to learn more about developing programming
languages, but I didn't want to just copy his code verbatim. Also, I'm not the
biggest fan of C# myself (it smells like Microsoft).

Below are the links to Immo's Github page as well as the YouTube series along
with which I am following:

- https://github.com/terrajobst
- https://www.youtube.com/playlist?list=PLRAdsfhKI4OWNOSfS7EUu5GRAVmze1t2y

## Building

One of my goals with this project is to keep it completely dependency-free. My
biggest gripe about Rust is that its ecosystem has followed in the footsteps of
NPM, whereby every crate has a million dependencies and everything takes ages
to build. I prefer my code to be entirely home-grown, especially for a project
of relatively simple functionality such as this.

To build and run the project, just do what you would do for any other Cargo
project:

```bash
cargo run
```

The program cannot process actual source code files at the moment, so there are
no arguments to pass. That will be changing very soon though!

## License

Even though I am copying the functionality of a program written by someone
else, all of my code is original and licensed under the MIT license. Of course,
that means you can do what you please as long as I am duly credited. See the
`LICENSE` file for more information.
