rustc --cfg tests --test -o test-day%1.exe day%1.rs && test-day%1.exe && rustc -O day%1.rs && day%1 < day%1.txt
