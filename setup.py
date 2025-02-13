from setuptools import setup

setup(
    name="ruvo",
    version="0.1.0",
    packages=["ruvo"],
    entry_points={
        "console_scripts": [
            "ruvo=ruvo.cli:main",
        ],
    },
)