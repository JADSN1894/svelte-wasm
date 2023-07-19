run:
    @watchexec -rc -e rs -- wasm-pack build  --target web --mode force --debug  --out-dir ../src/lib/encoder ./encoder