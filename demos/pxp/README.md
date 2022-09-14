# A Sample `primitives-matmul-cpp` integration

This project demonstrates how to run `primitives-matmul-cpp` application within Occlum.

## Prerequiste
- The PXP functionality supported graphic card should be installed in target environment.
- Docker image of `confidential_computing/release` should be imported in host:
```
docker load -i confidential_computing_release.tar.gz
```

## Launch interactive container
Launch a container with the `confidential_computing/release` image with the following command:
```
docker run -idt --device=/dev/sgx/enclave --device=/dev/sgx/provision --device=/dev/dri:/dev/dri --name pxp --net=host tepin.sc.intel.com:5000/confidential_computing/release:0.7
```

This new `pxp` container will include:
- The `primitives-matmul-cpp` bin and all it's depedency `.so` files.
- The Occlum environment.

## (Optional) Run `primitives-matmul-cpp` on container
```
source /opt/intel/oneapi/setvars.sh
cd /pxp-oneDNN-sample/build/src/
export LD_LIBRARY_PATH=$PWD:$LD_LIBRARY_PATH
cd /pxp-oneDNN-sample/build/examples
./primitives-matmul-cpp gpu heavy
```

## Run `primitives-matmul-cpp` on Occlum
These following commands should be executed inside that `pxp` container:
```
[ -d occlum_workspace ] || occlum new occlum_workspace
cd occlum_workspace && rm -rf image
cp ../Occlum.json ./Occlum.json
LD_LIBRARY_PATH=/pxp-oneDNN-sample/build/src:/opt/intel/oneapi/compiler/2021.4.0/linux/lib:/opt/intel/oneapi/compiler/2021.4.0/linux/compiler/lib/intel64_lin:/opt/intel/oneapi/tbb/2021.4.0/lib/intel64/gcc4.8:/usr/lib/x86_64-linux-gnu:/opt/intel/oneapi/compiler/2021.4.0/linux/lib/x64 copy_bom -f ../pxp.yaml --root image --include-dir /opt/occlum/etc/template
SGX_MODE=HW occlum build -f
OCCLUM_LOG_LEVEL=off occlum run /bin/primitives-matmul-cpp gpu heavy 2>&1 | tee ../log.txt
```
