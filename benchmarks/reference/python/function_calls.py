def square_plus(x):
    return x*x+3

i=0
total=0
while i<300000:
    total+=square_plus(i)
    i+=1
print(total)
