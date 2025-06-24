@echo off
setlocal enabledelayedexpansion

if not exist "encoded_python\" mkdir "encoded_python"
if not exist "decoded_python\" mkdir "decoded_python"

for %%f in ("data\*") do (
	python .\src_python\main.py "%%f" "encoded_python\%%~nxf"
	python .\src_python\main.py "encoded_python\%%~nxf" "decoded_python\%%~nxf"
)

echo All files in data proccesed
