# Synthetic Population Generation

## Setup:

* Ensure you've installed flatc and ran `.\codegen.sh` as described in the parent directory README

* From [WorldPop](https://www.worldpop.org/project/categories?id=3) download "Constrained Individual Countries 2020 UN 
adjusted 100m resolution" for Great Britain. Place in `.\data` leaving the default filename "gbr_ppp_2020_UNadj_constrained.tif"
  
*  From [Dryad](https://datadryad.org/stash/dataset/doi:10.5061/dryad.pc8m3) download the multilayer temporal network of 
   public transport in Great Britain. Place within the `.\data` directory and unzip into a directory called `uk_aggregate` 
   so that the following files are present:
    * `.\data\uk_aggregate\Data_Release_v1.11\edges.csv`
   
    * `.\data\uk_aggregate\Data_Release_v1.11\nodes.csv`
   
    * `.\data\uk_aggregate\Data_Release_v1.11\layers.csv`
    
* Using a version of python greater than 3.8, run:
    * `python -m venv venv`
    
    * `. .\venv\bin\activate`

    * `python -m pip install -r requirements.txt`
    

## Usage

* Activate the virtual environment with `. .\venv\bin\activate`
  
* Run: `jupyter-lab`

* In Jupyter open up the `synthetic_population\population_gen.ipynb` notebook

* Update the boundary, and download a osm.pbf [bbbike](https://extract.bbbike.org/) if it crosses normal geographic boundaries, 
  otherwise use pyrosm to automatically download one like "Greater London"