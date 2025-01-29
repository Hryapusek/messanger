import grpc_tools.protoc as protoc
import os.path as path

PROTO_SOURCE_DIR = path.join("..", "api", "proto")
PROTO_OUT_DIR = path.join("src", "proto")

PROTO_FILES = ["auth.proto"]

PROTO_FILES_STR = [path.join(PROTO_SOURCE_DIR, f) for f in PROTO_FILES]

args = [
    "protoc",
    f"-I{PROTO_SOURCE_DIR}",
    f"--python_out={PROTO_OUT_DIR}",
    f"--pyi_out={PROTO_OUT_DIR}",
    f"--grpc_python_out={PROTO_OUT_DIR}",
    *PROTO_FILES_STR,
]

print(f"Args: {args}")

protoc.main(args)
