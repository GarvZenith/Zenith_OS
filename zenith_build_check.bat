@echo off
echo ========================================================
echo          ZENITH OS - BARE-METAL KERNEL BUILD
echo ========================================================
echo Target: x86_64-unknown-none (Bare-Metal Rust)
echo Location: E:\Project\NewOS\zenith_os\kernel
echo.

cd /d E:\Project\NewOS\zenith_os\kernel
cargo build --target x86_64-unknown-none

if %ERRORLEVEL% EQU 0 (
    echo.
    echo [SUCCESS] Zenith OS bare-metal kernel compiled cleanly!
    echo Binary: E:\Project\NewOS\zenith_os\kernel\target\x86_64-unknown-none\debug\zenith_kernel
) else (
    echo.
    echo [ERROR] Kernel compilation failed!
)
pause
