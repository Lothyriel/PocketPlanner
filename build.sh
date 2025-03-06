wasm-pack build ./src/app --target web --no-typescript

cp ./src/app/pkg/app_bg.wasm ./public
cp ./src/app/pkg/app.js ./public

echo "Published .wasm artifacts"
