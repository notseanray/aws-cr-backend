SU=doas

startdocker:
	$(SU) rc-service docker start

dockerrun:
	docker run --rm \
    	-u $(id -u):$(id -g) \
		-e BIN=bootstrap \
		-v ${PWD}:/CR-api/src \
		-v ${CARGO_HOME}/registry:/cargo/registry \
		-v ${CARGO_HOME}/git:/cargo/git \
		softprops/lambda-rust

dockerrunlocal:
	docker run --rm \
    	-u $(id -u):$(id -g) \
		-e BIN=bootstrap \
		-e PACKAGE=false \
		-v ${PWD} \
		-v ${CARGO_HOME}/registry:/cargo/registry \
		-v ${CARGO_HOME}/git:/cargo/git \
		softprops/lambda-rust

preparepackage:
	rustup target add x86_64-unknown-linux-musl

package:
	cargo build --release --target x86_64-unknown-linux-musl
	zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
