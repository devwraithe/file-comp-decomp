## File Comp Decomp Testing

### 1. Build the project:
```rust
cargo build
```
### 2. Create a test input file:
```rust
echo "This is a test file for compression and decompression." > test_input.txt
```
### 3. Run the compression:
```rust
cargo run -- --compress --input test_input.txt --output test_output.gz
```
### 4. Run the decompression:
```rust
cargo run -- --decompress --input test_output.gz --output test_decompressed.txt
```
### 5. Verify the contents:
```rust
diff test_input.txt test_decompressed.txt
```
