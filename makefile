all: public/css/index.css public/index.html public/load.js public/wasm/ragroc_bg.wasm 


public/wasm public/css:
	@mkdir -p public/wasm
	@mkdir -p public/css

public/index.html public/load.js: content/index.html content/load.js
	rsync -r content/* public/

public/wasm/ragroc_bg.wasm: wasm/src/* wasm/Cargo.toml public/wasm
	wasm-pack build wasm --target web
	rsync wasm/pkg/ragroc* wasm/pkg/package.json public/wasm/

public/css/index.css: scss/index.scss public/css
	sass scss/index.scss scss/index.css
	cleancss -o scss/index.mini.css scss/index.css
	rsync scss/index.mini.css scss/index.css.map public/css/

