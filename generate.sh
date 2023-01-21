# use svd2rust 0.28.0 or later
svd2rust -i bl808.svd --target riscv --const_generic --keep_list
rm -rf src
form -i lib.rs -o src/ 
rm lib.rs
cargo fmt
