/bin/bash: :w: command not found
import random
import sys

sys.setrecursionlimit(15000)


# Ths s O(n^2)
def brute_force_correlation(L1, goal, variation, counter):
    L2 = generateL2(L1)

    cor = correlation(L1, L2)

    print("Attempt " + str(counter) + ": " + str(cor))

    if goal - variation <= cor <= goal + variation :
        return L2
    else:
        return brute_force_correlation(L1, goal, variation, counter + 1)



def generateL2(L1):
    L2 = []

    for i in range(len(L1)):
        L2.append(random.randint(0, 100))

    return L2




def correlation(listX, listY):
    # number of pairs of scores
    n = len(listX)
    p = len(listY)
    if n != p:
        print(n)
        print(p)
        raise ("L1 and L2 not same length, DIM ERROR")
    # sum of the products of paired scores
    Exy = maximumSOP(listX, listY)
    # sum of x scores
    Ex = sum(listX)
    # sum of y scores
    Ey = sum(listY)
    # some of x^2 scores
    Ex2 = sqaureSum(listX)
    # some of y^2 scores
    Ey2 = sqaureSum(listY)

    return ((n * Exy) - (Ex * Ey)) / math.sqrt((n * Ex2 - (Ex ** 2))*  ((n * Ey2 - (Ey ** 2))))


def maximumSOP(a, b) :

    # Variable to store the sum of  
    # products of array elements  
    sop = 0

    # length of the arrays  
    n = len(a)

    # Traversing both the arrays  
    # and calculating sum of product 
    for i in range(n) :
        sop += a[i] * b[i]

    return sop

def sqaureSum(a) :

    # Variable to store the sum of  
    # products of array elements  
    s = 0

    # length of the arrays  
    n = len(a)


    # Traversing both the arrays  
    # and calculating sum of product 
    for i in range(n) :
        s += a[i] ** 2

    return s



print(brute_force_correlation([2, 2.3, 3.3, 3.7, 4.2, 4.6, 4.5, 5, 5.5, 5.7, 6.1, 6.4], 0.75, 0.001, 0))
