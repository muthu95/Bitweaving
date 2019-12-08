import numpy as np
import matplotlib.pyplot as plt

class norm1:
    def __init__(self, a1, b1, c1):
        self.a1 = a1
        self.b1 = b1
        self.c1 = c1
        
    def dist_curve(self):
        plt.plot(self.c1, 1/(self.b1 * np.sqrt(2 * np.pi)) *
            np.exp( - (self.c1 - self.a1)**2 / (2 * self.b1**2) ), linewidth=2, color='y')
        plt.show()

#Vary the mean and SD to generate different plots
mean1 = 500000
sd1 = 100000

c = np.random.normal(mean1, sd1, 1000000)
c = np.round(c)
minVal = min(c)
#print (c)
#print minVal
result = map(lambda x: x + minVal, c)
#print (result)
#print min(result)
#print max(result)
result.sort()
#print (result)

import random
x = random.sample(range(0, 42949600), 1000000)
#print (x)
x.sort()

with open('dataset', 'w') as f:
    for item in x:
        print >> f, int(item)
        
w1, x1, z1 = plt.hist(x, 100, normed=True) #hist

hist1 = norm1(mean1, sd1, x1)
#plot1 = hist1.dist_curve()
