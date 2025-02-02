import sys
import os

SOURCE_DIRECTORY = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(SOURCE_DIRECTORY, "proto"))

import grpc
from google.protobuf.empty_pb2 import Empty
import proto.auth_pb2_grpc as auth_grpc
from proto.auth_pb2 import *

def login(channel):
  stub = auth_grpc.AuthServiceStub(channel)
  response = stub.Login(LoginRequest(
    email="sus",
    password="Samvel25"
  ))
  print(response)

def register(channel):
  stub = auth_grpc.AuthServiceStub(channel)
  response = stub.RegisterUser(RegisterUserRequest(
    email="sus",
    password="Samvel25"
  ))
  print(response)

def main():
  channel = grpc.insecure_channel('localhost:50051')
  # register(channel)
  login(channel)

if __name__ == '__main__':
  main()