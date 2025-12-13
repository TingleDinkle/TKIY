# TKIY Code Editor

> "Strange is the night where black stars rise,  
> And strange moons circle through the skies,  
> But stranger still is Lost Carcosa."

## What Is This?

**TKIY (The King In Yellow) Code Editor** is an immersive, 3D web-based interface for the **Yellow Compiler**—an esoteric programming language where code execution actively degrades program stability and "Sanity."

The experience begins in a dark void where the *King in Yellow* grimoire floats in silence. Upon opening the book, the user is consumed by a **Yellow Mist**, transporting them to a 2D **"Minecraft Writable Book"** interface to script their incantations.

As you execute code, the **Sanity System** degrades. Critical sanity levels trigger visual hallucinations, screen shaking, phantom variable manifestations, and warnings from worlds that shouldn't exist.

Go to https://github.com/TingleDinkle/yellow_compiler for reference on how to use the language

## Motivations & Lore

This project serves as a functional artifact inspired by the Minecraft ARG series surrounding the characters **Avery** and **Derlord**. 

It specifically references the events documented in the video:  
**[Searching For A World That Doesn’t Exist](https://www.youtube.com/watch?v=3V7Rvo4Gvic)**

The editor acts as a bridge to that "non-existent world," incorporating specific easter eggs and warnings found in Derlord's logs:
* **The Crossroads:** A liminal space where the entity waits.
* **The Warning:** *"Whatever you do at the crossroads, don't turn left."*
* **Phantom Data:** Variables named `Avery`, `Derlord`, and `The_Oasis` may spontaneously appear in your memory fragments when sanity is low.

## Installation

This project combines a **Rust** backend (compiled to WebAssembly) with a **React/Three.js** frontend.

### Prerequisites
* [Rust & Cargo](https://www.rust-lang.org/)
* [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
* [Node.js](https://nodejs.org/)

### Setup

1.  **Compile the Brain (Wasm):**
    ```bash
    # Inside the project root
    wasm-pack build --target web
    
    # Move the generated 'pkg' folder to where React expects it
    # (Windows)
    move pkg src\pkg
    # (Mac/Linux)
    mv pkg src/pkg
    ```

2.  **Launch the Interface:**
    ```bash
    npm install
    npm run dev
    ```

## Credits

### Inspiration
* **Wifies:** For the video *Searching For A World That Doesn’t Exist*.
* **Robert W. Chambers:** For the original collection *The King in Yellow*.

### Assets
* **3D Model:** "黃衣之王The King in Yellow_Signalis_Fan_art" ([https://skfb.ly/pBsTF](https://skfb.ly/pBsTF)) by **Yung Hao Chang** is licensed under [Creative Commons Attribution](http://creativecommons.org/licenses/by/4.0/).
