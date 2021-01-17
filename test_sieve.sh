RUST_PORT=3001
JS_PORT=3000
NUMBER_TO_TEST_SIEVE=10000000
echo "Benchmarking JS"
ab -n 64 -c 32 http://127.0.0.1:$JS_PORT/sieve/$NUMBER_TO_TEST_SIEVE > js_bench
echo "Benchmarking RUST"
ab -n 64 -c 32 http://127.0.0.1:$RUST_PORT/sieve/$NUMBER_TO_TEST_SIEVE > rust_bench
echo "Diff bench output"
diff js_bench rust_bench
rm js_bench rust_bench