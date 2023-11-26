import numpy as np
import matplotlib.pyplot as plt
import time

from qiskit import QuantumRegister, QuantumCircuit
from qiskit import Aer
from qiskit.visualization import plot_state_city
import qiskit.quantum_info as qi

backend = Aer.get_backend('statevector_simulator')

N = 14

q = QuantumRegister(N)
circ = QuantumCircuit(q)

#print(qi.Statevector.from_instruction(circ))

start = time.time()

for j in range(N):
    for k in range(j):
        circ.cp(np.pi/float(2**(j-k)), q[j], q[k])
    circ.h(q[j])

print("Elapsed: {0:.2f}ms".format((time.time()-start)*1000))

#for j in range(N >> 1):
#    circ.swap(q[j], q[N-j-1])

#print(circ)

job = backend.run(circ)

result = job.result()

outputstate = result.get_statevector(circ, decimals=3)
#plot_state_city(outputstate)
#print(outputstate)

#plt.show()
