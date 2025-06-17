import { sleep } from 'k6';
import http from 'k6/http';

const baseEndpoints = ['http://rust-service-v1:8080', 'http://rust-service-v2:8080'];

export const options = {
    vus: 1, // 1 virtual user
    duration: '0s', // Run indefinitely
};

let endpointIndex = 0; // Track which endpoint to use

export default function () {
    // Send a request to rust-service-v1 or rust-service-v2 alternately
    const baseEndpoint = baseEndpoints[endpointIndex];
    http.get(baseEndpoint);

    // Switch between rust-service-v1 and rust-service-v2 for the next request
    endpointIndex = (endpointIndex + 1) % baseEndpoints.length;

    // Wait for 1 second before sending the next request
    sleep(1);
} 