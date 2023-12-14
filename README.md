
# Pogey

Pogey is a command-line tool for looking up Pokemon data. This project is built using [Rustemon](https://github.com/mlemesle/rustemon) and the [PokeAPI](https://pokeapi.co/).

At this moment it supports evolution data lookup by Pokemon name, but I plan on adding more features as I go. Some of these include:
- Location data
- Base Stats
- Moveset
- Interface with save file from emulator
- Open bulbapedia/smogon link by name

## Installation and Use

To install, just copy the source code into a new folder on your pc. 

In a new terminal window, navigate to the folder and type `cargo run {pokemon_name}` to retrieve data for a Pokemon.