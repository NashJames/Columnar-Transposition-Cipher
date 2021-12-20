# Columnar Transposition Cipher

This implementation was originally created for [TheAlgorithms](https://github.com/TheAlgorithms/Rust), with a focus on idiomatic code and genericity. It was expanded upon to add a command-line interface and allow multiple keywords.

## Usage:

Encryption:

`cargo run -- "Write your message here." "Cipher Keyword"`

Decryption:

`cargo run -- -d "HEO MRW TIE USE AER RYS EG" "Cipher Keyword"`

## Explanation:

The Transposition Cipher is a method of encryption by which a message is shifted
according to a regular system, so that the ciphertext is a rearrangement of the
original message. The most commonly referred to Transposition Cipher is the
Columnar method, which is demonstrated [here](https://en.wikipedia.org/wiki/Transposition_cipher#Columnar_transposition).

The Double Columnar Transposition Cipher has a fairly comprehensive cultural and historical significance surrounding both World Wars. It holds partial credit for the advancements surrounding modern day encryption.
