*Plan of Attack:*
Now that the server has defended against simple replay attacks by making the OTPs truly One-Time, we must get a little more clever. The OTP server relies is trusting time, but time may not be so trustworthy. If from the attacker we can somehow control the time on the OTP server, then we can go back to a previous time slice and use an old code, even if it was invalidated for that slice. In order to defend against this sort of attack, the OTP server _must_ persist **all** historically used passcodes. This seems somewhat intractable as the space used by such a system will grow unbounded in time. 

In order to attack this server, we proceed as follows:
1. Passively capture the first OTP. Store it and when it was used.
2. Wait for a new OTP in a different time slice to be captured.
3. Rewind time on the compromised NTP server to the time stamp captured in step 1.
4. Wait a little bit for the OTP server to pick up the new time.
5. Replay the first captured OTP.
6. Profit.

First, setup the server to pull time periodically from the NTP server running on host 2. Under normal circumstances it wouldn't poll this frequently, but to demonstrate the attack we do it every 1-2 seconds.
```nohup ./sync_time.sh > time.log &```
To see what this is doing, run 
```tail -f time.log```
Now also on host 3 boot up the server, but this time make sure we burn the codes!
```
ROCKET_ADDRESS=0.0.0.0 ./server
```
On host 2, setup the attack. We use the exact same arp spoof method as before, but this time we configure the attacker to execute the more complicated rewind attack. As the attacker is on the same host as the ntp server, it rewinds the system clock and then restarts the NTP server so it picks it up. The OTP server will pick this change up shortly after.
```
./spoof.sh
OTP_REPLAY_MODE=rewind ./attack
```

With the attacker prepped, run this on the client host (host 1):
```
./client 10.4.9.3 && sleep 30 && ./client 10.4.9.3
```
This ensures that we get two different time slices given our slice size is 30 seconds. 

**NOTE:** _This takes a long time because we have to wait for a time slice of 30 seconds to pass before generating a new OTP! Please be patient :)_

Once this completes, on the attacker we see 
```
Starting capture...
Capture initialized.
Setting tcp filter...
Listening...
Captured OTP: "96714c524515c7a9441095daf016c095"
Waiting for one more otp
Captured OTP: "96714c524515c7a9441095daf016c095"
Captured OTP: "fb1fb1c6f00a2447709e1ad6dd18e663"
Second OTP captured
Rewinding time of auth server...
Rewinding to 1573095240
1573095240
Time rewind complete. Executing replay attack with old OTP.
SUCESSS: Replay attack worked!
```

We successfully sniffed out both of the OTPs sent, rewinded time to the beginning of the first time slice for the first OTP and then replayed that code to the server. Because the 'burning' code on the OTP server doesn't remember _all_ of the used codes, we've bypassed the burning by going back in time!

You can see that host 3 is now behind host 2 in time 
```
root@host1:~# date
Wed Nov  6 22:04:08 EST 2019
root@host3:~# date
Wed Nov  6 22:03:20 EST 2019
```

**DISCLAIMER:** _For unknown reasons, sometimes the attacker doesn't pick up the second OTP request. If this happens please re-do `./spoof` and try again._
