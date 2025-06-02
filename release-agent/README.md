# Release Agent

# Running MinIO

To start MinIO, run the following commands:

```bash
mkdir -p ~/minio/data

docker run \
    -p 9000:9000 \
    -p 9001:9001 \
    --name minio \
    -v ~/minio/data:/data \
    -e "MINIO_ROOT_USER=ROOTNAME" \
    -e "MINIO_ROOT_PASSWORD=CHANGEME123" \
    quay.io/minio/minio server /data --console-address ":9001"
```

In the MinIO console, accessible at `http://172.17.0.2:9001/`, you need to create a bucket named `sealci-bucket`.
