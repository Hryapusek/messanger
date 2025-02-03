# Test client

## Example usage

### Register
```sh
python src/main.py -ca localhost:50051 -r --cln example_name --cluuid example_uuid
```

Same with description

```sh
python src/main.py -ca localhost:50051 -r -cln example_name -cluuid example_uuid -cldescr some_description
```

### Unregister
```sh
python src/main.py -ca localhost:50051 -u -cluuid example_uuid
```

### List clients
```sh
python src/main.py -ca localhost:50051 -l
```

### Write binary log
```sh
python src/main.py -ca localhost:50051 -b -cluuid example_uuid
```

### Write text log
```sh
python src/main.py -ca localhost:50051 -t -cluuid example_uuid
```

### Write text log with custom message
```sh
python src/main.py -ca localhost:50051 -t -cluuid example_uuid -tm "example message"
```

### Stream write binary log
```sh
python src/main.py -ca localhost:50051 -bs -cluuid example_uuid -bsf data.txt
```

### Write binary log with custom message
```sh
python src/main.py -ca localhost:50051 -b -cluuid example_uuid -bm "example message"
```

### Stream write text log
```sh
python src/main.py -ca localhost:50051 -ts -cluuid example_uuid -tsf data.txt
```
