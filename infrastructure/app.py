#!/usr/bin/env python3
import aws_cdk as cdk

from stack.stack import ServerlessRustStack

app = cdk.App()
ServerlessRustStack(app, "ServerlessRustStack")

app.synth()
