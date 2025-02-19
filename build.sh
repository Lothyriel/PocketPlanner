cargo build --release

wasm-pack build ./src/app --target web --release

cp ./src/app/pkg/app_bg.wasm ./public
cp ./src/app/pkg/app.js ./public
