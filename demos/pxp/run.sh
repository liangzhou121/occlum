#!/bin/bash
cp /pxp-oneDNN-sample/build/examples/primitives-matmul-cpp ./
occlum new occlum_workspace
cd occlum_workspace && rm -rf image
cp ../Occlum.json ./Occlum.json
LD_LIBRARY_PATH=/pxp-oneDNN-sample/build/src:/opt/intel/oneapi/compiler/2021.4.0/linux/lib:/opt/intel/oneapi/compiler/2021.4.0/linux/compiler/lib/intel64_lin:/opt/intel/oneapi/tbb/2021.4.0/lib/intel64/gcc4.8:/usr/lib/x86_64-linux-gnu:/opt/intel/oneapi/compiler/2021.4.0/linux/lib/x64 copy_bom -f ../pxp.yaml --root image --include-dir /opt/occlum/etc/template
SGX_MODE=HW occlum build
OCCLUM_LOG_LEVEL=off occlum run /bin/primitives-matmul-cpp gpu heavy 2>&1 | tee ../log.txt
