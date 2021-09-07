from setuptools import setup
from setuptools_rust import RustExtension


setup(
    name="concord",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: Apache-2.0 License",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    packages=["concord"],
    rust_extensions=[
        RustExtension(
            "concord.concord",
            debug=False,
        ),
    ],
    include_package_data=True,
    zip_safe=False,
)