
i=0
x=1.0
while i<1000000:
    x=x*1.0000001+0.0000001
    i+=1
print(format(x,".15g"))
