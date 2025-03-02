cargo build --release -p api

wasm-pack build ./src/app --target web --no-typescript --release

cp ./src/app/pkg/app_bg.wasm ./public
cp ./src/app/pkg/app.js ./public

echo "Published .wasm"
