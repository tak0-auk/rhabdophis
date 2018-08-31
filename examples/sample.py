print('''hello world''')
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

def perm(l):
        # Compute the list of all permutations of l
    if len(l) <= 1:
                  return [l]
    r = []
    for i in range(len(l)):
             s = l[:i] + l[i+1:]
             p = perm(s)
             for x in p:
                # comment
                r.append(l[i:i+1] + x)
    return r
print("'lite")
    # comment
