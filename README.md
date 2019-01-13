# Fog of War

![demo]('./demo.gif')

Game project to explore [Rust](https://www.rust-lang.org/) using game framework [ggez](http://ggez.rs/).

---

## Usage

No releases created but by all means, clone and run locally.

It uses a font that needs to be copied into the target resources. To make it easier, run the build script:

```
git clone https://github.com/berto/fog-of-war
cd ./fog-of-war
bash ./build.sh
cargo run
```

## TODO

### Features

- Boost ability for player to better avoid baddies
- Levels: increasing with difficulty

### Code

- Add test coverage
- Refactor Player struct to abstract the prize and baddies
