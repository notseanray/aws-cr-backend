#### CR http/REST api backend

This is a REST api backend running on AWS Lambda with dynamodb for the CircuitRunners website.

Heavily a work in progress.

##### setting up a development environment:

Install docker, cargo, rustc x86 glib, rustc musl (doing so via rustup is recommended), and unzip.

Ensure docker is running and functional (run the hello world container to test)

```
$ git clone --depth=1 git@github.com:notseanray/aws-cr-backend.git
$ make crosscompile 
$ make zip # creates zip archive of the lambda function
$ make testlambda # enter your json payload into stdin here
```
