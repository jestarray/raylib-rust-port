# raylib-rust-port

This is an attempt to port raylib for my specific usecase. Use it at your peril.
Goals:
- [ ] Support sdl3 only
- [ ] gl3.3 only and es2/3(es3 is a superset of es2 so you can't support just es3)
- [x] Android support with sdl3 layer only
    - [ ] use rust build system to copy the sdl3 android wrapper stuff instead of pythong script
- [ ] won't support audio, models(for now) etc
- [ ] strip most of raylib Image and replace it with rust image crate
- [ ] use glam vectors and methods and phase out rust vector types
- [ ] review all `libc::` uses and place them with rust versions
- [ ] have AI port a lot of 2d examples from raylib since our functions are nearly 1 to 1


# stripping
## step 1:
Use `unifdef` to strip out ifdef chunks:
```
unifdef -UGRAPHICS_API_OPENGL_SOFTWARE -UGRAPHICS_API_OPENGL_11 -UGRAPHICS_API_OPENGL_21 rlgl_n.h > rlgl_nogl_11_21_software.h
```

## step 2:
Since there's no need to support old rendering platforms, it frees us to make the gl33 and es2 the default in that they should just always be on. Prompt AI to do this for you or generate a perl script to strip the following if def blocks
```
#if defined(GRAPHICS_API_OPENGL_33) || defined(GRAPHICS_API_OPENGL_ES2)
#if (defined(GRAPHICS_API_OPENGL_33) || defined(GRAPHICS_API_OPENGL_ES2))
```

# step 2.5:
- stripping other things to reduce code confusion from the AI:
```
Some C files wrap most of the entire file under an #if. You should remove them first, e.g:
#if SUPPORT_MODULE_RTEXTURES
// entire implementation
end

// the default, do not prefer triangles
unifdef -DSUPPORT_MODULE_RSHAPES -DSUPPORT_QUADS_DRAW_MODE rshapes.c > rshapes.c

// dont care about generating (gradient, spot, perlin-noise, cellular)
unifdef -USUPPORT_IMAGE_GENERATION rtextures.c > rtextures.c

// use only sdl3. NOTE! You should also look into getting AI to rename SDL2 functions to SDL3 because a lot are used
unifdef -DUSING_VERSION_SDL3 -DUSING_SDL3_PACKAGE -UGRAPHICS_API_OPENGL_SOFTWARE -DUSING_SDL3_PROJECT -UUSING_SDL2_PROJECT rcore_desktop_sdl.c > rcore_desktop_sdl3.c

// -700 lines
unifdef -DSUPPORT_MODULE_RTEXTURES \
           -DSUPPORT_FILEFORMAT_BMP \
           -DSUPPORT_FILEFORMAT_PNG \
           -USUPPORT_FILEFORMAT_GIF \
           -USUPPORT_FILEFORMAT_QOI \
           -USUPPORT_FILEFORMAT_PEP \
           -USUPPORT_FILEFORMAT_DDS \
           -USUPPORT_FILEFORMAT_TGA \
           -USUPPORT_FILEFORMAT_JPG \
           -USUPPORT_FILEFORMAT_PSD \
           -USUPPORT_FILEFORMAT_HDR \
           -USUPPORT_FILEFORMAT_PIC \
           -USUPPORT_FILEFORMAT_PNM \
           -USUPPORT_FILEFORMAT_PKM \
           -USUPPORT_FILEFORMAT_KTX \
           -USUPPORT_FILEFORMAT_PVR \
           -USUPPORT_FILEFORMAT_ASTC \
           -USUPPORT_IMAGE_EXPORT \
           -USUPPORT_IMAGE_GENERATION \
           rtextures.c > rtextures2.c

// remove fnt support: -446 lines
unifdef -DSUPPORT_MODULE_RTEXT -DSUPPORT_FILEFORMAT_TTF -USUPPORT_FILEFORMAT_FNT -USUPPORT_FILEFORMAT_BDF rtext.c > rtext2.c
```

## step 3:
Prompt AI or do the porting manually. I recommend prompting AI first cause it actually does a pretty good first pass that can reduce the work load substantially if you visually diff it as well.
If you do decide to manually fix it up, I'd prompt AI again generate a file that compares the C and Rust port so you can easily provide examples in the future when raylib updates

# running examples:
```
cargo run --example texture_demo
```

For an ARM64 Android phone with OpenGL ES 3.0, run `./android/build.sh`.
See [Android build and device instructions](android/README.md) for prerequisites,
APK installation, and the shared SDL entry point.


# todo:
- [ ] port rcore and the rest
- [ ] it would be easier to just look at the raylib cheatsheet and write down a list of functions you don't want to port, have AI to write a perl script or something to remove those functions from the source, and then feed it into the AI
- [ ] use rust version of glad, stb_image, stb_truetype, and others, and remove the need to link to C libs other than SDL3


# finished:
- [x] rlgl
- [x] rshapes
- [x] rcore
- [x] rcore_desktop_sdl
- [ ] rtextures
- [ ] rtext
- [ ] rmodels


# slight changes:
- [ ] InitPlatform() checks for `SDL_GetError()` and prints them
- REMOVED from rcore(because they pull in libc and external deps): SetRandomSeed, GetRandomValue, LoadRandomSequence, UnloadRandomSequence