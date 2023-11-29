# sunhaven_toolkit
Companion app to the video game Sun Haven.
This was created with Rust and Rocket with Tera templates.
https://www.rust-lang.org/
https://rocket.rs/
https://keats.github.io/tera/

Before you launch the app you need to set what port you want it to run on in the Rocket.toml file.
By default it runs on port 8123.

The easiest way to then launch the app is with Cargo.
https://doc.rust-lang.org/cargo/getting-started/installation.html

Once you have cargo installed you should be able to run the app by browsing to the sunhaven_toolkit folder and executing the command 'cargo run'

If you want you can run cargo build to obtain a more optimized executable.
https://doc.rust-lang.org/cargo/commands/cargo-build.html

USAGE:
After the app is running all you have to do is browse to:
0.0.0.0:{{port}}
127.0.0.1:{{port}
or whatever your current rfc1918 address is :{{port}}

The main page will let you select to create a new tracker file or load an existing tracker file.
These files are saved as .csv files in the /csv folder. You can manually edit them if you wish, but there are no headers so that may be a challenge, and if you accidentally put erronious data in the csv file it may not load again.

If you chose to create a new file don't use any spaces in the file name. (I'll fix this in the future)

Once you create or load a file you should then see the main community / altar tracker page.

If multiple people are using the tracker at the same time you need to make sure you press the update button before you save or you'll overwrite what others have previously saved. (Fixable in the future)

Ideas for future content:
Notes page for manual notes
Snaccoon tracker
Romance items / tracker
Map that shows where certain items can be found (Combat)
Hover over item tips on where to find in the main community / altar tracker
Improved UI, Look, and Feel.
Pre-built executables for different OSs

SECURITY NOTE!
If you choose to host this on a server with people you don't trust or on the public internet you may be making yourself vulnerable to a number of different attacks. It has not been tested or pentested for security at this time. The intended use case is on a local area network with people you know and trust not to do or attempt anything malicious.
