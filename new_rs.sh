mkdir "$1"
cd "$1"
touch main.rs
cargo init . --name solution
code .
