#include <linux/perf_event.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/ioctl.h>
#include <sys/syscall.h>
#include <unistd.h>
#include <string.h>
#include <time.h>
#include <sys/types.h>

static int perf_event_open(struct perf_event_attr *hw_event, pid_t pid, int cpu, int group_fd, unsigned long flags)
{
    int ret;
    ret = syscall(SYS_perf_event_open, hw_event, pid, cpu, group_fd, flags);
    return ret;
}

struct InstructionNumberReader
{
    size_t size;
    int *cpus; // fd
};

struct InstructionNumberReader *createInstructionNumberReader(const int *Cpus, size_t numCpus, pid_t pid)
{
    struct InstructionNumberReader *reader = malloc(sizeof(struct InstructionNumberReader));
    if (reader == NULL)
    {
        return NULL;
    }

    // Configure the perf_event_attr structure
    struct perf_event_attr pe;
    memset(&pe, 0, sizeof(struct perf_event_attr));
    pe.type = PERF_TYPE_HARDWARE;
    pe.size = sizeof(struct perf_event_attr);
    pe.config = PERF_COUNT_HW_INSTRUCTIONS;
    pe.disabled = 1;

    // Create perf events
    reader->cpus = malloc(numCpus * sizeof(int));
    if (reader->cpus == NULL)
    {
        free(reader);
        reader = NULL;
        return NULL;
    }
    reader->size = numCpus;

    for (size_t i = 0; i < numCpus; i++)
    {
        int fd = perf_event_open(&pe, pid, Cpus[i], -1, 0);
        if (fd == -1)
        {
            for (size_t j = 0; j < i; j++)
            {
                close(reader->cpus[j]);
            }
            free(reader->cpus);
            free(reader);
            reader = NULL;
            return NULL;
        }
        reader->cpus[i] = fd;
    }

    return reader;
}

void destroyInstructionNumberReader(struct InstructionNumberReader *reader)
{
    if (reader == NULL)
    {
        return;
    }

    for (size_t i = 0; i < reader->size; i++)
    {
        close(reader->cpus[i]);
    }
    free(reader->cpus);
    free(reader);
    
    reader = NULL;
}

void enableInstructionNumberReader(struct InstructionNumberReader *reader)
{
    for (size_t i = 0; i < reader->size; i++)
    {
        int fd = reader->cpus[i];
        ioctl(fd, PERF_EVENT_IOC_RESET, 0);
        ioctl(fd, PERF_EVENT_IOC_ENABLE, 0);
    }
}

void disableInstructionNumberReader(struct InstructionNumberReader *reader)
{
    if (reader->cpus == NULL)
        return;

    for (size_t i = 0; i < reader->size; i++)
        ioctl(reader->cpus[i], PERF_EVENT_IOC_DISABLE, 0);
}

long long readInstructionNumberReader(struct InstructionNumberReader *reader, int Cpu)
{
    long long instructions = 0;
    if (read(reader->cpus[Cpu], &instructions, sizeof(long long)) == -1)
        return -1;
    else
        return instructions;
}
