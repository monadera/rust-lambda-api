from aws_cdk.aws_apigatewayv2 import (
    CorsHttpMethod,
    CorsPreflightOptions,
    HttpApi,
    HttpMethod,
)
from aws_cdk.aws_apigatewayv2_authorizers import HttpJwtAuthorizer
from aws_cdk.aws_apigatewayv2_integrations import HttpLambdaIntegration
from aws_cdk.aws_cognito import UserPool, UserPoolClient
from aws_cdk.aws_lambda import IFunction
from constructs import Construct

ALLOWED_HEADERS = ["Authorization", "Content-Type"]
ALLOWED_METHODS = [
    CorsHttpMethod.GET,
    CorsHttpMethod.OPTIONS,
    CorsHttpMethod.POST,
]


class ServerlessApi(Construct):
    def __init__(
        self,
        scope: Construct,
        construct_id: str,
        *,
        handler: IFunction,
        user_pool: UserPool,
        user_pool_client: UserPoolClient,
    ) -> None:
        super().__init__(scope, construct_id)

        self._api = self.build_http_api()
        authorizer = self.build_authorizer(user_pool, user_pool_client)
        self.setup_lambda_integration(self._api, handler, authorizer)

    @property
    def endpoint_url(self) -> str:
        return self._api.api_endpoint

    def build_http_api(self) -> HttpApi:
        cors_options = CorsPreflightOptions(
            allow_headers=ALLOWED_HEADERS, allow_methods=ALLOWED_METHODS
        )
        return HttpApi(self, "ServerlessRustApi", cors_preflight=cors_options)

    @staticmethod
    def build_authorizer(
        user_pool: UserPool, user_pool_client: UserPoolClient
    ) -> HttpJwtAuthorizer:
        issuer = f"https://cognito-idp.{user_pool.env.region}.amazonaws.com/{user_pool.user_pool_id}"
        return HttpJwtAuthorizer(
            "JwtAuthorizer", issuer, jwt_audience=[user_pool_client.user_pool_client_id]
        )

    @staticmethod
    def setup_lambda_integration(
        api: HttpApi, handler: IFunction, authorizer: HttpJwtAuthorizer
    ) -> None:
        integration = HttpLambdaIntegration("LambdaIntegration", handler)
        api.add_routes(
            path="/{proxy+}",
            methods=[HttpMethod.GET, HttpMethod.POST],
            authorizer=authorizer,
            integration=integration,
        )
