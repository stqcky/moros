out=/home/stacky/share/moros

cross build --target x86_64-pc-windows-gnu
mkdir -p share
cp ./target/x86_64-pc-windows-gnu/debug/injector.exe $out
cp ./target/x86_64-pc-windows-gnu/debug/moros.dll $out
