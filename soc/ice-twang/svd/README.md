The SVD file generator for icetwang-soc.

# How to generate the SVD file
The SVD file is generated using svdtools.

To install svdtools run:
```
pip3 innstall --upgrade --user svdtools
```

Or if you are on Arch you can install the svdtools AUR.

You can then run the `gensvd.sh` script. This will generate the svd file and
drop it into `../../../firmware/icetwang-pac`.

# How to add registers

Just edit the icetwang-soc.yaml file and add more registers.

For syntax refer to the [svdtools
documentation](https://pypi.org/project/svdtools/#device-and-peripheral-yaml-format).
