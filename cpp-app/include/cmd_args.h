#ifndef CMD_ARGS_H
#define CMD_ARGS_H

#include <thread>
#include <cstdint>
#include <mutex>
#include <condition_variable>
#include <pthread.h>
#include <cstdio>
#include <cstdlib>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

// For error output in parse_args
#include <iostream>

typedef struct log_node {
    char* message;
    struct log_node* next;
} log_node_t;

typedef struct {
    bool run_modoe;
    bool show_help;
    bool llm_mode;
    char bench_mode;
} CmdArgs;

static int log_thread_running = 1;
static pthread_t log_thread_id;
static log_node_t* log_head = NULL;
static log_node_t* log_tail = NULL;
static pthread_cond_t log_cv = PTHREAD_COND_INITIALIZER;
static pthread_mutex_t log_mutex = PTHREAD_MUTEX_INITIALIZER;
#define RESOURCE_PATH "./resources"

static CmdArgs parse_args(int argc, char* argv[]) {
    CmdArgs args = {false, false, false, 0};
    for (int i = 1; i < argc; ++i) {
        if (strcmp(argv[i], "--run") == 0 || strcmp(argv[i], "-r") == 0) {
            args.run_modoe = true;
        } else if (strcmp(argv[i], "--bench") == 0 || strcmp(argv[i], "-b") == 0) {
            if (i + 1 < argc) {
                args.bench_mode = argv[++i][0];
            } else {
                std::cerr << "Error: --bench requires a mode argument." << std::endl;
                args.show_help = true;
            }
        } else if (strcmp(argv[i], "llm") == 0 || strcmp(argv[i], "--llm") == 0) {
            args.llm_mode = true;
        } else if (strcmp(argv[i], "--help") == 0 || strcmp(argv[i], "-h") == 0 || strcmp(argv[i], "/?") == 0) {
            args.show_help = true;
        } else {
            std::cerr << "Unknown argument: " << argv[i] << std::endl;
            args.show_help = true;
        }
    }
    return args;
}

static void ensure_directory_exists(const char* dirPath) {
    struct stat st = {0};
    if (stat(dirPath, &st) == -1) {
        if (mkdir(dirPath, 0700) != 0) {
            fprintf(stderr, "[LOGGER] Directory create failed: %s\n", dirPath);
        }
    }
}
static void* log_thread_func(void* arg) {
    while (log_thread_running) {
        pthread_mutex_lock(&log_mutex);
        while (log_head == NULL && log_thread_running)
            pthread_cond_wait(&log_cv, &log_mutex);

        while (log_head) {
            log_node_t* node = log_head;
            log_head = node->next;
            if (!log_head) log_tail = NULL;
            pthread_mutex_unlock(&log_mutex);

            // 파일 이름 결정
            const char* level_start = strchr(node->message, '[') + 1;
            const char* level_end = strchr(level_start, ']');
            char level[16] = {0};
            strncpy(level, level_start, level_end - level_start);

            char filepath[256];
            ensure_directory_exists(RESOURCE_PATH);
            snprintf(filepath, sizeof(filepath), "%s/%s.log", RESOURCE_PATH, level);

            FILE* fp = fopen(filepath, "a");
            if (fp) {
                fputs(node->message, fp);
                fclose(fp);
            }

            free(node->message);
            free(node);
            pthread_mutex_lock(&log_mutex);
        }
        pthread_mutex_unlock(&log_mutex);
    }
    return NULL;
}

static void start_log_thread() { pthread_create(&log_thread_id, nullptr, log_thread_func, nullptr); }
static void stop_log_thread() {
    pthread_mutex_lock(&log_mutex);
    log_thread_running = 0;
    pthread_cond_signal(&log_cv);
    pthread_mutex_unlock(&log_mutex);
    pthread_join(log_thread_id, nullptr);
}
static void log_message(const char* format, ...) {  
    va_list args;
    va_start(args, format);
    char* message = (char*)malloc(1024);
    vsnprintf(message, 1024, format, args);
    va_end(args);

    log_node_t* node = (log_node_t*)malloc(sizeof(log_node_t));
    node->message = message;
    node->next = NULL;

    pthread_mutex_lock(&log_mutex);
    if (log_tail) {
        log_tail->next = node;
        log_tail = node;
    } else {
        log_head = log_tail = node;
    }
    pthread_cond_signal(&log_cv);
    pthread_mutex_unlock(&log_mutex);
}

#endif // CMD_ARGS_H