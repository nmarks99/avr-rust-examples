#!/usr/bin/env python3
from matplotlib import pyplot as plt
import numpy as np

# PID Equation: u = kp*err + ki*Ierrg + kp


def plant(u):
    val = u
    return val

def controller(Kp,Ki,Kd,err,int_err,drv_err):
    u = Kp*err + Ki*int_err + Kd*drv_err 
    return u


Kp = 0.45
Ki = 0.3
Kd = 0.01

val = 0
target = 5.0
err = 0 
int_err = 0
drv_err = 0
last_err = err
arr = []

while True:
    arr.append(val)
    u = controller(Kp,Ki,Kd,err,int_err,drv_err)
    
    val = plant(u)

    err = target - val
    drv_err = err - last_err

    count = 0
    if count > 1000:
        int_err = 0
        count = 0
    else:
        int_err += err
        count += 1
    
    if len(arr) > 50:
        break
    last_err = err

plt.style.use("ggplot")
fig, ax = plt.subplots()
#  ax.plot(np.linspace(0,max(arr),len(arr)),arr,"-bo")
ax.plot(arr,"-bo")
plt.show()







