wasm-pack build ./src/app --target web --no-typescript --dev

cp ./src/app/pkg/app_bg.wasm ./public
cp ./src/app/pkg/app.js ./public

echo "Published .wasm artifacts"
