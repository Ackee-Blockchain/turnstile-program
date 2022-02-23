# Build Solana program
```
cd program
```
```
cargo build-bpf
```
# Test Solana program with Rust
```
cd test
```
Run tests
```
cargo run
```
# Test Solana program with TypeScript Client
```
cd client
```
Install npm packages
```
npm install
```
Start solana test validator
```
solana-test-validator -r
```
Deploy solana program (must be already builded)
```
solana program deploy ../program/target/deploy/turnstile.so --program-id keys/program.json
```
Airdrop SOLs to initializer 
```
solana airdrop 10 keys/initializer.json
```
Run `init` instruction
```
npm run init
```
Run `coin` instruction
```
npm run coin
```
Run `push` instruction
```
npm run push
```
