backend-add-windows-target:
	cd backend && rustup target add x86_64-pc-windows-gnu

backend-release:
	cd backend && cargo build --release

backend-release-windows:
	cd backend && cargo build --target x86_64-pc-windows-gnu --release