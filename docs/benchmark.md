# Benchmark

## Setup

1. Start iperf server

```
iperf3 -s -B 127.0.0.1
```

2. Start srp_gateway for tcp and udp

```
srp_gateway -c examples/tcp/gateway.toml
```

3. Start srp_proxy

```
srp_proxy -c examples/tcp/proxy.toml
srp_proxy -c examples/udp/proxy.toml
```


## TCP

4. Start iperf client

```
iperf3 -c 127.0.0.1 -p 8080
Connecting to host 127.0.0.1, port 8080
[  4] local 127.0.0.1 port 41598 connected to 127.0.0.1 port 8080
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  4]   0.00-1.00   sec   239 MBytes  2.00 Gbits/sec    0    639 KBytes       
[  4]   1.00-2.00   sec   257 MBytes  2.16 Gbits/sec    0    639 KBytes       
[  4]   2.00-3.00   sec   375 MBytes  3.14 Gbits/sec    0    639 KBytes       
[  4]   3.00-4.00   sec   433 MBytes  3.63 Gbits/sec    0    639 KBytes       
[  4]   4.00-5.00   sec   420 MBytes  3.52 Gbits/sec    0    639 KBytes       
[  4]   5.00-6.00   sec   416 MBytes  3.49 Gbits/sec    0    639 KBytes       
[  4]   6.00-7.00   sec   431 MBytes  3.62 Gbits/sec    0    639 KBytes       
[  4]   7.00-8.00   sec   437 MBytes  3.66 Gbits/sec    0   1023 KBytes       
[  4]   8.00-9.00   sec   429 MBytes  3.60 Gbits/sec    0   1023 KBytes       
[  4]   9.00-10.00  sec   428 MBytes  3.59 Gbits/sec    0   1023 KBytes       
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  4]   0.00-10.00  sec  3.77 GBytes  3.24 Gbits/sec    0            sender
[  4]   0.00-10.04  sec  3.77 GBytes  3.22 Gbits/sec                  receiver
```


## UDP

```
iperf3 -u -c 127.0.0.1 -p 8080 -l 1000
Connecting to host 127.0.0.1, port 8080
[  4] local 127.0.0.1 port 40628 connected to 127.0.0.1 port 8080
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  4]   0.00-1.00   sec   129 KBytes  1.05 Mbits/sec  132  
[  4]   1.00-2.00   sec   128 KBytes  1.05 Mbits/sec  131  
[  4]   2.00-3.00   sec   128 KBytes  1.05 Mbits/sec  131  
[  4]   3.00-4.00   sec   128 KBytes  1.05 Mbits/sec  131  
[  4]   4.00-5.00   sec   128 KBytes  1.05 Mbits/sec  131  
[  4]   5.00-6.00   sec   128 KBytes  1.05 Mbits/sec  131  
[  4]   6.00-7.00   sec   128 KBytes  1.05 Mbits/sec  131  
[  4]   7.00-8.00   sec   128 KBytes  1.05 Mbits/sec  131  
[  4]   8.00-9.00   sec   128 KBytes  1.05 Mbits/sec  131  
[  4]   9.00-10.00  sec   128 KBytes  1.05 Mbits/sec  131  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  4]   0.00-10.00  sec  1.25 MBytes  1.05 Mbits/sec  0.000 ms  0/1311 (0%)  sender
[  4]   0.00-10.04  sec  1.25 MBytes  1.04 Mbits/sec  0.362 ms  0/1311 (0%)  receiver

iperf Done.
```
