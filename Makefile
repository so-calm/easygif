all: x64-msvc-node x64-linux-node

x64-msvc-node:
	cargo b -r -Fnode --target x86_64-pc-windows-msvc --target-dir target_node
	node scripts/build target_node/x86_64-pc-windows-msvc/release/easygif.dll artifacts/x64-msvc-easygif.node

x64-linux-node:
	cargo b -r -Fnode --target x86_64-unknown-linux-gnu --target-dir target_node
	node scripts/build target_node/x86_64-unknown-linux-gnu/release/libeasygif.so artifacts/x64-linux-easygif.node
