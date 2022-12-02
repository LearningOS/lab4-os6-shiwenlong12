# 具体实现 Trap 上下文保存和恢复的汇编代码。
.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text
    .globl __alltraps
    .globl __restore
    # 使用 .align 将 __alltraps 的地址 4 字节对齐，这是 RISC-V 特权级规范的要求；
    .align 2
__alltraps:
    # csrrw 原型是csrrw rd csr rs 可以将 CSR 当前的值读到通用寄存器 rd中，
    # 然后将 rs通用寄存器 的值写入该 CSR 。
    # 因此这里起到的是交换 sscratch 和 sp 的效果。
    # 在这一行之前 sp 指向用户栈， sscratch 指向内核栈（原因稍后说明），
    # 现在 sp 指向内核栈， sscratch 指向用户栈。
    csrrw sp, sscratch, sp
    # now sp->kernel stack, sscratch->user stack
    # allocate a TrapContext on kernel stack
    # 我们准备在内核栈上保存 Trap 上下文，于是预先分配 34*8字节的栈帧，
    # 这里改动的是 sp ，说明确实是在内核栈上。
    addi sp, sp, -34*8
    # save general-purpose registers
    # 保存 Trap 上下文的通用寄存器 x0~x31，跳过 x0 和 tp(x4)，
    # 在栈帧分配之后，我们可用于保存 Trap 上下文的地址区间为 [sp, sp+8*34)
    # 通用寄存器 xn 应该被保存在地址区间 [sp+8*n, sp+8*(n+1))
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~x31
    # 我们通过类似循环的 .rept 每次使用 SAVE_GP 宏来保存，
    # 我们需要在 trap.S 开头加上 .altmacro 才能正常使用 .rept 命令。
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they were saved on kernel stack
    # 我们将 CSR sstatus 和 sepc 的值分别读到寄存器 t0 和 t1 中然后保存到内核栈对应的位置上。
    # 指令cssr rd csr 的功能就是将 CSR 的值读到寄存器 rd中。
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it on the kernel stack
    # 专门处理 sp 的问题。
    # 首先将 sscratch 的值读到寄存器 t2 并保存到内核栈上，
    # 注意： sscratch 的值是进入 Trap 之前的 sp 的值，指向 用户栈。而现在的 sp 则指向内核栈。
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # set input argument of trap_handler(cx: &mut TrapContext)
    # 让寄存器 a0 指向内核栈的栈指针也就是我们刚刚保存的 Trap 上下文的地址，
    # 这是由于我们接下来要调用 trap_handler 进行 Trap 处理，它的第一个参数 cx 由调用规范要从 a0 中获取。
    mv a0, sp
    call trap_handler

__restore:
    # case1: start running app by __restore
    # case2: back to U after handling trap
    mv sp, a0
    # now sp->kernel stack(after allocated), sscratch->user stack
    # restore sstatus/sepc
    # 负责从内核栈顶的 Trap 上下文恢复通用寄存器和 CSR 。
    # 注意我们要先恢复 CSR 再恢复通用寄存器，这样我们使用的三个临时寄存器 才能被正确恢复。
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # release TrapContext on kernel stack
    # 之前，sp 指向保存了 Trap 上下文之后的内核栈栈顶， sscratch 指向用户栈栈顶。
    # 在内核栈上回收 Trap 上下文所 占用的内存，回归进入 Trap 之前的内核栈栈顶。
    addi sp, sp, 34*8
    # now sp->kernel stack, sscratch->user stack
    # 再次交换 sscratch 和 sp，
    # 现在 sp 重新指向用户栈栈顶，sscratch 也依然保存 进入 Trap 之前的状态并指向内核栈栈顶。
    csrrw sp, sscratch, sp
    # 在应用程序控制流状态被还原之后,我们使用 sret 指令回到 U 特权级继续运行应用程序控制流。
    sret
