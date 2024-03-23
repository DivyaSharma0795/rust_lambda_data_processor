# Rust Lambda Data Processor

## Overview

This project implements an AWS Lambda function in Rust using Cargo Lambda. The Lambda function receives a JSON payload containing three inputs, "id", "name" and "age". It takes that input and then populates a DynamoDB database. The processed data can be triggered via API Gateway.

## Requirements

- Rust compiler and Cargo package manager installed.
- AWS account with permission to create Lambda functions.
- API Gateway endpoint for triggering the Lambda function.
- Enable Cloudwatch logs and X-ray tracing for the function

## Project Structure

1. **Lambda Function Code:**
   - Create a Rust project using Cargo.
   - Add dependencies for AWS Lambda and serde for JSON serialization.
   - Implement the Lambda function logic to receive three integers, process them, and return the results.
   
2. **API Gateway Integration:**
   - Configure an API Gateway endpoint to trigger the Lambda function.
   - Set up the necessary permissions and roles for the API Gateway to invoke the Lambda function.


## Setup Instructions

1. Clone the repository:

   ```
   git clone https://gitlab.com/dukeaiml/IDS721/ds655_ids721_miniproject06.git
   ```

2. Navigate to the project directory:

   ```
   cd ds655_ids721_miniproject06
   ```

3. Build the project:

   ```
   cargo build --release
   ```

The functionality for the lambda function is simple. Takes in data in JSON format for `id`, `name` and `age` and populates that in a DynamoDB database.



## Screenshots 

## Function

![Screenshot 2](screenshots/Screenshot 2024-03-23 at 01-33-06 CloudWatch us-east-2.png)

## Log streams

![Screenshot 3](screenshots/Screenshot 2024-03-23 at 01-29-26 Name_Reversal - Lambda.png)

# X-Ray Tracing

![Screenshot 4](screenshots/Screenshot%202024-03-23%20at%2001-30-14%20Name_Reversal%20-%20Lambda.png)

-----

## License

This project is licensed under the [MIT License](LICENSE).


