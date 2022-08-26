exec: test.s
	docker run --rm \
		-i \
		-v $$PWD/test.s:/home/test.s \
		c /bin/bash -c "cc -o app /home/test.s; ./app"

_docker.build:
	docker build . -t c --no-cache



test:
	cargo run "a = 3; b = 5 * 6 - 8; a + b / 2;"

