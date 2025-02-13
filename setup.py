from setuptools import setup

setup(
    name="ruvo",
    version="0.1.0",
    packages=["ruvo"],
    entry_points={
        "console_scripts": [
            "venv-manager=ruvo.cli:main",
        ],
    },
)