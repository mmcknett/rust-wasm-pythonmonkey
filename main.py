import pythonmonkey as pm

my_lib = pm.require('./pkg/adder')

print(my_lib.add(5, 8))