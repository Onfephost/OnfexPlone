from setuptools import setup

setup(
    name="onfex",
    version="2.0.0",
    py_modules=["onfex_run"],
    entry_points={
        "console_scripts": [
            "onfex=onfex_run:main"
        ]
    }
)