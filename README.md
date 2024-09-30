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
2. make sure you configure prometheus accordingly
3. (optional) use grafana to display the metrics (I may publish my dashboard sometime)

## Notes
- Will only work when the phone is online, does not store the time when stat was sent from phone
- With this setup, we only handle local network incoming requests obvs.
