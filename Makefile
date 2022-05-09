SU=doas

# this is more specific to my system (I use gentoo btw)
startdocker:
	$(SU) rc-service docker start

preparepackage:
	rustup target add x86_64-unknown-linux-musl

zip:
	#cargo build --release --target x86_64-unknown-linux-musl
	-mkdir package
	zip -j package/rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap

crosscompile:
	docker run \
		-v cargo-cache:$(CARGO_HOME)/registry \
		-v "$(PWD):/volume" \
		--rm -it clux/muslrust cargo build --release

testlambda:
	unzip -o \
		package/rust.zip \
		-d /tmp/lambda && \
	  docker run \
		-i -e DOCKER_LAMBDA_USE_STDIN=1 \
		--env-file .env \
		--rm \
		-v /tmp/lambda:/var/task \
		lambci/lambda:provided
all:
	crosscompile
	zip	
	testlambda
