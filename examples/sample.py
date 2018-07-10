print('hello world')
print(123+1)
print(1 <= 2)

def decorater(func):
    def wrapper(*args, **kwargs):
        print('---start---')
        func(*args, **kwargs)
        print('---end---')
    return wrapper

class Sample:
    def hello():
        print('hello')

    @decorater
    def print(self):
        print('it is sample!')

def decorater(func):
    def wrapper(*args, **kwargs):
        print('---start---')
        func(*args, **kwargs)
        print('---end---')
    return wrapper

Sample.hello()
s = Sample()
s.print()

