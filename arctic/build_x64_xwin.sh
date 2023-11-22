# /bin/bash
# cargo +nightly build -Z build-std=std --target x86_64-pc-windows-gnu --release <-- No XWin
RUSTFLAGS="-Zthreads=20" cargo xwin build --release --target x86_64-pc-windows-msvc $@
