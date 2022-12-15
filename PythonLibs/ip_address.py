#!/usr/bin/python3
"""Module to test the ipaddress library"""

import sys
from ipaddress import IPv4Address, IPv4Network

import requests

req_addr = requests.get("http://ifconfig.me/ip")
if req_addr.status_code != 200:
    print(f"Bad status code: {req_addr.status_code}")
    sys.exit(1)

global_addr = IPv4Address(req_addr.text)
print(global_addr)

# CIDR: Classless Inter-Domain Routing notation
net = IPv4Network(str(global_addr) + "/24", strict=False)
print(net.num_addresses)
print(net.prefixlen)
print(net.netmask)
