@echo off
setlocal enabledelayedexpansion

if not exist "encoded\" mkdir "encoded"
if not exist "decoded\" mkdir "decoded"

cargo run -release

for %%f in ("data\*") do (
	.\target\release\sroda_15.exe "%%f" "encoded\%%~nxf"
	.\target\release\sroda_15.exe "encoded\%%~nxf" "decoded\%%~nxf"
)

echo All files in data proccesed
