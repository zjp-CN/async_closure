# test all
cargo test --workspace

# run all the examples and print outputs
for example in $(ls examples/)
do
  case $example in
    *".rs"*)
      cargo run --example ${example%%.rs}
      ;;

    *)
      cd examples/$example

      cargo r

      for example in $(ls examples/)
      do
        cargo run --example ${example%%.rs}
      done

      cd ../../
  esac
done

echo done!
