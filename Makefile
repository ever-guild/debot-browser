debot-browser/pkg:
	cargo run --package builder --profile release
demo/node_modules:
	cd demo && npm install
release-pkg: debot-browser/pkg
	cp CHANGELOG.md debot-browser/pkg
	cd debot-browser/pkg && npm publish --access public
gh-local:
	git clone --depth=1 --branch=demo git@github.com:ever-guild/debot-browser.git gh-local
release-demo: debot-browser/pkg demo/node_modules gh-local
	cd demo && npm run build
	git -C gh-local rm -rf .
	cp -r demo/dist/. gh-local
	git -C gh-local add .
	git -C gh-local commit -m "update demo"
	git -C gh-local push origin demo
start: demo/node_modules
	cd demo && npm run dev
clean:
	cargo clean
	rm -rf debot-browser/pkg
	rm -rf demo/node_modules
	rm -rf gh-local
test:
	cargo test
fmt:
	cargo fmt
fmt-check:
	cargo fmt --all -- --check
lint:
	cargo clippy --all-targets
