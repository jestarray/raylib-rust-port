# raylib-rust-port

This is an attempt to port raylib for my specific usecase.
- Support sdl3 only
- gl3.3 only and es2/3
- won't support audio, etc


# stripping
## step 1:
Use `unifdef` to strip out ifdef chunks:
Do not strip es2 if you want to support es3 because es3 is just a superset of es2.
```
unifdef -UGRAPHICS_API_OPENGL_SOFTWARE -UGRAPHICS_API_OPENGL_11 -UGRAPHICS_API_OPENGL_21 rlgl_n.h > rlgl_nogl_11_21_software.h
```

## step 2:
Since there's no need to support old rendering platforms, it frees us to make the gl33 and es2 the default in that they should just always be on. Prompt AI to do this for you or generate a perl script to strip the following if def blocks
```
#if defined(GRAPHICS_API_OPENGL_33) || defined(GRAPHICS_API_OPENGL_ES2)
#if (defined(GRAPHICS_API_OPENGL_33) || defined(GRAPHICS_API_OPENGL_ES2))
```

## step 3:
Prompt AI or do the porting manually. I recommend prompting AI first cause it actually does a pretty good first pass that can reduce the work load substantially if you visually diff it as well.
If you do decide to manually fix it up, I'd prompt AI again generate a file that compares the C and Rust port so you can easily provide examples in the future when raylib updates
