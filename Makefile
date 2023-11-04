install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

container-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

refactor: format lint

deploy:
	#deploy goes here
		
all: install lint test format deploy
