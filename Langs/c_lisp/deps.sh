MPC_C="https://raw.githubusercontent.com/orangeduck/mpc/master/mpc.c"
MPC_H="https://raw.githubusercontent.com/orangeduck/mpc/master/mpc.h"

# sudo dnf install libedit libedit-devel
curl -sS $MPC_C > src/mpc.c
curl -sS $MPC_H > src/mpc.h
