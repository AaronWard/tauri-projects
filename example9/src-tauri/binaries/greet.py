import sys

def greet(name):
    return f"Hello, {name}! - from python"

if __name__ == "__main__":
    name = sys.argv[1] if len(sys.argv) > 1 else "World"
    print(greet(name))
