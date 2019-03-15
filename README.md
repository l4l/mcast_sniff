# Simpe Multicast Sniffer

Usage: `MCAST_IP=239.3.0.1 MCAST_PORT=1234 IFACE_IP=0.0.0.0 cargo run`

A bit more complex example:

```bash
mkdir -p tmp
cargo build --release

for i in `seq 0 255`; do
  for j in `seq 255`; do
  (MCAST_IP=239.$i.0.$j MCAST_PORT=1234 IFACE_IP=0.0.0.0 target/release/mcast_sniff; echo $?> "tmp/239.${i}.0.${j}_status") &
  done
done

grep -R '0$' tmp | sed 's/tmp\///' | sed 's/_status:0//'

```
