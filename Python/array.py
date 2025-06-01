a = [1,2,3,45,5]



#using index
for i in range(len(a)):
  print(a[i])

#directly accessing python
for i in a:
  print(i)

#adding into array
a.append(10)
print(a)

#removing the alst item
a.pop()
print(a)



#removing via index
a.remove(2)
print(a)
