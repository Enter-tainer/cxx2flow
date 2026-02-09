#ifndef _UNISTD_H
#define _UNISTD_H

#include <stdint.h>

typedef long ssize_t;
typedef unsigned int uid_t;
typedef unsigned int gid_t;
typedef long off_t;
typedef int pid_t;

ssize_t read(int fd, void *buf, size_t count);
ssize_t write(int fd, const void *buf, size_t count);
int close(int fd);
int dup(int oldfd);
off_t lseek(int fd, off_t offset, int whence);
pid_t getpid(void);
unsigned int sleep(unsigned int seconds);

#define STDIN_FILENO  0
#define STDOUT_FILENO 1
#define STDERR_FILENO 2

#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

#endif
