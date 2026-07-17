from setuptools import setup

setup(
    name="onfex",
    version="1.6.2",
    py_modules=["onfex_run"],
    entry_points={
        "console_scripts": [
            "onfex=onfex_run:main"
        ]
    }
)