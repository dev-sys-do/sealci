docker compose -f ../controller/docker-compose.yml up -d
docker compose -f ../release-agent/docker-compose.yml up -d 
ip tuntap add mode tap tap0
ip link set tap0 up