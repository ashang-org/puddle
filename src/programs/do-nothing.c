int _start() {
  asm(
      "mov $127,%ebx\n" /* exit code        */
      "mov $1,%eax\n"   /* syscall #1, exit */
      "int $0x80\n"     /* invoke syscall   */
      );
}
