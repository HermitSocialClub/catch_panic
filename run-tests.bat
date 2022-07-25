@echo off
setlocal

::: On windows, `jvm.dll` must be in the PATH in order for doctests to run
for /F "tokens=* USEBACKQ" %%F in (`where java.exe`) do (
    set JAVA_PATH=%%F
)
set JAVA_SERVER_DIR=%JAVA_PATH:~0,-8%server
set PATH=%PATH%;%JAVA_SERVER_DIR%

cargo test
