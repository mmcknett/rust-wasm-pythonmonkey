import pythonmonkey as pm

my_lib = pm.require('./pkg/adder')

print(my_lib.add(5, 8))
print(my_lib.sub(1, 2))