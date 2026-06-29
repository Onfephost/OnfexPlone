import math

def softmax(x):
    m = max(x)
    e = [math.exp(i - m) for i in x]
    s = sum(e)
    return [i/s for i in e]
print(softmax([1]*7))