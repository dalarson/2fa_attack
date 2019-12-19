You must be on a computer that has passwordless root ssh access to 10.4.9.[1-3].
The servers must be fully reformated.
Execute `./deploy.sh otp*` to prepare the servers. 

If you see this, just run the deploy again. It should be idempotent.
```
E: Could not get lock /var/lib/dpkg/lock-frontend - open (11: Resource temporarily unavailable)
E: Unable to acquire the dpkg frontend lock (/var/lib/dpkg/lock-frontend), is another process using it?
```
If you see
```
FATAL -> Failed to fork.
```
Then it ran out of ram. Please increase the ram for the server and start over - I can't do anything about this.


Once the deploy script has succesfully completed, you can move on to executing the attacks described in `ATTACK_[1-2].md`.

