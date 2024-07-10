import pathlib

from aws_cdk import CfnOutput, Stack
from aws_cdk.aws_cognito import (
    AuthFlow,
    StandardAttribute,
    StandardAttributes,
    UserPool,
    UserPoolClient,
)
from aws_cdk.aws_dynamodb import Attribute, AttributeType, Table
from aws_cdk.aws_lambda import Architecture, Code, Function, Runtime
from constructs import Construct

from stack.api import ServerlessApi


class ServerlessRustStack(Stack):
    def __init__(self, scope: Construct, construct_id: str) -> None:
        super().__init__(scope, construct_id)

        database_table = self.create_database_table()
        handler = self.create_function(
            "serverless", table_name=database_table.table_name
        )
        database_table.grant_read_write_data(handler)

        user_pool, user_pool_client = self.create_user_pool()
        api = ServerlessApi(
            self,
            "ServerlessApi",
            handler=handler,
            user_pool=user_pool,
            user_pool_client=user_pool_client,
        )

        CfnOutput(
            self,
            "ApiEndpointUrl",
            value=api.endpoint_url,
            description="The URL of the API endpoint.",
        )

    def create_database_table(self) -> Table:
        partition_key = Attribute(name="pk", type=AttributeType.STRING)
        return Table(
            self,
            "DatabaseTable",
            table_name="currencies",
            partition_key=partition_key,
        )

    def create_function(self, bin_name: str, *, table_name: str) -> Function:
        code_path = pathlib.Path.cwd() / "target" / "lambda" / bin_name
        code = Code.from_asset(code_path.as_posix())

        return Function(
            self,
            "ApiHandler",
            code=code,
            architecture=Architecture.ARM_64,
            runtime=Runtime.PROVIDED_AL2023,
            memory_size=256,
            handler="does-not-matter",
            environment={
                "TABLE_NAME": table_name,
            },
        )

    def create_user_pool(self) -> tuple[UserPool, UserPoolClient]:
        standard_attributes = StandardAttributes(
            given_name=StandardAttribute(required=True, mutable=True),
            family_name=StandardAttribute(required=True, mutable=True),
        )
        user_pool = UserPool(
            self,
            "UserPool",
            user_pool_name="rust-demo",
            self_sign_up_enabled=False,
            standard_attributes=standard_attributes,
        )
        auth_flows = AuthFlow(admin_user_password=True, user_srp=True)
        user_pool_client = user_pool.add_client("app-client", auth_flows=auth_flows)

        return user_pool, user_pool_client
