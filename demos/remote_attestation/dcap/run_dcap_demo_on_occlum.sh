#!/bin/bash
set -e

BLUE='\033[1;34m'
NC='\033[0m'
INSTANCE_DIR="dcap_demo_instance"

make -j
rm -rf ${INSTANCE_DIR} && occlum new ${INSTANCE_DIR}
cd ${INSTANCE_DIR}
cp ../bin/dcap_demo image/bin
cp ../bin/libdcap.so image/opt/occlum/glibc/lib
occlum build

echo -e "${BLUE}occlum run /bin/dcap_demo${NC}"
occlum run /bin/dcap_demo
