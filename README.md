## Install


On linux:
```
sudo apt install libx11-dev libxext-dev libxrandr-dev libxcursor-dev libxfixes-dev libxi-dev libxss-dev libxtst-dev libxkbcommon-dev
```
and if you'll be targeting wasm:
```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli

```


On other platforms figure it out yourself here https://github.com/libsdl-org/SDL/blob/main/INSTALL.md

## Build

For native:

```
cargo build
```

For web via wasm:

```
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/mikumikutower.wasm --out-dir web/pkg --target web
```

If you want to build both then just run `make` and it will do the trick and copy all the files you need into the right place.
Then just serve the web folder out via a python or php server as you please.

## Side notes about the game

If you have interest in seeing how this game was made from start to finish, then you can read the full dev blog here https://peetseater.space/blag/2026-05-25-miku-miku-tower-defense

If you want to play your own music while in the game, then title the wav files 1.wav, 2.wav, etc up to 999.wav into the assets/audio/cc-vocaloid folder.
Supported file types are whatever sdl3 supports, so see the console output when the game loads to check that out.

There is no audio controls in the game, so turn it down and then up as needed. 

If you'd like to have your own sprites in the game then replace the various assets in the made-by-me folder and keep the same sizes.

## Credits

The lovely vocaloid related sprites are sourced from the Miku n Pop game, see the README in the asset folder for more details.
The wav file for background music has a README file next to it that tells you how to play your own wavs in the game.
The font used in the game is BoldPixels by YukiPixels and you can find the links to their work in the license file in the assets folder.
The sound effects were created with https://sfxr.me
Other wav files used in non-level scenes were found on the internet over the years and I don't have the sources to all their originals.
Hatsune Miku the character design herself is creative commons as noted here https://piapro.net/intl/en_for_creators.html though most people know her for her synthesizer which is owned by Crypton Future Media and Sony.

## Enhancments / exercise for the bored

 - Enhance with proper Result types in places for better error handling
 - Add proper logging library
 - Make game options take in properties from cli
 - Make game ticks per second configurable or centralized
 - increase damage of enemies over time
 - Deal with fonts better