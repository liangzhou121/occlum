#!/bin/bash
[ -d instance ] || occlum new instance
cd instance
copy_bom -f ../nginx.yaml --root image --include-dir /opt/occlum/etc/template
occlum build
#OCCLUM_LOG_LEVEL=trace occlum start
OCCLUM_LOG_LEVEL=trace occlum run /bin/nginx -c /tmp/nginx-test.conf 2>&1 | tee ../log.txt
#occlum run /bin/nginx -c /tmp/nginx-test.conf
