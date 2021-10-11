#!/bin/sh

svd patch icetwang-soc.yaml
xmllint --format svdtemplate.svd.patched > ../../../firmware/icetwang-pac/icetwang-soc.svd
