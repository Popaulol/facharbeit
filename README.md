This is just a file dump of a school Project.

# Running the Code
The code here has only ever been tested on my amd64 Arch Linux System and i don't intend to ever support anything else, if you still want to try running this on you own machine and are having issues, I am still open to try helping you to get this running this, so feel free to create an issue but i don't gurantee anything.

To actually Build the code you first need to install [Rustc and Cargo](https://www.rust-lang.org/) and then you need to run
```bash
cargo +nightly build --release
```
in the root directory of the Project, this will result in the final executable being created in `target/release/facharbeit`.
This can now be used with any chess interface or Turnament tool that supports the UCI Protocol for chess Engines such as [CuteChess](https://cutechess.com/).
