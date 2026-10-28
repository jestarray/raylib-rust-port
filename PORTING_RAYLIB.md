Guide to porting raylib with AI and stripping features out that you don't need or making a default baseline(GL 3.3)

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

# todo port:
- [ ] rmodels
- [ ] have AI port a lot of 2d examples from raylib since our functions are nearly 1 to 1

# major differences between raylib and raylib-rs safe bindings:
- rtext functions will use rust &str, so they will diverge from the C a lot more
- Image crate replaces most of the image loading and Image drawing code
- Using swash instead of stb truetype
- Using glam vectors/matrix instead of Raylib ported math types

# slight changes:
- [ ] InitPlatform() checks for `SDL_GetError()` and prints them
- REMOVED from rcore(because they pull in libc and external deps): SetRandomSeed, GetRandomValue, LoadRandomSequence, UnloadRandomSequence