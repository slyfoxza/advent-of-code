rustc --cfg tests --test -o test-day%1.exe day%1.rs && test-day%1.exe && rustc -O day%1.rs
@IF %ERRORLEVEL% NEQ 0 EXIT /b %ERRORLEVEL%
IF EXIST day%1.txt (
    day%1 < day%1.txt
) ELSE (
    day%1
)
