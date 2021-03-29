update:
	cargo update
	python ./helpers/helpers.py
docker:
	bash ./helpers/docker-build.sh