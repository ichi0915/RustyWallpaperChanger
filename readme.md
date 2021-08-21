# Rusty wallpaper changer
This is a simple Rust program that pics a random image from the img dir and changes the desktop wallpaper/background every 60 seconds, it uses a modified version of the library https://github.com/reujab/wallpaper.rs.

## Considerations
* It leaves no window
* It runs indefinitely
* Currently it only works on windows

### Extra
* It leaves a file with the location of the previews wallpaper
* You can take the exe and a directory call img with pictures and run it

## Compile
To compile use
```bash
cargo build
```

To compile and run use
```bash
cargo run
```
