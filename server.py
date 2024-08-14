from flask import Flask, request, jsonify, Response
from functools import wraps
import os
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

app = Flask(__name__)

# Retrieve the credentials from the environment variables
USERNAME = os.getenv("FLASK_USERNAME")
PASSWORD = os.getenv("FLASK_PASSWORD")

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

@app.route('/api', methods=['POST'])
@requires_auth
def receive_json():
    if request.is_json:
        data = request.get_json()
        # Here you would send the data to Prometheus
        # For now, we'll just return it in the response for debugging purposes
        return jsonify({"status": "success", "data": data}), 200
    else:
        return jsonify({"status": "failure", "message": "Request body must be JSON"}), 400

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
