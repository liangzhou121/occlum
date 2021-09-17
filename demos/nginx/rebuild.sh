#!/bin/bash
cd instance
copy_bom -f ../nginx.yaml --root image --include-dir /opt/occlum/etc/template
occlum build
#OCCLUM_LOG_LEVEL=trace occlum run /usr/sbin/nginx -c /tmp/nginx-test.conf
occlum run /usr/sbin/nginx -c /tmp/nginx-test.conf
