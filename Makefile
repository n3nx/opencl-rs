update:
	cargo update
	python ./helpers/helpers.py
docker:
	bash ./helpers/docker-build.sh
fmt:
	rustup run nightly cargo fmt
test:
	docker run -v $$PWD:'/mnt' -w '/mnt' -it diabloxenon/opencl:latest cargo test
Cargo.toml:
	cargo build