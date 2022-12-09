#include<stdio.h>
#include<unistd.h>
#include<sys/syscall.h>

int main() {
  long res = syscall(451);
  printf("%d\n",res);
  return res;
}
