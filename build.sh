out=/home/stacky/share/moros

if [[ $1 = --release ]]; then
    cross build --release --target x86_64-pc-windows-gnu
    cp ./target/x86_64-pc-windows-gnu/release/injector.exe $out
    cp ./target/x86_64-pc-windows-gnu/release/moros.dll $out
else
    cross build --target x86_64-pc-windows-gnu
    cp ./target/x86_64-pc-windows-gnu/debug/injector.exe $out
    cp ./target/x86_64-pc-windows-gnu/debug/moros.dll $out
fi

