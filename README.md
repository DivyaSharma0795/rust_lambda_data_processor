# Rust Lambda Data Processor

## Overview

This project implements an AWS Lambda function in Rust using Cargo Lambda. The Lambda function receives a JSON payload containing three inputs, "id", "name" and "age". It takes that input and then populates a DynamoDB database. The processed data can be triggered via API Gateway.

This is an extension from the project from week 5, with enabled the logging and X-ray tracing for week 6.

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
   git clone https://gitlab.com/dukeaiml/IDS721/fj-49-week-6-aws-lambda-logging.git
   ```

2. Navigate to the project directory:

   ```
   cd fj49_week_6_aws_lambda_logging
   ```

3. Build the project:

   ```
   cargo build --release
   ```

Since the rust lambda function is similar to week 5, the functionality is simple. Takes in data in JSON format for `id`, `name` and `age` and populates that in a DynamoDB database.

You can read full details ![here](https://gitlab.com/dukeaiml/IDS721/week5_cloud_fj49)


## Screenshots 

## Function

![Screenshot 2](screenshots/function.png)

## Log streams

![Screenshot 3](screenshots/logstreams.png)

# X-Ray Tracing

![Screenshot 4](screenshots/Xray_tracing.png)

-----

## License

This project is licensed under the [MIT License](LICENSE).


