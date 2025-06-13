mkdir rust-git
cp -r src rust-git/src
cp Cargo.toml rust-git/Cargo.toml
cp Cargo.lock rust-git/Cargo.lock
cp README.md rust-git/README.md

mkdir -p rust-git/target/release
mkdir -p rust-git/target/debug
cp target/release/rust-git rust-git/target/release/rust-git
cp target/release/rust-git rust-git/target/debug/rust-git

mkdir ci_out
mv rust-git ci_out/rust-git

cd ci_out
zip -r rust-git.zip rust-git