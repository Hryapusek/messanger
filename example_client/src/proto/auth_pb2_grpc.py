# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc
import warnings

import auth_pb2 as auth__pb2
from google.protobuf import empty_pb2 as google_dot_protobuf_dot_empty__pb2

GRPC_GENERATED_VERSION = '1.70.0'
GRPC_VERSION = grpc.__version__
_version_not_supported = False

try:
    from grpc._utilities import first_version_is_lower
    _version_not_supported = first_version_is_lower(GRPC_VERSION, GRPC_GENERATED_VERSION)
except ImportError:
    _version_not_supported = True

if _version_not_supported:
    raise RuntimeError(
        f'The grpc package installed is at version {GRPC_VERSION},'
        + f' but the generated code in auth_pb2_grpc.py depends on'
        + f' grpcio>={GRPC_GENERATED_VERSION}.'
        + f' Please upgrade your grpc module to grpcio>={GRPC_GENERATED_VERSION}'
        + f' or downgrade your generated code using grpcio-tools<={GRPC_VERSION}.'
    )


class AuthServiceStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.HelloWorld = channel.unary_unary(
                '/auth.AuthService/HelloWorld',
                request_serializer=google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,
                response_deserializer=auth__pb2.HelloWorldResponse.FromString,
                _registered_method=True)


class AuthServiceServicer(object):
    """Missing associated documentation comment in .proto file."""

    def HelloWorld(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_AuthServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'HelloWorld': grpc.unary_unary_rpc_method_handler(
                    servicer.HelloWorld,
                    request_deserializer=google_dot_protobuf_dot_empty__pb2.Empty.FromString,
                    response_serializer=auth__pb2.HelloWorldResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'auth.AuthService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))
    server.add_registered_method_handlers('auth.AuthService', rpc_method_handlers)


 # This class is part of an EXPERIMENTAL API.
class AuthService(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def HelloWorld(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/auth.AuthService/HelloWorld',
            google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,
            auth__pb2.HelloWorldResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)
