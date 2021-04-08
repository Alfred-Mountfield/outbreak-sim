# Python Utilities and Components

## Requirements:

* [Python 3.x > 3.8](https://www.python.org/downloads/)
* [FlatBuffer Compiler accessible on the path](https://google.github.io/flatbuffers/flatbuffers_guide_building.html)

## Setup:

### For Unix-based systems

* Ensure you've installed flatc and ran `.\codegen.sh` as described in the parent directory README
* Using a version of Python greater than 3.8, run:
    * `python -m venv venv`
    * `. .\venv\bin\activate`
    * `python -m pip install -r requirements.txt`

### For Windows

* It's recommended to install Windows Subsystem for Linux (WSL) and follow the instructions above, it might be possible
  to run some of Python modules on Windows however the spatial dependencies like GDAL can be difficult to get working
  and are currently not tested or supported for this project.

## Usage

* Activate the virtual environment with `. .\venv\bin\activate`

* Run: `jupyter-lab`

* Follow instructions in either:
    * [Synthetic Environment Generation README](synthetic_environments/README.md)
    * [Report Analysis README](report_analysis/README.md)