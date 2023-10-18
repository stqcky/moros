out=/home/stacky/share/moros

cross build -r --target x86_64-pc-windows-gnu
mkdir -p $out
cp ./target/x86_64-pc-windows-gnu/release/injector.exe $out
cp ./target/x86_64-pc-windows-gnu/release/moros.dll $out
strip $out/moros.dll
