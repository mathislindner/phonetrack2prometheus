# phonetrack2prometheus
A flask based server that will exposes a route (with authentication) for a user to send their live location from the [nextcloud phonetrack app](https://github.com/julien-nc/phonetrack/tree/main) that is maintaned by the kind @julien-nc 
## get started
1. the .env should look sth like this:
```
FLASK_USERNAME=USERNAME
FLASK_PASSWORD=YOURWELLSECUREDPW
FLASK_HOST=0.0.0.0
FLASK_PORT=5000
```
2. make sure you configure prometheus accordingly:
```
- job_name: 'phonetrack'
    scheme: https
    static_configs:
      - targets: ['address.to.server.that.hosts.phonetrack2prometheus']
    basic_auth:
      username: 'USERNAME'
      password: 'YOURWELLSECUREDPW'

```
3. (optional) use grafana to display the metrics (I may publish my dashboard sometime)
4. configure phontrack on your phone with authentication 

## Notes
- **DISCLAIMER**: there are a few things that are not best practice, make sure your know what you are doing. This is just a boilerplate for people who are too lazy to reverse engineer the readings of the phonetrack json
- Will only work when the phone is online, does not store the time when stat was sent from phone
- With this setup, we only handle local network incoming requests obvs.
