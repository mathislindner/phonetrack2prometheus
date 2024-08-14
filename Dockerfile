# Use the official Python image from the Docker Hub
FROM python:3.9-slim

# Set the working directory in the container
WORKDIR /app

# Copy the requirements file into the container
COPY requirements.txt .

# Install any dependencies specified in requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Copy the rest of the working directory contents into the container
COPY . .

# Expose the port the app runs on
EXPOSE 5000

# Run the application
CMD ["python", "server.py"]