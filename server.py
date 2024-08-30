from flask import Flask, request, jsonify, Response
from functools import wraps
import os
import logging
from dotenv import load_dotenv
from prometheus_client import CollectorRegistry, Gauge, generate_latest, CONTENT_TYPE_LATEST

# Load environment variables from .env file
load_dotenv()

app = Flask(__name__)

# Setup logging
logging.basicConfig(level=logging.DEBUG)  # Set to DEBUG for detailed output
logger = logging.getLogger(__name__)
logger.setLevel(logging.DEBUG)

# Retrieve the credentials from the environment variables
USERNAME = os.getenv("FLASK_USERNAME")
PASSWORD = os.getenv("FLASK_PASSWORD")
FLASK_HOST = os.getenv("FLASK_HOST")
FLASK_PORT = os.getenv("FLASK_PORT")

# Create Prometheus registry
registry = CollectorRegistry()

# Define your metrics (Gauges, Counters, etc.)
location_lat = Gauge('device_latitude', 'Latitude of the device', registry=registry)
location_lon = Gauge('device_longitude', 'Longitude of the device', registry=registry)
battery_level = Gauge('device_battery_level', 'Battery level of the device', registry=registry)
#velocity = Gauge('device_velocity', 'Velocity of the device', registry=registry)
altitude = Gauge('device_altitude', 'Altitude of the device', registry=registry)
accuracy = Gauge('device_accuracy', 'Accuracy of the device', registry=registry)

def check_auth(username, password):
    """Check if a username/password combination is valid."""
    return username == USERNAME and password == PASSWORD

def authenticate():
    """Sends a 401 response that enables basic auth"""
    return Response(
        'Could not verify your access level for that URL.\n'
        'You have to login with proper credentials', 401,
        {'WWW-Authenticate': 'Basic realm="Login Required"'})

def requires_auth(f):
    @wraps(f)
    def decorated(*args, **kwargs):
        auth = request.authorization
        if not auth or not check_auth(auth.username, auth.password):
            return authenticate()
        return f(*args, **kwargs)
    return decorated

@app.route('/health', methods=['GET'])
def health_check():
    return jsonify({"status": "success", "message": "Server is healthy"}), 200

@app.route('/metrics', methods=['GET'])
@requires_auth
def metrics():
    logger.debug("Metrics endpoint reached")
    # Expose the metrics for Prometheus scraping
    return Response(generate_latest(registry), mimetype=CONTENT_TYPE_LATEST)

def process_json(data):
    def process_value(value):
        try:
            return float(value)
        except ValueError:
            return 0
    try:
        lat = process_value(data.get('lat', 0))
        lon = process_value(data.get('lon', 0))
        batt = process_value(data.get('batt', 0))
        #vel = process_value(data['vel'])
        alt = process_value(data.get('alt', 0))
        acc = process_value(data.get('acc', 0))

        # Update Prometheus metrics
        location_lat.set(lat)
        location_lon.set(lon)
        battery_level.set(batt)
        #velocity.set(vel)
        altitude.set(alt)
        accuracy.set(acc)
        logger.debug(f"Processed data: lat={lat}, lon={lon}, batt={batt}, alt={alt}, acc={acc}")
        return True
    
    except (ValueError) as e:
        logger.error(f"Error processing data: {data}. Error: {e}")
        return False

@app.route('/api', methods=['POST'])
@requires_auth
def receive_json():
    if request.is_json:
        data = request.get_json()

        # Check if the data is a list (i.e., multiple JSONs sent at once)
        if isinstance(data, list):
            logger.debug(f"Received a batch of {len(data)} JSON objects.")
            successes, failures = 0, 0

            for item in data:
                if process_json(item):
                    successes += 1
                else:
                    failures += 1

            return jsonify({
                "status": "success",
                "message": f"Processed {successes} JSON objects, {failures} failed."
            }), 200

        # Handle single JSON object
        elif isinstance(data, dict):
            logger.debug("Received a single JSON object.")
            if process_json(data):
                return jsonify({"status": "success", "message": "Processed JSON"}), 200
            else:
                return jsonify({"status": "failure", "message": "Error processing JSON"}), 400

    else:
        logger.warning("Request body is not JSON.")
        return jsonify({"status": "failure", "message": "Request body must be JSON"}), 400

if __name__ == '__main__':
    app.run(host=FLASK_HOST, port=FLASK_PORT)
