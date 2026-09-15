# raylib-rust-port

This is an attempt to port raylib for my specific usecase.
- Support sdl3 only
- gl3.3 only and es2/3
- won't support audio, etc

I forgot but I ran some C if-def program to strip out all the un-needed ifdefs and then started the port
run `unifdef` to strip gles1 support