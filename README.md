[![codecov](https://codecov.io/gh/aws-cli-tools/whoami/branch/main/graph/badge.svg?token=NW4955XIZT)](https://codecov.io/gh/aws-cli-tools/whoami)
[![Actions Status](https://github.com/aws-cli-tools/whoami/workflows/Code%20Gating/badge.svg?branch=main)](https://github.com/aws-cli-tools/whoami/workflows/Code%20Gating/badge.svg?branch=main)

# whoami
he `whoami` CLI is a small Rust application that allows users to identify their current AWS identity. The CLI uses AWS Security Token Service (STS) to make a GetCallerIdentity request, which returns details about the IAM user or role whose credentials are used to call the operation.

The CLI is flexible and allows output in either a standard string or JSON format, depending on the user's preferences. It also allows users to specify the AWS Region and profile to use for requests.

## Usage
To run the CLI:
```bash
whoami [OPTIONS]
```


Options:

* `-h, --help` Prints help information
* `-o, --output_type` The output format (default is string) --> Fix
* `-p, --profile` The AWS profile to use
* `-r, --region` The AWS region to use

## Contributing

