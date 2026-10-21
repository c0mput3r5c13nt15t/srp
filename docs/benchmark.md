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


## UDP

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


## TCP

```
iperf3 -u -c 127.0.0.1 -p 8080
Connecting to host 127.0.0.1, port 8080
[  4] local 127.0.0.1 port 37585 connected to 127.0.0.1 port 8080
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  4]   0.00-1.00   sec   160 KBytes  1.31 Mbits/sec  5  
[  4]   1.00-2.00   sec   128 KBytes  1.05 Mbits/sec  4  
[  4]   2.00-3.00   sec   128 KBytes  1.05 Mbits/sec  4  
[  4]   3.00-4.00   sec   128 KBytes  1.05 Mbits/sec  4  
[  4]   4.00-5.00   sec   128 KBytes  1.05 Mbits/sec  4  
[  4]   5.00-6.00   sec   128 KBytes  1.05 Mbits/sec  4  
[  4]   6.00-7.00   sec   128 KBytes  1.05 Mbits/sec  4  
[  4]   7.00-8.00   sec   128 KBytes  1.05 Mbits/sec  4  
[  4]   8.00-9.00   sec   128 KBytes  1.05 Mbits/sec  4  
[  4]   9.00-10.00  sec   128 KBytes  1.05 Mbits/sec  4  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  4]   0.00-10.00  sec  1.28 MBytes  1.07 Mbits/sec  0.000 ms  0/41 (0%)  sender
[  4]   0.00-10.05  sec  0.00 Bytes  0.00 bits/sec  0.000 ms  0/0 (0%)  receiver

iperf Done.
```
