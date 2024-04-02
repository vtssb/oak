all:
	tar xzf ${CLOUD_HYPERVISOR_SRC_TAR}
	cargo build --all --features="tdx" --manifest-path cloud-hypervisor-${CLOUD_HYPERVISOR_COMMIT_HASH}/Cargo.toml
	mkdir -p target
	cp ./cloud-hypervisor-${CLOUD_HYPERVISOR_COMMIT_HASH}/target/x86_64-unknown-linux-gnu/debug/cloud-hypervisor target/

clean:
	rm -rf ./cloud-hypervisor-${CLOUD_HYPERVISOR_COMMIT_HASH} ./target
