.PHONY: setup run clean

setup:
	@echo "Creating Python3 Virtual Environment..."
	python3 -m venv venv
	@echo "Activating Virtual Environment..."
	. venv/bin/activate
	@echo "Installing Dependencies..."
	pip install -r requirements.txt

run:
	@echo "Starting Jupyter Notebook..."
	. venv/bin/activate; \
	jupyter notebook

clean:
	@echo "Cleaning up..."
	rm -rf venv
	find . -type f -name '*.pyc' -delete
	find . -type d -name '__pycache__' -exec rm -rf {} +
