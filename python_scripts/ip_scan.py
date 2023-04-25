import nmap
import time
import sys

nm = nmap.PortScanner()
args = sys.argv

def nmap_py(ip_addr, args):
    start = time.time()
    scan_result = nm.scan(ip_addr, arguments=f"{args}")
    print(scan_result)
    end = time.time()
    print(end - start)

with open(args[1], "r") as file:
    iplist = file.read()
    nmap_py(iplist, args[2])
