# test all
cargo test --workspace

# run all the examples and print outputs
for example in $(ls examples/)
do
  cargo run --example ${example%%.rs}
done
