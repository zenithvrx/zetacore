# holocron

holocron is a Rust library with Python bindings for efficient vector storage and similarity search.

## Features

- Store and manage vector records with associated metadata
- Perform cosine similarity search on stored vectors
- Efficient add, get, and delete operations
- Python bindings for easy integration with Python projects

## Installation

### Python

```bash
pip install holocron
```

## Usage

### Python

```python
import holocron

store = holocron.VectorStore([])
store.add([
    { "id": "vector_1", "values": [1.0, 2.0] },
    { "id": "vector_2", "values": [5.3, 3.9] },
])

result = store.query([1.5, 2.5], 1)
print(f"Nearest vector: {result[0]}")
```

## Contributing

Contributions are welcome!

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.


