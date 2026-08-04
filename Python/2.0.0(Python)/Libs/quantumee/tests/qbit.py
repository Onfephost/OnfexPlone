import math
import random
import os
class Qubit:
    def __init__(self, alpha=1.0, beta=0.0):
        "Starting state |0>"
        self.alpha = alpha
        self.beta = 1 - alpha

    def show(self):
        print(f"|ψ> = {self.alpha:.3f}|0> + {self.beta:.3f}|1>")

    def X(self):
        "Pauli-X (NOT)"
        self.alpha, self.beta = self.beta, self.alpha

    def H(self):
        "Hadamard"
        a = (self.alpha + self.beta) / math.sqrt(2)
        b = (self.alpha - self.beta) / math.sqrt(2)
        self.alpha = a
        self.beta = b

    def measure(self):
        p0 = self.alpha ** 2
        print(p0)
        if math.pi/10 > p0:
            self.alpha = 1
            self.beta = 0
            return 0
        else:
            self.alpha = 0
            self.beta = 1
            return 1

os.system('cls' if os.name == 'nt' else 'clear')
q = Qubit(0.4)
print("Starting state:")
q.show()

print("\nHadamard gate applied...")
q.H()
q.show()

print("\nMeasurement:")
print(q.measure())

print("\nAfter measurement:")
q.show()
print(random.random())