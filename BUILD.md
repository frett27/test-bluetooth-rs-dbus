
Build the sample
================


  cargo run

cross compile for armv7

  cd docker
  ./create.sh
  cd ..

  ./build.sh


Generate proxies :


### installing elements
    cargo install dbus-codegen

## generating proxies

dbus-codegen-rust -s -d org.bluez -p "/" > src/bluez.rs