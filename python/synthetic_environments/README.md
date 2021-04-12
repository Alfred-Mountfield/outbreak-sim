# Synthetic Environment Generation

TODO Description

## Prerequisites

* Follow setup guide in [parent README](../README.md)

* From [WorldPop](https://www.worldpop.org/project/categories?id=3) download "Constrained Individual Countries 2020 UN
  adjusted 100m resolution" for Great Britain. Place in `.\data` leaving the default filename "
  gbr_ppp_2020_UNadj_constrained.tif"

* From [Dryad](https://datadryad.org/stash/dataset/doi:10.5061/dryad.pc8m3) download the multilayer temporal network of
  public transport in Great Britain. Place within the `.\data` directory and unzip into a directory
  called `uk_aggregate`
  so that the following files are present:
    * `.\data\uk_aggregate\Data_Release_v1.11\edges.csv`

    * `.\data\uk_aggregate\Data_Release_v1.11\nodes.csv`

    * `.\data\uk_aggregate\Data_Release_v1.11\layers.csv`

## Main Usage

* In Jupyter open up the [population_gen.ipynb](population_gen.ipynb) notebook

* Update the boundary, and download a osm.pbf [bbbike](https://extract.bbbike.org/) if it crosses normal geographic
  boundaries, otherwise use Pyrosm to automatically download one from [Geofabrik](http://download.geofabrik.de/) like "
  Greater London"

* TODO WIP

## Additional Notebooks

Section WIP