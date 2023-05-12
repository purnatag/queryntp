# queryntp  
Rust code for querying an NTP server, used by two threads. The aim was to spot any delay between the reception of timestamp by the two threads fron the NTP server.
However, there is none, as the protocol correctly calculates the network delays between the two threads, if any.
Also, two is probably too small a number to detect such discrepancies.
