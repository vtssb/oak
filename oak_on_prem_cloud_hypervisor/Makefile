all:
	tar xf ${CLOUD_HYPERVISOR_SRC_TAR}
	cargo build --features="tdx" --manifest-path cloud-hypervisor-${CLOUD_HYPERVISOR_VER}/Cargo.toml
	mkdir -p target
	cp ./cloud-hypervisor-${CLOUD_HYPERVISOR_VER}/target/x86_64-unknown-linux-gnu/debug/cloud-hypervisor target/

clean:
	rm -rf ./cloud-hypervisor-${CLOUD_HYPERVISOR_VER} ./target
